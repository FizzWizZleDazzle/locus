//! Equation grader — split on `=`, compare LHS-RHS differences.
//!
//! Two checks:
//! 1. Direct equivalence of (LHS - RHS) expressions
//! 2. Proportionality: user_diff / expected_diff is a constant (scalar multiple)

use super::parse::split_equation;
use super::{ExprEngine, GradeResult, are_equivalent, are_proportional};

pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str) -> GradeResult {
    // Parse expected equation
    let (exp_lhs, exp_rhs) = match split_equation(answer_key) {
        Ok(pair) => pair,
        Err(e) => return GradeResult::Error(format!("Invalid equation answer key: {}", e)),
    };

    // Parse user equation
    let (usr_lhs, usr_rhs) = match split_equation(user_input) {
        Ok(pair) => pair,
        Err(_) => return GradeResult::Invalid("Expected an equation with '='".into()),
    };

    // Parse all sides as expressions
    let exp_l = match E::parse(&exp_lhs) {
        Ok(e) => e,
        Err(e) => return GradeResult::Error(format!("Invalid answer key LHS: {}", e)),
    };
    let exp_r = match E::parse(&exp_rhs) {
        Ok(e) => e,
        Err(e) => return GradeResult::Error(format!("Invalid answer key RHS: {}", e)),
    };
    let usr_l = match E::parse(&usr_lhs) {
        Ok(e) => e,
        Err(_) => return GradeResult::Invalid("Could not parse left side of equation".into()),
    };
    let usr_r = match E::parse(&usr_rhs) {
        Ok(e) => e,
        Err(_) => return GradeResult::Invalid("Could not parse right side of equation".into()),
    };

    // Compute diffs: LHS - RHS for each
    let exp_diff = exp_l.sub(&exp_r);
    let usr_diff = usr_l.sub(&usr_r);

    // Check 1: direct equivalence of the diff expressions
    if are_equivalent(&usr_diff, &exp_diff) {
        return GradeResult::Correct;
    }

    // Check 2: proportionality — user_diff = k * expected_diff for some constant k
    if are_proportional(&usr_diff, &exp_diff) {
        return GradeResult::Correct;
    }

    GradeResult::Incorrect
}
