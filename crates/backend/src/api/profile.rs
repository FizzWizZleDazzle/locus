//! Public profile endpoint

use axum::{Json, extract::{Path, State}};
use chrono::Utc;

use locus_common::{
    DailyActivityDay, DailyActivityResponse, DailyDayStatus, PublicProfileResponse,
    TopicStatsEntry, badges::compute_all_badges,
};

use super::AppState;
use crate::{
    AppError,
    models::{
        User,
        daily_puzzle::{DailyPuzzleAttempt, get_daily_puzzle_streak},
    },
};

/// GET /profile/{username} — public profile, no auth required
pub async fn get_public_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<PublicProfileResponse>, AppError> {
    let user = User::find_by_username(&state.pool, &username)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Per-topic stats (same query as stats.rs)
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

    let topics: Vec<TopicStatsEntry> = topic_rows
        .iter()
        .map(
            |(topic, total, correct, elo, peak_elo, topic_streak, peak_topic_streak)| {
                TopicStatsEntry {
                    topic: topic.clone(),
                    total: *total,
                    correct: *correct,
                    elo: *elo,
                    peak_elo: *peak_elo,
                    topic_streak: *topic_streak,
                    peak_topic_streak: *peak_topic_streak,
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

    // Daily puzzle streak
    let daily_puzzle_streak = get_daily_puzzle_streak(&state.pool, user.id).await?;

    let badges = compute_all_badges(
        global_row.2,
        daily_puzzle_streak,
        global_row.1,
        global_row.0,
        &topics,
    );

    // Activity matrix (same pattern as daily.rs)
    let activity_rows = DailyPuzzleAttempt::get_activity_matrix(&state.pool, user.id).await?;

    let mut puzzle_dates = std::collections::HashMap::new();
    for row in &activity_rows {
        if let Some(date) = row.puzzle_date {
            let status = match row.status.as_str() {
                "solved_same_day" => DailyDayStatus::SolvedSameDay,
                "solved_late" => DailyDayStatus::SolvedLate,
                _ => DailyDayStatus::Missed,
            };
            puzzle_dates.insert(date, status);
        }
    }

    let today = Utc::now().date_naive();
    let mut days = Vec::with_capacity(365);
    for i in (0..365).rev() {
        let date = today - chrono::Duration::days(i);
        let status = puzzle_dates
            .remove(&date)
            .unwrap_or(DailyDayStatus::NoPuzzle);
        days.push(DailyActivityDay { date, status });
    }

    Ok(Json(PublicProfileResponse {
        username: user.username,
        member_since: user.created_at,
        badges,
        topics,
        total_attempts: global_row.0,
        correct_attempts: global_row.1,
        current_streak: global_row.2,
        daily_puzzle_streak,
        activity: DailyActivityResponse {
            streak: daily_puzzle_streak,
            days,
        },
    }))
}
