//! API routes and handlers

mod auth;
mod problems;
mod submit;
mod leaderboard;
mod factory;
mod oauth;
mod topics;

use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
};
use sqlx::PgPool;

use locus_common::ApiError;
use crate::rate_limit;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub api_key: String,
    pub http_client: reqwest::Client,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub oauth_redirect_base: String,
    pub topic_cache: crate::topics::TopicCache,
    pub email_service: crate::email::EmailService,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        jwt_secret: String,
        api_key: String,
        http_client: reqwest::Client,
        google_client_id: Option<String>,
        google_client_secret: Option<String>,
        github_client_id: Option<String>,
        github_client_secret: Option<String>,
        oauth_redirect_base: String,
        topic_cache: crate::topics::TopicCache,
        email_service: crate::email::EmailService,
    ) -> Self {
        Self {
            pool,
            jwt_secret,
            api_key,
            http_client,
            google_client_id,
            google_client_secret,
            github_client_id,
            github_client_secret,
            oauth_redirect_base,
            topic_cache,
            email_service,
        }
    }
}

/// Build the API router
pub fn router() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(health))
        // Auth routes (with specific rate limiting)
        .route("/auth/register", post(auth::register))
            .layer(rate_limit::auth_rate_limiter())
        .route("/auth/login", post(auth::login))
            .layer(rate_limit::login_rate_limiter())
        .route("/auth/set-password", post(auth::set_password))
        .route("/auth/change-password", post(auth::change_password))
        .route("/auth/change-username", post(auth::change_username))
        .route("/auth/delete-account", post(auth::delete_account))
        .route("/auth/unlink-oauth", post(auth::unlink_oauth))
        .route("/auth/verify-email", post(auth::verify_email))
        .route("/auth/resend-verification", post(auth::resend_verification))
        .route("/auth/forgot-password", post(auth::forgot_password))
        .route("/auth/validate-reset-token", post(auth::validate_reset_token))
        .route("/auth/reset-password", post(auth::reset_password))
        // OAuth routes
        .route("/auth/oauth/{provider}", get(oauth::oauth_redirect))
        .route("/auth/oauth/{provider}/callback", get(oauth::oauth_callback))
        // Problem routes
        .route("/problem", get(problems::get_problem))
        // Topics
        .route("/topics", get(topics::get_topics))
        // Submit route
        .route("/submit", post(submit::submit_answer))
        // Leaderboard
        .route("/leaderboard", get(leaderboard::get_leaderboard))
        // User profile
        .route("/user/me", get(auth::get_me))
        // Factory endpoint (internal)
        .route("/internal/problems", post(factory::create_problem))
}

/// Health check endpoint
async fn health() -> &'static str {
    "OK"
}

/// Convert AppError to HTTP response
impl axum::response::IntoResponse for crate::AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            crate::AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            crate::AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            crate::AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            crate::AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            crate::AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
            }
        };

        (status, Json(ApiError::new(message))).into_response()
    }
}
