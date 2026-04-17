//! Shared types for the Locus physics learning platform.
//!
//! This crate defines the data models shared between the backend (Axum) and
//! frontend (Leptos WASM).  It intentionally has **no** dependency on Rapier2D
//! or any rendering library so it compiles cleanly for both native and WASM
//! targets.

pub mod challenge;
pub mod scene;
pub mod scoring;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Physics Topics
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PhysicsTopic {
    Mechanics,
    Waves,
    Electricity,
}

impl PhysicsTopic {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Mechanics => "Mechanics",
            Self::Waves => "Waves & Oscillations",
            Self::Electricity => "Electricity & Circuits",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mechanics => "mechanics",
            Self::Waves => "waves",
            Self::Electricity => "electricity",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "mechanics" => Some(Self::Mechanics),
            "waves" => Some(Self::Waves),
            "electricity" => Some(Self::Electricity),
            _ => None,
        }
    }

    pub fn subtopics(&self) -> &'static [PhysicsSubtopic] {
        match self {
            Self::Mechanics => &[
                PhysicsSubtopic::ProjectileMotion,
                PhysicsSubtopic::InclinedPlane,
                PhysicsSubtopic::Collisions,
                PhysicsSubtopic::Springs,
                PhysicsSubtopic::Friction,
                PhysicsSubtopic::CircularMotion,
            ],
            Self::Waves => &[
                PhysicsSubtopic::Pendulum,
                PhysicsSubtopic::SpringOscillation,
            ],
            Self::Electricity => &[],
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Mechanics, Self::Waves, Self::Electricity]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PhysicsSubtopic {
    ProjectileMotion,
    InclinedPlane,
    Collisions,
    Springs,
    Friction,
    CircularMotion,
    Pendulum,
    SpringOscillation,
}

impl PhysicsSubtopic {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::ProjectileMotion => "Projectile Motion",
            Self::InclinedPlane => "Inclined Planes",
            Self::Collisions => "Collisions",
            Self::Springs => "Springs & Hooke's Law",
            Self::Friction => "Friction",
            Self::CircularMotion => "Circular Motion",
            Self::Pendulum => "Pendulums",
            Self::SpringOscillation => "Spring Oscillation",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectileMotion => "projectile_motion",
            Self::InclinedPlane => "inclined_plane",
            Self::Collisions => "collisions",
            Self::Springs => "springs",
            Self::Friction => "friction",
            Self::CircularMotion => "circular_motion",
            Self::Pendulum => "pendulum",
            Self::SpringOscillation => "spring_oscillation",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "projectile_motion" => Some(Self::ProjectileMotion),
            "inclined_plane" => Some(Self::InclinedPlane),
            "collisions" => Some(Self::Collisions),
            "springs" => Some(Self::Springs),
            "friction" => Some(Self::Friction),
            "circular_motion" => Some(Self::CircularMotion),
            "pendulum" => Some(Self::Pendulum),
            "spring_oscillation" => Some(Self::SpringOscillation),
            _ => None,
        }
    }
}

// ============================================================================
// API Request / Response Types
// ============================================================================

/// Summary returned in problem list views (no scene data).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsProblemSummary {
    pub id: Uuid,
    pub title: String,
    pub description_latex: String,
    pub difficulty: i32,
    pub physics_topic: String,
    pub physics_subtopic: String,
    /// Whether the current user has solved this problem (None if not authenticated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_solved: Option<bool>,
}

/// Full problem payload including scene definition and challenge stages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsProblemResponse {
    pub id: Uuid,
    pub title: String,
    pub description_latex: String,
    pub difficulty: i32,
    pub physics_topic: String,
    pub physics_subtopic: String,
    pub scene_definition: scene::SceneDefinition,
    pub parameters: Vec<scene::ParameterSpec>,
    pub challenge_stages: Vec<challenge::ChallengeStage>,
    pub what_if_prompts: Vec<challenge::WhatIfPrompt>,
    pub answer_spec: AnswerSpec,
    #[serde(default)]
    pub question_image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_solved: Option<bool>,
}

/// Specification for the numeric answers the student must produce.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerSpec {
    pub parts: Vec<AnswerPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerPart {
    pub label: String,
    pub unit: String,
    /// The correct numeric answer.
    pub answer: f64,
    /// Absolute tolerance for grading (e.g. 0.1 means +/- 0.1).
    pub tolerance: f64,
}

/// Request body for submitting a physics answer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSubmitRequest {
    pub problem_id: Uuid,
    pub answers: Vec<PhysicsAnswerInput>,
    #[serde(default)]
    pub parameters_used: serde_json::Value,
    #[serde(default)]
    pub hints_used: i32,
    #[serde(default)]
    pub fbd_attempts: i32,
    #[serde(default)]
    pub stages_completed: i32,
    #[serde(default)]
    pub what_ifs_explored: i32,
    #[serde(default)]
    pub time_taken_ms: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsAnswerInput {
    pub part_index: usize,
    pub value: f64,
}

/// Response after submitting a physics answer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSubmitResponse {
    pub is_correct: bool,
    /// Per-part correctness.
    pub part_results: Vec<bool>,
    pub score: scoring::AttemptScore,
}

/// Query parameters for fetching physics problems.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicsProblemsQuery {
    #[serde(default)]
    pub physics_topic: Option<String>,
    #[serde(default)]
    pub physics_subtopic: Option<String>,
    #[serde(default)]
    pub difficulty: Option<i32>,
    #[serde(default = "default_physics_count")]
    pub count: u32,
}

fn default_physics_count() -> u32 {
    10
}

/// Per-topic progress for the authenticated user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsProgressEntry {
    pub topic: String,
    pub problems_attempted: i64,
    pub problems_solved: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsProgressResponse {
    pub entries: Vec<PhysicsProgressEntry>,
}

/// Topic listing (mirrors the math topics endpoint shape).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsTopicInfo {
    pub id: String,
    pub display_name: String,
    pub sort_order: i32,
    pub enabled: bool,
    pub subtopics: Vec<PhysicsSubtopicInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSubtopicInfo {
    pub id: String,
    pub display_name: String,
    pub sort_order: i32,
    pub enabled: bool,
}
