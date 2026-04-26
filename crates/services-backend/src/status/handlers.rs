use axum::{Json, extract::State, http::StatusCode};

use super::models;
use crate::AppState;

pub async fn current(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let check = models::get_latest(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match check {
        Some(c) => Ok(Json(serde_json::json!({
            "is_healthy": c.is_healthy,
            "status_code": c.status_code,
            "response_time_ms": c.response_time_ms,
            "checked_at": c.checked_at,
        }))),
        None => Ok(Json(serde_json::json!({
            "is_healthy": null,
            "message": "No checks yet"
        }))),
    }
}

pub async fn history(
    State(state): State<AppState>,
) -> Result<Json<Vec<models::StatusCheck>>, (StatusCode, String)> {
    let checks = models::get_history_24h(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(checks))
}

pub async fn uptime(
    State(state): State<AppState>,
) -> Result<Json<models::UptimeResponse>, (StatusCode, String)> {
    let data = models::get_uptime_30d(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(data))
}
