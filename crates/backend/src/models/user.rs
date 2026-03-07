//! User model

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use locus_common::UserProfile;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub email_verified: bool,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Create a new user with password
    pub async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Self, sqlx::Error> {
        let email = email.to_lowercase();
        sqlx::query_as(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, email_verified, email_verified_at, created_at
            "#,
        )
        .bind(username)
        .bind(&email)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    /// Create a new user via OAuth (no password)
    pub async fn create_oauth(
        pool: &PgPool,
        username: &str,
        email: &str,
    ) -> Result<Self, sqlx::Error> {
        let email = email.to_lowercase();
        sqlx::query_as(
            r#"
            INSERT INTO users (username, email, password_hash, email_verified, email_verified_at)
            VALUES ($1, $2, NULL, TRUE, NOW())
            RETURNING id, username, email, password_hash, email_verified, email_verified_at, created_at
            "#,
        )
        .bind(username)
        .bind(&email)
        .fetch_one(pool)
        .await
    }

    /// Set or update the password hash for a user
    pub async fn set_password_hash(
        pool: &PgPool,
        user_id: Uuid,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
            .bind(password_hash)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Find user by username
    pub async fn find_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash, email_verified, email_verified_at, created_at
            FROM users WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await
    }

    /// Find user by email (case-insensitive)
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        let email = email.to_lowercase();
        sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash, email_verified, email_verified_at, created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(&email)
        .fetch_optional(pool)
        .await
    }

    /// Find user by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash, email_verified, email_verified_at, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map(Some)
        .or_else(|e| match e {
            sqlx::Error::RowNotFound => Ok(None),
            e => Err(e),
        })
    }

    /// Get user's ELO for a specific topic (uses PostgreSQL function)
    pub async fn get_elo_for_topic(
        pool: &PgPool,
        user_id: Uuid,
        topic: &str,
    ) -> Result<i32, sqlx::Error> {
        let result: (i32,) = sqlx::query_as("SELECT get_user_elo($1, $2)")
            .bind(user_id)
            .bind(topic)
            .fetch_one(pool)
            .await?;
        Ok(result.0)
    }

    /// Update user's ELO for a specific topic (uses PostgreSQL function)
    pub async fn update_elo_for_topic(
        pool: &PgPool,
        user_id: Uuid,
        topic: &str,
        new_elo: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT update_user_elo($1, $2, $3)")
            .bind(user_id)
            .bind(topic)
            .bind(new_elo)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Update ELO and streaks for a topic, returning the new topic_streak
    pub async fn update_elo_and_streaks<'e, E>(
        executor: E,
        user_id: Uuid,
        topic: &str,
        new_elo: i32,
        is_correct: bool,
    ) -> Result<i32, sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        // Use a single upsert + streak update query to avoid needing two executor uses
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO user_topic_elo (user_id, topic, elo, peak_elo, topic_streak, peak_topic_streak)
            VALUES ($1, $2, $3, $3, CASE WHEN $4 THEN 1 ELSE 0 END, CASE WHEN $4 THEN 1 ELSE 0 END)
            ON CONFLICT (user_id, topic) DO UPDATE SET
              elo = $3,
              topic_streak = CASE WHEN $4 THEN user_topic_elo.topic_streak + 1 ELSE 0 END,
              peak_topic_streak = GREATEST(user_topic_elo.peak_topic_streak,
                                   CASE WHEN $4 THEN user_topic_elo.topic_streak + 1 ELSE 0 END),
              peak_elo = GREATEST(user_topic_elo.peak_elo, $3),
              updated_at = NOW()
            RETURNING topic_streak
            "#,
        )
        .bind(user_id)
        .bind(topic)
        .bind(new_elo)
        .bind(is_correct)
        .fetch_one(executor)
        .await?;

        Ok(row.0)
    }

    /// Update the global daily streak for a user (call only on correct answer)
    pub async fn update_daily_streak<'e, E>(executor: E, user_id: Uuid) -> Result<(), sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query(
            r#"
            UPDATE users SET
              current_streak = CASE
                WHEN last_active_date = CURRENT_DATE - INTERVAL '1 day' THEN current_streak + 1
                WHEN last_active_date = CURRENT_DATE                      THEN current_streak
                ELSE 1
              END,
              last_active_date = CURRENT_DATE
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Get all ELO ratings for a user
    pub async fn get_all_elos(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<HashMap<String, i32>, sqlx::Error> {
        let rows: Vec<(String, i32)> =
            sqlx::query_as("SELECT topic, elo FROM user_topic_elo WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(pool)
                .await?;

        Ok(rows.into_iter().collect())
    }

    /// Get leaderboard for a specific topic
    pub async fn leaderboard(
        pool: &PgPool,
        topic: &str,
        limit: i32,
    ) -> Result<Vec<LeaderboardRow>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT u.username, e.elo,
                   ROW_NUMBER() OVER (ORDER BY e.elo DESC) as rank
            FROM user_topic_elo e
            JOIN users u ON u.id = e.user_id
            WHERE e.topic = $1
            ORDER BY e.elo DESC
            LIMIT $2
            "#,
        )
        .bind(topic)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    /// Check if username exists
    pub async fn username_exists(pool: &PgPool, username: &str) -> Result<bool, sqlx::Error> {
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await?;
        Ok(result.0 > 0)
    }

    /// Check if email exists (case-insensitive)
    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, sqlx::Error> {
        let email = email.to_lowercase();
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind(&email)
            .fetch_one(pool)
            .await?;
        Ok(result.0 > 0)
    }

    /// Mark user's email as verified
    pub async fn mark_email_verified(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET email_verified = TRUE, email_verified_at = NOW() WHERE id = $1",
        )
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Get list of OAuth providers linked to this user
    pub async fn get_oauth_providers(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<String>, sqlx::Error> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT provider FROM oauth_accounts WHERE user_id = $1 ORDER BY provider",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(provider,)| provider).collect())
    }

    /// Update username (with uniqueness check done by caller)
    pub async fn update_username(
        pool: &PgPool,
        user_id: Uuid,
        new_username: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET username = $1 WHERE id = $2")
            .bind(new_username)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Delete user account (cascades to oauth_accounts, attempts, etc.)
    pub async fn delete_account(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Convert to API response type with all ELO ratings (single query)
    pub async fn to_profile(&self, pool: &PgPool) -> Result<UserProfile, sqlx::Error> {
        let row: (i32, serde_json::Value, serde_json::Value) = sqlx::query_as(
            r#"
            SELECT
              COALESCE(u.current_streak, 0),
              COALESCE(
                (SELECT json_object_agg(topic, elo) FROM user_topic_elo WHERE user_id = $1),
                '{}'::json
              ),
              COALESCE(
                (SELECT json_agg(provider ORDER BY provider) FROM oauth_accounts WHERE user_id = $1),
                '[]'::json
              )
            FROM users u WHERE u.id = $1
            "#,
        )
        .bind(self.id)
        .fetch_one(pool)
        .await?;

        let elos: HashMap<String, i32> =
            serde_json::from_value(row.1).unwrap_or_default();
        let oauth_providers: Vec<String> =
            serde_json::from_value(row.2).unwrap_or_default();

        Ok(UserProfile {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            email_verified: self.email_verified,
            elo_ratings: elos,
            has_password: self.password_hash.is_some(),
            oauth_providers,
            created_at: self.created_at,
            current_streak: row.0,
        })
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OAuthAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub provider_email: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl OAuthAccount {
    /// Find an OAuth account by provider and provider user ID
    pub async fn find_by_provider(
        pool: &PgPool,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, user_id, provider, provider_user_id, provider_email, created_at
            FROM oauth_accounts
            WHERE provider = $1 AND provider_user_id = $2
            "#,
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(pool)
        .await
    }

    /// Create a new OAuth account link
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        provider: &str,
        provider_user_id: &str,
        provider_email: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO oauth_accounts (user_id, provider, provider_user_id, provider_email)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, provider, provider_user_id, provider_email, created_at
            "#,
        )
        .bind(user_id)
        .bind(provider)
        .bind(provider_user_id)
        .bind(provider_email)
        .fetch_one(pool)
        .await
    }

    /// Delete OAuth account by user ID and provider
    pub async fn delete_by_user_and_provider(
        pool: &PgPool,
        user_id: Uuid,
        provider: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM oauth_accounts WHERE user_id = $1 AND provider = $2")
            .bind(user_id)
            .bind(provider)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Count OAuth accounts for a user
    pub async fn count_by_user(pool: &PgPool, user_id: Uuid) -> Result<i64, sqlx::Error> {
        let result: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM oauth_accounts WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(pool)
                .await?;
        Ok(result.0)
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LeaderboardRow {
    pub username: String,
    pub elo: i32,
    pub rank: i64,
}
