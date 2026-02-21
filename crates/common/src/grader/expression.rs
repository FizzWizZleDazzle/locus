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
