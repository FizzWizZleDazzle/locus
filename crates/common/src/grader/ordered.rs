//! Ordered grader — element-wise ordered comparison (Tuple + List).

use super::parse::split_top_level;
use super::{ExprEngine, GradeResult, are_equivalent};

/// Strip optional `(` `)` or `[` `]` from input.
fn strip_delimiters(input: &str) -> &str {
    let s = input.trim();
    if (s.starts_with('(') && s.ends_with(')')) || (s.starts_with('[') && s.ends_with(']')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str) -> GradeResult {
    let expected_str = strip_delimiters(answer_key);
    let user_str = strip_delimiters(user_input);

    let expected_parts = split_top_level(expected_str, ',');
    let user_parts = split_top_level(user_str, ',');

    if expected_parts.is_empty() {
        return GradeResult::Error("Empty answer key for ordered type".into());
    }

    if user_parts.len() != expected_parts.len() {
        return GradeResult::Incorrect;
    }

    // Parse and compare element-wise (order matters)
    for (i, (exp_s, usr_s)) in expected_parts.iter().zip(user_parts.iter()).enumerate() {
        let exp = match E::parse(exp_s.trim()) {
            Ok(e) => e,
            Err(e) => {
                return GradeResult::Error(format!("Invalid answer key element {}: {}", i + 1, e));
            }
        };

        let usr = match E::parse(usr_s.trim()) {
            Ok(e) => e,
            Err(_) => return GradeResult::Invalid(format!("Could not parse element {}", i + 1)),
        };

        if !are_equivalent(&exp, &usr) {
            return GradeResult::Incorrect;
        }
    }

    GradeResult::Correct
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::NumExpr;
    use super::*;

    // ── Tuple tests ──

    #[test]
    fn test_tuple_correct() {
        assert_eq!(grade::<NumExpr>("(3, -4)", "3, -4"), GradeResult::Correct);
    }

    #[test]
    fn test_tuple_with_parens_on_both() {
        assert_eq!(grade::<NumExpr>("(1, 2)", "(1, 2)"), GradeResult::Correct);
    }

    #[test]
    fn test_tuple_wrong_order() {
        assert_eq!(grade::<NumExpr>("(-4, 3)", "3, -4"), GradeResult::Incorrect);
    }

    #[test]
    fn test_tuple_wrong_element() {
        assert_eq!(grade::<NumExpr>("(3, -5)", "3, -4"), GradeResult::Incorrect);
    }

    #[test]
    fn test_tuple_wrong_count() {
        assert_eq!(
            grade::<NumExpr>("(3, -4, 1)", "3, -4"),
            GradeResult::Incorrect
        );
    }

    #[test]
    fn test_tuple_unparseable() {
        assert!(matches!(
            grade::<NumExpr>("(abc, 3)", "3, -4"),
            GradeResult::Invalid(_)
        ));
    }

    // ── List tests ──

    #[test]
    fn test_list_correct() {
        assert_eq!(
            grade::<NumExpr>("[1, 2, 3]", "[1, 2, 3]"),
            GradeResult::Correct
        );
    }

    #[test]
    fn test_list_wrong_order() {
        assert_eq!(
            grade::<NumExpr>("[3, 2, 1]", "[1, 2, 3]"),
            GradeResult::Incorrect
        );
    }

    #[test]
    fn test_pipeline_tuple() {
        use crate::latex::convert_latex_to_plain;
        let plain = convert_latex_to_plain("\\left(5, -4\\right)");
        assert_eq!(plain, "(5, -4)");
        assert_eq!(grade::<NumExpr>(&plain, "5, -4"), GradeResult::Correct);
    }

    #[test]
    fn test_pipeline_list() {
        use crate::latex::convert_latex_to_plain;
        let plain = convert_latex_to_plain("\\left[1, 2, 3\\right]");
        assert_eq!(plain, "[1, 2, 3]");
        assert_eq!(grade::<NumExpr>(&plain, "[1, 2, 3]"), GradeResult::Correct);
    }

    mod symengine_tests {
        use super::super::*;
        use crate::latex::convert_latex_to_plain;
        use crate::symengine::Expr;

        #[test]
        fn test_tuple() {
            assert_eq!(grade::<Expr>("(3, -4)", "3, -4"), GradeResult::Correct);
        }

        #[test]
        fn test_tuple_expression_element() {
            assert_eq!(grade::<Expr>("(1+2, 4)", "3, 4"), GradeResult::Correct);
        }

        #[test]
        fn test_tuple_wrong_order() {
            assert_eq!(grade::<Expr>("(-4, 3)", "3, -4"), GradeResult::Incorrect);
        }

        #[test]
        fn test_list() {
            assert_eq!(
                grade::<Expr>("[1, 2, 3]", "[1, 2, 3]"),
                GradeResult::Correct
            );
        }

        #[test]
        fn test_pipeline_tuple_latex() {
            let plain = convert_latex_to_plain("\\left(5, -4\\right)");
            assert_eq!(grade::<Expr>(&plain, "5, -4"), GradeResult::Correct);
        }

        #[test]
        fn test_pipeline_tuple_with_fraction() {
            let plain = convert_latex_to_plain("\\left(\\frac{1}{2}, 3\\right)");
            assert_eq!(grade::<Expr>(&plain, "1/2, 3"), GradeResult::Correct);
        }
    }
}
