//! Server-side answer grading.
//!
//! Uses the shared SymEngine-backed grading logic from `locus_common::grader`.

use locus_common::GradingMode;
use locus_common::grader;

/// Check if a user's answer matches the expected answer.
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode) -> bool {
    grader::check_answer_expr(user_input, answer_key, mode).is_correct()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(check_answer("x^2 + 1", "x^2 + 1", GradingMode::Equivalent));
    }

    #[test]
    fn test_whitespace_ignored() {
        assert!(check_answer("x^2 + 1", "x^2+1", GradingMode::Equivalent));
    }

    #[test]
    fn test_case_sensitive_symbols() {
        // SymEngine treats X and x as different symbols
        assert!(!check_answer("X^2 + 1", "x^2 + 1", GradingMode::Equivalent));
    }

    #[test]
    fn test_different_answers() {
        assert!(!check_answer("x^2 + 2", "x^2 + 1", GradingMode::Equivalent));
    }

    #[test]
    fn test_symbolic_equivalence() {
        // SymEngine can handle reordered terms
        assert!(check_answer("1 + x^2", "x^2 + 1", GradingMode::Equivalent));
    }

    #[test]
    fn test_factor_mode() {
        assert!(check_answer("(x+1)*(x-1)", "x^2-1", GradingMode::Factor));
        assert!(!check_answer("x^2-1", "x^2-1", GradingMode::Factor));
    }

    #[test]
    fn test_expand_mode() {
        assert!(check_answer("x^2+2*x+1", "(x+1)^2", GradingMode::Expand));
        assert!(!check_answer("(x+1)^2", "(x+1)^2", GradingMode::Expand));
    }
}
