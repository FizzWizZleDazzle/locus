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
