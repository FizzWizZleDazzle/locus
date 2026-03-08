//! Common types shared between frontend and backend

pub mod badges;
pub mod constants;
pub mod elo;
pub mod grader;
pub mod latex;
pub mod svg_compress;
pub mod symengine;
pub mod validation;

use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    DifferentialEquations,
    MultivariableCalculus,
    LinearAlgebra,
    Test,
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
            Self::DifferentialEquations => "Differential Equations",
            Self::MultivariableCalculus => "Multivariable Calculus",
            Self::LinearAlgebra => "Linear Algebra",
            Self::Test => "Test",
        }
    }

    pub fn subtopics(&self) -> &'static [&'static str] {
        match self {
            Self::Arithmetic => &[
                "addition",
                "subtraction",
                "multiplication",
                "long_division",
                "fractions",
                "mixed_numbers",
                "decimals",
                "percentages",
                "order_of_operations",
                "ratios_proportions",
            ],
            Self::Algebra1 => &[
                "one_step_equations",
                "two_step_equations",
                "multi_step_equations",
                "linear_inequalities",
                "compound_inequalities",
                "slope_and_intercept",
                "graphing_lines",
                "systems_substitution",
                "systems_elimination",
                "exponent_rules",
                "polynomial_operations",
                "factoring_gcf",
                "factoring_trinomials",
                "quadratic_formula",
            ],
            Self::Geometry => &[
                "angle_relationships",
                "triangle_properties",
                "triangle_congruence",
                "similar_triangles",
                "circle_theorems",
                "arc_length_sectors",
                "area_of_polygons",
                "perimeter",
                "surface_area",
                "volume",
                "pythagorean_theorem",
                "right_triangle_trig",
            ],
            Self::Algebra2 => &[
                "complex_number_operations",
                "complex_number_equations",
                "rational_expressions",
                "rational_equations",
                "radical_expressions",
                "radical_equations",
                "exponential_growth_decay",
                "exponential_equations",
                "logarithm_properties",
                "logarithmic_equations",
                "arithmetic_sequences",
                "geometric_sequences",
            ],
            Self::Precalculus => &[
                "domain_and_range",
                "function_composition",
                "inverse_functions",
                "transformations",
                "unit_circle",
                "graphing_trig",
                "trig_identities",
                "sum_difference_formulas",
                "inverse_trig_functions",
                "law_of_sines_cosines",
                "polar_coordinates",
                "polar_curves",
                "vector_operations",
                "dot_cross_product",
            ],
            Self::Calculus => &[
                "limits_algebraic",
                "limits_at_infinity",
                "continuity",
                "derivative_rules",
                "chain_rule",
                "implicit_differentiation",
                "related_rates",
                "curve_sketching",
                "optimization",
                "lhopitals_rule",
                "antiderivatives",
                "u_substitution",
                "integration_by_parts",
                "definite_integrals",
                "area_between_curves",
                "volumes_of_revolution",
            ],
            Self::DifferentialEquations => &[
                "separable_equations",
                "first_order_linear",
                "exact_equations",
                "homogeneous_equations",
                "second_order_constant",
                "characteristic_equation",
                "undetermined_coefficients",
                "variation_of_parameters",
                "laplace_transforms",
                "systems_of_odes",
            ],
            Self::MultivariableCalculus => &[
                "partial_derivatives",
                "gradient",
                "directional_derivatives",
                "lagrange_multipliers",
                "double_integrals",
                "triple_integrals",
                "change_of_variables",
                "line_integrals",
                "greens_theorem",
                "stokes_divergence",
            ],
            Self::LinearAlgebra => &[
                "row_reduction",
                "matrix_arithmetic",
                "matrix_inverses",
                "determinants",
                "vector_spaces",
                "subspaces",
                "linear_independence",
                "eigenvalues",
                "diagonalization",
                "linear_transformations",
            ],
            Self::Test => &[
                "expressions",
                "numerics",
                "sets",
                "tuples",
                "lists",
                "intervals",
                "inequalities",
                "equations",
                "booleans",
                "words",
                "matrices",
                "multipart",
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
            Self::DifferentialEquations,
            Self::MultivariableCalculus,
            Self::LinearAlgebra,
            Self::Test,
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
            "differential_equations" => Some(Self::DifferentialEquations),
            "multivariable_calculus" => Some(Self::MultivariableCalculus),
            "linear_algebra" => Some(Self::LinearAlgebra),
            "test" => Some(Self::Test),
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
            Self::DifferentialEquations => "differential_equations",
            Self::MultivariableCalculus => "multivariable_calculus",
            Self::LinearAlgebra => "linear_algebra",
            Self::Test => "test",
        }
    }
}

pub fn subtopic_display_name(subtopic: &str) -> String {
    subtopic
        .replace("_", " ")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AnswerType {
    #[default]
    Expression,
    Numeric,
    Set,
    Tuple,
    List,
    Interval,
    Inequality,
    Equation,
    Boolean,
    Word,
    Matrix,
    MultiPart,
}

impl AnswerType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "expression" => Some(Self::Expression),
            "numeric" => Some(Self::Numeric),
            "set" => Some(Self::Set),
            "tuple" => Some(Self::Tuple),
            "list" => Some(Self::List),
            "interval" => Some(Self::Interval),
            "inequality" => Some(Self::Inequality),
            "equation" => Some(Self::Equation),
            "boolean" => Some(Self::Boolean),
            "word" => Some(Self::Word),
            "matrix" => Some(Self::Matrix),
            "multi_part" => Some(Self::MultiPart),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Expression => "expression",
            Self::Numeric => "numeric",
            Self::Set => "set",
            Self::Tuple => "tuple",
            Self::List => "list",
            Self::Interval => "interval",
            Self::Inequality => "inequality",
            Self::Equation => "equation",
            Self::Boolean => "boolean",
            Self::Word => "word",
            Self::Matrix => "matrix",
            Self::MultiPart => "multi_part",
        }
    }

    /// Returns an answer format hint for the frontend to display, or None if no hint needed.
    pub fn hint(&self) -> Option<&'static str> {
        match self {
            Self::Expression | Self::Numeric => None,
            Self::Set => Some("Enter as a set, e.g. {1, 2, 3}"),
            Self::Tuple => Some("Enter as an ordered pair, e.g. (3, 5)"),
            Self::List => Some("Enter as a list, e.g. [-3, -1]"),
            Self::Interval => Some("Use interval notation, e.g. (1, 7] or [-2, 4)"),
            Self::Inequality => Some("Enter an inequality, e.g. x > -4"),
            Self::Equation => Some("Enter an equation, e.g. y = 2x + 3"),
            Self::Boolean => Some("Answer: true or false"),
            Self::Word => Some("Type your answer as a word"),
            Self::Matrix => Some("Enter as [[row1], [row2]], e.g. [[1, 2], [3, 4]]"),
            Self::MultiPart => Some("Enter each part separately"),
        }
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
    pub accepted_tos: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub elo_ratings: HashMap<String, i32>,
    pub has_password: bool,
    pub oauth_providers: Vec<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub current_streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPasswordRequest {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeUsernameRequest {
    pub new_username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccountRequest {
    pub password: Option<String>,
    pub confirmation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkOAuthRequest {
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemResponse {
    pub id: Uuid,
    pub question_latex: String,
    pub difficulty: i32,
    pub main_topic: String,
    pub subtopic: String,
    pub grading_mode: GradingMode,
    pub answer_type: AnswerType,
    pub calculator_allowed: String,
    /// Only included for practice mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_key: Option<String>,
    #[serde(default)]
    pub solution_latex: String,
    #[serde(default)]
    pub question_image: String,
    #[serde(default)]
    pub time_limit_seconds: Option<i32>,
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
    #[serde(default)]
    pub topic_streak: i32,
    /// Returned only when is_correct == false (safe since attempt is already recorded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_latex: Option<String>,
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
    /// Number of problems to fetch (for /problems endpoint, default 30)
    #[serde(default = "default_count")]
    pub count: u32,
}

fn default_count() -> u32 {
    constants::PROBLEM_BATCH_SIZE
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

// ============================================================================
// Daily Puzzle Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySubmitRequest {
    pub daily_puzzle_id: Uuid,
    pub user_input: String,
    #[serde(default)]
    pub hints_used: i32,
    #[serde(default)]
    pub time_taken_ms: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyPuzzleResponse {
    pub id: Uuid,
    pub puzzle_date: NaiveDate,
    pub title: String,
    pub problem: ProblemResponse,
    pub hints_available: usize,
    pub source: String,
    pub user_status: Option<DailyUserStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyUserStatus {
    pub solved: bool,
    pub solved_same_day: bool,
    pub attempts: i64,
    pub hints_revealed: i32,
    pub streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySubmitResponse {
    pub is_correct: bool,
    pub attempts: i64,
    pub solved: bool,
    pub streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyArchiveEntry {
    pub puzzle_date: NaiveDate,
    pub title: String,
    pub difficulty: i32,
    pub main_topic: String,
    pub solve_rate: f64,
    pub user_solved: Option<bool>,
    pub user_solved_same_day: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyPuzzleDetailResponse {
    pub id: Uuid,
    pub puzzle_date: NaiveDate,
    pub title: String,
    pub problem: ProblemResponse,
    pub editorial_latex: String,
    pub hints: Vec<String>,
    pub source: String,
    pub stats: DailyPuzzleStats,
    pub user_status: Option<DailyUserStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyPuzzleStats {
    pub total_attempts: i64,
    pub total_solves: i64,
    pub solve_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyActivityResponse {
    pub streak: i32,
    pub days: Vec<DailyActivityDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyActivityDay {
    pub date: NaiveDate,
    pub status: DailyDayStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DailyDayStatus {
    NoPuzzle,
    Missed,
    SolvedLate,
    SolvedSameDay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyArchiveQuery {
    #[serde(default = "default_archive_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_archive_limit() -> i64 {
    30
}

// ============================================================================
// Stats Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicStatsEntry {
    pub topic: String,
    pub total: i64,
    pub correct: i64,
    pub elo: i32,
    pub peak_elo: i32,
    pub topic_streak: i32,
    pub peak_topic_streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsResponse {
    pub total_attempts: i64,
    pub correct_attempts: i64,
    pub current_streak: i32,
    pub topics: Vec<TopicStatsEntry>,
    pub badges: Vec<badges::BadgeDisplay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EloHistoryPoint {
    pub day: NaiveDate,
    pub elo: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EloHistoryResponse {
    pub topic: String,
    pub history: Vec<EloHistoryPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfileResponse {
    pub username: String,
    pub member_since: DateTime<Utc>,
    pub badges: Vec<badges::BadgeDisplay>,
    pub topics: Vec<TopicStatsEntry>,
    pub total_attempts: i64,
    pub correct_attempts: i64,
    pub current_streak: i32,
    pub daily_puzzle_streak: i32,
    pub activity: DailyActivityResponse,
}
