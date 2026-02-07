//! Shared grading logic for both frontend and backend.
//!
//! The grading algorithm is defined generically over `ExprEngine`,
//! which is implemented by the frontend (SymEngine WASM FFI) and
//! backend (SymEngine native or wasmtime).

use crate::GradingMode;

// Test points for numerical equivalence checking.
// Chosen to avoid common edge cases (0, 1, pi/2, etc.)
const TEST_POINTS: &[f64] = &[0.7, 1.3, 2.1, -0.4, 0.31];
const NUMERICAL_TOLERANCE: f64 = 1e-6;

/// Trait abstracting symbolic expression operations.
///
/// Both the frontend (WASM FFI) and backend implement this trait,
/// ensuring the same grading algorithm runs everywhere.
pub trait ExprEngine: Sized {
    type Error: std::fmt::Display;

    /// Parse a mathematical expression string (e.g. "x^2 + 1")
    fn parse(input: &str) -> Result<Self, Self::Error>;

    /// Expand the expression (distribute products, collect terms)
    fn expand(&self) -> Self;

    /// Subtract another expression: self - other
    fn sub(&self, other: &Self) -> Self;

    /// Check structural equality (same internal representation)
    fn equals(&self, other: &Self) -> bool;

    /// Check if this expression is the number zero
    fn is_zero(&self) -> bool;

    /// Get names of free symbols (variables) in the expression
    fn free_symbols(&self) -> Vec<String>;

    /// Substitute a named variable with a float value
    fn subs_float(&self, var_name: &str, val: f64) -> Self;

    /// Evaluate to a float. Returns None if not fully numeric.
    fn to_float(&self) -> Option<f64>;
}

/// Result of grading an answer
#[derive(Debug, Clone, PartialEq)]
pub enum GradeResult {
    /// Answer is correct
    Correct,
    /// Answer is incorrect
    Incorrect,
    /// Input could not be parsed
    Invalid(String),
    /// Grading error (e.g. invalid answer key)
    Error(String),
}

impl GradeResult {
    pub fn is_correct(&self) -> bool {
        matches!(self, GradeResult::Correct)
    }
}

/// Check if a user's answer matches the expected answer.
///
/// Uses a two-stage equivalence check:
/// 1. Symbolic: expand(user - answer) == 0 (exact, handles all polynomial equivalence)
/// 2. Numerical fallback: substitute test values and check |f(user) - f(answer)| < epsilon
///    (handles trig identities, logarithmic equivalences, etc.)
///
/// For Factor/Expand modes, additionally verifies the answer form:
/// - Factor: user's answer must NOT already be in expanded form
/// - Expand: user's answer MUST be in expanded form
pub fn check_answer<E: ExprEngine>(user_input: &str, answer_key: &str, mode: GradingMode) -> GradeResult {
    let user_expr = match E::parse(user_input) {
        Ok(e) => e,
        Err(_) => return GradeResult::Invalid("Could not parse your answer".into()),
    };

    let answer_expr = match E::parse(answer_key) {
        Ok(e) => e,
        Err(_) => return GradeResult::Error("Invalid answer key".into()),
    };

    if !are_equivalent(&user_expr, &answer_expr) {
        return GradeResult::Incorrect;
    }

    match mode {
        GradingMode::Equivalent => GradeResult::Correct,
        GradingMode::Factor => {
            // User's answer must NOT already be in expanded form.
            // If expand(user) == user structurally, the user just typed
            // the expanded form back (e.g. "x^2 - 1" instead of "(x+1)(x-1)").
            if user_expr.expand().equals(&user_expr) {
                GradeResult::Incorrect
            } else {
                GradeResult::Correct
            }
        }
        GradingMode::Expand => {
            // User's answer MUST be in expanded form.
            // If expand(user) != user, it means they submitted an unexpanded
            // form (e.g. "(x+1)^2" instead of "x^2+2x+1").
            if user_expr.expand().equals(&user_expr) {
                GradeResult::Correct
            } else {
                GradeResult::Incorrect
            }
        }
    }
}

/// Two-stage equivalence check:
/// 1. Symbolic: expand(a - b) == 0
/// 2. Numerical: evaluate at test points and check the difference is ~0
fn are_equivalent<E: ExprEngine>(a: &E, b: &E) -> bool {
    // Stage 1: exact symbolic check
    let diff = a.sub(b);
    let expanded = diff.expand();
    if expanded.is_zero() {
        return true;
    }

    // Stage 2: numerical evaluation fallback
    // This catches equivalences that expand() can't simplify,
    // like sin(x)^2 + cos(x)^2 == 1
    let symbols = expanded.free_symbols();
    if symbols.is_empty() {
        // No free symbols but not symbolically zero — try evaluating
        if let Some(val) = expanded.to_float() {
            return val.abs() < NUMERICAL_TOLERANCE;
        }
        return false;
    }

    // Substitute each variable with test values and check
    for &test_val in TEST_POINTS {
        let mut subst = a.sub(b);
        for sym in &symbols {
            subst = subst.subs_float(sym, test_val);
        }
        match subst.to_float() {
            Some(val) if val.abs() < NUMERICAL_TOLERANCE => continue,
            Some(_) => return false,    // Non-zero at this point → not equivalent
            None => return false,       // Can't evaluate → can't confirm
        }
    }

    // All test points passed — expressions are equivalent (high confidence)
    true
}
