//! Attempt model

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Attempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub problem_id: Uuid,
    pub user_input: String,
    pub is_correct: bool,
    pub elo_before: i32,
    pub elo_after: i32,
    pub time_taken_ms: Option<i32>,
    pub main_topic: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Attempt {
    /// Record a new attempt
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        problem_id: Uuid,
        user_input: &str,
        is_correct: bool,
        elo_before: i32,
        elo_after: i32,
        time_taken_ms: Option<i32>,
        main_topic: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO attempts (user_id, problem_id, user_input, is_correct, elo_before, elo_after, time_taken_ms, main_topic)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, user_id, problem_id, user_input, is_correct, elo_before, elo_after, time_taken_ms, main_topic, created_at
            "#,
        )
        .bind(user_id)
        .bind(problem_id)
        .bind(user_input)
        .bind(is_correct)
        .bind(elo_before)
        .bind(elo_after)
        .bind(time_taken_ms)
        .bind(main_topic)
        .fetch_one(pool)
        .await
    }

    /// Get user's recent attempts
    pub async fn get_user_attempts(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, user_id, problem_id, user_input, is_correct, elo_before, elo_after, time_taken_ms, main_topic, created_at
            FROM attempts
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    /// Get user's statistics
    pub async fn get_user_stats(pool: &PgPool, user_id: Uuid) -> Result<UserStats, sqlx::Error> {
        let result: (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE is_correct) as correct
            FROM attempts
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(UserStats {
            total_attempts: result.0,
            correct_attempts: result.1,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UserStats {
    pub total_attempts: i64,
    pub correct_attempts: i64,
}
