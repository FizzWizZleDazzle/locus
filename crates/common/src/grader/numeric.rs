//! Numeric grader — pure number comparison via SymEngine equivalence.

use super::{ExprEngine, GradeResult, are_equivalent};

pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str) -> GradeResult {
    let user_expr = match E::parse(user_input) {
        Ok(e) => e,
        Err(_) => return GradeResult::Invalid("Could not parse your answer".into()),
    };

    let answer_expr = match E::parse(answer_key) {
        Ok(e) => e,
        Err(_) => return GradeResult::Error("Invalid answer key".into()),
    };

    if are_equivalent(&user_expr, &answer_expr) {
        GradeResult::Correct
    } else {
        GradeResult::Incorrect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_utils::NumExpr;

    #[test]
    fn test_exact_match() {
        assert_eq!(grade::<NumExpr>("42", "42"), GradeResult::Correct);
    }

    #[test]
    fn test_decimal() {
        assert_eq!(grade::<NumExpr>("3.14", "3.14"), GradeResult::Correct);
    }

    #[test]
    fn test_negative() {
        assert_eq!(grade::<NumExpr>("-7", "-7"), GradeResult::Correct);
    }

    #[test]
    fn test_wrong_value() {
        assert_eq!(grade::<NumExpr>("5", "6"), GradeResult::Incorrect);
    }

    #[test]
    fn test_unparseable_input() {
        assert!(matches!(grade::<NumExpr>("abc", "42"), GradeResult::Invalid(_)));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(grade::<NumExpr>("  42  ", "42"), GradeResult::Correct);
    }

    #[test]
    fn test_zero() {
        assert_eq!(grade::<NumExpr>("0", "0"), GradeResult::Correct);
    }

    #[test]
    fn test_pipeline_numeric() {
        use crate::latex::convert_latex_to_plain;
        let plain = convert_latex_to_plain("\\frac{1}{2}");
        // (1)/(2) = 0.5 — but NumExpr can't parse this, so it returns Invalid.
        // This is expected: the real SymEngine handles it.
        assert!(matches!(grade::<NumExpr>(&plain, "0.5"), GradeResult::Invalid(_)));
    }

    mod symengine_tests {
        use super::super::*;
        use crate::symengine::Expr;
        use crate::latex::convert_latex_to_plain;

        #[test]
        fn test_integer() {
            assert_eq!(grade::<Expr>("42", "42"), GradeResult::Correct);
        }

        #[test]
        fn test_negative() {
            assert_eq!(grade::<Expr>("-7", "-7"), GradeResult::Correct);
        }

        #[test]
        fn test_fraction_equivalence() {
            assert_eq!(grade::<Expr>("1/2", "1/2"), GradeResult::Correct);
        }

        #[test]
        fn test_expression_equivalence() {
            assert_eq!(grade::<Expr>("2+3", "5"), GradeResult::Correct);
        }

        #[test]
        fn test_wrong_value() {
            assert_eq!(grade::<Expr>("5", "6"), GradeResult::Incorrect);
        }

        #[test]
        fn test_pipeline_fraction() {
            let plain = convert_latex_to_plain("\\frac{1}{2}");
            assert_eq!(grade::<Expr>(&plain, "1/2"), GradeResult::Correct);
        }

        #[test]
        fn test_pipeline_sqrt() {
            let plain = convert_latex_to_plain("\\sqrt{4}");
            assert_eq!(grade::<Expr>(&plain, "2"), GradeResult::Correct);
        }
    }
}
