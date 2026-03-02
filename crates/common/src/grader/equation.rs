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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_utils::NumExpr;

    #[test]
    fn test_exact_match() {
        assert_eq!(grade::<NumExpr>("1 = 1", "1 = 1"), GradeResult::Correct);
    }

    #[test]
    fn test_different_sides() {
        // 3 = 3 matches 3 = 3 (diff = 0 for both)
        assert_eq!(grade::<NumExpr>("3 = 3", "3 = 3"), GradeResult::Correct);
    }

    #[test]
    fn test_no_equals() {
        assert!(matches!(grade::<NumExpr>("3", "1 = 1"), GradeResult::Invalid(_)));
    }

    #[test]
    fn test_wrong_equation() {
        assert_eq!(grade::<NumExpr>("1 = 2", "1 = 1"), GradeResult::Incorrect);
    }

    mod symengine_tests {
        use super::super::*;
        use crate::symengine::Expr;
        use crate::latex::convert_latex_to_plain;

        #[test]
        fn test_simple_equation() {
            assert_eq!(grade::<Expr>("x = 5", "x = 5"), GradeResult::Correct);
        }

        #[test]
        fn test_equivalent_rearranged() {
            // x - 5 = 0 is proportional to x = 5 (diff: x-5 vs x-5)
            assert_eq!(grade::<Expr>("x - 5 = 0", "x = 5"), GradeResult::Correct);
        }

        #[test]
        fn test_sides_swapped() {
            // 5 = x has diff 5-x = -(x-5), proportional to x-5
            assert_eq!(grade::<Expr>("5 = x", "x = 5"), GradeResult::Correct);
        }

        #[test]
        fn test_scaled_equation() {
            // 2*x = 10 has diff 2*x-10 = 2*(x-5), proportional to x-5
            assert_eq!(grade::<Expr>("2*x = 10", "x = 5"), GradeResult::Correct);
        }

        #[test]
        fn test_wrong_equation() {
            assert_eq!(grade::<Expr>("x = 6", "x = 5"), GradeResult::Incorrect);
        }

        #[test]
        fn test_pipeline_equation() {
            let plain = convert_latex_to_plain("x^{2}+y^{2}=1");
            assert_eq!(grade::<Expr>(&plain, "x^2 + y^2 = 1"), GradeResult::Correct);
        }
    }
}
