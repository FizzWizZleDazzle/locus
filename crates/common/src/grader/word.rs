//! Word grader — case-insensitive exact string match.

use super::GradeResult;

pub fn grade(user_input: &str, answer_key: &str) -> GradeResult {
    let user = user_input.trim().to_lowercase();
    let expected = answer_key.trim().to_lowercase();

    if user.is_empty() {
        return GradeResult::Invalid("No answer provided".into());
    }

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
    fn test_exact_match() {
        assert!(grade("maximum", "maximum").is_correct());
    }

    #[test]
    fn test_case_insensitive() {
        assert!(grade("Maximum", "maximum").is_correct());
        assert!(grade("MINIMUM", "minimum").is_correct());
    }

    #[test]
    fn test_whitespace_trimmed() {
        assert!(grade("  maximum  ", "maximum").is_correct());
    }

    #[test]
    fn test_wrong_word() {
        assert!(!grade("minimum", "maximum").is_correct());
    }

    #[test]
    fn test_empty_input() {
        assert!(matches!(grade("", "maximum"), GradeResult::Invalid(_)));
    }
}
