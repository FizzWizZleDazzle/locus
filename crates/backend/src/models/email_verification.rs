//! Email verification token model

use chrono::{DateTime, Duration, Utc};
use rand;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}

impl EmailVerificationToken {
    /// Generate a cryptographically secure random token
    pub fn generate_token() -> String {
        let mut bytes = [0u8; 32];
        rand::fill(&mut bytes[..]);
        hex::encode(bytes)
    }

    /// Create a new verification token for a user (1 hour expiry)
    pub async fn create(pool: &PgPool, user_id: Uuid) -> Result<Self, sqlx::Error> {
        let token = Self::generate_token();
        let expires_at = Utc::now() + Duration::hours(1);

        sqlx::query_as(
            r#"
            INSERT INTO email_verification_tokens (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token, created_at, expires_at, used_at
            "#,
        )
        .bind(user_id)
        .bind(&token)
        .bind(expires_at)
        .fetch_one(pool)
        .await
    }

    /// Find a token by its value
    pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, user_id, token, created_at, expires_at, used_at
            FROM email_verification_tokens
            WHERE token = $1
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await
    }

    /// Check if token is valid (not expired and not used)
    pub fn is_valid(&self) -> bool {
        self.used_at.is_none() && self.expires_at > Utc::now()
    }

    /// Mark token as used
    pub async fn mark_used(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE email_verification_tokens SET used_at = NOW() WHERE id = $1")
            .bind(self.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Check if user can send another verification email (rate limit: 1 per minute)
    pub async fn can_send_email(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let one_minute_ago = Utc::now() - Duration::minutes(1);

        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM email_verification_sends
            WHERE user_id = $1 AND sent_at > $2
            "#,
        )
        .bind(user_id)
        .bind(one_minute_ago)
        .fetch_one(pool)
        .await?;

        Ok(count.0 == 0)
    }

    /// Record that a verification email was sent
    pub async fn record_send(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO email_verification_sends (user_id) VALUES ($1)")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
