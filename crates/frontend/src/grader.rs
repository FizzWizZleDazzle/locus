//! Client-side grading for practice mode.
//!
//! Delegates to the shared grading logic in `locus_common::grader`,
//! which uses SymEngine (linked via common crate's build.rs).
//!
//! # Input Processing Pipeline
//!
//! User input flows through this pipeline:
//! 1. MathQuill editor outputs LaTeX
//! 2. `convert_latex_to_plain()` converts to plain notation (done in MathField component)
//! 3. Grading system works with plain notation internally
//!
//! The `preprocess_input()` function is kept as a safety net — it runs
//! `convert_latex_to_plain()` on any residual LaTeX that might slip through.

use locus_common::{AnswerType, GradingMode};

// Re-export GradeResult so existing code doesn't need to change imports
pub use locus_common::grader::GradeResult;

/// Check if a user's answer matches the expected answer.
pub fn check_answer(
    user_input: &str,
    answer_key: &str,
    mode: GradingMode,
    answer_type: AnswerType,
) -> GradeResult {
    locus_common::grader::grade_answer(user_input, answer_key, answer_type, mode)
}

/// Preprocess user input: convert any residual LaTeX to plain text.
///
/// With MathQuill, the MathField component already converts LaTeX to plain text
/// via `convert_latex_to_plain()` on every edit. This function is a safety net
/// for any edge cases where raw LaTeX might still reach the grading pipeline.
pub fn preprocess_input(input: &str) -> String {
    locus_common::latex::convert_latex_to_plain(input)
}
