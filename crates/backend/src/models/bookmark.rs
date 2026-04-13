//! Problem bookmark model

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use locus_common::{BookmarkItem, BookmarkListResponse};

pub struct Bookmark;

#[derive(Debug, Clone, sqlx::FromRow)]
struct BookmarkRow {
    pub problem_id: Uuid,
    pub bookmarked_at: DateTime<Utc>,
    pub question_latex: String,
    pub answer_key: String,
    pub solution_latex: String,
    pub main_topic: String,
    pub subtopic: String,
    pub difficulty: i32,
}

impl From<BookmarkRow> for BookmarkItem {
    fn from(r: BookmarkRow) -> Self {
        Self {
            problem_id: r.problem_id,
            bookmarked_at: r.bookmarked_at,
            question_latex: r.question_latex,
            answer_key: r.answer_key,
            solution_latex: r.solution_latex,
            main_topic: r.main_topic,
            subtopic: r.subtopic,
            difficulty: r.difficulty,
        }
    }
}

impl Bookmark {
    /// Add a bookmark (idempotent)
    pub async fn add(pool: &PgPool, user_id: Uuid, problem_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO problem_bookmarks (user_id, problem_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(problem_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Remove a bookmark
    pub async fn remove(
        pool: &PgPool,
        user_id: Uuid,
        problem_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM problem_bookmarks WHERE user_id = $1 AND problem_id = $2",
        )
        .bind(user_id)
        .bind(problem_id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// List bookmarked problems with pagination and optional topic filter
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        main_topic: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<BookmarkListResponse, sqlx::Error> {
        if let Some(topic) = main_topic {
            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM problem_bookmarks b JOIN problems p ON p.id = b.problem_id WHERE b.user_id = $1 AND p.main_topic = $2",
            )
            .bind(user_id)
            .bind(topic)
            .fetch_one(pool)
            .await?;

            let rows: Vec<BookmarkRow> = sqlx::query_as(
                r#"
                SELECT b.problem_id, b.created_at as bookmarked_at,
                       p.question_latex, p.answer_key, p.solution_latex, p.main_topic, p.subtopic, p.difficulty
                FROM problem_bookmarks b
                JOIN problems p ON p.id = b.problem_id
                WHERE b.user_id = $1 AND p.main_topic = $2
                ORDER BY b.created_at DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(user_id)
            .bind(topic)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

            Ok(BookmarkListResponse {
                items: rows.into_iter().map(|r| r.into()).collect(),
                total: total.0,
            })
        } else {
            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM problem_bookmarks WHERE user_id = $1",
            )
            .bind(user_id)
            .fetch_one(pool)
            .await?;

            let rows: Vec<BookmarkRow> = sqlx::query_as(
                r#"
                SELECT b.problem_id, b.created_at as bookmarked_at,
                       p.question_latex, p.answer_key, p.solution_latex, p.main_topic, p.subtopic, p.difficulty
                FROM problem_bookmarks b
                JOIN problems p ON p.id = b.problem_id
                WHERE b.user_id = $1
                ORDER BY b.created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

            Ok(BookmarkListResponse {
                items: rows.into_iter().map(|r| r.into()).collect(),
                total: total.0,
            })
        }
    }

    /// Batch check which problem IDs are bookmarked by a user
    pub async fn check(
        pool: &PgPool,
        user_id: Uuid,
        problem_ids: &[Uuid],
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        if problem_ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows: Vec<(Uuid,)> = sqlx::query_as(
            "SELECT problem_id FROM problem_bookmarks WHERE user_id = $1 AND problem_id = ANY($2)",
        )
        .bind(user_id)
        .bind(problem_ids)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(id,)| id).collect())
    }
}
