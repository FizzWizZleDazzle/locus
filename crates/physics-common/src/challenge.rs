//! Challenge stage types — the Predict-Test-Reflect pedagogy model.
//!
//! Each physics problem is a sequence of [`ChallengeStage`]s that force the
//! student to *think* before the simulation reveals anything.  The simulation
//! is locked until the student commits a prediction.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Top-level challenge definition
// ============================================================================

/// The full interactive challenge associated with a physics problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemChallenge {
    pub stages: Vec<ChallengeStage>,
    /// Post-solve exploration prompts (Phase 6: "What if?").
    #[serde(default)]
    pub what_if_prompts: Vec<WhatIfPrompt>,
    /// Common errors catalogue used during the Reflection stage.
    #[serde(default)]
    pub common_errors: Vec<CommonError>,
}

// ============================================================================
// Challenge stages
// ============================================================================

/// A single step in the guided challenge flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeStage {
    /// Unique identifier within this problem (e.g. "identify_quantities").
    pub id: String,
    /// Human-readable title shown in the step navigator.
    pub title: String,
    /// The question / prompt shown to the student.
    pub prompt_text: String,
    /// Hint text available at the cost of a hint token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint_text: Option<String>,
    /// Shown when the student answers this stage correctly.
    pub success_response: String,
    /// The interactive challenge content for this stage.
    pub stage_data: StageData,
}

/// The various interactive challenge types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StageData {
    /// Phase 1: Identify which physical quantities matter.
    IdentifyQuantities {
        /// Quantities that are relevant to this problem.
        correct: Vec<QuantityOption>,
        /// Plausible but irrelevant distractors.
        distractors: Vec<QuantityOption>,
        /// Per-quantity explanation of why it is / isn't relevant.
        explanations: HashMap<String, String>,
    },

    /// Phase 2: Build a free-body diagram by placing force arrows.
    FreebodyDiagram {
        /// The body ID on which forces should be placed.
        target_body: String,
        /// The forces the student must correctly place.
        expected_forces: Vec<ForceSpec>,
        /// How close the direction needs to be (degrees).
        #[serde(default = "default_fbd_tolerance")]
        direction_tolerance_deg: f32,
        /// Per-force hint shown when that specific force is missing / wrong.
        #[serde(default)]
        per_force_hints: HashMap<String, String>,
    },

    /// Phase 3: Assemble the governing equation from term building blocks.
    EquationBuilder {
        /// Axis / context label, e.g. "parallel to the incline".
        axis_label: String,
        /// The correct ordered sequence of terms.
        correct_terms: Vec<EquationTerm>,
        /// All available terms including distractors.
        available_terms: Vec<EquationTerm>,
        /// Per-mistake feedback keyed by a serialised wrong-combo signature.
        #[serde(default)]
        error_feedback: HashMap<String, String>,
    },

    /// Phase 4: Commit a quantitative prediction before the sim unlocks.
    Prediction {
        /// The question, e.g. "What will the acceleration be?"
        question: String,
        /// Correct numeric answer.
        answer: f64,
        /// Unit label.
        unit: String,
        /// Percentage tolerance for "close enough".
        #[serde(default = "default_prediction_tolerance")]
        tolerance_pct: f32,
        /// Whether the simulation should auto-play after the prediction is locked.
        #[serde(default = "default_true")]
        sim_runs_after: bool,
        /// How far the sim runs (seconds of sim-time) after unlocking.
        #[serde(default = "default_sim_duration")]
        sim_end_time: f32,
    },

    /// Phase 5: Reflect on prediction accuracy and diagnose errors.
    Reflection {
        /// When this reflection stage triggers.
        trigger: ReflectionTrigger,
        /// The diagnostic options the student picks from.
        diagnostic_options: Vec<DiagnosticOption>,
        /// Micro-lessons keyed by `DiagnosticOption.id`.
        #[serde(default)]
        micro_lessons: HashMap<String, MicroLesson>,
    },
}

fn default_fbd_tolerance() -> f32 {
    15.0
}

fn default_prediction_tolerance() -> f32 {
    5.0
}

fn default_true() -> bool {
    true
}

fn default_sim_duration() -> f32 {
    5.0
}

// ============================================================================
// Sub-types used by stages
// ============================================================================

/// A selectable physical quantity (Phase 1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityOption {
    pub id: String,
    pub label: String,
    /// LaTeX symbol shown next to the label, e.g. `m`, `g`, `\\theta`.
    #[serde(default)]
    pub symbol_latex: String,
}

/// A force vector that should appear in the FBD (Phase 2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceSpec {
    /// Unique id, e.g. "gravity", "normal", "friction".
    pub id: String,
    /// Display label, e.g. "Weight (mg)".
    pub label: String,
    /// Expected direction in degrees (0 = right, 90 = up).
    pub direction_deg: f32,
    /// Colour used when rendering this force arrow correctly.
    #[serde(default = "default_force_color")]
    pub color: String,
    /// LaTeX label rendered alongside the arrow.
    #[serde(default)]
    pub label_latex: String,
}

fn default_force_color() -> String {
    "#ef4444".into() // red-500
}

/// A term in an equation (Phase 3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationTerm {
    /// Unique id.
    pub id: String,
    /// LaTeX representation, e.g. `mg\\sin\\theta`.
    pub latex: String,
    /// Sign: `"+"` or `"-"`.
    pub sign: String,
}

/// When a Reflection stage triggers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReflectionTrigger {
    /// Only shown when the student's prediction was wrong.
    WrongPrediction,
    /// Always shown regardless of accuracy.
    Always,
}

/// A common-error diagnosis option (Phase 5).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticOption {
    pub id: String,
    pub label: String,
    /// Is this the correct diagnosis?
    pub is_correct: bool,
}

/// A short targeted lesson shown after error diagnosis (Phase 5).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroLesson {
    /// 2-3 sentence LaTeX explanation.
    pub explanation_latex: String,
    /// Optional overlay to highlight on the canvas (e.g. "gravity_component").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_overlay: Option<String>,
}

// ============================================================================
// Post-solve exploration (Phase 6)
// ============================================================================

/// A "What if?" exploration prompt shown after the student solves the problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfPrompt {
    /// The question, e.g. "What happens if you double the mass?"
    pub question: String,
    /// Which parameter slider this relates to.
    pub parameter_key: String,
    /// Suggested value to try.
    pub suggested_value: f64,
    /// The insight the student should arrive at.
    pub expected_insight: String,
}

/// A catalogued common error for the Reflection stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonError {
    pub id: String,
    pub description: String,
    pub micro_lesson: MicroLesson,
}
