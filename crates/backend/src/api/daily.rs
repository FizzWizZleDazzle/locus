//! Daily puzzle endpoints

use axum::{Json, extract::{Path, Query, State}};
use chrono::{NaiveDate, Utc};

use locus_common::{
    DailyActivityDay, DailyActivityResponse, DailyArchiveEntry, DailyArchiveQuery,
    DailyDayStatus, DailyPuzzleDetailResponse, DailyPuzzleResponse, DailyPuzzleStats,
    DailySubmitRequest, DailySubmitResponse, DailyUserStatus,
};

use crate::{
    AppError,
    auth::AuthUser,
    grader::check_answer,
    models::{
        Problem,
        daily_puzzle::{
            DailyPuzzle, DailyPuzzleAttempt, get_daily_puzzle_streak, update_daily_puzzle_streak,
        },
    },
};

use super::AppState;

/// GET /daily/today - Get today's puzzle (cached)
pub async fn get_today(
    State(state): State<AppState>,
    user: Option<AuthUser>,
) -> Result<Json<DailyPuzzleResponse>, AppError> {
    let today = Utc::now().date_naive();

    // Try cache first
    let (dp, problem) = {
        let cache = state.daily_puzzle_cache.read().await;
        match &*cache {
            Some((cached_date, dp, problem)) if *cached_date == today => {
                (dp.clone(), problem.clone())
            }
            _ => {
                drop(cache);
                // Cache miss — query DB
                let dp = DailyPuzzle::get_by_date(&state.pool, today)
                    .await?
                    .ok_or_else(|| AppError::NotFound("No puzzle scheduled for today".into()))?;
                let problem = Problem::find_by_id(&state.pool, dp.problem_id)
                    .await?
                    .ok_or_else(|| {
                        AppError::Internal("Daily puzzle references missing problem".into())
                    })?;
                // Update cache
                let mut cache = state.daily_puzzle_cache.write().await;
                *cache = Some((today, dp.clone(), problem.clone()));
                (dp, problem)
            }
        }
    };

    let puzzle_date = dp.puzzle_date.unwrap_or(today);
    let hints = dp.hints_vec();

    let user_status = if let Some(ref u) = user {
        build_user_status(&state.pool, u.id, dp.id, puzzle_date).await?
    } else {
        None
    };

    Ok(Json(DailyPuzzleResponse {
        id: dp.id,
        puzzle_date,
        title: dp.title,
        problem: problem.to_response(false), // no answer for active puzzle
        hints_available: hints.len(),
        source: dp.source,
        user_status,
    }))
}

/// GET /daily/puzzle/{date} - Get a past puzzle with full details
pub async fn get_puzzle_by_date(
    State(state): State<AppState>,
    Path(date_str): Path<String>,
    user: Option<AuthUser>,
) -> Result<Json<DailyPuzzleDetailResponse>, AppError> {
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid date format, use YYYY-MM-DD".into()))?;

    let today = Utc::now().date_naive();
    if date > today {
        return Err(AppError::NotFound("Cannot view future puzzles".into()));
    }

    let dp = DailyPuzzle::get_by_date(&state.pool, date)
        .await?
        .ok_or_else(|| AppError::NotFound("No puzzle for this date".into()))?;

    let problem = Problem::find_by_id(&state.pool, dp.problem_id)
        .await?
        .ok_or_else(|| AppError::Internal("Daily puzzle references missing problem".into()))?;

    let puzzle_date = dp.puzzle_date.unwrap_or(date);
    let is_past = date < today;

    let stats_row = DailyPuzzleAttempt::get_puzzle_stats(&state.pool, dp.id).await?;

    let user_status = if let Some(ref u) = user {
        build_user_status(&state.pool, u.id, dp.id, puzzle_date).await?
    } else {
        None
    };

    let hints = dp.hints_vec();
    Ok(Json(DailyPuzzleDetailResponse {
        id: dp.id,
        puzzle_date,
        title: dp.title,
        problem: problem.to_response(is_past), // include answer for past puzzles
        editorial_latex: if is_past { dp.editorial_latex } else { String::new() },
        hints,
        source: dp.source,
        stats: DailyPuzzleStats {
            total_attempts: stats_row.total_attempts,
            total_solves: stats_row.total_solves,
            solve_rate: stats_row.solve_rate,
        },
        user_status,
    }))
}

/// POST /daily/submit - Submit an answer for a daily puzzle
pub async fn submit_daily(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<DailySubmitRequest>,
) -> Result<Json<DailySubmitResponse>, AppError> {
    if req.user_input.len() > 10_000 {
        return Err(AppError::BadRequest("Input too long".into()));
    }

    // Get the daily puzzle
    let dp = sqlx::query_as::<_, DailyPuzzle>(
        r#"
        SELECT id, problem_id, puzzle_date, title, hints, editorial_latex, source, status, created_at
        FROM daily_puzzles WHERE id = $1
        "#,
    )
    .bind(req.daily_puzzle_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Daily puzzle not found".into()))?;

    // Get the underlying problem for grading
    let problem = Problem::find_by_id(&state.pool, dp.problem_id)
        .await?
        .ok_or_else(|| AppError::Internal("Daily puzzle references missing problem".into()))?;

    // Grade the answer (offload to blocking thread — SymEngine uses a global mutex)
    let user_input = req.user_input.clone();
    let answer_key = problem.answer_key.clone();
    let grading_mode = problem.get_grading_mode();
    let answer_type = problem.get_answer_type();
    let is_correct = tokio::task::spawn_blocking(move || {
        check_answer(&user_input, &answer_key, grading_mode, answer_type)
    })
    .await
    .map_err(|e| crate::AppError::Internal(format!("Grading task failed: {}", e)))?;

    // Reject post-solve submissions to prevent attempt count inflation
    let puzzle_date = dp.puzzle_date.unwrap_or_else(|| Utc::now().date_naive());
    let existing = DailyPuzzleAttempt::get_user_status(&state.pool, user.id, dp.id, puzzle_date).await?;
    if let Some(ref s) = existing {
        if s.solved.unwrap_or(false) {
            let streak = get_daily_puzzle_streak(&state.pool, user.id).await?;
            return Ok(Json(DailySubmitResponse {
                is_correct,
                attempts: s.attempts,
                solved: true,
                streak,
            }));
        }
    }

    // Wrap attempt + streak update in a transaction
    let mut tx = state.pool.begin().await?;

    // Record the attempt
    DailyPuzzleAttempt::create(
        &mut *tx,
        user.id,
        dp.id,
        &req.user_input,
        is_correct,
        req.hints_used,
        req.time_taken_ms,
    )
    .await?;

    // Update streak on correct answer (only for same-day solves)
    let today = Utc::now().date_naive();
    let mut streak = get_daily_puzzle_streak(&state.pool, user.id).await?;

    if is_correct && puzzle_date == today {
        streak = update_daily_puzzle_streak(&mut *tx, user.id).await?;
    }

    tx.commit().await?;

    // Check if user has now solved this puzzle
    let status = DailyPuzzleAttempt::get_user_status(&state.pool, user.id, dp.id, puzzle_date)
        .await?;
    let solved = status.as_ref().map(|s| s.solved.unwrap_or(false)).unwrap_or(false);
    let attempts = status.as_ref().map(|s| s.attempts).unwrap_or(1);

    Ok(Json(DailySubmitResponse {
        is_correct,
        attempts,
        solved,
        streak,
    }))
}

/// GET /daily/archive - Paginated list of past puzzles
pub async fn get_archive(
    State(state): State<AppState>,
    Query(query): Query<DailyArchiveQuery>,
    user: Option<AuthUser>,
) -> Result<Json<Vec<DailyArchiveEntry>>, AppError> {
    let limit = query.limit.max(1).min(100);
    let offset = query.offset.max(0);

    let entries = if let Some(ref u) = user {
        let rows = DailyPuzzle::get_archive_with_user(&state.pool, u.id, limit, offset).await?;
        rows.into_iter()
            .map(|r| DailyArchiveEntry {
                puzzle_date: r.puzzle_date.unwrap_or_default(),
                title: r.title,
                difficulty: r.difficulty,
                main_topic: r.main_topic,
                solve_rate: r.solve_rate,
                user_solved: r.user_solved,
                user_solved_same_day: r.user_solved_same_day,
            })
            .collect()
    } else {
        let rows = DailyPuzzle::get_archive(&state.pool, limit, offset).await?;
        rows.into_iter()
            .map(|r| DailyArchiveEntry {
                puzzle_date: r.puzzle_date.unwrap_or_default(),
                title: r.title,
                difficulty: r.difficulty,
                main_topic: r.main_topic,
                solve_rate: r.solve_rate,
                user_solved: None,
                user_solved_same_day: None,
            })
            .collect()
    };

    Ok(Json(entries))
}

/// GET /daily/activity - Activity matrix for last 365 days
pub async fn get_activity(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<DailyActivityResponse>, AppError> {
    let streak = get_daily_puzzle_streak(&state.pool, user.id).await?;
    let rows = DailyPuzzleAttempt::get_activity_matrix(&state.pool, user.id).await?;

    // Build a set of dates with puzzle data
    let mut puzzle_dates = std::collections::HashMap::new();
    for row in &rows {
        if let Some(date) = row.puzzle_date {
            let status = match row.status.as_str() {
                "solved_same_day" => DailyDayStatus::SolvedSameDay,
                "solved_late" => DailyDayStatus::SolvedLate,
                _ => DailyDayStatus::Missed,
            };
            puzzle_dates.insert(date, status);
        }
    }

    // Build 365 days of activity data
    let today = Utc::now().date_naive();
    let mut days = Vec::with_capacity(365);
    for i in (0..365).rev() {
        let date = today - chrono::Duration::days(i);
        let status = puzzle_dates
            .remove(&date)
            .unwrap_or(DailyDayStatus::NoPuzzle);
        days.push(DailyActivityDay { date, status });
    }

    Ok(Json(DailyActivityResponse { streak, days }))
}

/// Helper: build DailyUserStatus for a user + puzzle
async fn build_user_status(
    pool: &PgPool,
    user_id: uuid::Uuid,
    daily_puzzle_id: uuid::Uuid,
    puzzle_date: NaiveDate,
) -> Result<Option<DailyUserStatus>, AppError> {
    let streak = get_daily_puzzle_streak(pool, user_id).await?;

    let status =
        DailyPuzzleAttempt::get_user_status(pool, user_id, daily_puzzle_id, puzzle_date).await?;

    Ok(Some(match status {
        Some(s) => DailyUserStatus {
            solved: s.solved.unwrap_or(false),
            solved_same_day: s.solved_same_day.unwrap_or(false),
            attempts: s.attempts,
            hints_revealed: s.max_hints_used.unwrap_or(0),
            streak,
        },
        None => DailyUserStatus {
            solved: false,
            solved_same_day: false,
            attempts: 0,
            hints_revealed: 0,
            streak,
        },
    }))
}

use sqlx::PgPool;
