//! Client-side grading for practice mode.
//!
//! Delegates to the shared grading logic in `locus_common::grader`,
//! which uses SymEngine (linked via common crate's build.rs).

use locus_common::GradingMode;

// Re-export GradeResult so existing code doesn't need to change imports
pub use locus_common::grader::GradeResult;

/// Check if a user's answer matches the expected answer.
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode) -> GradeResult {
    locus_common::grader::check_answer_expr(user_input, answer_key, mode)
}

/// Preprocess user input for SymEngine: convert LaTeX to plain math notation.
pub fn preprocess_input(input: &str) -> String {
    locus_common::latex::convert_latex_to_plain(input)
}
