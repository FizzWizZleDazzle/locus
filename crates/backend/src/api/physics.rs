//! Physics learning platform API routes.
//!
//! All routes are nested under `/api/physics/`.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use locus_common::ApiError;
use locus_physics_common::{
    AnswerSpec, PhysicsProgressEntry, PhysicsProgressResponse, PhysicsProblemsQuery,
    PhysicsSubmitRequest, PhysicsSubmitResponse, PhysicsTopicInfo, PhysicsSubtopicInfo,
    scoring::AttemptScore,
};

use crate::api::AppState;
use crate::auth::AuthUser;
use crate::models::{PhysicsAttempt, PhysicsProblem, PhysicsProgress};

/// Build the physics sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/topics", get(get_topics))
        .route("/problems", get(get_problems))
        .route("/problem/{id}", get(get_problem))
        .route("/submit", post(submit_answer))
        .route("/progress", get(get_progress))
}

// ============================================================================
// GET /physics/topics
// ============================================================================

async fn get_topics(
    State(state): State<AppState>,
) -> Result<Json<Vec<PhysicsTopicInfo>>, (StatusCode, Json<ApiError>)> {
    let topics: Vec<(String, String, i32, bool)> = sqlx::query_as(
        "SELECT id, display_name, sort_order, enabled FROM physics_topics ORDER BY sort_order",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch physics topics: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Database error")),
        )
    })?;

    let mut result = Vec::new();
    for (id, display_name, sort_order, enabled) in &topics {
        let subtopics: Vec<(String, String, i32, bool)> = sqlx::query_as(
            "SELECT id, display_name, sort_order, enabled FROM physics_subtopics WHERE topic_id = $1 ORDER BY sort_order",
        )
        .bind(id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch physics subtopics: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new("Database error")),
            )
        })?;

        result.push(PhysicsTopicInfo {
            id: id.clone(),
            display_name: display_name.clone(),
            sort_order: *sort_order,
            enabled: *enabled,
            subtopics: subtopics
                .into_iter()
                .map(|(sid, sname, sorder, senabled)| PhysicsSubtopicInfo {
                    id: sid,
                    display_name: sname,
                    sort_order: sorder,
                    enabled: senabled,
                })
                .collect(),
        });
    }

    Ok(Json(result))
}

// ============================================================================
// GET /physics/problems
// ============================================================================

async fn get_problems(
    State(state): State<AppState>,
    user: Option<AuthUser>,
    Query(query): Query<PhysicsProblemsQuery>,
) -> Result<Json<Vec<locus_physics_common::PhysicsProblemSummary>>, (StatusCode, Json<ApiError>)> {
    let problems = PhysicsProblem::list(
        &state.pool,
        query.physics_topic.as_deref(),
        query.physics_subtopic.as_deref(),
        query.difficulty,
        query.count,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch physics problems: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Database error")),
        )
    })?;

    let mut summaries: Vec<locus_physics_common::PhysicsProblemSummary> =
        problems.iter().map(|p| p.to_summary()).collect();

    // Annotate with user-solved status if authenticated
    if let Some(ref auth_user) = user {
        for summary in &mut summaries {
            let solved = PhysicsAttempt::user_has_solved(&state.pool, auth_user.id, summary.id)
                .await
                .unwrap_or(false);
            summary.user_solved = Some(solved);
        }
    }

    Ok(Json(summaries))
}

// ============================================================================
// GET /physics/problem/:id
// ============================================================================

async fn get_problem(
    State(state): State<AppState>,
    user: Option<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<locus_physics_common::PhysicsProblemResponse>, (StatusCode, Json<ApiError>)> {
    let problem = PhysicsProblem::find_by_id(&state.pool, id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch physics problem: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new("Database error")),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError::new("Problem not found")),
            )
        })?;

    let mut response = problem.to_response();

    if let Some(ref auth_user) = user {
        let solved = PhysicsAttempt::user_has_solved(&state.pool, auth_user.id, id)
            .await
            .unwrap_or(false);
        response.user_solved = Some(solved);
    }

    Ok(Json(response))
}

// ============================================================================
// POST /physics/submit
// ============================================================================

async fn submit_answer(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<PhysicsSubmitRequest>,
) -> Result<Json<PhysicsSubmitResponse>, (StatusCode, Json<ApiError>)> {
    // Fetch the problem for grading
    let problem = PhysicsProblem::find_by_id(&state.pool, req.problem_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch physics problem for grading: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new("Database error")),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError::new("Problem not found")),
            )
        })?;

    let answer_spec: AnswerSpec =
        serde_json::from_value(problem.answer_spec.clone()).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new("Invalid answer specification")),
            )
        })?;

    // Grade each answer part
    let mut part_results = vec![false; answer_spec.parts.len()];
    let mut parts_correct = 0usize;
    let mut prediction_error_pct: Option<f64> = None;

    for input in &req.answers {
        if input.part_index >= answer_spec.parts.len() {
            continue;
        }
        let part = &answer_spec.parts[input.part_index];
        let diff = (input.value - part.answer).abs();
        let is_close = diff <= part.tolerance;
        part_results[input.part_index] = is_close;
        if is_close {
            parts_correct += 1;
        }
        // Use the first part's error as the prediction accuracy metric
        if prediction_error_pct.is_none() && part.answer.abs() > 1e-9 {
            prediction_error_pct = Some((diff / part.answer.abs()) * 100.0);
        }
    }

    let is_correct = parts_correct == answer_spec.parts.len() && !answer_spec.parts.is_empty();

    // Compute total challenge stages from the problem
    let total_stages: i32 = serde_json::from_value::<Vec<serde_json::Value>>(
        problem.challenge_stages.clone(),
    )
    .map(|v| v.len() as i32)
    .unwrap_or(0);

    // Compute score
    let score = AttemptScore::compute(
        is_correct,
        parts_correct,
        answer_spec.parts.len(),
        req.fbd_attempts,
        req.stages_completed,
        total_stages,
        prediction_error_pct,
        req.hints_used,
        req.what_ifs_explored,
    );

    // Record attempt
    PhysicsAttempt::create(
        &state.pool,
        user.id,
        req.problem_id,
        serde_json::to_value(&req.answers).unwrap_or_default(),
        is_correct,
        req.hints_used,
        req.fbd_attempts,
        prediction_error_pct,
        req.stages_completed,
        req.what_ifs_explored,
        &score,
        Some(req.parameters_used.clone()),
        req.time_taken_ms,
        &problem.physics_topic,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to record physics attempt: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Database error")),
        )
    })?;

    Ok(Json(PhysicsSubmitResponse {
        is_correct,
        part_results,
        score,
    }))
}

// ============================================================================
// GET /physics/progress
// ============================================================================

async fn get_progress(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<PhysicsProgressResponse>, (StatusCode, Json<ApiError>)> {
    let rows = PhysicsProgress::for_user(&state.pool, user.id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch physics progress: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new("Database error")),
            )
        })?;

    Ok(Json(PhysicsProgressResponse {
        entries: rows
            .into_iter()
            .map(|r| PhysicsProgressEntry {
                topic: r.topic,
                problems_attempted: r.problems_attempted as i64,
                problems_solved: r.problems_solved as i64,
            })
            .collect(),
    }))
}
