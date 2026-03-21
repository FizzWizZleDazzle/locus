pub mod handlers;
pub mod models;
pub mod monitor;

use axum::{Router, routing::get};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/current", get(handlers::current))
        .route("/history", get(handlers::history))
        .route("/uptime", get(handlers::uptime))
}
