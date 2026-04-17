//! API routes and handlers

mod auth;
mod daily;
mod leaderboard;
mod oauth;
mod physics;
mod problems;
mod profile;
mod stats;
mod submit;
mod topics;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::rate_limit;
use locus_common::{ApiError, LeaderboardEntry};

/// Cached leaderboard entry with timestamp
pub type LeaderboardCache = Arc<RwLock<HashMap<String, (Instant, Vec<LeaderboardEntry>)>>>;

/// Cached daily puzzle + problem for today
pub type DailyPuzzleCache = Arc<RwLock<Option<(chrono::NaiveDate, crate::models::daily_puzzle::DailyPuzzle, crate::models::Problem)>>>;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub http_client: reqwest::Client,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub oauth_redirect_base: String,
    pub frontend_base_url: String,
    pub topic_cache: crate::topics::TopicCache,
    pub email_service: crate::email::EmailService,
    pub is_production: bool,
    pub cookie_domain: Option<String>,
    pub leaderboard_cache: LeaderboardCache,
    pub daily_puzzle_cache: DailyPuzzleCache,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        jwt_secret: String,
        http_client: reqwest::Client,
        google_client_id: Option<String>,
        google_client_secret: Option<String>,
        github_client_id: Option<String>,
        github_client_secret: Option<String>,
        oauth_redirect_base: String,
        frontend_base_url: String,
        topic_cache: crate::topics::TopicCache,
        email_service: crate::email::EmailService,
        is_production: bool,
        cookie_domain: Option<String>,
    ) -> Self {
        Self {
            pool,
            jwt_secret,
            http_client,
            google_client_id,
            google_client_secret,
            github_client_id,
            github_client_secret,
            oauth_redirect_base,
            frontend_base_url,
            topic_cache,
            email_service,
            is_production,
            cookie_domain,
            leaderboard_cache: Arc::new(RwLock::new(HashMap::new())),
            daily_puzzle_cache: Arc::new(RwLock::new(None)),
        }
    }
}

/// Build the API router
pub fn router() -> Router<AppState> {
    // Sensitive auth routes with stricter rate limiting (5 per 15 min)
    let sensitive_auth_routes = Router::new()
        .route("/auth/forgot-password", post(auth::forgot_password))
        .route("/auth/resend-verification", post(auth::resend_verification))
        .route("/auth/reset-password", post(auth::reset_password))
        .layer(rate_limit::sensitive_rate_limiter());

    Router::new()
        // Auth routes (with specific rate limiting)
        .route("/auth/register", post(auth::register))
        .layer(rate_limit::auth_rate_limiter())
        .route("/auth/login", post(auth::login))
        .layer(rate_limit::login_rate_limiter())
        .route("/auth/set-password", post(auth::set_password))
        .route("/auth/change-password", post(auth::change_password))
        .route("/auth/change-username", post(auth::change_username))
        .route("/auth/delete-account", post(auth::delete_account))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/unlink-oauth", post(auth::unlink_oauth))
        .route("/auth/verify-email", post(auth::verify_email))
        .route(
            "/auth/validate-reset-token",
            post(auth::validate_reset_token),
        )
        .merge(sensitive_auth_routes)
        // OAuth routes
        .route("/auth/oauth/{provider}", get(oauth::oauth_redirect))
        .route(
            "/auth/oauth/{provider}/callback",
            get(oauth::oauth_callback),
        )
        // OAuth linking routes (requires authentication)
        .route(
            "/auth/oauth/link/{provider}",
            get(oauth::oauth_redirect_link),
        )
        // Problem routes
        .route("/problems", get(problems::get_problems))
        .route("/problem", get(problems::get_problem)) // deprecated
        // Topics
        .route("/topics", get(topics::get_topics))
        // Submit route
        .route("/submit", post(submit::submit_answer))
        // Leaderboard
        .route("/leaderboard", get(leaderboard::get_leaderboard))
        // User profile
        .route("/user/me", get(auth::get_me))
        // User stats
        .route("/user/stats", get(stats::get_user_stats))
        .route("/user/elo-history", get(stats::get_elo_history))
        // Daily puzzle
        .route("/daily/today", get(daily::get_today))
        .route("/daily/puzzle/{date}", get(daily::get_puzzle_by_date))
        .route("/daily/submit", post(daily::submit_daily))
        .route("/daily/archive", get(daily::get_archive))
        .route("/daily/activity", get(daily::get_activity))
        // Public profile
        .route("/profile/{username}", get(profile::get_public_profile))
        // Physics learning platform
        .nest("/physics", physics::router())
}

/// Health check endpoint
pub async fn health() -> &'static str {
    "OK"
}

/// Convert AppError to HTTP response
impl axum::response::IntoResponse for crate::AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            crate::AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            crate::AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            crate::AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            crate::AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            crate::AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal error".to_string(),
                )
            }
        };

        (status, Json(ApiError::new(message))).into_response()
    }
}
