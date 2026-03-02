//! Matrix grader — 2D element-wise comparison.
//!
//! Format: `[[a, b], [c, d]]` — outer brackets contain rows,
//! each row is `[element, element, ...]`.

use super::parse::split_top_level;
use super::{ExprEngine, GradeResult, are_equivalent};

/// Parse a matrix string like `[[1, 2], [3, 4]]` into a 2D grid of strings.
fn parse_matrix(input: &str) -> Result<Vec<Vec<String>>, String> {
    let s = input.trim();

    // Strip outer brackets
    if !s.starts_with('[') || !s.ends_with(']') {
        return Err("Matrix must be enclosed in [ ]".into());
    }
    let inner = &s[1..s.len() - 1];

    // Split into rows at top level
    let row_strs = split_top_level(inner, ',');

    let mut rows = Vec::new();
    for row_str in &row_strs {
        let r = row_str.trim();
        if !r.starts_with('[') || !r.ends_with(']') {
            return Err("Each matrix row must be enclosed in [ ]".into());
        }
        let row_inner = &r[1..r.len() - 1];
        let elements = split_top_level(row_inner, ',');
        rows.push(elements);
    }

    Ok(rows)
}

pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str) -> GradeResult {
    let expected_rows = match parse_matrix(answer_key) {
        Ok(r) => r,
        Err(e) => return GradeResult::Error(format!("Invalid matrix answer key: {}", e)),
    };

    let user_rows = match parse_matrix(user_input) {
        Ok(r) => r,
        Err(_) => return GradeResult::Invalid("Could not parse matrix input".into()),
    };

    // Check dimensions
    if user_rows.len() != expected_rows.len() {
        return GradeResult::Incorrect;
    }

    for (i, (exp_row, usr_row)) in expected_rows.iter().zip(user_rows.iter()).enumerate() {
        if exp_row.len() != usr_row.len() {
            return GradeResult::Incorrect;
        }

        for (j, (exp_s, usr_s)) in exp_row.iter().zip(usr_row.iter()).enumerate() {
            let exp = match E::parse(exp_s.trim()) {
                Ok(e) => e,
                Err(e) => {
                    return GradeResult::Error(format!("Invalid answer key [{},{}]: {}", i, j, e));
                }
            };
            let usr = match E::parse(usr_s.trim()) {
                Ok(e) => e,
                Err(_) => {
                    return GradeResult::Invalid(format!("Could not parse element [{},{}]", i, j));
                }
            };
            if !are_equivalent(&exp, &usr) {
                return GradeResult::Incorrect;
            }
        }
    }

    GradeResult::Correct
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_utils::NumExpr;

    #[test]
    fn test_2x2_correct() {
        assert_eq!(
            grade::<NumExpr>("[[1,2],[3,4]]", "[[1,2],[3,4]]"),
            GradeResult::Correct,
        );
    }

    #[test]
    fn test_2x2_wrong_element() {
        assert_eq!(
            grade::<NumExpr>("[[1,2],[3,5]]", "[[1,2],[3,4]]"),
            GradeResult::Incorrect,
        );
    }

    #[test]
    fn test_wrong_dimensions() {
        assert_eq!(
            grade::<NumExpr>("[[1,2,3],[4,5,6]]", "[[1,2],[3,4]]"),
            GradeResult::Incorrect,
        );
    }

    #[test]
    fn test_not_a_matrix() {
        assert!(matches!(
            grade::<NumExpr>("not a matrix", "[[1,2],[3,4]]"),
            GradeResult::Invalid(_),
        ));
    }

    #[test]
    fn test_3x3_identity() {
        assert_eq!(
            grade::<NumExpr>("[[1,0,0],[0,1,0],[0,0,1]]", "[[1,0,0],[0,1,0],[0,0,1]]"),
            GradeResult::Correct,
        );
    }

    mod symengine_tests {
        use super::super::*;
        use crate::symengine::Expr;
        use crate::latex::convert_latex_to_plain;

        #[test]
        fn test_basic_matrix() {
            assert_eq!(
                grade::<Expr>("[[1,2],[3,4]]", "[[1,2],[3,4]]"),
                GradeResult::Correct,
            );
        }

        #[test]
        fn test_expression_elements() {
            assert_eq!(
                grade::<Expr>("[[1+1,2],[3,4]]", "[[2,2],[3,4]]"),
                GradeResult::Correct,
            );
        }

        #[test]
        fn test_pipeline_matrix() {
            let plain = convert_latex_to_plain(
                "\\begin{pmatrix}1&2\\\\3&4\\end{pmatrix}",
            );
            assert_eq!(grade::<Expr>(&plain, "[[1,2],[3,4]]"), GradeResult::Correct);
        }

        #[test]
        fn test_pipeline_matrix_with_negatives() {
            let plain = convert_latex_to_plain(
                "\\begin{pmatrix}-1&0\\\\0&-1\\end{pmatrix}",
            );
            assert_eq!(grade::<Expr>(&plain, "[[-1,0],[0,-1]]"), GradeResult::Correct);
        }
    }
}
