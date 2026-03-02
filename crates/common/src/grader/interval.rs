//! Interval grader — compare interval bounds and types.
//!
//! DB answer key format: `open:1,closed:7` means (1, 7].
//! Unions: `open:-inf,closed:0|open:2,open:inf` means (-inf, 0] U (2, inf).
//!
//! User input format: `(1, 7]`, `[-2, 4)`, `(-inf, 3]`.
//! Unions with `U` or `union`.

use super::{ExprEngine, GradeResult, are_equivalent};

#[derive(Debug, Clone, PartialEq)]
enum BoundType {
    Open,
    Closed,
}

#[derive(Debug, Clone)]
struct Bound {
    kind: BoundType,
    value: String, // SymEngine expression string, or "inf"/"-inf"
}

#[derive(Debug, Clone)]
struct Interval {
    left: Bound,
    right: Bound,
}

/// Parse DB answer key format: `open:1,closed:7`
fn parse_key_interval(s: &str) -> Result<Interval, String> {
    let parts: Vec<&str> = s.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err(format!("Expected two bounds separated by comma: {}", s));
    }

    let left = parse_key_bound(parts[0].trim())?;
    let right = parse_key_bound(parts[1].trim())?;
    Ok(Interval { left, right })
}

fn parse_key_bound(s: &str) -> Result<Bound, String> {
    if let Some(val) = s.strip_prefix("open:") {
        Ok(Bound {
            kind: BoundType::Open,
            value: val.trim().to_string(),
        })
    } else if let Some(val) = s.strip_prefix("closed:") {
        Ok(Bound {
            kind: BoundType::Closed,
            value: val.trim().to_string(),
        })
    } else {
        Err(format!(
            "Expected 'open:value' or 'closed:value', got: {}",
            s
        ))
    }
}

/// Parse user input format: `(1, 7]`, `[-2, 4)`, `(-inf, 3]`
fn parse_user_interval(s: &str) -> Result<Interval, String> {
    let s = s.trim();
    if s.len() < 3 {
        return Err("Interval too short".into());
    }

    let first_char = s.chars().next().unwrap();
    let last_char = s.chars().last().unwrap();

    let left_type = match first_char {
        '(' => BoundType::Open,
        '[' => BoundType::Closed,
        _ => {
            return Err(format!(
                "Interval must start with '(' or '[', got '{}'",
                first_char
            ));
        }
    };

    let right_type = match last_char {
        ')' => BoundType::Open,
        ']' => BoundType::Closed,
        _ => {
            return Err(format!(
                "Interval must end with ')' or ']', got '{}'",
                last_char
            ));
        }
    };

    // Strip delimiters
    let inner = &s[1..s.len() - 1];

    // Split on comma — but we need to be careful with negative numbers and expressions.
    // Use a simple approach: find the first comma at depth 0
    let mut depth = 0i32;
    let mut comma_pos = None;
    for (i, ch) in inner.chars().enumerate() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                comma_pos = Some(i);
                break;
            }
            _ => {}
        }
    }

    let comma_pos = comma_pos.ok_or_else(|| "No comma found in interval".to_string())?;

    let left_val = inner[..comma_pos].trim().to_string();
    let right_val = inner[comma_pos + 1..].trim().to_string();

    if left_val.is_empty() || right_val.is_empty() {
        return Err("Interval bounds cannot be empty".into());
    }

    Ok(Interval {
        left: Bound {
            kind: left_type,
            value: normalize_inf(&left_val),
        },
        right: Bound {
            kind: right_type,
            value: normalize_inf(&right_val),
        },
    })
}

fn normalize_inf(s: &str) -> String {
    let s = s.trim();
    match s {
        "inf" | "infinity" | "oo" | "+inf" | "+infinity" | "+oo" => "inf".to_string(),
        "-inf" | "-infinity" | "-oo" => "-inf".to_string(),
        other => other.to_string(),
    }
}

fn is_inf(s: &str) -> bool {
    matches!(s, "inf" | "-inf")
}

fn bounds_equivalent<E: ExprEngine>(a: &Bound, b: &Bound) -> bool {
    if a.kind != b.kind {
        return false;
    }

    let a_val = normalize_inf(&a.value);
    let b_val = normalize_inf(&b.value);

    // Handle infinity
    if is_inf(&a_val) || is_inf(&b_val) {
        return a_val == b_val;
    }

    // Parse as expressions and check equivalence
    let a_expr = match E::parse(&a_val) {
        Ok(e) => e,
        Err(_) => return false,
    };
    let b_expr = match E::parse(&b_val) {
        Ok(e) => e,
        Err(_) => return false,
    };

    are_equivalent(&a_expr, &b_expr)
}

fn intervals_equivalent<E: ExprEngine>(a: &Interval, b: &Interval) -> bool {
    bounds_equivalent::<E>(&a.left, &b.left) && bounds_equivalent::<E>(&a.right, &b.right)
}

pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str) -> GradeResult {
    // Split on | for answer key unions
    let key_parts: Vec<&str> = answer_key.split('|').collect();

    // Split on U/union for user input unions
    let user_str = user_input.trim();
    let user_parts: Vec<&str> = if user_str.contains(" U ") || user_str.contains(" union ") {
        // Split on " U " or " union "
        let re_split: Vec<&str> = user_str
            .split(" U ")
            .flat_map(|s| s.split(" union "))
            .collect();
        re_split
    } else {
        vec![user_str]
    };

    if key_parts.len() != user_parts.len() {
        return GradeResult::Incorrect;
    }

    // Parse key intervals
    let key_intervals: Vec<Interval> = match key_parts
        .iter()
        .map(|s| parse_key_interval(s.trim()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(v) => v,
        Err(e) => return GradeResult::Error(format!("Invalid interval answer key: {}", e)),
    };

    // Parse user intervals
    let user_intervals: Vec<Interval> = match user_parts
        .iter()
        .map(|s| parse_user_interval(s.trim()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(v) => v,
        Err(_) => return GradeResult::Invalid("Could not parse interval notation".into()),
    };

    // For single intervals, compare directly
    if key_intervals.len() == 1 {
        if intervals_equivalent::<E>(&user_intervals[0], &key_intervals[0]) {
            return GradeResult::Correct;
        }
        return GradeResult::Incorrect;
    }

    // For unions, match each component (sorted comparison would be ideal,
    // but for simplicity do unordered bipartite matching)
    let mut matched = vec![false; user_intervals.len()];
    for ki in &key_intervals {
        let mut found = false;
        for (j, ui) in user_intervals.iter().enumerate() {
            if !matched[j] && intervals_equivalent::<E>(ki, ui) {
                matched[j] = true;
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

    // ── Basic correct cases ──

    #[test]
    fn test_open_open() {
        let r = grade::<NumExpr>("(1, 7)", "open:1,open:7");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_open_closed() {
        let r = grade::<NumExpr>("(1, 7]", "open:1,closed:7");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_closed_open() {
        let r = grade::<NumExpr>("[-2, 4)", "closed:-2,open:4");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_closed_closed() {
        let r = grade::<NumExpr>("[0, 10]", "closed:0,closed:10");
        assert_eq!(r, GradeResult::Correct);
    }

    // ── Infinity bounds ──

    #[test]
    fn test_neg_inf_to_value() {
        let r = grade::<NumExpr>("(-inf, 3]", "open:-inf,closed:3");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_value_to_pos_inf() {
        let r = grade::<NumExpr>("(5, inf)", "open:5,open:inf");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_neg_inf_to_pos_inf() {
        let r = grade::<NumExpr>("(-inf, inf)", "open:-inf,open:inf");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_oo_notation() {
        // "oo" is what convert_latex_to_plain produces for \infty / \inf
        let r = grade::<NumExpr>("(-oo, 6]", "open:-inf,closed:6");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_neg_oo_notation() {
        let r = grade::<NumExpr>("(-oo, oo)", "open:-inf,open:inf");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_infinity_spelled_out() {
        let r = grade::<NumExpr>("(-infinity, 5]", "open:-inf,closed:5");
        assert_eq!(r, GradeResult::Correct);
    }

    // ── Incorrect answers ──

    #[test]
    fn test_wrong_bound_type() {
        // User writes open, answer expects closed
        let r = grade::<NumExpr>("(1, 7)", "open:1,closed:7");
        assert_eq!(r, GradeResult::Incorrect);
    }

    #[test]
    fn test_wrong_value() {
        let r = grade::<NumExpr>("(1, 8]", "open:1,closed:7");
        assert_eq!(r, GradeResult::Incorrect);
    }

    #[test]
    fn test_swapped_brackets() {
        let r = grade::<NumExpr>("[1, 7)", "open:1,closed:7");
        assert_eq!(r, GradeResult::Incorrect);
    }

    // ── Invalid user input ──

    #[test]
    fn test_empty_input() {
        let r = grade::<NumExpr>("", "open:1,closed:7");
        assert_eq!(r, GradeResult::Invalid("Could not parse interval notation".into()));
    }

    #[test]
    fn test_no_comma() {
        let r = grade::<NumExpr>("(1 7)", "open:1,closed:7");
        assert_eq!(r, GradeResult::Invalid("Could not parse interval notation".into()));
    }

    #[test]
    fn test_empty_bounds() {
        // This is what the MathField template produces initially: ( ,)
        let r = grade::<NumExpr>("( ,)", "open:1,closed:7");
        assert_eq!(r, GradeResult::Invalid("Could not parse interval notation".into()));
    }

    #[test]
    fn test_one_empty_bound() {
        let r = grade::<NumExpr>("(1, )", "open:1,closed:7");
        assert_eq!(r, GradeResult::Invalid("Could not parse interval notation".into()));
    }

    #[test]
    fn test_missing_delimiters() {
        let r = grade::<NumExpr>("1, 7", "open:1,closed:7");
        assert_eq!(r, GradeResult::Invalid("Could not parse interval notation".into()));
    }

    // ── Union intervals ──

    #[test]
    fn test_union_correct() {
        let r = grade::<NumExpr>(
            "(-inf, 0] U (2, inf)",
            "open:-inf,closed:0|open:2,open:inf",
        );
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_union_wrong_component_count() {
        let r = grade::<NumExpr>(
            "(-inf, inf)",
            "open:-inf,closed:0|open:2,open:inf",
        );
        assert_eq!(r, GradeResult::Incorrect);
    }

    #[test]
    fn test_union_with_word() {
        let r = grade::<NumExpr>(
            "(-inf, 0] union (2, inf)",
            "open:-inf,closed:0|open:2,open:inf",
        );
        assert_eq!(r, GradeResult::Correct);
    }

    // ── Negative values ──

    #[test]
    fn test_negative_bounds() {
        let r = grade::<NumExpr>("(-5, -1]", "open:-5,closed:-1");
        assert_eq!(r, GradeResult::Correct);
    }

    // ── Decimal values ──

    #[test]
    fn test_decimal_bounds() {
        let r = grade::<NumExpr>("(0.5, 2.5)", "open:0.5,open:2.5");
        assert_eq!(r, GradeResult::Correct);
    }

    // ── Whitespace tolerance ──

    #[test]
    fn test_extra_whitespace() {
        let r = grade::<NumExpr>("  ( 1 , 7 ]  ", "open:1,closed:7");
        assert_eq!(r, GradeResult::Correct);
    }

    // ── Invalid answer key ──

    #[test]
    fn test_bad_answer_key() {
        let r = grade::<NumExpr>("(1, 7]", "bad_format");
        assert!(matches!(r, GradeResult::Error(_)));
    }

    // ── Parser unit tests ──

    #[test]
    fn test_normalize_inf_variants() {
        assert_eq!(normalize_inf("inf"), "inf");
        assert_eq!(normalize_inf("oo"), "inf");
        assert_eq!(normalize_inf("+inf"), "inf");
        assert_eq!(normalize_inf("infinity"), "inf");
        assert_eq!(normalize_inf("+infinity"), "inf");
        assert_eq!(normalize_inf("+oo"), "inf");
        assert_eq!(normalize_inf("-inf"), "-inf");
        assert_eq!(normalize_inf("-oo"), "-inf");
        assert_eq!(normalize_inf("-infinity"), "-inf");
        assert_eq!(normalize_inf("42"), "42");
    }

    // ── End-to-end: LaTeX → plain → grade ──

    #[test]
    fn test_pipeline_interval_with_inf() {
        use crate::latex::convert_latex_to_plain;
        let latex = "\\left(-\\inf ,6\\right]";
        let plain = convert_latex_to_plain(latex);
        assert_eq!(plain, "(-oo ,6]");
        let r = grade::<NumExpr>(&plain, "open:-inf,closed:6");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_pipeline_interval_with_infty() {
        use crate::latex::convert_latex_to_plain;
        let latex = "\\left(-\\infty,6\\right]";
        let plain = convert_latex_to_plain(latex);
        assert_eq!(plain, "(-oo,6]");
        let r = grade::<NumExpr>(&plain, "open:-inf,closed:6");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_pipeline_interval_positive_inf() {
        use crate::latex::convert_latex_to_plain;
        let latex = "\\left(3,\\inf\\right)";
        let plain = convert_latex_to_plain(latex);
        assert_eq!(plain, "(3,oo)");
        let r = grade::<NumExpr>(&plain, "open:3,open:inf");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_pipeline_closed_brackets() {
        use crate::latex::convert_latex_to_plain;
        let latex = "\\left[-2,4\\right]";
        let plain = convert_latex_to_plain(latex);
        assert_eq!(plain, "[-2,4]");
        let r = grade::<NumExpr>(&plain, "closed:-2,closed:4");
        assert_eq!(r, GradeResult::Correct);
    }

    #[test]
    fn test_pipeline_empty_template_does_not_crash() {
        use crate::latex::convert_latex_to_plain;
        let latex = "\\left( ,\\right)";
        let plain = convert_latex_to_plain(latex);
        // Should produce Invalid, not crash
        let r = grade::<NumExpr>(&plain, "open:1,closed:7");
        assert!(matches!(r, GradeResult::Invalid(_)));
    }

    // ── SymEngine (real CAS) tests ──

    mod symengine_tests {
        use super::super::*;
        use crate::symengine::Expr;
        use crate::latex::convert_latex_to_plain;

        #[test]
        fn test_basic_interval() {
            assert_eq!(grade::<Expr>("(1, 7]", "open:1,closed:7"), GradeResult::Correct);
        }

        #[test]
        fn test_equivalent_expressions_as_bounds() {
            // 2+1 == 3, so (0, 2+1] should match (0, 3]
            assert_eq!(grade::<Expr>("(0, 2+1]", "open:0,closed:3"), GradeResult::Correct);
        }

        #[test]
        fn test_fraction_bounds() {
            assert_eq!(grade::<Expr>("(1/2, 3/4)", "open:1/2,open:3/4"), GradeResult::Correct);
        }

        #[test]
        fn test_inf_bounds_symengine() {
            assert_eq!(grade::<Expr>("(-oo, 6]", "open:-inf,closed:6"), GradeResult::Correct);
            assert_eq!(grade::<Expr>("(3, oo)", "open:3,open:inf"), GradeResult::Correct);
        }

        #[test]
        fn test_pipeline_inf_latex_to_grade() {
            let latex = "\\left(-\\inf ,6\\right]";
            let plain = convert_latex_to_plain(latex);
            assert_eq!(grade::<Expr>(&plain, "open:-inf,closed:6"), GradeResult::Correct);
        }

        #[test]
        fn test_pipeline_infty_latex_to_grade() {
            let latex = "\\left(-\\infty,6\\right]";
            let plain = convert_latex_to_plain(latex);
            assert_eq!(grade::<Expr>(&plain, "open:-inf,closed:6"), GradeResult::Correct);
        }

        #[test]
        fn test_pipeline_mixed_bracket_inf() {
            let latex = "\\left[0,\\inf\\right)";
            let plain = convert_latex_to_plain(latex);
            assert_eq!(grade::<Expr>(&plain, "closed:0,open:inf"), GradeResult::Correct);
        }

        #[test]
        fn test_empty_template_does_not_crash() {
            let latex = "\\left( ,\\right)";
            let plain = convert_latex_to_plain(latex);
            let r = grade::<Expr>(&plain, "open:1,closed:7");
            assert!(matches!(r, GradeResult::Invalid(_)));
        }

        #[test]
        fn test_negative_bounds() {
            assert_eq!(grade::<Expr>("(-5, -1]", "open:-5,closed:-1"), GradeResult::Correct);
        }

        #[test]
        fn test_wrong_value_symengine() {
            assert_eq!(grade::<Expr>("(1, 8]", "open:1,closed:7"), GradeResult::Incorrect);
        }

        #[test]
        fn test_union_symengine() {
            assert_eq!(
                grade::<Expr>("(-oo, 0] U (2, oo)", "open:-inf,closed:0|open:2,open:inf"),
                GradeResult::Correct,
            );
        }
    }
}
