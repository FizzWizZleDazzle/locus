//! Client-side grading for practice mode

use locus_common::GradingMode;
use crate::symengine::Expr;

/// Check if a user's answer matches the expected answer
///
/// This runs entirely in the browser for instant feedback in practice mode.
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode) -> GradeResult {
    // Parse both expressions
    let user_expr = match Expr::parse(user_input) {
        Ok(e) => e,
        Err(_) => return GradeResult::Invalid("Could not parse your answer".into()),
    };

    let answer_expr = match Expr::parse(answer_key) {
        Ok(e) => e,
        Err(_) => return GradeResult::Error("Invalid answer key".into()),
    };

    let is_correct = match mode {
        GradingMode::Equivalent => {
            // Expand both and compare
            user_expr.expand().equals(&answer_expr.expand())
        }
        GradingMode::Factor => {
            // Must be in factored form AND equal
            user_expr.is_mul() && user_expr.equals(&answer_expr)
        }
    };

    if is_correct {
        GradeResult::Correct
    } else {
        GradeResult::Incorrect
    }
}

/// Preprocess input to add implicit multiplication
///
/// Converts LaTeX-like notation to SymEngine-compatible format:
/// - "2x" -> "2*x"
/// - "xy" -> "x*y"
/// - "2sin(x)" -> "2*sin(x)"
/// - Preserves functions: sin, cos, tan, sqrt, ln, log, exp, etc.
pub fn preprocess_input(input: &str) -> String {
    const FUNCTIONS: &[&str] = &[
        "sin", "cos", "tan", "sec", "csc", "cot",
        "sinh", "cosh", "tanh", "sech", "csch", "coth",
        "arcsin", "arccos", "arctan", "asin", "acos", "atan",
        "sqrt", "exp", "ln", "log", "abs",
    ];

    let mut result = String::with_capacity(input.len() * 2);
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Check if we're at the start of a function name
        if c.is_ascii_alphabetic() {
            let mut matched_func = None;
            for func in FUNCTIONS {
                if input[i..].starts_with(func) {
                    // Make sure it's not part of a larger word
                    let after_idx = i + func.len();
                    let is_complete = after_idx >= chars.len()
                        || !chars[after_idx].is_ascii_alphabetic();

                    if is_complete {
                        matched_func = Some(*func);
                        break;
                    }
                }
            }

            if let Some(func) = matched_func {
                // Insert multiplication before function if needed
                if !result.is_empty() {
                    let last_char = result.chars().last().unwrap();
                    if last_char.is_ascii_alphanumeric() || last_char == ')' {
                        result.push('*');
                    }
                }

                // Add the function name
                result.push_str(func);
                i += func.len();
                continue;
            }
        }

        result.push(c);

        // Check if we need to insert '*' after this character
        // But only if the next position is NOT the start of a function
        if i + 1 < chars.len() {
            let next = chars[i + 1];

            // Check if next position starts a function
            let next_is_func = if next.is_ascii_alphabetic() {
                FUNCTIONS.iter().any(|func| {
                    let after_idx = i + 1 + func.len();
                    input[i + 1..].starts_with(func)
                        && (after_idx >= chars.len() || !chars[after_idx].is_ascii_alphabetic())
                })
            } else {
                false
            };

            if !next_is_func {
                let needs_mul = match (c, next) {
                    // digit followed by letter: 2x -> 2*x
                    (d, l) if d.is_ascii_digit() && l.is_ascii_alphabetic() => true,
                    // letter followed by letter: xy -> x*y
                    (a, b) if a.is_ascii_alphabetic() && b.is_ascii_alphabetic() => true,
                    // closing paren followed by letter or digit or opening paren
                    (')', l) if l.is_ascii_alphanumeric() || l == '(' => true,
                    // letter/digit followed by opening paren
                    (a, '(') if a.is_ascii_alphanumeric() => true,
                    _ => false,
                };

                if needs_mul {
                    result.push('*');
                }
            }
        }

        i += 1;
    }

    result
}

/// Result of grading an answer
#[derive(Debug, Clone, PartialEq)]
pub enum GradeResult {
    /// Answer is correct
    Correct,
    /// Answer is incorrect
    Incorrect,
    /// Input could not be parsed
    Invalid(String),
    /// Grading error
    Error(String),
}

impl GradeResult {
    pub fn is_correct(&self) -> bool {
        matches!(self, GradeResult::Correct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_2x() {
        assert_eq!(preprocess_input("2x"), "2*x");
    }

    #[test]
    fn test_preprocess_xy() {
        assert_eq!(preprocess_input("xy"), "x*y");
    }

    #[test]
    fn test_preprocess_2x_squared() {
        assert_eq!(preprocess_input("2x^2"), "2*x^2");
    }

    #[test]
    fn test_preprocess_parens() {
        assert_eq!(preprocess_input("2(x+1)"), "2*(x+1)");
        assert_eq!(preprocess_input("(x+1)(x-1)"), "(x+1)*(x-1)");
    }

    #[test]
    fn test_preprocess_functions() {
        assert_eq!(preprocess_input("sin(x)"), "sin(x)");
        assert_eq!(preprocess_input("cos(x)"), "cos(x)");
        assert_eq!(preprocess_input("sqrt(x)"), "sqrt(x)");
        assert_eq!(preprocess_input("2sin(x)"), "2*sin(x)");
        assert_eq!(preprocess_input("sin(x)cos(x)"), "sin(x)*cos(x)");
        assert_eq!(preprocess_input("xsin(x)"), "x*sin(x)");
        assert_eq!(preprocess_input("ln(x)"), "ln(x)");
        assert_eq!(preprocess_input("exp(x)"), "exp(x)");
    }

    #[test]
    fn test_check_correct() {
        let result = check_answer("x^2+1", "x^2+1", GradingMode::Equivalent);
        assert_eq!(result, GradeResult::Correct);
    }

    #[test]
    fn test_check_incorrect() {
        let result = check_answer("x^2+2", "x^2+1", GradingMode::Equivalent);
        assert_eq!(result, GradeResult::Incorrect);
    }
}
