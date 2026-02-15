//! Client-side grading for practice mode.
//!
//! Delegates to the shared grading logic in `locus_common::grader`,
//! which uses SymEngine (linked via common crate's build.rs).

use locus_common::{AnswerType, GradingMode};

// Re-export GradeResult so existing code doesn't need to change imports
pub use locus_common::grader::GradeResult;

/// Check if a user's answer matches the expected answer.
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode, answer_type: AnswerType) -> GradeResult {
    locus_common::grader::grade_answer(user_input, answer_key, answer_type, mode)
}

/// Preprocess user input for SymEngine: convert MathJSON or LaTeX to plain math notation.
pub fn preprocess_input(input: &str) -> String {
    let trimmed = input.trim();

    // Detect structured notation that Compute Engine/MathJSON doesn't handle well:
    // - Intervals: (1, 7], [-2, 4), etc.
    // - Also skip for LaTeX brace commands
    let has_interval_brackets = (trimmed.starts_with("\\left(") || trimmed.starts_with("\\left["))
        && (trimmed.contains("\\right)") || trimmed.contains("\\right]"));

    let looks_like_interval_latex = trimmed.contains("\\left") && trimmed.contains("\\right");

    // Try MathJSON only if it's not interval-like notation
    if !looks_like_interval_latex && (trimmed.starts_with('[') || trimmed.starts_with('"')) {
        if let Ok(plain) = locus_common::mathjson::convert_mathjson_to_plain(input) {
            return plain;
        }
    }

    // Fallback to regex LaTeX converter (handles intervals, sets, structured types)
    locus_common::latex::convert_latex_to_plain(input)
}
