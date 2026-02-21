//! Server-side answer grading.
//!
//! Uses the shared grading logic from `locus_common::grader`.

use locus_common::{AnswerType, GradingMode};
use locus_common::grader;

/// Check if a user's answer matches the expected answer.
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode, answer_type: AnswerType) -> bool {
    grader::grade_answer(user_input, answer_key, answer_type, mode).is_correct()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(check_answer("x^2 + 1", "x^2 + 1", GradingMode::Equivalent, AnswerType::Expression));
    }

    #[test]
    fn test_whitespace_ignored() {
        assert!(check_answer("x^2 + 1", "x^2+1", GradingMode::Equivalent, AnswerType::Expression));
    }

    #[test]
    fn test_case_sensitive_symbols() {
        assert!(!check_answer("X^2 + 1", "x^2 + 1", GradingMode::Equivalent, AnswerType::Expression));
    }

    #[test]
    fn test_different_answers() {
        assert!(!check_answer("x^2 + 2", "x^2 + 1", GradingMode::Equivalent, AnswerType::Expression));
    }

    #[test]
    fn test_symbolic_equivalence() {
        assert!(check_answer("1 + x^2", "x^2 + 1", GradingMode::Equivalent, AnswerType::Expression));
    }

    #[test]
    fn test_factor_mode() {
        assert!(check_answer("(x+1)*(x-1)", "x^2-1", GradingMode::Factor, AnswerType::Expression));
        assert!(!check_answer("x^2-1", "x^2-1", GradingMode::Factor, AnswerType::Expression));
    }

    #[test]
    fn test_expand_mode() {
        assert!(check_answer("x^2+2*x+1", "(x+1)^2", GradingMode::Expand, AnswerType::Expression));
        assert!(!check_answer("(x+1)^2", "(x+1)^2", GradingMode::Expand, AnswerType::Expression));
    }

    #[test]
    fn test_boolean_type() {
        assert!(check_answer("true", "true", GradingMode::Equivalent, AnswerType::Boolean));
        assert!(check_answer("yes", "true", GradingMode::Equivalent, AnswerType::Boolean));
        assert!(!check_answer("false", "true", GradingMode::Equivalent, AnswerType::Boolean));
    }

    #[test]
    fn test_word_type() {
        assert!(check_answer("Maximum", "maximum", GradingMode::Equivalent, AnswerType::Word));
        assert!(!check_answer("minimum", "maximum", GradingMode::Equivalent, AnswerType::Word));
    }

    #[test]
    fn test_numeric_type() {
        assert!(check_answer("274", "274", GradingMode::Equivalent, AnswerType::Numeric));
        assert!(check_answer("1/4", "0.25", GradingMode::Equivalent, AnswerType::Numeric));
    }
}
