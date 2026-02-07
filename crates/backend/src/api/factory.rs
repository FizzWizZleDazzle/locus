//! Factory content submission endpoint

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    auth::ApiKeyAuth,
    models::Problem,
    AppError,
};
use crate::api::AppState;
use locus_common::{CreateProblemRequest, CreateProblemResponse, GradingMode, MainTopic};

/// Create a new problem (Factory endpoint)
///
/// This endpoint is for the Factory (Python content generation pipeline) to submit
/// problems to the Locus backend. It requires API key authentication.
pub async fn create_problem(
    State(state): State<AppState>,
    _auth: ApiKeyAuth,  // Validates API key automatically
    Json(req): Json<CreateProblemRequest>,
) -> Result<(StatusCode, Json<CreateProblemResponse>), AppError> {
    // Validate request fields
    validate_problem_request(&req)?;

    // Parse grading mode
    let grading_mode = match req.grading_mode.to_lowercase().as_str() {
        "equivalent" => GradingMode::Equivalent,
        "factor" => GradingMode::Factor,
        "expand" => GradingMode::Expand,
        _ => {
            return Err(AppError::BadRequest(
                "grading_mode must be 'equivalent', 'factor', or 'expand'".to_string(),
            ));
        }
    };

    // Insert problem into database
    let problem = Problem::create(
        &state.pool,
        &req.question_latex,
        &req.answer_key,
        req.difficulty,
        &req.main_topic,
        &req.subtopic,
        grading_mode,
    )
    .await?;

    tracing::info!(
        "Factory created problem: id={}, topic={}, subtopic={}, difficulty={}",
        problem.id,
        problem.main_topic,
        problem.subtopic,
        problem.difficulty
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateProblemResponse {
            id: problem.id,
            message: "Problem created successfully".to_string(),
        }),
    ))
}

/// Validate problem request fields
fn validate_problem_request(req: &CreateProblemRequest) -> Result<(), AppError> {
    // Validate question_latex is non-empty
    if req.question_latex.trim().is_empty() {
        return Err(AppError::BadRequest(
            "question_latex cannot be empty".to_string(),
        ));
    }

    // Validate answer_key is non-empty
    if req.answer_key.trim().is_empty() {
        return Err(AppError::BadRequest(
            "answer_key cannot be empty".to_string(),
        ));
    }

    // Validate difficulty range
    if req.difficulty < 0 || req.difficulty > 3000 {
        return Err(AppError::BadRequest(
            "difficulty must be between 0 and 3000".to_string(),
        ));
    }

    // Validate main_topic
    let main_topic = MainTopic::from_str(&req.main_topic).ok_or_else(|| {
        AppError::BadRequest(format!(
            "Invalid main_topic '{}'. Valid topics: arithmetic, algebra1, geometry, algebra2, precalculus, calculus, multivariable_calculus, linear_algebra",
            req.main_topic
        ))
    })?;

    // Validate subtopic belongs to main_topic
    let allowed_subtopics = main_topic.subtopics();
    if !allowed_subtopics.contains(&req.subtopic.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Invalid subtopic '{}' for main_topic '{}'. Allowed subtopics: {}",
            req.subtopic,
            req.main_topic,
            allowed_subtopics.join(", ")
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_problem_request_valid() {
        let req = CreateProblemRequest {
            question_latex: "2 + 2".to_string(),
            answer_key: "4".to_string(),
            difficulty: 1000,
            main_topic: "arithmetic".to_string(),
            subtopic: "addition_subtraction".to_string(),
            grading_mode: "equivalent".to_string(),
        };
        assert!(validate_problem_request(&req).is_ok());
    }

    #[test]
    fn test_validate_problem_request_empty_question() {
        let req = CreateProblemRequest {
            question_latex: "  ".to_string(),
            answer_key: "4".to_string(),
            difficulty: 1000,
            main_topic: "arithmetic".to_string(),
            subtopic: "addition_subtraction".to_string(),
            grading_mode: "equivalent".to_string(),
        };
        assert!(validate_problem_request(&req).is_err());
    }

    #[test]
    fn test_validate_problem_request_invalid_difficulty() {
        let req = CreateProblemRequest {
            question_latex: "2 + 2".to_string(),
            answer_key: "4".to_string(),
            difficulty: 5000,
            main_topic: "arithmetic".to_string(),
            subtopic: "addition_subtraction".to_string(),
            grading_mode: "equivalent".to_string(),
        };
        assert!(validate_problem_request(&req).is_err());
    }

    #[test]
    fn test_validate_problem_request_invalid_topic() {
        let req = CreateProblemRequest {
            question_latex: "2 + 2".to_string(),
            answer_key: "4".to_string(),
            difficulty: 1000,
            main_topic: "invalid_topic".to_string(),
            subtopic: "addition_subtraction".to_string(),
            grading_mode: "equivalent".to_string(),
        };
        assert!(validate_problem_request(&req).is_err());
    }

    #[test]
    fn test_validate_problem_request_invalid_subtopic() {
        let req = CreateProblemRequest {
            question_latex: "2 + 2".to_string(),
            answer_key: "4".to_string(),
            difficulty: 1000,
            main_topic: "arithmetic".to_string(),
            subtopic: "derivatives".to_string(),  // calculus subtopic
            grading_mode: "equivalent".to_string(),
        };
        assert!(validate_problem_request(&req).is_err());
    }
}
