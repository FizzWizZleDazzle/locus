//! Problem endpoints

use axum::{
    Json,
    extract::{Query, State},
};

use locus_common::{ProblemQuery, ProblemResponse, constants::DEFAULT_DIFFICULTY};

use super::AppState;
use crate::{
    AppError,
    auth::AuthUser,
    models::{Problem, User},
};

/// Get a problem
///
/// If `practice=true`, includes the answer key for instant client-side grading.
/// Otherwise, the answer is withheld for ranked mode.
pub async fn get_problem(
    State(state): State<AppState>,
    user: Option<AuthUser>,
    Query(query): Query<ProblemQuery>,
) -> Result<Json<ProblemResponse>, AppError> {
    // For ranked mode, require authentication
    if !query.practice && user.is_none() {
        return Err(AppError::Auth(
            "Authentication required for ranked mode".into(),
        ));
    }

    // Parse subtopics from comma-separated string
    let subtopics: Option<Vec<String>> = query.subtopics.as_ref().map(|s| {
        s.split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect()
    });

    // Get user's ELO for the specific topic (if authenticated and topic specified)
    let target_elo = match (&user, &query.main_topic, query.elo) {
        (_, _, Some(elo)) => Some(elo), // Explicit ELO provided
        (Some(u), Some(topic), None) => {
            // Get ELO for the specific topic
            let elo = User::get_elo_for_topic(&state.pool, u.id, topic).await?;
            Some(elo)
        }
        _ => Some(DEFAULT_DIFFICULTY), // Default for practice mode without topic
    };

    // Get a random problem matching criteria
    let problem = Problem::get_random(
        &state.pool,
        target_elo,
        query.main_topic.as_deref(),
        subtopics.as_deref(),
    )
    .await?
    .ok_or_else(|| AppError::NotFound("No problems available for selected topics".into()))?;

    // Include answer only for practice mode
    Ok(Json(problem.to_response(query.practice)))
}
