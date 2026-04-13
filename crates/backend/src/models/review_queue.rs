//! Spaced repetition review queue model (SM-2 algorithm)

use sqlx::PgPool;
use uuid::Uuid;

use locus_common::{ReviewCompleteResponse, ReviewQueueItem, ReviewQueueResponse};

pub struct ReviewQueue;

#[derive(Debug, Clone, sqlx::FromRow)]
struct ReviewQueueRow {
    pub review_id: Uuid,
    pub problem_id: Uuid,
    pub question_latex: String,
    pub answer_key: String,
    pub solution_latex: String,
    pub main_topic: String,
    pub subtopic: String,
    pub difficulty: i32,
    pub grading_mode: String,
    pub answer_type: String,
    pub review_count: i32,
    pub question_image: String,
}

impl From<ReviewQueueRow> for ReviewQueueItem {
    fn from(r: ReviewQueueRow) -> Self {
        Self {
            review_id: r.review_id,
            problem_id: r.problem_id,
            question_latex: r.question_latex,
            answer_key: r.answer_key,
            solution_latex: r.solution_latex,
            main_topic: r.main_topic,
            subtopic: r.subtopic,
            difficulty: r.difficulty,
            grading_mode: match r.grading_mode.as_str() {
                "factor" => locus_common::GradingMode::Factor,
                "expand" => locus_common::GradingMode::Expand,
                _ => locus_common::GradingMode::Equivalent,
            },
            answer_type: locus_common::AnswerType::from_str(&r.answer_type).unwrap_or_default(),
            review_count: r.review_count,
            question_image: r.question_image,
        }
    }
}

impl ReviewQueue {
    /// Add a problem to the review queue, or reset its schedule if it already exists
    pub async fn add_or_reset<'e, E>(
        executor: E,
        user_id: Uuid,
        problem_id: Uuid,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query(
            r#"
            INSERT INTO review_queue (user_id, problem_id, next_review, interval_days, ease_factor, review_count)
            VALUES ($1, $2, CURRENT_DATE, 1, 2.5, 0)
            ON CONFLICT (user_id, problem_id)
            DO UPDATE SET next_review = CURRENT_DATE, interval_days = 1, ease_factor = 2.5, review_count = 0
            "#,
        )
        .bind(user_id)
        .bind(problem_id)
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Get due review items (next_review <= today) with problem details
    pub async fn get_due(
        pool: &PgPool,
        user_id: Uuid,
        limit: i64,
    ) -> Result<ReviewQueueResponse, sqlx::Error> {
        let rows: Vec<ReviewQueueRow> = sqlx::query_as(
            r#"
            SELECT r.id as review_id, r.problem_id,
                   p.question_latex, p.answer_key, p.solution_latex, p.main_topic, p.subtopic,
                   p.difficulty, p.grading_mode, p.answer_type, r.review_count, p.question_image
            FROM review_queue r
            JOIN problems p ON p.id = r.problem_id
            WHERE r.user_id = $1 AND r.next_review <= CURRENT_DATE
            ORDER BY r.next_review ASC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let due_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM review_queue WHERE user_id = $1 AND next_review <= CURRENT_DATE",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let upcoming_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM review_queue WHERE user_id = $1 AND next_review > CURRENT_DATE",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(ReviewQueueResponse {
            items: rows.into_iter().map(|r| r.into()).collect(),
            due_count: due_count.0,
            upcoming_count: upcoming_count.0,
        })
    }

    /// Complete a review using SM-2 algorithm
    /// quality: 0-5 rating (0-2 = forgot, 3 = hard, 4 = good, 5 = easy)
    pub async fn complete_review(
        pool: &PgPool,
        review_id: Uuid,
        user_id: Uuid,
        quality: i32,
    ) -> Result<ReviewCompleteResponse, sqlx::Error> {
        // Get current review state
        let row: (i32, f32, i32) = sqlx::query_as(
            "SELECT interval_days, ease_factor, review_count FROM review_queue WHERE id = $1 AND user_id = $2",
        )
        .bind(review_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let (interval, ease, count) = (row.0, row.1, row.2);
        let q = quality.clamp(0, 5);

        // SM-2 algorithm
        let (new_interval, new_ease) = if q < 3 {
            // Failed: reset interval
            (1, ease)
        } else {
            let new_interval = match count {
                0 => 1,
                1 => 6,
                _ => (interval as f32 * ease).round() as i32,
            };
            // Update ease factor
            let new_ease = (ease + 0.1 - (5.0 - q as f32) * (0.08 + (5.0 - q as f32) * 0.02))
                .max(1.3);
            (new_interval.max(1), new_ease)
        };

        let next_review =
            chrono::Utc::now().date_naive() + chrono::Duration::days(new_interval as i64);

        sqlx::query(
            r#"
            UPDATE review_queue
            SET interval_days = $1, ease_factor = $2, review_count = review_count + 1, next_review = $3
            WHERE id = $4 AND user_id = $5
            "#,
        )
        .bind(new_interval)
        .bind(new_ease)
        .bind(next_review)
        .bind(review_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(ReviewCompleteResponse {
            next_review,
            new_interval,
        })
    }
}
