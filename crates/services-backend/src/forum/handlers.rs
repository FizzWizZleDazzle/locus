use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::auth::{AuthUser, is_admin};
use super::models;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub category: Option<String>,
    pub status: Option<String>,
    pub sort: Option<String>,
    pub page: Option<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostListResponse {
    pub posts: Vec<models::ForumPost>,
    pub has_more: bool,
}

pub async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PostListResponse>, (StatusCode, String)> {
    let sort = params.sort.as_deref().unwrap_or("newest");
    let page = params.page.unwrap_or(1).max(1);

    let (posts, has_more) = models::list_posts(
        &state.pool,
        params.category.as_deref(),
        params.status.as_deref(),
        sort,
        page,
        params.search.as_deref(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PostListResponse { posts, has_more }))
}

#[derive(Debug, Serialize)]
pub struct PostDetailResponse {
    pub post: models::ForumPost,
    pub comments: Vec<models::ForumComment>,
    pub user_voted: bool,
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    user: Option<AuthUser>,
) -> Result<Json<PostDetailResponse>, (StatusCode, String)> {
    let post = models::get_post(&state.pool, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Post not found".to_string()))?;

    let comments = models::get_comments(&state.pool, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user_voted = if let Some(u) = &user {
        models::has_voted(&state.pool, u.id, post_id)
            .await
            .unwrap_or(false)
    } else {
        false
    };

    Ok(Json(PostDetailResponse { post, comments, user_voted }))
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub category: String,
    pub title: String,
    pub body: String,
}

pub async fn create_post(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<models::ForumPost>), (StatusCode, String)> {
    // Validate category
    if req.category != "feature_request" && req.category != "bug_report" {
        return Err((StatusCode::BAD_REQUEST, "Invalid category".to_string()));
    }
    if req.title.trim().is_empty() || req.title.len() > 200 {
        return Err((StatusCode::BAD_REQUEST, "Title must be 1-200 characters".to_string()));
    }
    if req.body.trim().is_empty() || req.body.len() > 10000 {
        return Err((StatusCode::BAD_REQUEST, "Body must be 1-10000 characters".to_string()));
    }

    let post = models::create_post(
        &state.pool,
        user.id,
        &user.username,
        &req.category,
        req.title.trim(),
        req.body.trim(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(post)))
}

#[derive(Debug, Deserialize)]
pub struct CommentRequest {
    pub body: String,
}

pub async fn add_comment(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    user: AuthUser,
    Json(req): Json<CommentRequest>,
) -> Result<(StatusCode, Json<models::ForumComment>), (StatusCode, String)> {
    // Check post exists and not locked
    let post = models::get_post(&state.pool, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Post not found".to_string()))?;

    if post.locked {
        return Err((StatusCode::FORBIDDEN, "Post is locked".to_string()));
    }

    if req.body.trim().is_empty() || req.body.len() > 5000 {
        return Err((StatusCode::BAD_REQUEST, "Comment must be 1-5000 characters".to_string()));
    }

    let comment = models::create_comment(&state.pool, post_id, user.id, &user.username, req.body.trim())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(comment)))
}

#[derive(Debug, Serialize)]
pub struct VoteResponse {
    pub voted: bool,
    pub upvotes: i32,
}

pub async fn toggle_vote(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    user: AuthUser,
) -> Result<Json<VoteResponse>, (StatusCode, String)> {
    // Verify post exists
    let _post = models::get_post(&state.pool, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Post not found".to_string()))?;

    let voted = models::toggle_vote(&state.pool, user.id, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Get updated count
    let post = models::get_post(&state.pool, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Post not found".to_string()))?;

    Ok(Json(VoteResponse { voted, upvotes: post.upvotes }))
}

// Admin handlers

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    user: AuthUser,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !is_admin(&state.pool, user.id).await {
        return Err((StatusCode::FORBIDDEN, "Admin only".to_string()));
    }

    let valid = ["open", "planned", "in_progress", "done", "wontfix"];
    if !valid.contains(&req.status.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid status".to_string()));
    }

    models::update_post_status(&state.pool, post_id, &req.status)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

pub async fn toggle_pin(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    user: AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if !is_admin(&state.pool, user.id).await {
        return Err((StatusCode::FORBIDDEN, "Admin only".to_string()));
    }

    let pinned = models::toggle_pin(&state.pool, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "pinned": pinned })))
}

pub async fn toggle_lock(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    user: AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if !is_admin(&state.pool, user.id).await {
        return Err((StatusCode::FORBIDDEN, "Admin only".to_string()));
    }

    let locked = models::toggle_lock(&state.pool, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "locked": locked })))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<i32>,
    user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !is_admin(&state.pool, user.id).await {
        return Err((StatusCode::FORBIDDEN, "Admin only".to_string()));
    }

    models::delete_comment(&state.pool, comment_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}
