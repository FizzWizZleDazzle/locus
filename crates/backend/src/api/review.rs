//! Review & mastery endpoints

use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use uuid::Uuid;

use locus_common::{
    BookmarkCheckRequest, BookmarkCheckResponse, BookmarkListResponse, BookmarkRequest,
    ReviewCompleteRequest, ReviewCompleteResponse, ReviewQueueResponse, SuccessResponse,
    SubtopicAccuracy, WeakAreaResponse, WrongAnswerReviewResponse,
};

use crate::{AppError, auth::AuthUser, models::{Attempt, Bookmark, ReviewQueue}};

use super::AppState;

// ============================================================================
// Wrong Answer Review
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct WrongAnswerQuery {
    #[serde(default)]
    pub main_topic: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

pub async fn get_wrong_answers(
    State(state): State<AppState>,
    user: AuthUser,
    Query(q): Query<WrongAnswerQuery>,
) -> Result<Json<WrongAnswerReviewResponse>, AppError> {
    let limit = q.limit.min(100).max(1);
    let (items, total) =
        Attempt::get_wrong_answers(&state.pool, user.id, q.main_topic.as_deref(), limit, q.offset)
            .await?;
    Ok(Json(WrongAnswerReviewResponse { items, total }))
}

// ============================================================================
// Bookmarks
// ============================================================================

pub async fn add_bookmark(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<BookmarkRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    Bookmark::add(&state.pool, user.id, req.problem_id).await?;
    Ok(Json(SuccessResponse {
        success: true,
        message: None,
    }))
}

pub async fn remove_bookmark(
    State(state): State<AppState>,
    user: AuthUser,
    Path(problem_id): Path<Uuid>,
) -> Result<Json<SuccessResponse>, AppError> {
    Bookmark::remove(&state.pool, user.id, problem_id).await?;
    Ok(Json(SuccessResponse {
        success: true,
        message: None,
    }))
}

#[derive(Debug, Deserialize)]
pub struct BookmarkListQuery {
    #[serde(default)]
    pub main_topic: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

pub async fn list_bookmarks(
    State(state): State<AppState>,
    user: AuthUser,
    Query(q): Query<BookmarkListQuery>,
) -> Result<Json<BookmarkListResponse>, AppError> {
    let limit = q.limit.min(100).max(1);
    let resp =
        Bookmark::list(&state.pool, user.id, q.main_topic.as_deref(), limit, q.offset).await?;
    Ok(Json(resp))
}

pub async fn check_bookmarks(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<BookmarkCheckRequest>,
) -> Result<Json<BookmarkCheckResponse>, AppError> {
    if req.problem_ids.len() > 100 {
        return Err(AppError::BadRequest("Too many problem IDs".into()));
    }
    let bookmarked = Bookmark::check(&state.pool, user.id, &req.problem_ids).await?;
    Ok(Json(BookmarkCheckResponse { bookmarked }))
}

// ============================================================================
// Weak Areas
// ============================================================================

#[derive(Debug, Clone, sqlx::FromRow)]
struct SubtopicAccuracyRow {
    main_topic: String,
    subtopic: String,
    total: i64,
    correct: i64,
    accuracy: f64,
}

impl From<SubtopicAccuracyRow> for SubtopicAccuracy {
    fn from(r: SubtopicAccuracyRow) -> Self {
        Self {
            main_topic: r.main_topic,
            subtopic: r.subtopic,
            total: r.total,
            correct: r.correct,
            accuracy: r.accuracy,
        }
    }
}

pub async fn get_weak_areas(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<WeakAreaResponse>, AppError> {
    let rows: Vec<SubtopicAccuracyRow> = sqlx::query_as(
        r#"
        SELECT p.main_topic, p.subtopic,
               COUNT(*) as total,
               COUNT(*) FILTER (WHERE a.is_correct) as correct,
               CASE WHEN COUNT(*) > 0
                    THEN COUNT(*) FILTER (WHERE a.is_correct)::float / COUNT(*)::float
                    ELSE 0.0
               END as accuracy
        FROM attempts a
        JOIN problems p ON p.id = a.problem_id
        WHERE a.user_id = $1
        GROUP BY p.main_topic, p.subtopic
        ORDER BY p.main_topic, p.subtopic
        "#,
    )
    .bind(user.id)
    .fetch_all(&state.pool)
    .await?;

    let all: Vec<SubtopicAccuracy> = rows.into_iter().map(|r| r.into()).collect();

    // Top 5 weakest areas with at least 5 attempts
    let mut weakest: Vec<SubtopicAccuracy> = all
        .iter()
        .filter(|s| s.total >= 5)
        .cloned()
        .collect();
    weakest.sort_by(|a, b| a.accuracy.partial_cmp(&b.accuracy).unwrap_or(std::cmp::Ordering::Equal));
    weakest.truncate(5);

    Ok(Json(WeakAreaResponse { weakest, all }))
}

// ============================================================================
// Review Queue (Spaced Repetition)
// ============================================================================

pub async fn get_review_queue(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<ReviewQueueResponse>, AppError> {
    let resp = ReviewQueue::get_due(&state.pool, user.id, 20).await?;
    Ok(Json(resp))
}

pub async fn complete_review(
    State(state): State<AppState>,
    user: AuthUser,
    Path(review_id): Path<Uuid>,
    Json(req): Json<ReviewCompleteRequest>,
) -> Result<Json<ReviewCompleteResponse>, AppError> {
    if req.quality < 0 || req.quality > 5 {
        return Err(AppError::BadRequest("Quality must be 0-5".into()));
    }
    let resp = ReviewQueue::complete_review(&state.pool, review_id, user.id, req.quality).await?;
    Ok(Json(resp))
}
