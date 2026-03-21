pub mod handlers;
pub mod models;

use axum::{Router, routing::{get, post, patch, delete}};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/posts", get(handlers::list_posts).post(handlers::create_post))
        .route("/posts/{id}", get(handlers::get_post))
        .route("/posts/{id}/comment", post(handlers::add_comment))
        .route("/posts/{id}/vote", post(handlers::toggle_vote))
        .route("/posts/{id}/status", patch(handlers::update_status))
        .route("/posts/{id}/pin", patch(handlers::toggle_pin))
        .route("/posts/{id}/lock", patch(handlers::toggle_lock))
        .route("/comments/{id}", delete(handlers::delete_comment))
}
