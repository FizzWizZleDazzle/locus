//! Expression grader — symbolic equivalence with optional Factor/Expand form checks.

use super::{ExprEngine, GradeResult, are_equivalent};
use crate::GradingMode;

/// Grade an expression answer with Factor/Expand mode support.
pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str, mode: GradingMode) -> GradeResult {
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
            if user_expr.expand().equals(&user_expr) {
                GradeResult::Incorrect
            } else {
                GradeResult::Correct
            }
        }
        GradingMode::Expand => {
            // User's answer MUST be in expanded form.
            if user_expr.expand().equals(&user_expr) {
                GradeResult::Correct
            } else {
                GradeResult::Incorrect
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symengine::Expr;
    use crate::latex::convert_latex_to_plain;

    #[test]
    fn test_equivalent_expression() {
        assert_eq!(
            grade::<Expr>("x^2 + 2*x + 1", "x^2 + 2*x + 1", GradingMode::Equivalent),
            GradeResult::Correct,
        );
    }

    #[test]
    fn test_equivalent_different_form() {
        assert_eq!(
            grade::<Expr>("(x+1)^2", "x^2 + 2*x + 1", GradingMode::Equivalent),
            GradeResult::Correct,
        );
    }

    #[test]
    fn test_wrong_expression() {
        assert_eq!(
            grade::<Expr>("x^2 + 1", "x^2 + 2*x + 1", GradingMode::Equivalent),
            GradeResult::Incorrect,
        );
    }

    #[test]
    fn test_unparseable() {
        assert!(matches!(
            grade::<Expr>("@#$", "x", GradingMode::Equivalent),
            GradeResult::Invalid(_),
        ));
    }

    #[test]
    fn test_pipeline_expression() {
        let plain = convert_latex_to_plain("x^{2}+2x+1");
        assert_eq!(
            grade::<Expr>(&plain, "x^2 + 2*x + 1", GradingMode::Equivalent),
            GradeResult::Correct,
        );
    }

    #[test]
    fn test_pipeline_fraction_expression() {
        let plain = convert_latex_to_plain("\\frac{x+1}{x-1}");
        assert_eq!(
            grade::<Expr>(&plain, "(x+1)/(x-1)", GradingMode::Equivalent),
            GradeResult::Correct,
        );
    }

    #[test]
    fn test_pipeline_trig() {
        let plain = convert_latex_to_plain("\\sin(x)");
        assert_eq!(
            grade::<Expr>(&plain, "sin(x)", GradingMode::Equivalent),
            GradeResult::Correct,
        );
    }

    #[test]
    fn test_pipeline_sqrt() {
        let plain = convert_latex_to_plain("\\sqrt{x+1}");
        assert_eq!(
            grade::<Expr>(&plain, "sqrt(x+1)", GradingMode::Equivalent),
            GradeResult::Correct,
        );
    }
}
