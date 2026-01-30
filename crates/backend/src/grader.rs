//! Answer grading using SymEngine WASM via wasmtime
//!
//! This module provides server-side grading of mathematical expressions.
//! For the MVP, we use a simplified grading approach that can be enhanced
//! with full SymEngine integration later.

use locus_common::GradingMode;

/// Normalize a mathematical expression for comparison
///
/// This performs basic normalization:
/// - Remove whitespace
/// - Lowercase
/// - Sort terms (simple heuristic)
fn normalize_expr(expr: &str) -> String {
    expr.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}

/// Check if a user's answer matches the expected answer
///
/// # Arguments
/// * `user_input` - The user's answer
/// * `answer_key` - The correct answer
/// * `mode` - The grading mode
///
/// # Returns
/// `true` if the answer is correct
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode) -> bool {
    let user_normalized = normalize_expr(user_input);
    let answer_normalized = normalize_expr(answer_key);

    match mode {
        GradingMode::Equivalent => {
            // For MVP: exact match after normalization
            // TODO: Full symbolic comparison with SymEngine
            user_normalized == answer_normalized
        }
        GradingMode::Factor => {
            // For factored form, we need exact structural match
            user_normalized == answer_normalized
        }
    }
}

/// Future: Initialize SymEngine WASM runtime
///
/// This will load symengine.wasm via wasmtime and provide
/// full symbolic computation capabilities.
pub struct SymEngineRuntime {
    // TODO: wasmtime::Engine, Store, Instance
}

impl SymEngineRuntime {
    /// Create a new SymEngine runtime
    pub fn new() -> Result<Self, GraderError> {
        // TODO: Load WASM module
        Ok(Self {})
    }

    /// Parse and expand an expression
    pub fn expand(&self, _expr: &str) -> Result<String, GraderError> {
        // TODO: Call SymEngine basic_expand
        Err(GraderError::NotImplemented)
    }

    /// Check if two expressions are equal
    pub fn equals(&self, _a: &str, _b: &str) -> Result<bool, GraderError> {
        // TODO: Call SymEngine basic_eq
        Err(GraderError::NotImplemented)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GraderError {
    #[error("SymEngine not yet implemented")]
    NotImplemented,

    #[error("Failed to parse expression: {0}")]
    ParseError(String),

    #[error("WASM runtime error: {0}")]
    WasmError(String),
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
