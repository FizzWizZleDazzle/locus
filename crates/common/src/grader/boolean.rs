//! Boolean grader — true/false answers.

use super::GradeResult;

/// Parse a boolean value from user input.
fn parse_bool(input: &str) -> Option<bool> {
    match input.trim().to_lowercase().as_str() {
        "true" | "yes" | "t" | "1" => Some(true),
        "false" | "no" | "f" | "0" => Some(false),
        _ => None,
    }
}

pub fn grade(user_input: &str, answer_key: &str) -> GradeResult {
    let expected = match parse_bool(answer_key) {
        Some(b) => b,
        None => return GradeResult::Error(format!("Invalid boolean answer key: {}", answer_key)),
    };

    let user = match parse_bool(user_input) {
        Some(b) => b,
        None => return GradeResult::Invalid("Expected true or false".into()),
    };

    if user == expected {
        GradeResult::Correct
    } else {
        GradeResult::Incorrect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_match() {
        assert!(grade("true", "true").is_correct());
        assert!(grade("True", "true").is_correct());
        assert!(grade("yes", "true").is_correct());
        assert!(grade("1", "true").is_correct());
        assert!(grade("t", "true").is_correct());
    }

    #[test]
    fn test_false_match() {
        assert!(grade("false", "false").is_correct());
        assert!(grade("False", "false").is_correct());
        assert!(grade("no", "false").is_correct());
        assert!(grade("0", "false").is_correct());
    }

    #[test]
    fn test_mismatch() {
        assert!(!grade("true", "false").is_correct());
        assert!(!grade("false", "true").is_correct());
    }

    #[test]
    fn test_invalid_input() {
        assert!(matches!(grade("maybe", "true"), GradeResult::Invalid(_)));
    }
}
