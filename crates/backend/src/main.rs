//! Locus Backend Server

mod api;
mod auth;
mod config;
mod db;
mod email;
mod grader;
mod models;
mod rate_limit;
mod topics;

use axum::{Router, http::Method, middleware, routing::get};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "locus_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::from_env()?;

    // Log startup information
    tracing::info!("Starting Locus backend");
    tracing::info!("Environment: {:?}", config.environment);
    tracing::info!("Allowed CORS origins: {:?}", config.allowed_origins);
    // Connect to database
    tracing::info!("DB pool max_connections: {}", config.max_db_connections);
    let pool = db::create_pool(&config.database_url, config.max_db_connections).await?;

    // Run migrations (unless SKIP_MIGRATIONS=true)
    if std::env::var("SKIP_MIGRATIONS").unwrap_or_default() != "true" {
        sqlx::migrate!("./migrations").run(&pool).await?;
        tracing::info!("Database migrations completed");
    } else {
        tracing::warn!("Skipping migrations (SKIP_MIGRATIONS=true)");
    }

    // Initialize topic cache
    let topic_cache = topics::TopicCache::new(pool.clone()).await?;
    tracing::info!("Topic cache initialized");

    // Spawn background refresh task
    topic_cache.clone().spawn_refresh_task();
    tracing::info!("Topic cache refresh task started");

    // Build HTTP client for OAuth
    let http_client = reqwest::Client::new();

    // Initialize email service
    let email_service = email::EmailService::new(
        config.resend_api_key.clone(),
        config.resend_from_email.clone(),
        config.resend_from_name.clone(),
        config.frontend_base_url.clone(),
    );
    tracing::info!("Email service initialized");

    // Log OAuth configuration
    if config.google_client_id.is_some() {
        tracing::info!("Google OAuth configured");
    }
    if config.github_client_id.is_some() {
        tracing::info!("GitHub OAuth configured");
    }

    let is_production = config.environment == config::Environment::Production;

    // Build application state
    let state = api::AppState::new(
        pool,
        config.jwt_secret.clone(),
        http_client,
        config.google_client_id.clone(),
        config.google_client_secret.clone(),
        config.github_client_id.clone(),
        config.github_client_secret.clone(),
        config.oauth_redirect_base.clone(),
        config.frontend_base_url.clone(),
        topic_cache,
        email_service,
        is_production,
    );

    // Parse allowed origins for CORS
    let allowed_origins: Vec<_> = config
        .allowed_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    // Build router
    let app = Router::new()
        // Health endpoint (no rate limiting for Kubernetes probes)
        .route("/api/health", get(api::health))
        // API routes (with rate limiting)
        .nest(
            "/api",
            api::router().layer(rate_limit::general_rate_limiter()),
        )
        .layer(middleware::from_fn(security_headers))
        .layer(
            CorsLayer::new()
                .allow_origin(allowed_origins)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_TYPE,
                ])
                .allow_credentials(true),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);

    // Use into_make_service_with_connect_info to enable IP tracking for rate limiting
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}

/// Security headers middleware
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
    headers.insert("x-xss-protection", "0".parse().unwrap());
    res
}

// Re-export for convenience
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
