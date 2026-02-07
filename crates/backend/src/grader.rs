//! Server-side answer grading.
//!
//! Uses the shared grading logic from `locus_common::grader`.
//! Currently uses string normalization as a fallback since SymEngine
//! is not yet linked natively. Once a native SymEngine ExprEngine
//! implementation is added, switch `check_answer` to use it via
//! `grader::check_answer::<NativeExpr>(...)`.

use locus_common::GradingMode;
use locus_common::grader::{self as common_grader, ExprEngine};

/// Fallback expression engine using string normalization.
///
/// This is a stopgap until native SymEngine is linked.
/// It normalizes expressions (lowercase, strip whitespace) and
/// compares strings. It does NOT handle symbolic equivalence.
///
/// TODO: Replace with native SymEngine or wasmtime-based ExprEngine
/// to get full symbolic grading on the server.
struct StringExpr {
    normalized: String,
}

impl ExprEngine for StringExpr {
    type Error = String;

    fn parse(input: &str) -> Result<Self, Self::Error> {
        let normalized = input.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_lowercase();
        Ok(Self { normalized })
    }

    fn expand(&self) -> Self {
        // String normalization can't expand — return as-is
        Self { normalized: self.normalized.clone() }
    }

    fn sub(&self, other: &Self) -> Self {
        // Can't do symbolic subtraction with strings.
        // Return a sentinel that signals "not zero" unless strings match.
        if self.normalized == other.normalized {
            Self { normalized: "0".to_string() }
        } else {
            Self { normalized: format!("({})-({})", self.normalized, other.normalized) }
        }
    }

    fn equals(&self, other: &Self) -> bool {
        self.normalized == other.normalized
    }

    fn is_zero(&self) -> bool {
        self.normalized == "0"
    }

    fn free_symbols(&self) -> Vec<String> {
        Vec::new()
    }

    fn subs_float(&self, _var_name: &str, _val: f64) -> Self {
        Self { normalized: self.normalized.clone() }
    }

    fn to_float(&self) -> Option<f64> {
        self.normalized.parse::<f64>().ok()
    }
}

/// Check if a user's answer matches the expected answer.
///
/// Uses the shared grading algorithm from locus_common.
/// Currently backed by string normalization (MVP fallback).
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode) -> bool {
    common_grader::check_answer::<StringExpr>(user_input, answer_key, mode).is_correct()
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
    fn test_case_insensitive() {
        assert!(check_answer("X^2 + 1", "x^2 + 1", GradingMode::Equivalent));
    }

    #[test]
    fn test_different_answers() {
        assert!(!check_answer("x^2 + 2", "x^2 + 1", GradingMode::Equivalent));
    }
}
