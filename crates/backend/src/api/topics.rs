//! Topics API endpoint

use axum::{
    extract::State,
    Json,
};

use crate::api::AppState;
use crate::topics::Topic;

/// GET /api/topics
/// Returns all enabled topics and subtopics
pub async fn get_topics(
    State(state): State<AppState>,
) -> Json<Vec<Topic>> {
    let topics = state.topic_cache.get_enabled().await;
    Json(topics)
}
