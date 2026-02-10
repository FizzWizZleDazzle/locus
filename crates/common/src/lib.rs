//! Common types shared between frontend and backend

pub mod grader;
pub mod symengine;
pub mod elo;
pub mod validation;
pub mod latex;
pub mod constants;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ============================================================================
// Topics and Subtopics
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MainTopic {
    Arithmetic,
    Algebra1,
    Geometry,
    Algebra2,
    Precalculus,
    Calculus,
    MultivariableCalculus,
    LinearAlgebra,
}

impl MainTopic {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Arithmetic => "Arithmetic",
            Self::Algebra1 => "Algebra 1",
            Self::Geometry => "Geometry",
            Self::Algebra2 => "Algebra 2",
            Self::Precalculus => "Precalculus",
            Self::Calculus => "Calculus",
            Self::MultivariableCalculus => "Multivariable Calculus",
            Self::LinearAlgebra => "Linear Algebra",
        }
    }

    pub fn subtopics(&self) -> &'static [&'static str] {
        match self {
            Self::Arithmetic => &[
                "addition_subtraction",
                "multiplication_division",
                "fractions",
                "decimals",
                "percentages",
                "order_of_operations",
            ],
            Self::Algebra1 => &[
                "linear_equations",
                "inequalities",
                "graphing_lines",
                "systems_of_equations",
                "exponents",
                "polynomials",
                "factoring",
                "quadratic_equations",
            ],
            Self::Geometry => &[
                "angles",
                "triangles",
                "circles",
                "area_perimeter",
                "volume_surface_area",
                "pythagorean_theorem",
                "trigonometry_basics",
            ],
            Self::Algebra2 => &[
                "complex_numbers",
                "rational_expressions",
                "radical_expressions",
                "exponential_functions",
                "logarithms",
                "sequences_series",
                "conic_sections",
            ],
            Self::Precalculus => &[
                "functions",
                "trigonometric_functions",
                "trigonometric_identities",
                "inverse_trig",
                "polar_coordinates",
                "vectors",
                "matrices",
            ],
            Self::Calculus => &[
                "limits",
                "derivatives",
                "integration",
                "applications_of_derivatives",
                "applications_of_integration",
            ],
            Self::MultivariableCalculus => &[
                "partial_derivatives",
                "multiple_integrals",
                "vector_calculus",
                "line_integrals",
                "surface_integrals",
            ],
            Self::LinearAlgebra => &[
                "matrix_operations",
                "determinants",
                "vector_spaces",
                "eigenvalues_eigenvectors",
                "linear_transformations",
            ],
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Arithmetic,
            Self::Algebra1,
            Self::Geometry,
            Self::Algebra2,
            Self::Precalculus,
            Self::Calculus,
            Self::MultivariableCalculus,
            Self::LinearAlgebra,
        ]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "arithmetic" => Some(Self::Arithmetic),
            "algebra1" => Some(Self::Algebra1),
            "geometry" => Some(Self::Geometry),
            "algebra2" => Some(Self::Algebra2),
            "precalculus" => Some(Self::Precalculus),
            "calculus" => Some(Self::Calculus),
            "multivariable_calculus" => Some(Self::MultivariableCalculus),
            "linear_algebra" => Some(Self::LinearAlgebra),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Arithmetic => "arithmetic",
            Self::Algebra1 => "algebra1",
            Self::Geometry => "geometry",
            Self::Algebra2 => "algebra2",
            Self::Precalculus => "precalculus",
            Self::Calculus => "calculus",
            Self::MultivariableCalculus => "multivariable_calculus",
            Self::LinearAlgebra => "linear_algebra",
        }
    }
}

pub fn subtopic_display_name(subtopic: &str) -> String {
    subtopic.replace("_", " ")
        .split(' ')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ============================================================================
// Grading
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GradingMode {
    /// Check if expressions are mathematically equivalent
    Equivalent,
    /// Check if expression is in factored form and equals answer
    Factor,
    /// Check if expression is in expanded form and equals answer
    Expand,
}

impl Default for GradingMode {
    fn default() -> Self {
        Self::Equivalent
    }
}

// ============================================================================
// API Request/Response Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub elo_arithmetic: i32,
    pub elo_algebra1: i32,
    pub elo_geometry: i32,
    pub elo_algebra2: i32,
    pub elo_precalculus: i32,
    pub elo_calculus: i32,
    pub elo_multivariable_calculus: i32,
    pub elo_linear_algebra: i32,
    pub has_password: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPasswordRequest {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemResponse {
    pub id: Uuid,
    pub question_latex: String,
    pub difficulty: i32,
    pub main_topic: String,
    pub subtopic: String,
    pub grading_mode: GradingMode,
    /// Only included for practice mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitRequest {
    pub problem_id: Uuid,
    pub user_input: String,
    #[serde(default)]
    pub time_taken_ms: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitResponse {
    pub is_correct: bool,
    pub elo_before: i32,
    pub elo_after: i32,
    pub elo_change: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i32,
    pub username: String,
    pub elo: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntry>,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemQuery {
    /// Target ELO for problem difficulty
    #[serde(default)]
    pub elo: Option<i32>,
    /// Main topic
    #[serde(default)]
    pub main_topic: Option<String>,
    /// Subtopic filters (comma-separated)
    #[serde(default)]
    pub subtopics: Option<String>,
    /// Whether this is practice mode (includes answer)
    #[serde(default)]
    pub practice: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn new(error: impl Into<String>) -> Self {
        Self { error: error.into() }
    }
}

// ============================================================================
// Factory Submission Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProblemRequest {
    pub question_latex: String,
    pub answer_key: String,
    pub difficulty: i32,
    pub main_topic: String,
    pub subtopic: String,
    pub grading_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProblemResponse {
    pub id: Uuid,
    pub message: String,
}
