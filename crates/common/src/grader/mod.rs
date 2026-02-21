//! Shared grading logic for both frontend and backend.
//!
//! The grading algorithm dispatches to type-specific graders based on `AnswerType`.
//! `GradingMode` (Factor/Expand) only applies to Expression type.

mod boolean;
mod equation;
mod expression;
mod inequality;
mod interval;
mod matrix;
mod multipart;
mod numeric;
mod ordered;
pub mod parse;
mod set;
mod word;

use crate::symengine::{Expr, ExprError};
use crate::{AnswerType, GradingMode};

// Test points for numerical equivalence checking.
// Chosen to avoid common edge cases (0, 1, pi/2, etc.)
pub(crate) const TEST_POINTS: &[f64] = &[0.7, 1.3, 2.1, -0.4, 0.31];
pub(crate) const NUMERICAL_TOLERANCE: f64 = 1e-6;

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

// ============================================================================
// ExprEngine implementation for SymEngine FFI Expr
// ============================================================================

impl ExprEngine for Expr {
    type Error = ExprError;

    fn parse(input: &str) -> Result<Self, Self::Error> {
        Expr::parse(input)
    }

    fn expand(&self) -> Self {
        Expr::expand(self)
    }

    fn sub(&self, other: &Self) -> Self {
        Expr::sub(self, other)
    }

    fn equals(&self, other: &Self) -> bool {
        Expr::equals(self, other)
    }

    fn is_zero(&self) -> bool {
        Expr::is_zero(self)
    }

    fn free_symbols(&self) -> Vec<String> {
        Expr::free_symbols(self)
    }

    fn subs_float(&self, var_name: &str, val: f64) -> Self {
        Expr::subs_float(self, var_name, val)
    }

    fn to_float(&self) -> Option<f64> {
        Expr::to_float(self)
    }
}

// ============================================================================
// Main dispatcher
// ============================================================================

/// Grade a user's answer against the expected answer, dispatching to
/// the appropriate type-specific grader based on `answer_type`.
///
/// `GradingMode` only applies to `Expression` type; ignored for all others.
pub fn grade_answer(
    user: &str,
    expected: &str,
    answer_type: AnswerType,
    mode: GradingMode,
) -> GradeResult {
    match answer_type {
        AnswerType::Expression => expression::grade::<Expr>(user, expected, mode),
        AnswerType::Numeric => numeric::grade::<Expr>(user, expected),
        AnswerType::Set => set::grade::<Expr>(user, expected),
        AnswerType::Tuple | AnswerType::List => ordered::grade::<Expr>(user, expected),
        AnswerType::Interval => interval::grade::<Expr>(user, expected),
        AnswerType::Inequality => inequality::grade::<Expr>(user, expected),
        AnswerType::Equation => equation::grade::<Expr>(user, expected),
        AnswerType::Boolean => boolean::grade(user, expected),
        AnswerType::Word => word::grade(user, expected),
        AnswerType::Matrix => matrix::grade::<Expr>(user, expected),
        AnswerType::MultiPart => multipart::grade(user, expected),
    }
}

/// Backward-compatible convenience function: grade as Expression type.
///
/// Both frontend (WASM) and backend (native) can call this directly.
pub fn check_answer_expr(user_input: &str, answer_key: &str, mode: GradingMode) -> GradeResult {
    grade_answer(user_input, answer_key, AnswerType::Expression, mode)
}

/// Backward-compatible generic check_answer (Expression type only).
pub fn check_answer<E: ExprEngine>(
    user_input: &str,
    answer_key: &str,
    mode: GradingMode,
) -> GradeResult {
    expression::grade::<E>(user_input, answer_key, mode)
}

// ============================================================================
// Shared equivalence logic
// ============================================================================

/// Two-stage equivalence check:
/// 1. Symbolic: expand(a - b) == 0
/// 2. Numerical: evaluate at test points and check the difference is ~0
pub(crate) fn are_equivalent<E: ExprEngine>(a: &E, b: &E) -> bool {
    // Stage 1: exact symbolic check
    let diff = a.sub(b);
    let expanded = diff.expand();
    if expanded.is_zero() {
        return true;
    }

    // Stage 2: numerical evaluation fallback
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
            Some(_) => return false,
            None => return false,
        }
    }

    true
}

/// Check if a / b is a constant (proportional expressions).
/// Returns true if the ratio is the same non-zero constant at all test points.
pub(crate) fn are_proportional<E: ExprEngine>(a: &E, b: &E) -> bool {
    let a_syms = a.free_symbols();
    let b_syms = b.free_symbols();

    let mut all_syms: Vec<String> = a_syms;
    for s in b_syms {
        if !all_syms.contains(&s) {
            all_syms.push(s);
        }
    }

    if all_syms.is_empty() {
        let a_val = a.to_float();
        let b_val = b.to_float();
        match (a_val, b_val) {
            (Some(av), Some(bv)) if bv.abs() > NUMERICAL_TOLERANCE => {
                av.abs() > NUMERICAL_TOLERANCE
            }
            _ => false,
        }
    } else {
        let mut ratio: Option<f64> = None;
        for &test_val in TEST_POINTS {
            let a_eval = {
                let mut tmp = a.subs_float(&all_syms[0], test_val);
                for sym in &all_syms[1..] {
                    tmp = tmp.subs_float(sym, test_val);
                }
                tmp
            };
            let b_eval = {
                let mut tmp = b.subs_float(&all_syms[0], test_val);
                for sym in &all_syms[1..] {
                    tmp = tmp.subs_float(sym, test_val);
                }
                tmp
            };

            match (a_eval.to_float(), b_eval.to_float()) {
                (Some(av), Some(bv)) => {
                    if bv.abs() < NUMERICAL_TOLERANCE {
                        if av.abs() < NUMERICAL_TOLERANCE {
                            continue;
                        } else {
                            return false;
                        }
                    }
                    let r = av / bv;
                    match ratio {
                        None => ratio = Some(r),
                        Some(prev) => {
                            if (r - prev).abs() > NUMERICAL_TOLERANCE * 100.0 {
                                return false;
                            }
                        }
                    }
                }
                _ => return false,
            }
        }
        ratio.is_some()
    }
}
