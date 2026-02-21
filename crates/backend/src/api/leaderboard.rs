//! Leaderboard endpoint

use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use locus_common::{LeaderboardEntry, LeaderboardResponse};

use super::AppState;
use crate::{AppError, models::User};

#[derive(Debug, Deserialize)]
pub struct LeaderboardQuery {
    /// Topic to filter by (defaults to "calculus")
    #[serde(default = "default_topic")]
    pub topic: String,
}

fn default_topic() -> String {
    "calculus".to_string()
}

/// Get the leaderboard (top 100 users by ELO for a specific topic)
pub async fn get_leaderboard(
    State(state): State<AppState>,
    Query(query): Query<LeaderboardQuery>,
) -> Result<Json<LeaderboardResponse>, AppError> {
    let rows = User::leaderboard(&state.pool, &query.topic, 100).await?;

    let entries = rows
        .into_iter()
        .map(|row| LeaderboardEntry {
            rank: row.rank as i32,
            username: row.username,
            elo: row.elo,
        })
        .collect();

    Ok(Json(LeaderboardResponse {
        entries,
        topic: query.topic,
    }))
}
