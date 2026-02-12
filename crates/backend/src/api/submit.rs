//! Answer submission endpoint

use axum::{extract::State, Json};

use locus_common::{SubmitRequest, SubmitResponse};

use crate::{
    auth::AuthUser,
    grader::check_answer,
    models::{Attempt, Problem, User},
    AppError,
};

use locus_common::elo::calculate_new_elo;
use super::AppState;

/// Submit an answer for grading
///
/// This is used for ranked mode. The answer is verified server-side
/// and the user's ELO is updated accordingly.
pub async fn submit_answer(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>, AppError> {
    // Get the problem
    let problem = Problem::find_by_id(&state.pool, req.problem_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Problem not found".into()))?;

    // Get current user's ELO for this specific topic
    let elo_before = User::get_elo_for_topic(&state.pool, user.id, &problem.main_topic).await?;

    // Grade the answer
    let is_correct = check_answer(&req.user_input, &problem.answer_key, problem.get_grading_mode());

    // Calculate new ELO
    let elo_after = calculate_new_elo(elo_before, problem.difficulty, is_correct, req.time_taken_ms);
    let elo_change = elo_after - elo_before;

    // Update user's ELO for this topic
    User::update_elo_for_topic(&state.pool, user.id, &problem.main_topic, elo_after).await?;

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
    }))
}
