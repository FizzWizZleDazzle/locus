//! API routes and handlers

mod auth;
mod problems;
mod submit;
mod leaderboard;
mod factory;

use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
};
use sqlx::PgPool;
use std::sync::Arc;

use locus_common::ApiError;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub api_key: String,
}

impl AppState {
    pub fn new(pool: PgPool, jwt_secret: String, api_key: String) -> Self {
        Self { pool, jwt_secret, api_key }
    }
}

/// Build the API router
pub fn router() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(health))
        // Auth routes
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        // Problem routes
        .route("/problem", get(problems::get_problem))
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
