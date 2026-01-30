//! User model

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;

use locus_common::UserProfile;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Create a new user
    pub async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, created_at
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    /// Find user by email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await
    }

    /// Find user by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash, created_at
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
    pub async fn get_elo_for_topic(pool: &PgPool, user_id: Uuid, topic: &str) -> Result<i32, sqlx::Error> {
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

    /// Get all ELO ratings for a user
    pub async fn get_all_elos(pool: &PgPool, user_id: Uuid) -> Result<HashMap<String, i32>, sqlx::Error> {
        let rows: Vec<(String, i32)> = sqlx::query_as(
            "SELECT topic, elo FROM user_topic_elo WHERE user_id = $1"
        )
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
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;
        Ok(result.0 > 0)
    }

    /// Check if email exists
    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, sqlx::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_one(pool)
        .await?;
        Ok(result.0 > 0)
    }

    /// Convert to API response type with all ELO ratings
    pub async fn to_profile(&self, pool: &PgPool) -> Result<UserProfile, sqlx::Error> {
        let elos = Self::get_all_elos(pool, self.id).await?;

        Ok(UserProfile {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            elo_arithmetic: *elos.get("arithmetic").unwrap_or(&1500),
            elo_algebra1: *elos.get("algebra1").unwrap_or(&1500),
            elo_geometry: *elos.get("geometry").unwrap_or(&1500),
            elo_algebra2: *elos.get("algebra2").unwrap_or(&1500),
            elo_precalculus: *elos.get("precalculus").unwrap_or(&1500),
            elo_calculus: *elos.get("calculus").unwrap_or(&1500),
            elo_multivariable_calculus: *elos.get("multivariable_calculus").unwrap_or(&1500),
            elo_linear_algebra: *elos.get("linear_algebra").unwrap_or(&1500),
            created_at: self.created_at,
        })
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LeaderboardRow {
    pub username: String,
    pub elo: i32,
    pub rank: i64,
}
