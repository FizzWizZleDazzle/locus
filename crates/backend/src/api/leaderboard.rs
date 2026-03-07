//! Leaderboard endpoint

use std::time::{Duration, Instant};

use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use locus_common::{LeaderboardEntry, LeaderboardResponse};

use super::AppState;
use crate::{AppError, models::User};

/// Cache TTL: 5 minutes
const LEADERBOARD_CACHE_TTL: Duration = Duration::from_secs(5 * 60);

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
    // Check cache
    {
        let cache = state.leaderboard_cache.read().await;
        if let Some((cached_at, entries)) = cache.get(&query.topic) {
            if cached_at.elapsed() < LEADERBOARD_CACHE_TTL {
                return Ok(Json(LeaderboardResponse {
                    entries: entries.clone(),
                    topic: query.topic,
                }));
            }
        }
    }

    // Cache miss or expired — query DB
    let rows = User::leaderboard(&state.pool, &query.topic, 100).await?;

    let entries: Vec<LeaderboardEntry> = rows
        .into_iter()
        .map(|row| LeaderboardEntry {
            rank: row.rank as i32,
            username: row.username,
            elo: row.elo,
        })
        .collect();

    // Update cache
    {
        let mut cache = state.leaderboard_cache.write().await;
        cache.insert(query.topic.clone(), (Instant::now(), entries.clone()));
    }

    Ok(Json(LeaderboardResponse {
        entries,
        topic: query.topic,
    }))
}
