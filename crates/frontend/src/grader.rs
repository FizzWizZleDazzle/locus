//! Client-side grading for practice mode.
//!
//! Delegates to the shared grading logic in `locus_common::grader`,
//! which uses SymEngine (linked via common crate's build.rs).
//!
//! # Input Processing Pipeline
//!
//! User input flows through this pipeline:
//! 1. MathLive editor outputs MathJSON (structured AST)
//! 2. `preprocess_input()` converts to plain notation
//! 3. Grading system works with plain notation internally
//!
//! The preprocessor tries MathJSON conversion first (when Compute Engine is loaded),
//! then falls back to regex-based LaTeX conversion for special cases like intervals.

use locus_common::{AnswerType, GradingMode};

// Re-export GradeResult so existing code doesn't need to change imports
pub use locus_common::grader::GradeResult;

/// Check if a user's answer matches the expected answer.
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode, answer_type: AnswerType) -> GradeResult {
    locus_common::grader::grade_answer(user_input, answer_key, answer_type, mode)
}

/// Detect if input looks like interval notation with LaTeX delimiters
///
/// Intervals use \left( or \left[ with matching \right) or \right]
/// Example: "\left(1, 7\right]" or "\left[-2, 4\right)"
fn is_interval_latex(input: &str) -> bool {
    input.contains("\\left") && input.contains("\\right")
}

/// Detect if input is MathJSON format
///
/// MathJSON starts with [ (for arrays/expressions) or " (for strings)
/// Example: '["Add", "x", 1]' or '"Pi"'
fn is_mathjson(input: &str) -> bool {
    let trimmed = input.trim_start();
    trimmed.starts_with('[') || trimmed.starts_with('"')
}

/// Preprocess user input for SymEngine: convert MathJSON or LaTeX to plain math notation.
///
/// # Conversion Priority
///
/// 1. **Interval notation** (highest priority)
///    - Uses LaTeX converter because MathJSON can't represent intervals
///    - Example: "\left(1, 7\right]" → "(1, 7]"
///
/// 2. **MathJSON** (when Compute Engine is loaded)
///    - Structured AST from MathLive, most accurate conversion
///    - Example: '["Add", "x", 1]' → "x + 1"
///
/// 3. **LaTeX fallback** (for everything else)
///    - Regex-based converter handles all LaTeX syntax
///    - Example: "\frac{1}{2}" → "(1)/(2)"
pub fn preprocess_input(input: &str) -> String {
    // Intervals use LaTeX notation that MathJSON can't parse
    if is_interval_latex(input) {
        return locus_common::latex::convert_latex_to_plain(input);
    }

    // Try MathJSON conversion first (when Compute Engine is loaded)
    if is_mathjson(input) {
        if let Ok(plain) = locus_common::mathjson::convert_mathjson_to_plain(input) {
            return plain;
        }
    }

    // Fallback to LaTeX converter for all other cases
    locus_common::latex::convert_latex_to_plain(input)
}
