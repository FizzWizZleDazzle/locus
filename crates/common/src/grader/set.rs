//! Set grader — unordered element-wise comparison.

use super::{ExprEngine, GradeResult, are_equivalent};
use super::parse::split_top_level;

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
