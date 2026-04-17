//! Physics problem database model

use sqlx::PgPool;
use uuid::Uuid;

use locus_physics_common::{
    AnswerSpec, PhysicsProblemResponse, PhysicsProblemSummary,
    challenge::{ChallengeStage, WhatIfPrompt},
    scene::{ParameterSpec, SceneDefinition},
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhysicsProblem {
    pub id: Uuid,
    pub title: String,
    pub description_latex: String,
    pub difficulty: i32,
    pub physics_topic: String,
    pub physics_subtopic: String,
    pub scene_definition: serde_json::Value,
    pub parameters: serde_json::Value,
    pub challenge_stages: serde_json::Value,
    pub what_if_prompts: serde_json::Value,
    pub common_errors: serde_json::Value,
    pub answer_spec: serde_json::Value,
    pub question_image: String,
}

impl PhysicsProblem {
    /// Fetch problems filtered by topic, subtopic, and/or difficulty.
    pub async fn list(
        pool: &PgPool,
        topic: Option<&str>,
        subtopic: Option<&str>,
        difficulty: Option<i32>,
        count: u32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let limit = count.max(1).min(50) as i64;

        match (topic, subtopic) {
            (Some(t), Some(st)) => {
                sqlx::query_as(
                    r#"
                    SELECT id, title, description_latex, difficulty, physics_topic, physics_subtopic,
                           scene_definition, parameters, challenge_stages, what_if_prompts,
                           common_errors, answer_spec, question_image
                    FROM physics_problems
                    WHERE physics_topic = $1 AND physics_subtopic = $2
                    ORDER BY difficulty, RANDOM()
                    LIMIT $3
                    "#,
                )
                .bind(t)
                .bind(st)
                .bind(limit)
                .fetch_all(pool)
                .await
            }
            (Some(t), None) => {
                sqlx::query_as(
                    r#"
                    SELECT id, title, description_latex, difficulty, physics_topic, physics_subtopic,
                           scene_definition, parameters, challenge_stages, what_if_prompts,
                           common_errors, answer_spec, question_image
                    FROM physics_problems
                    WHERE physics_topic = $1
                    ORDER BY difficulty, RANDOM()
                    LIMIT $2
                    "#,
                )
                .bind(t)
                .bind(limit)
                .fetch_all(pool)
                .await
            }
            _ => {
                sqlx::query_as(
                    r#"
                    SELECT id, title, description_latex, difficulty, physics_topic, physics_subtopic,
                           scene_definition, parameters, challenge_stages, what_if_prompts,
                           common_errors, answer_spec, question_image
                    FROM physics_problems
                    ORDER BY difficulty, RANDOM()
                    LIMIT $1
                    "#,
                )
                .bind(limit)
                .fetch_all(pool)
                .await
            }
        }
    }

    /// Fetch a single problem by ID.
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, title, description_latex, difficulty, physics_topic, physics_subtopic,
                   scene_definition, parameters, challenge_stages, what_if_prompts,
                   common_errors, answer_spec, question_image
            FROM physics_problems
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// Convert to summary response (no scene/challenge data).
    pub fn to_summary(&self) -> PhysicsProblemSummary {
        PhysicsProblemSummary {
            id: self.id,
            title: self.title.clone(),
            description_latex: self.description_latex.clone(),
            difficulty: self.difficulty,
            physics_topic: self.physics_topic.clone(),
            physics_subtopic: self.physics_subtopic.clone(),
            user_solved: None,
        }
    }

    /// Convert to full response.
    pub fn to_response(&self) -> PhysicsProblemResponse {
        let scene_definition: SceneDefinition =
            serde_json::from_value(self.scene_definition.clone()).unwrap_or_else(|_| {
                SceneDefinition {
                    gravity: [0.0, -9.81],
                    bodies: vec![],
                    constraints: vec![],
                    boundaries: vec![],
                    camera: [0.0, 0.0, 1.0],
                    pixels_per_metre: 50.0,
                }
            });

        let parameters: Vec<ParameterSpec> =
            serde_json::from_value(self.parameters.clone()).unwrap_or_default();

        let challenge_stages: Vec<ChallengeStage> =
            serde_json::from_value(self.challenge_stages.clone()).unwrap_or_default();

        let what_if_prompts: Vec<WhatIfPrompt> =
            serde_json::from_value(self.what_if_prompts.clone()).unwrap_or_default();

        let answer_spec: AnswerSpec =
            serde_json::from_value(self.answer_spec.clone()).unwrap_or(AnswerSpec { parts: vec![] });

        PhysicsProblemResponse {
            id: self.id,
            title: self.title.clone(),
            description_latex: self.description_latex.clone(),
            difficulty: self.difficulty,
            physics_topic: self.physics_topic.clone(),
            physics_subtopic: self.physics_subtopic.clone(),
            scene_definition,
            parameters,
            challenge_stages,
            what_if_prompts,
            answer_spec,
            question_image: self.question_image.clone(),
            user_solved: None,
        }
    }
}
