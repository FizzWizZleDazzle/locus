//! Ordered grader — element-wise ordered comparison (Tuple + List).

use super::{ExprEngine, GradeResult, are_equivalent};
use super::parse::split_top_level;

/// Strip optional `(` `)` or `[` `]` from input.
fn strip_delimiters(input: &str) -> &str {
    let s = input.trim();
    if (s.starts_with('(') && s.ends_with(')'))
        || (s.starts_with('[') && s.ends_with(']'))
    {
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
            Err(e) => return GradeResult::Error(format!("Invalid answer key element {}: {}", i + 1, e)),
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
