//! User stats endpoints

use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use locus_common::{EloHistoryPoint, EloHistoryResponse, TopicStatsEntry, UserStatsResponse};

use super::AppState;
use crate::{AppError, auth::AuthUser};

pub async fn get_user_stats(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<UserStatsResponse>, AppError> {
    // Per-topic stats
    let topic_rows: Vec<(String, i64, i64, i32, i32, i32, i32)> = sqlx::query_as(
        r#"
        SELECT
          a.main_topic,
          COUNT(*) as total,
          COUNT(*) FILTER (WHERE a.is_correct) as correct,
          ute.elo,
          ute.peak_elo,
          ute.topic_streak,
          ute.peak_topic_streak
        FROM attempts a
        JOIN user_topic_elo ute ON ute.user_id = a.user_id AND ute.topic = a.main_topic
        WHERE a.user_id = $1
        GROUP BY a.main_topic, ute.user_id, ute.topic
        ORDER BY a.main_topic
        "#,
    )
    .bind(user.id)
    .fetch_all(&state.pool)
    .await?;

    let topics = topic_rows
        .into_iter()
        .map(
            |(topic, total, correct, elo, peak_elo, topic_streak, peak_topic_streak)| {
                TopicStatsEntry {
                    topic,
                    total,
                    correct,
                    elo,
                    peak_elo,
                    topic_streak,
                    peak_topic_streak,
                }
            },
        )
        .collect();

    // Global stats
    let global_row: (i64, i64, i32) = sqlx::query_as(
        r#"
        SELECT
          COUNT(*) as total_attempts,
          COUNT(*) FILTER (WHERE a.is_correct) as correct_attempts,
          COALESCE(MAX(u.current_streak), 0)
        FROM attempts a
        JOIN users u ON u.id = a.user_id
        WHERE a.user_id = $1
        "#,
    )
    .bind(user.id)
    .fetch_optional(&state.pool)
    .await?
    .unwrap_or((0, 0, 0));

    Ok(Json(UserStatsResponse {
        total_attempts: global_row.0,
        correct_attempts: global_row.1,
        current_streak: global_row.2,
        topics,
    }))
}

#[derive(Deserialize)]
pub struct EloHistoryQuery {
    pub topic: String,
}

pub async fn get_elo_history(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<EloHistoryQuery>,
) -> Result<Json<EloHistoryResponse>, AppError> {
    let rows: Vec<(chrono::NaiveDate, i32)> = sqlx::query_as(
        r#"
        SELECT DISTINCT ON (DATE(created_at))
          DATE(created_at) as day,
          elo_after as elo
        FROM attempts
        WHERE user_id = $1
          AND main_topic = $2
          AND created_at > NOW() - INTERVAL '30 days'
        ORDER BY DATE(created_at) DESC, created_at DESC
        "#,
    )
    .bind(user.id)
    .bind(&params.topic)
    .fetch_all(&state.pool)
    .await?;

    let history = rows
        .into_iter()
        .map(|(day, elo)| EloHistoryPoint { day, elo })
        .collect();

    Ok(Json(EloHistoryResponse {
        topic: params.topic,
        history,
    }))
}
