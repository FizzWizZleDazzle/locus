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

    // Grade the answer (offload to blocking thread — SymEngine uses a global mutex)
    let user_input = req.user_input.clone();
    let answer_key = problem.answer_key.clone();
    let grading_mode = problem.get_grading_mode();
    let answer_type = problem.get_answer_type();
    let is_correct = tokio::task::spawn_blocking(move || {
        check_answer(&user_input, &answer_key, grading_mode, answer_type)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Grading task failed: {}", e)))?;

    // Calculate new ELO
    let elo_after = calculate_new_elo(
        elo_before,
        problem.difficulty,
        is_correct,
        req.time_taken_ms,
        problem.time_limit_seconds,
    );
    let elo_change = elo_after - elo_before;

    // Wrap all writes in a transaction to prevent race conditions
    let mut tx = state.pool.begin().await?;

    // Update user's ELO and per-topic streak
    let topic_streak = User::update_elo_and_streaks(
        &mut *tx,
        user.id,
        &problem.main_topic,
        elo_after,
        is_correct,
    )
    .await?;

    // Update global daily streak on correct answer
    if is_correct {
        User::update_daily_streak(&mut *tx, user.id).await?;
    }

    // Record the attempt
    Attempt::create(
        &mut *tx,
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

    tx.commit().await?;

    Ok(Json(SubmitResponse {
        is_correct,
        elo_before,
        elo_after,
        elo_change,
        topic_streak,
        answer_key: if !is_correct {
            Some(problem.answer_key.clone())
        } else {
            None
        },
        solution_latex: if !is_correct && !problem.solution_latex.is_empty() {
            Some(problem.solution_latex.clone())
        } else {
            None
        },
    }))
}
