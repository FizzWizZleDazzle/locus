//! Problem model

use sqlx::PgPool;
use uuid::Uuid;

use locus_common::{
    AnswerType, GradingMode, ProblemResponse,
    constants::{DEFAULT_DIFFICULTY, PROBLEM_BATCH_MAX},
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Problem {
    pub id: Uuid,
    pub question_latex: String,
    pub answer_key: String,
    pub difficulty: i32,
    pub main_topic: String,
    pub subtopic: String,
    pub grading_mode: String,
    pub answer_type: String,
    pub calculator_allowed: String,
    pub solution_latex: String,
    pub question_image: String,
    pub time_limit_seconds: Option<i32>,
}

impl Problem {
    /// Get a random problem matching the given criteria
    pub async fn get_random(
        pool: &PgPool,
        target_elo: Option<i32>,
        main_topic: Option<&str>,
        subtopics: Option<&[String]>,
    ) -> Result<Option<Self>, sqlx::Error> {
        let target = target_elo.unwrap_or(DEFAULT_DIFFICULTY);

        // Build query based on filters
        match (main_topic, subtopics) {
            (Some(mt), Some(st)) if !st.is_empty() => {
                // Filter by main_topic and subtopics
                // Placeholders: $1=main_topic, $2...$N=subtopics, $N+1=target
                let subtopic_placeholders = (2..=st.len() + 1)
                    .map(|i| format!("${}", i))
                    .collect::<Vec<_>>()
                    .join(", ");
                let target_placeholder = st.len() + 2;

                let query_str = format!(
                    r#"
                    SELECT id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds
                    FROM problems
                    WHERE main_topic = $1 AND subtopic IN ({})
                    ORDER BY ABS(difficulty - ${}) + (RANDOM() * 200)
                    LIMIT 1
                    "#,
                    subtopic_placeholders, target_placeholder
                );

                let mut query = sqlx::query_as(&query_str).bind(mt);
                for st_val in st {
                    query = query.bind(st_val);
                }
                query = query.bind(target);
                query.fetch_optional(pool).await
            }
            (Some(mt), _) => {
                // Filter by main_topic only
                sqlx::query_as(
                    r#"
                    SELECT id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds
                    FROM problems
                    WHERE main_topic = $1
                    ORDER BY ABS(difficulty - $2) + (RANDOM() * 200)
                    LIMIT 1
                    "#,
                )
                .bind(mt)
                .bind(target)
                .fetch_optional(pool)
                .await
            }
            _ => {
                // No filter, any problem
                sqlx::query_as(
                    r#"
                    SELECT id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds
                    FROM problems
                    ORDER BY ABS(difficulty - $1) + (RANDOM() * 200)
                    LIMIT 1
                    "#,
                )
                .bind(target)
                .fetch_optional(pool)
                .await
            }
        }
    }

    /// Get multiple random problems matching the given criteria
    pub async fn get_random_batch(
        pool: &PgPool,
        target_elo: Option<i32>,
        main_topic: Option<&str>,
        subtopics: Option<&[String]>,
        count: u32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let target = target_elo.unwrap_or(DEFAULT_DIFFICULTY);
        let limit = count.max(1).min(PROBLEM_BATCH_MAX) as i64;

        match (main_topic, subtopics) {
            (Some(mt), Some(st)) if !st.is_empty() => {
                let subtopic_placeholders = (2..=st.len() + 1)
                    .map(|i| format!("${}", i))
                    .collect::<Vec<_>>()
                    .join(", ");
                let target_placeholder = st.len() + 2;
                let limit_placeholder = st.len() + 3;

                let query_str = format!(
                    r#"
                    SELECT id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds
                    FROM problems
                    WHERE main_topic = $1 AND subtopic IN ({})
                    ORDER BY ABS(difficulty - ${}) + (RANDOM() * 200)
                    LIMIT ${}
                    "#,
                    subtopic_placeholders, target_placeholder, limit_placeholder
                );

                let mut query = sqlx::query_as(&query_str).bind(mt);
                for st_val in st {
                    query = query.bind(st_val);
                }
                query = query.bind(target).bind(limit);
                query.fetch_all(pool).await
            }
            (Some(mt), _) => {
                sqlx::query_as(
                    r#"
                    SELECT id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds
                    FROM problems
                    WHERE main_topic = $1
                    ORDER BY ABS(difficulty - $2) + (RANDOM() * 200)
                    LIMIT $3
                    "#,
                )
                .bind(mt)
                .bind(target)
                .bind(limit)
                .fetch_all(pool)
                .await
            }
            _ => {
                sqlx::query_as(
                    r#"
                    SELECT id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds
                    FROM problems
                    ORDER BY ABS(difficulty - $1) + (RANDOM() * 200)
                    LIMIT $2
                    "#,
                )
                .bind(target)
                .bind(limit)
                .fetch_all(pool)
                .await
            }
        }
    }

    /// Get a problem by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds
            FROM problems
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// Insert a new problem
    pub async fn create(
        pool: &PgPool,
        question_latex: &str,
        answer_key: &str,
        difficulty: i32,
        main_topic: &str,
        subtopic: &str,
        grading_mode: GradingMode,
        answer_type: AnswerType,
        calculator_allowed: &str,
        solution_latex: &str,
        question_image: &str,
        time_limit_seconds: Option<i32>,
    ) -> Result<Self, sqlx::Error> {
        let mode_str = match grading_mode {
            GradingMode::Equivalent => "equivalent",
            GradingMode::Factor => "factor",
            GradingMode::Expand => "expand",
        };

        sqlx::query_as(
            r#"
            INSERT INTO problems (question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds
            "#,
        )
        .bind(question_latex)
        .bind(answer_key)
        .bind(difficulty)
        .bind(main_topic)
        .bind(subtopic)
        .bind(mode_str)
        .bind(answer_type.as_str())
        .bind(calculator_allowed)
        .bind(solution_latex)
        .bind(question_image)
        .bind(time_limit_seconds)
        .fetch_one(pool)
        .await
    }

    /// Get grading mode enum
    pub fn get_grading_mode(&self) -> GradingMode {
        match self.grading_mode.as_str() {
            "factor" => GradingMode::Factor,
            "expand" => GradingMode::Expand,
            _ => GradingMode::Equivalent,
        }
    }

    /// Get answer type enum
    pub fn get_answer_type(&self) -> AnswerType {
        AnswerType::from_str(&self.answer_type).unwrap_or_default()
    }

    /// Convert to API response (with or without answer)
    pub fn to_response(&self, include_answer: bool) -> ProblemResponse {
        ProblemResponse {
            id: self.id,
            question_latex: self.question_latex.clone(),
            difficulty: self.difficulty,
            main_topic: self.main_topic.clone(),
            subtopic: self.subtopic.clone(),
            grading_mode: self.get_grading_mode(),
            answer_type: self.get_answer_type(),
            calculator_allowed: self.calculator_allowed.clone(),
            answer_key: if include_answer {
                Some(self.answer_key.clone())
            } else {
                None
            },
            solution_latex: if include_answer {
                self.solution_latex.clone()
            } else {
                String::new()
            },
            question_image: self.question_image.clone(),
            time_limit_seconds: self.time_limit_seconds,
        }
    }
}
