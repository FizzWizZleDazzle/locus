use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

const COOKIE_NAME: &str = "locus_token";

// JWT Claims (must match main backend's Claims struct)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

// Cookie helpers

pub fn build_clear_cookie(secure: bool, cookie_domain: Option<&str>) -> String {
    let secure_flag = if secure { "; Secure" } else { "" };
    let domain_flag = cookie_domain.map_or(String::new(), |d| format!("; Domain={}", d));
    format!(
        "{}=; HttpOnly; SameSite=Lax; Path=/api; Max-Age=0{}{}",
        COOKIE_NAME, secure_flag, domain_flag
    )
}

fn extract_token_from_cookies(cookie_header: &str) -> Option<&str> {
    cookie_header.split(';').map(|s| s.trim()).find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        if name.trim() == COOKIE_NAME {
            Some(value.trim())
        } else {
            None
        }
    })
}

// AuthUser extractor

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
        let token = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|value| value.to_str().ok())
            .and_then(extract_token_from_cookies)
            .ok_or((StatusCode::UNAUTHORIZED, "Missing authentication"))?;

        let claims = verify_token(token, &state.jwt_secret)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

        Ok(AuthUser {
            id: claims.sub,
            username: claims.username,
        })
    }
}

// Optional auth extractor

impl FromRequestParts<AppState> for Option<AuthUser> {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(AuthUser::from_request_parts(parts, state).await.ok())
    }
}

