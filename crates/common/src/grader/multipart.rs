//! MultiPart grader — split on `|||`, each part graded independently.
//!
//! Answer key format: `tuple:5,-4|||numeric:4`
//! User input format: `(5, -4)|||4`

use crate::{AnswerType, GradingMode};
use super::GradeResult;

pub fn grade(user_input: &str, answer_key: &str) -> GradeResult {
    let key_parts: Vec<&str> = answer_key.split("|||").collect();
    let user_parts: Vec<&str> = user_input.split("|||").collect();

    if key_parts.len() != user_parts.len() {
        return GradeResult::Incorrect;
    }

    for (i, (key_part, user_part)) in key_parts.iter().zip(user_parts.iter()).enumerate() {
        let key_part = key_part.trim();
        let user_part = user_part.trim();

        // Parse type prefix from answer key: `type:value`
        let (part_type, key_value) = match key_part.split_once(':') {
            Some((t, v)) => {
                let answer_type = match AnswerType::from_str(t) {
                    Some(at) => at,
                    None => return GradeResult::Error(format!("Unknown type '{}' in part {}", t, i + 1)),
                };
                (answer_type, v)
            }
            None => {
                // No type prefix — default to expression
                (AnswerType::Expression, key_part)
            }
        };

        // Grade this part using the specified type
        let result = super::grade_answer(user_part, key_value, part_type, GradingMode::Equivalent);

        match result {
            GradeResult::Correct => continue,
            GradeResult::Incorrect => return GradeResult::Incorrect,
            GradeResult::Invalid(msg) => return GradeResult::Invalid(format!("Part {}: {}", i + 1, msg)),
            GradeResult::Error(msg) => return GradeResult::Error(format!("Part {}: {}", i + 1, msg)),
        }
    }

    GradeResult::Correct
}
