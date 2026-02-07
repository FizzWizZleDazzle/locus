//! API key authentication for service-to-service communication

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};

use crate::api::AppState;
use locus_common::ApiError;

/// API key authentication extractor
///
/// Use this in route handlers to require API key authentication:
/// ```ignore
/// async fn my_handler(_auth: ApiKeyAuth) -> impl IntoResponse {
///     // Handler code here
/// }
/// ```
pub struct ApiKeyAuth;

impl FromRequestParts<AppState> for ApiKeyAuth {
    type Rejection = (StatusCode, Json<ApiError>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Get X-API-Key header
        let api_key_header = parts
            .headers
            .get("x-api-key")
            .and_then(|value| value.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                Json(ApiError::new("Missing API key")),
            ))?;

        // Get expected API key from state
        let expected_key = &state.api_key;

        // Use constant-time comparison to prevent timing attacks
        if !constant_time_compare(api_key_header.as_bytes(), expected_key.as_bytes()) {
            tracing::warn!("Invalid API key attempt");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError::new("Invalid API key")),
            ));
        }

        Ok(ApiKeyAuth)
    }
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }

    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare(b"hello", b"hello"));
        assert!(!constant_time_compare(b"hello", b"world"));
        assert!(!constant_time_compare(b"short", b"longer"));
        assert!(!constant_time_compare(b"", b"nonempty"));
    }
}
