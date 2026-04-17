//! Physics attempt database model

use sqlx::PgPool;
use uuid::Uuid;

use locus_physics_common::scoring::AttemptScore;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhysicsAttempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub problem_id: Uuid,
    pub user_answers: serde_json::Value,
    pub is_correct: bool,
    pub hints_used: i32,
    pub fbd_attempts: i32,
    pub prediction_accuracy: Option<f64>,
    pub stages_completed: i32,
    pub what_ifs_explored: i32,
    pub score_correctness: i32,
    pub score_process: i32,
    pub score_prediction: i32,
    pub score_independence: i32,
    pub score_exploration: i32,
    pub parameters_used: Option<serde_json::Value>,
    pub time_taken_ms: Option<i32>,
}

impl PhysicsAttempt {
    /// Insert a new physics attempt and update user progress.
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        problem_id: Uuid,
        user_answers: serde_json::Value,
        is_correct: bool,
        hints_used: i32,
        fbd_attempts: i32,
        prediction_accuracy: Option<f64>,
        stages_completed: i32,
        what_ifs_explored: i32,
        score: &AttemptScore,
        parameters_used: Option<serde_json::Value>,
        time_taken_ms: Option<i32>,
        physics_topic: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let mut tx = pool.begin().await?;

        // Insert attempt
        let row: (Uuid,) = sqlx::query_as(
            r#"
            INSERT INTO physics_attempts (
                user_id, problem_id, user_answers, is_correct,
                hints_used, fbd_attempts, prediction_accuracy,
                stages_completed, what_ifs_explored,
                score_correctness, score_process, score_prediction,
                score_independence, score_exploration,
                parameters_used, time_taken_ms
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING id
            "#,
        )
        .bind(user_id)
        .bind(problem_id)
        .bind(&user_answers)
        .bind(is_correct)
        .bind(hints_used)
        .bind(fbd_attempts)
        .bind(prediction_accuracy)
        .bind(stages_completed)
        .bind(what_ifs_explored)
        .bind(score.correctness as i32)
        .bind(score.process as i32)
        .bind(score.prediction_accuracy as i32)
        .bind(score.independence as i32)
        .bind(score.exploration_bonus as i32)
        .bind(&parameters_used)
        .bind(time_taken_ms)
        .fetch_one(&mut *tx)
        .await?;

        // Upsert user progress
        sqlx::query(
            r#"
            INSERT INTO physics_user_progress (user_id, topic, problems_attempted, problems_solved, updated_at)
            VALUES ($1, $2, 1, $3, NOW())
            ON CONFLICT (user_id, topic)
            DO UPDATE SET
                problems_attempted = physics_user_progress.problems_attempted + 1,
                problems_solved = physics_user_progress.problems_solved + $3,
                updated_at = NOW()
            "#,
        )
        .bind(user_id)
        .bind(physics_topic)
        .bind(if is_correct { 1 } else { 0 })
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(row.0)
    }

    /// Check if a user has solved a specific problem.
    pub async fn user_has_solved(
        pool: &PgPool,
        user_id: Uuid,
        problem_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let row: Option<(bool,)> = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM physics_attempts
                WHERE user_id = $1 AND problem_id = $2 AND is_correct = true
            )
            "#,
        )
        .bind(user_id)
        .bind(problem_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.0).unwrap_or(false))
    }
}

/// Per-topic progress row.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhysicsProgress {
    pub topic: String,
    pub problems_attempted: i32,
    pub problems_solved: i32,
}

impl PhysicsProgress {
    /// Fetch all physics progress for a user.
    pub async fn for_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT topic, problems_attempted, problems_solved
            FROM physics_user_progress
            WHERE user_id = $1
            ORDER BY topic
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }
}
