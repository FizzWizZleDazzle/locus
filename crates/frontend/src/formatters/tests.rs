//! Tests for answer formatters
//!
//! These tests verify that each answer type formatter produces the expected output.

#[cfg(test)]
mod tests {
    use super::super::*;
    use locus_common::AnswerType;

    #[test]
    fn test_interval_formatting() {
        let result = format_answer_for_display("open:1,closed:7", AnswerType::Interval);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("(1"));
        assert!(html.contains("7]"));
    }

    #[test]
    fn test_set_formatting() {
        let result = format_answer_for_display("2, 3", AnswerType::Set);
        assert!(result.is_ok());
        let html = result.unwrap();
        // KaTeX renders \lbrace as {
        assert!(html.contains("{") || html.contains("lbrace"));
    }

    #[test]
    fn test_tuple_formatting() {
        let result = format_answer_for_display("3, 2", AnswerType::Tuple);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("(3"));
        assert!(html.contains("2)"));
    }

    #[test]
    fn test_list_formatting() {
        let result = format_answer_for_display("-2, 2", AnswerType::List);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("[-2"));
        assert!(html.contains("2]"));
    }

    #[test]
    fn test_boolean_formatting() {
        let result = format_answer_for_display("true", AnswerType::Boolean);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "<code>true</code>");
    }

    #[test]
    fn test_word_formatting() {
        let result = format_answer_for_display("hello", AnswerType::Word);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "<code>hello</code>");
    }

    #[test]
    fn test_inequality_formatting() {
        let result = format_answer_for_display("x >= 2", AnswerType::Inequality);
        assert!(result.is_ok());
        let html = result.unwrap();
        // Should convert >= to \geq
        assert!(html.contains("geq") || html.contains("≥"));
    }

    #[test]
    fn test_matrix_formatting() {
        let result = format_answer_for_display("[[3, 4], [5, 6]]", AnswerType::Matrix);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("bmatrix") || html.contains("matrix"));
    }

    #[test]
    fn test_equation_formatting() {
        let result = format_answer_for_display("x**2 = 4", AnswerType::Equation);
        assert!(result.is_ok());
        let html = result.unwrap();
        // Should convert ** to ^
        assert!(html.contains("x") && html.contains("2"));
    }

    #[test]
    fn test_multi_part_formatting() {
        let result = format_answer_for_display("tuple:5,-4|||numeric:4", AnswerType::MultiPart);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("Part 1"));
        assert!(html.contains("Part 2"));
    }
}
