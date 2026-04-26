mod auth;
mod config;
mod status;

use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    Json, Router,
    http::Method,
    middleware,
    routing::{get, post},
};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub is_production: bool,
    pub cookie_domain: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "locus_services_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;

    tracing::info!("Starting Locus services backend");
    tracing::info!("Environment: {:?}", config.environment);

    // Database
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    // Use separate migrations table to avoid conflicts with main backend
    let migrator = sqlx::migrate!("./migrations");
    pool.execute(sqlx::query(
        "CREATE TABLE IF NOT EXISTS _sqlx_migrations_services (
            version BIGINT PRIMARY KEY,
            description TEXT NOT NULL,
            installed_on TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            success BOOLEAN NOT NULL,
            checksum BYTEA NOT NULL,
            execution_time BIGINT NOT NULL
        )",
    ))
    .await?;
    // Run each migration manually against our custom table
    for migration in migrator.iter() {
        let version = migration.version;
        let already_run = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM _sqlx_migrations_services WHERE version = $1 AND success = true)"
        )
        .bind(version)
        .fetch_one(&pool)
        .await?;

        if !already_run {
            tracing::info!("Running migration {}: {}", version, migration.description);
            pool.execute(sqlx::raw_sql(migration.sql.as_ref())).await?;
            sqlx::query(
                "INSERT INTO _sqlx_migrations_services (version, description, success, checksum, execution_time)
                 VALUES ($1, $2, true, $3, 0)
                 ON CONFLICT (version) DO UPDATE SET success = true"
            )
            .bind(version)
            .bind(migration.description.as_ref())
            .bind(migration.checksum.as_ref())
            .execute(&pool)
            .await?;
        }
    }
    tracing::info!("Database migrations completed");

    let is_production = config.is_production();

    // Spawn health checker
    status::monitor::spawn_health_checker(
        pool.clone(),
        config.health_check_url.clone(),
        config.health_check_interval_secs,
    );
    tracing::info!(
        "Health checker started (interval: {}s)",
        config.health_check_interval_secs
    );

    let state = AppState {
        pool,
        jwt_secret: config.jwt_secret,
        is_production,
        cookie_domain: config.cookie_domain,
    };

    // CORS
    let allowed_origins: Vec<_> = config
        .allowed_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    let app = Router::new()
        .route("/health", get(health))
        // Auth routes (login handled by main app; shared cookie read here)
        .route("/api/auth/logout", post(auth_logout))
        .route("/api/auth/me", get(auth_me))
        // Status routes
        .nest("/api/status", status::router())
        .layer(middleware::from_fn(security_headers))
        .layer(
            CorsLayer::new()
                .allow_origin(allowed_origins)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::COOKIE])
                .allow_credentials(true),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}

// Health endpoint
async fn health() -> &'static str {
    "ok"
}

// Auth handlers

#[derive(Serialize)]
struct UserInfo {
    id: uuid::Uuid,
    username: String,
}

async fn auth_logout(State(state): State<AppState>) -> axum::response::Response {
    use axum::response::IntoResponse;

    let cookie = auth::build_clear_cookie(state.is_production, state.cookie_domain.as_deref());
    let mut response = StatusCode::OK.into_response();
    response
        .headers_mut()
        .insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());
    response
}

async fn auth_me(user: auth::AuthUser) -> Json<UserInfo> {
    Json(UserInfo {
        id: user.id,
        username: user.username,
    })
}

// Security headers middleware
async fn security_headers(
    req: axum::extract::Request,
    next: middleware::Next,
) -> axum::response::Response {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.insert("x-content-type-options", "nosniff".parse().unwrap());
    headers.insert("x-frame-options", "DENY".parse().unwrap());
    headers.insert(
        "referrer-policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    res
}
