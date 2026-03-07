//! Daily puzzle models

use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DailyPuzzle {
    pub id: Uuid,
    pub problem_id: Uuid,
    pub puzzle_date: Option<NaiveDate>,
    pub title: String,
    pub hints: serde_json::Value,
    pub editorial_latex: String,
    pub source: String,
    pub status: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl DailyPuzzle {
    /// Get the daily puzzle for a specific date
    pub async fn get_by_date(pool: &PgPool, date: NaiveDate) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, problem_id, puzzle_date, title, hints, editorial_latex, source, status, created_at
            FROM daily_puzzles
            WHERE puzzle_date = $1 AND status IN ('scheduled', 'archived')
            "#,
        )
        .bind(date)
        .fetch_optional(pool)
        .await
    }

    /// Get paginated archive of past puzzles
    pub async fn get_archive(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ArchiveRow>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                dp.puzzle_date,
                dp.title,
                p.difficulty,
                p.main_topic,
                COALESCE(stats.solve_rate, 0.0) as solve_rate
            FROM daily_puzzles dp
            JOIN problems p ON p.id = dp.problem_id
            LEFT JOIN LATERAL (
                SELECT
                    CASE WHEN COUNT(*) = 0 THEN 0.0
                         ELSE COUNT(*) FILTER (WHERE is_correct)::float / COUNT(*)::float
                    END as solve_rate
                FROM daily_puzzle_attempts
                WHERE daily_puzzle_id = dp.id
            ) stats ON true
            WHERE dp.puzzle_date < CURRENT_DATE AND dp.status IN ('scheduled', 'archived')
            ORDER BY dp.puzzle_date DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    /// Get archive with user solve status
    pub async fn get_archive_with_user(
        pool: &PgPool,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ArchiveRowWithUser>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                dp.puzzle_date,
                dp.title,
                p.difficulty,
                p.main_topic,
                COALESCE(stats.solve_rate, 0.0) as solve_rate,
                (user_solve.id IS NOT NULL) as user_solved,
                (user_solve.id IS NOT NULL AND user_solve.created_at::date = dp.puzzle_date) as user_solved_same_day
            FROM daily_puzzles dp
            JOIN problems p ON p.id = dp.problem_id
            LEFT JOIN LATERAL (
                SELECT
                    CASE WHEN COUNT(*) = 0 THEN 0.0
                         ELSE COUNT(*) FILTER (WHERE is_correct)::float / COUNT(*)::float
                    END as solve_rate
                FROM daily_puzzle_attempts
                WHERE daily_puzzle_id = dp.id
            ) stats ON true
            LEFT JOIN LATERAL (
                SELECT id, created_at FROM daily_puzzle_attempts
                WHERE user_id = $1 AND daily_puzzle_id = dp.id AND is_correct = true
                ORDER BY created_at ASC LIMIT 1
            ) user_solve ON true
            WHERE dp.puzzle_date < CURRENT_DATE AND dp.status IN ('scheduled', 'archived')
            ORDER BY dp.puzzle_date DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    /// Get hints as a Vec<String>
    pub fn hints_vec(&self) -> Vec<String> {
        serde_json::from_value(self.hints.clone()).unwrap_or_default()
    }

    /// Count of future scheduled puzzles (for monitoring pool depth)
    pub async fn get_pool_depth(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM daily_puzzles WHERE puzzle_date > CURRENT_DATE AND status = 'scheduled'",
        )
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ArchiveRow {
    pub puzzle_date: Option<NaiveDate>,
    pub title: String,
    pub difficulty: i32,
    pub main_topic: String,
    pub solve_rate: f64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ArchiveRowWithUser {
    pub puzzle_date: Option<NaiveDate>,
    pub title: String,
    pub difficulty: i32,
    pub main_topic: String,
    pub solve_rate: f64,
    pub user_solved: Option<bool>,
    pub user_solved_same_day: Option<bool>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DailyPuzzleAttempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub daily_puzzle_id: Uuid,
    pub user_input: String,
    pub is_correct: bool,
    pub hints_used: i32,
    pub time_taken_ms: Option<i32>,
    pub created_at: chrono::DateTime<Utc>,
}

impl DailyPuzzleAttempt {
    /// Record a new attempt
    pub async fn create<'e, E>(
        executor: E,
        user_id: Uuid,
        daily_puzzle_id: Uuid,
        user_input: &str,
        is_correct: bool,
        hints_used: i32,
        time_taken_ms: Option<i32>,
    ) -> Result<Self, sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO daily_puzzle_attempts (user_id, daily_puzzle_id, user_input, is_correct, hints_used, time_taken_ms)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, daily_puzzle_id, user_input, is_correct, hints_used, time_taken_ms, created_at
            "#,
        )
        .bind(user_id)
        .bind(daily_puzzle_id)
        .bind(user_input)
        .bind(is_correct)
        .bind(hints_used)
        .bind(time_taken_ms)
        .fetch_one(executor)
        .await
    }

    /// Get user status for a specific daily puzzle
    pub async fn get_user_status(
        pool: &PgPool,
        user_id: Uuid,
        daily_puzzle_id: Uuid,
        puzzle_date: NaiveDate,
    ) -> Result<Option<UserPuzzleStatus>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                bool_or(is_correct) as solved,
                bool_or(is_correct AND created_at::date = $3) as solved_same_day,
                COUNT(*)::bigint as attempts,
                MAX(hints_used) as max_hints_used
            FROM daily_puzzle_attempts
            WHERE user_id = $1 AND daily_puzzle_id = $2
            HAVING COUNT(*) > 0
            "#,
        )
        .bind(user_id)
        .bind(daily_puzzle_id)
        .bind(puzzle_date)
        .fetch_optional(pool)
        .await
    }

    /// Get activity matrix data for last 365 days
    pub async fn get_activity_matrix(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<ActivityRow>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                dp.puzzle_date,
                CASE
                    WHEN dpa.id IS NULL THEN 'missed'
                    WHEN dpa.created_at::date = dp.puzzle_date THEN 'solved_same_day'
                    ELSE 'solved_late'
                END as status
            FROM daily_puzzles dp
            LEFT JOIN LATERAL (
                SELECT id, created_at FROM daily_puzzle_attempts
                WHERE user_id = $1 AND daily_puzzle_id = dp.id AND is_correct = true
                ORDER BY created_at ASC LIMIT 1
            ) dpa ON true
            WHERE dp.puzzle_date >= CURRENT_DATE - INTERVAL '365 days'
              AND dp.puzzle_date <= CURRENT_DATE
              AND dp.status IN ('scheduled', 'archived')
            ORDER BY dp.puzzle_date
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Get puzzle stats (total attempts, solves, solve rate)
    pub async fn get_puzzle_stats(
        pool: &PgPool,
        daily_puzzle_id: Uuid,
    ) -> Result<PuzzleStatsRow, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                COUNT(*)::bigint as total_attempts,
                COUNT(*) FILTER (WHERE is_correct)::bigint as total_solves,
                CASE WHEN COUNT(*) = 0 THEN 0.0
                     ELSE COUNT(*) FILTER (WHERE is_correct)::float / COUNT(DISTINCT user_id)::float
                END as solve_rate
            FROM daily_puzzle_attempts
            WHERE daily_puzzle_id = $1
            "#,
        )
        .bind(daily_puzzle_id)
        .fetch_one(pool)
        .await
    }
}

/// Update daily puzzle streak for a user (call after correct answer)
pub async fn update_daily_puzzle_streak<'e, E>(
    executor: E,
    user_id: Uuid,
) -> Result<i32, sqlx::Error>
where
    E: sqlx::PgExecutor<'e>,
{
    let row: (i32,) = sqlx::query_as(
        r#"
        UPDATE users SET
          daily_puzzle_streak = CASE
            WHEN daily_puzzle_last_solve = CURRENT_DATE THEN daily_puzzle_streak
            WHEN daily_puzzle_last_solve = CURRENT_DATE - INTERVAL '1 day' THEN daily_puzzle_streak + 1
            ELSE 1
          END,
          daily_puzzle_last_solve = CURRENT_DATE
        WHERE id = $1
        RETURNING daily_puzzle_streak
        "#,
    )
    .bind(user_id)
    .fetch_one(executor)
    .await?;
    Ok(row.0)
}

/// Get current daily puzzle streak for a user
pub async fn get_daily_puzzle_streak(pool: &PgPool, user_id: Uuid) -> Result<i32, sqlx::Error> {
    let row: (i32,) =
        sqlx::query_as("SELECT daily_puzzle_streak FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
    Ok(row.0)
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserPuzzleStatus {
    pub solved: Option<bool>,
    pub solved_same_day: Option<bool>,
    pub attempts: i64,
    pub max_hints_used: Option<i32>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActivityRow {
    pub puzzle_date: Option<NaiveDate>,
    pub status: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PuzzleStatsRow {
    pub total_attempts: i64,
    pub total_solves: i64,
    pub solve_rate: f64,
}
