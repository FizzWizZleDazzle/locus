pub mod handlers;
pub mod models;
pub mod monitor;

use crate::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/current", get(handlers::current))
        .route("/history", get(handlers::history))
        .route("/uptime", get(handlers::uptime))
}
