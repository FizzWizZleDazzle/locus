//! JWT token handling

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: Uuid,
    /// Username
    pub username: String,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at (Unix timestamp)
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, expiry_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            username,
            exp: (now + Duration::hours(expiry_hours)).timestamp(),
            iat: now.timestamp(),
        }
    }
}

/// Create a new JWT token
pub fn create_token(
    user_id: Uuid,
    username: &str,
    secret: &str,
    expiry_hours: i64,
) -> Result<String, JwtError> {
    let claims = Claims::new(user_id, username.to_string(), expiry_hours);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| JwtError::Encode(e.to_string()))
}

/// Verify and decode a JWT token
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| JwtError::Decode(e.to_string()))
}

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("Failed to encode token: {0}")]
    Encode(String),

    #[error("Failed to decode token: {0}")]
    Decode(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_roundtrip() {
        let user_id = Uuid::new_v4();
        let username = "testuser";
        let secret = "test-secret";

        let token = create_token(user_id, username, secret, 24).unwrap();
        let claims = verify_token(&token, secret).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_invalid_secret() {
        let user_id = Uuid::new_v4();
        let token = create_token(user_id, "test", "secret1", 24).unwrap();
        let result = verify_token(&token, "secret2");
        assert!(result.is_err());
    }
}
