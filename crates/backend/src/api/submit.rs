//! Answer submission endpoint

use axum::{Json, extract::State};

use locus_common::{SubmitRequest, SubmitResponse};

use crate::{
    AppError,
    auth::AuthUser,
    grader::check_answer,
    models::{Attempt, Problem, User},
};

use super::AppState;
use locus_common::elo::calculate_new_elo;

/// Submit an answer for grading
///
/// This is used for ranked mode. The answer is verified server-side
/// and the user's ELO is updated accordingly.
pub async fn submit_answer(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>, AppError> {
    // Reject oversized input to prevent DoS / database bloat
    if req.user_input.len() > 10_000 {
        return Err(AppError::BadRequest("Input too long".into()));
    }

    // Get the problem
    let problem = Problem::find_by_id(&state.pool, req.problem_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Problem not found".into()))?;

    // Get current user's ELO for this specific topic
    let elo_before = User::get_elo_for_topic(&state.pool, user.id, &problem.main_topic).await?;

    // Grade the answer
    let is_correct = check_answer(
        &req.user_input,
        &problem.answer_key,
        problem.get_grading_mode(),
        problem.get_answer_type(),
    );

    // Calculate new ELO
    let elo_after = calculate_new_elo(
        elo_before,
        problem.difficulty,
        is_correct,
        req.time_taken_ms,
        problem.time_limit_seconds,
    );
    let elo_change = elo_after - elo_before;

    // Update user's ELO and per-topic streak
    let topic_streak = User::update_elo_and_streaks(
        &state.pool,
        user.id,
        &problem.main_topic,
        elo_after,
        is_correct,
    )
    .await?;

    // Update global daily streak on correct answer
    if is_correct {
        User::update_daily_streak(&state.pool, user.id).await?;
    }

    // Record the attempt
    Attempt::create(
        &state.pool,
        user.id,
        problem.id,
        &req.user_input,
        is_correct,
        elo_before,
        elo_after,
        req.time_taken_ms,
        &problem.main_topic,
    )
    .await?;

    Ok(Json(SubmitResponse {
        is_correct,
        elo_before,
        elo_after,
        elo_change,
        topic_streak,
    }))
}
