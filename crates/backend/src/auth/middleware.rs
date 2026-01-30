//! Authentication middleware

use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use uuid::Uuid;

use super::jwt::{verify_token, Claims};
use crate::api::AppState;

/// Authenticated user extractor
///
/// Use this in route handlers to require authentication:
/// ```ignore
/// async fn my_handler(user: AuthUser) -> impl IntoResponse {
///     // user.id and user.username are available
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Get Authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header"))?;

        // Extract Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization format"))?;

        // Verify token
        let claims: Claims = verify_token(token, &state.jwt_secret)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

        Ok(AuthUser {
            id: claims.sub,
            username: claims.username,
        })
    }
}

/// Optional authentication extractor
///
/// Use when authentication is optional:
/// ```ignore
/// async fn my_handler(user: Option<AuthUser>) -> impl IntoResponse {
///     if let Some(user) = user {
///         // authenticated
///     } else {
///         // anonymous
///     }
/// }
/// ```
impl FromRequestParts<AppState> for Option<AuthUser> {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(AuthUser::from_request_parts(parts, state).await.ok())
    }
}
