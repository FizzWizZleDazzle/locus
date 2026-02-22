//! Problem endpoints

use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};

use locus_common::{ProblemQuery, ProblemResponse, constants::DEFAULT_DIFFICULTY};

use super::AppState;
use crate::{
    AppError,
    auth::AuthUser,
    models::{Problem, User},
};

/// Resolve target ELO from query params and user state
async fn resolve_elo(
    state: &AppState,
    user: &Option<AuthUser>,
    query: &ProblemQuery,
) -> Result<Option<i32>, AppError> {
    match (user, &query.main_topic, query.elo) {
        (_, _, Some(elo)) => Ok(Some(elo)),
        (Some(u), Some(topic), None) => {
            let elo = User::get_elo_for_topic(&state.pool, u.id, topic).await?;
            Ok(Some(elo))
        }
        _ => Ok(Some(DEFAULT_DIFFICULTY)),
    }
}

/// Parse subtopics from comma-separated string
fn parse_subtopics(query: &ProblemQuery) -> Option<Vec<String>> {
    query.subtopics.as_ref().map(|s| {
        s.split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect()
    })
}

/// Get a single problem (DEPRECATED — use /problems instead)
pub async fn get_problem(
    State(state): State<AppState>,
    user: Option<AuthUser>,
    Query(query): Query<ProblemQuery>,
) -> Result<impl IntoResponse, AppError> {
    if !query.practice && user.is_none() {
        return Err(AppError::Auth(
            "Authentication required for ranked mode".into(),
        ));
    }

    let subtopics = parse_subtopics(&query);
    let target_elo = resolve_elo(&state, &user, &query).await?;

    let problem = Problem::get_random(
        &state.pool,
        target_elo,
        query.main_topic.as_deref(),
        subtopics.as_deref(),
    )
    .await?
    .ok_or_else(|| AppError::NotFound("No problems available for selected topics".into()))?;

    let mut headers = HeaderMap::new();
    headers.insert("Deprecation", HeaderValue::from_static("true"));
    headers.insert(
        "Link",
        HeaderValue::from_static("</api/problems>; rel=\"successor-version\""),
    );

    Ok((headers, Json(problem.to_response(query.practice))))
}

/// Get a batch of problems
pub async fn get_problems(
    State(state): State<AppState>,
    user: Option<AuthUser>,
    Query(query): Query<ProblemQuery>,
) -> Result<Json<Vec<ProblemResponse>>, AppError> {
    if !query.practice && user.is_none() {
        return Err(AppError::Auth(
            "Authentication required for ranked mode".into(),
        ));
    }

    let subtopics = parse_subtopics(&query);
    let target_elo = resolve_elo(&state, &user, &query).await?;

    let problems = Problem::get_random_batch(
        &state.pool,
        target_elo,
        query.main_topic.as_deref(),
        subtopics.as_deref(),
        query.count,
    )
    .await?;

    if problems.is_empty() {
        return Err(AppError::NotFound(
            "No problems available for selected topics".into(),
        ));
    }

    let responses: Vec<ProblemResponse> = problems
        .into_iter()
        .map(|p| p.to_response(query.practice))
        .collect();

    Ok(Json(responses))
}
