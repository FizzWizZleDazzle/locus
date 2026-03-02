//! Set grader — unordered element-wise comparison.

use super::parse::split_top_level;
use super::{ExprEngine, GradeResult, are_equivalent};

/// Strip optional `{` `}` braces from input.
fn strip_braces(input: &str) -> &str {
    let s = input.trim();
    if s.starts_with('{') && s.ends_with('}') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str) -> GradeResult {
    let expected_str = strip_braces(answer_key);
    let user_str = strip_braces(user_input);

    let expected_parts = split_top_level(expected_str, ',');
    let user_parts = split_top_level(user_str, ',');

    if expected_parts.is_empty() {
        return GradeResult::Error("Empty answer key for set".into());
    }

    if user_parts.len() != expected_parts.len() {
        return GradeResult::Incorrect;
    }

    // Parse all expected elements
    let expected_exprs: Vec<E> = match expected_parts
        .iter()
        .map(|s| E::parse(s.trim()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(v) => v,
        Err(e) => return GradeResult::Error(format!("Invalid answer key element: {}", e)),
    };

    // Parse all user elements
    let user_exprs: Vec<E> = match user_parts
        .iter()
        .map(|s| E::parse(s.trim()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(v) => v,
        Err(_) => return GradeResult::Invalid("Could not parse one or more elements".into()),
    };

    // Unordered matching: each expected element must match exactly one user element
    let mut matched = vec![false; user_exprs.len()];

    for exp in &expected_exprs {
        let mut found = false;
        for (i, usr) in user_exprs.iter().enumerate() {
            if !matched[i] && are_equivalent(exp, usr) {
                matched[i] = true;
                found = true;
                break;
            }
        }
        if !found {
            return GradeResult::Incorrect;
        }
    }

    GradeResult::Correct
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_utils::NumExpr;

    #[test]
    fn test_same_order() {
        assert_eq!(grade::<NumExpr>("{1, 2, 3}", "1, 2, 3"), GradeResult::Correct);
    }

    #[test]
    fn test_different_order() {
        assert_eq!(grade::<NumExpr>("{3, 1, 2}", "1, 2, 3"), GradeResult::Correct);
    }

    #[test]
    fn test_with_braces_on_both() {
        assert_eq!(grade::<NumExpr>("{1, 2}", "{1, 2}"), GradeResult::Correct);
    }

    #[test]
    fn test_wrong_element() {
        assert_eq!(grade::<NumExpr>("{1, 2, 4}", "1, 2, 3"), GradeResult::Incorrect);
    }

    #[test]
    fn test_wrong_count() {
        assert_eq!(grade::<NumExpr>("{1, 2}", "1, 2, 3"), GradeResult::Incorrect);
    }

    #[test]
    fn test_unparseable_element() {
        assert!(matches!(grade::<NumExpr>("{1, abc, 3}", "1, 2, 3"), GradeResult::Invalid(_)));
    }

    #[test]
    fn test_negative_elements() {
        assert_eq!(grade::<NumExpr>("{-1, -2, -3}", "-3, -2, -1"), GradeResult::Correct);
    }

    #[test]
    fn test_pipeline_set() {
        use crate::latex::convert_latex_to_plain;
        let plain = convert_latex_to_plain("\\left\\{1, 2, 3\\right\\}");
        assert_eq!(plain, "{1, 2, 3}");
        assert_eq!(grade::<NumExpr>(&plain, "1, 2, 3"), GradeResult::Correct);
    }

    mod symengine_tests {
        use super::super::*;
        use crate::symengine::Expr;
        use crate::latex::convert_latex_to_plain;

        #[test]
        fn test_basic_set() {
            assert_eq!(grade::<Expr>("{1, 2, 3}", "1, 2, 3"), GradeResult::Correct);
        }

        #[test]
        fn test_reordered() {
            assert_eq!(grade::<Expr>("{3, 1, 2}", "1, 2, 3"), GradeResult::Correct);
        }

        #[test]
        fn test_expression_elements() {
            // 1+1 == 2
            assert_eq!(grade::<Expr>("{1+1, 3}", "2, 3"), GradeResult::Correct);
        }

        #[test]
        fn test_wrong_element() {
            assert_eq!(grade::<Expr>("{1, 2, 4}", "1, 2, 3"), GradeResult::Incorrect);
        }

        #[test]
        fn test_pipeline_set_latex() {
            let plain = convert_latex_to_plain("\\left\\{-1, 0, 1\\right\\}");
            assert_eq!(grade::<Expr>(&plain, "-1, 0, 1"), GradeResult::Correct);
        }
    }
}
