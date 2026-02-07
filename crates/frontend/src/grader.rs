//! Client-side grading for practice mode.
//!
//! Implements the `ExprEngine` trait from `locus_common` using
//! SymEngine WASM FFI, then delegates grading to the shared logic.

use locus_common::GradingMode;
use locus_common::grader::{self, ExprEngine};
use crate::symengine::Expr;

// Re-export GradeResult so existing code doesn't need to change imports
pub use locus_common::grader::GradeResult;

// Implement the shared ExprEngine trait for our WASM-backed Expr type
impl ExprEngine for Expr {
    type Error = crate::symengine::ExprError;

    fn parse(input: &str) -> Result<Self, Self::Error> {
        Expr::parse(input)
    }

    fn expand(&self) -> Self {
        Expr::expand(self)
    }

    fn sub(&self, other: &Self) -> Self {
        Expr::sub(self, other)
    }

    fn equals(&self, other: &Self) -> bool {
        Expr::equals(self, other)
    }

    fn is_zero(&self) -> bool {
        Expr::is_zero(self)
    }

    fn free_symbols(&self) -> Vec<String> {
        Expr::free_symbols(self)
    }

    fn subs_float(&self, var_name: &str, val: f64) -> Self {
        Expr::subs_float(self, var_name, val)
    }

    fn to_float(&self) -> Option<f64> {
        Expr::to_float(self)
    }
}

/// Check if a user's answer matches the expected answer.
///
/// Delegates to the shared grading logic in `locus_common::grader`,
/// using SymEngine WASM for symbolic computation.
pub fn check_answer(user_input: &str, answer_key: &str, mode: GradingMode) -> GradeResult {
    grader::check_answer::<Expr>(user_input, answer_key, mode)
}

/// Convert LaTeX commands to plain text notation for SymEngine parsing.
///
/// Handles: \frac, \sqrt, trig functions, exponent braces, delimiters.
/// SymEngine's parser handles implicit multiplication (2x, xy, etc.)
/// so we only need to convert LaTeX syntax to plain math notation.
fn convert_latex_to_plain(input: &str) -> String {
    let mut result = input.to_string();

    // Remove \left and \right delimiters first
    result = result.replace("\\left", "");
    result = result.replace("\\right", "");

    // Convert fractions: \frac{a}{b} -> (a)/(b) - do this first before other replacements
    while let Some(frac_start) = result.find("\\frac") {
        if let Some(numerator) = extract_braced_content(&result[frac_start + 5..]) {
            let after_num = frac_start + 5 + numerator.len() + 2; // +2 for braces
            if let Some(denominator) = extract_braced_content(&result[after_num..]) {
                let frac_end = after_num + denominator.len() + 2;
                // Recursively convert LaTeX in numerator and denominator
                let num_plain = convert_latex_to_plain(&numerator);
                let den_plain = convert_latex_to_plain(&denominator);
                let fraction = format!("({})/({})", num_plain, den_plain);
                result.replace_range(frac_start..frac_end, &fraction);
                continue;
            }
        }
        break; // If parsing failed, stop to avoid infinite loop
    }

    // Handle \sqrt specially - it can have implicit argument like \sqrt2 or explicit \sqrt{x}
    while let Some(sqrt_pos) = result.find("\\sqrt") {
        let after_sqrt = sqrt_pos + 5; // Length of "\sqrt"

        if after_sqrt < result.len() {
            let rest = &result[after_sqrt..];

            if rest.starts_with('{') {
                // \sqrt{content} case - extract braced content
                if let Some(content) = extract_braced_content(rest) {
                    let sqrt_end = after_sqrt + content.len() + 2;
                    let converted = format!("sqrt({})", convert_latex_to_plain(&content));
                    result.replace_range(sqrt_pos..sqrt_end, &converted);
                    continue;
                }
            } else if let Some(next_char) = rest.chars().next() {
                // \sqrt2 or \sqrtx case - single character argument
                if next_char.is_alphanumeric() {
                    let sqrt_end = after_sqrt + next_char.len_utf8();
                    let converted = format!("sqrt({})", next_char);
                    result.replace_range(sqrt_pos..sqrt_end, &converted);
                    continue;
                }
            }
        }

        // Fallback: just remove backslash if no argument found
        result.replace_range(sqrt_pos..sqrt_pos + 5, "sqrt");
    }

    // Remove LaTeX function backslashes: \sin -> sin, \cos -> cos, etc.
    result = result.replace("\\sin", "sin");
    result = result.replace("\\cos", "cos");
    result = result.replace("\\tan", "tan");
    result = result.replace("\\sec", "sec");
    result = result.replace("\\csc", "csc");
    result = result.replace("\\cot", "cot");
    result = result.replace("\\ln", "ln");
    result = result.replace("\\log", "log");
    result = result.replace("\\exp", "exp");
    result = result.replace("\\abs", "abs");
    result = result.replace("\\cdot", "*");
    result = result.replace("\\times", "*");

    // Handle exponents with braces: x^{2} -> x^(2), e^{x+1} -> e^(x+1)
    while let Some(exp_start) = result.find("^{") {
        if let Some(exp_content) = extract_braced_content(&result[exp_start + 1..]) {
            let exp_end = exp_start + 1 + exp_content.len() + 2; // +1 for ^, +2 for braces
            let converted = format!("^({})", convert_latex_to_plain(&exp_content));
            result.replace_range(exp_start..exp_end, &converted);
        } else {
            break;
        }
    }

    // Handle subscripts with braces similarly
    while let Some(sub_start) = result.find("_{") {
        if let Some(sub_content) = extract_braced_content(&result[sub_start + 1..]) {
            let sub_end = sub_start + 1 + sub_content.len() + 2;
            let converted = format!("_{}", convert_latex_to_plain(&sub_content));
            result.replace_range(sub_start..sub_end, &converted);
        } else {
            break;
        }
    }

    // Convert any remaining unmatched braces to parens (for grouping)
    result = result.replace('{', "(");
    result = result.replace('}', ")");

    result
}

/// Extract content between braces starting at the beginning of the string
fn extract_braced_content(s: &str) -> Option<String> {
    let s = s.trim_start();
    if !s.starts_with('{') {
        return None;
    }

    let mut depth = 0;
    let mut content = String::new();

    for (_i, c) in s.chars().enumerate() {
        if c == '{' {
            depth += 1;
            if depth > 1 {
                content.push(c);
            }
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                return Some(content);
            }
            content.push(c);
        } else if depth > 0 {
            content.push(c);
        }
    }

    None
}

/// Preprocess user input for SymEngine: convert LaTeX to plain math notation.
///
/// SymEngine's parser handles implicit multiplication natively,
/// so we only need to convert LaTeX syntax here.
pub fn preprocess_input(input: &str) -> String {
    convert_latex_to_plain(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_functions() {
        assert_eq!(preprocess_input("\\cos(x)"), "cos(x)");
        assert_eq!(preprocess_input("\\sin(x)"), "sin(x)");
        assert_eq!(preprocess_input("\\tan(x)"), "tan(x)");
    }

    #[test]
    fn test_latex_fractions() {
        assert_eq!(preprocess_input("\\frac{1}{x}"), "(1)/(x)");
        assert_eq!(preprocess_input("\\frac{x^2}{2}"), "(x^2)/(2)");
    }

    #[test]
    fn test_latex_delimiters() {
        assert_eq!(preprocess_input("\\left(x\\right)"), "(x)");
    }

    #[test]
    fn test_latex_cdot_times() {
        assert_eq!(preprocess_input("2\\cdot x"), "2* x");
        assert_eq!(preprocess_input("a\\times b"), "a* b");
    }

    #[test]
    fn test_latex_exponents_with_braces() {
        assert_eq!(preprocess_input("x^{2}"), "x^(2)");
        assert_eq!(preprocess_input("e^{x+1}"), "e^(x+1)");
    }

    #[test]
    fn test_check_equivalent_correct() {
        let result = check_answer("x^2+1", "x^2+1", GradingMode::Equivalent);
        assert_eq!(result, GradeResult::Correct);
    }

    #[test]
    fn test_check_equivalent_incorrect() {
        let result = check_answer("x^2+2", "x^2+1", GradingMode::Equivalent);
        assert_eq!(result, GradeResult::Incorrect);
    }

    #[test]
    fn test_check_equivalent_reordered() {
        let result = check_answer("1+x^2", "x^2+1", GradingMode::Equivalent);
        assert_eq!(result, GradeResult::Correct);
    }

    #[test]
    fn test_factor_correct() {
        let result = check_answer("(x+1)*(x-1)", "x^2-1", GradingMode::Factor);
        assert_eq!(result, GradeResult::Correct);
    }

    #[test]
    fn test_factor_rejects_expanded_form() {
        let result = check_answer("x^2-1", "x^2-1", GradingMode::Factor);
        assert_eq!(result, GradeResult::Incorrect);
    }

    #[test]
    fn test_expand_correct() {
        let result = check_answer("x^2+2*x+1", "(x+1)^2", GradingMode::Expand);
        assert_eq!(result, GradeResult::Correct);
    }

    #[test]
    fn test_expand_rejects_factored_form() {
        let result = check_answer("(x+1)^2", "(x+1)^2", GradingMode::Expand);
        assert_eq!(result, GradeResult::Incorrect);
    }
}
