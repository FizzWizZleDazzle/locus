//! LaTeX to plain text conversion
//!
//! Converts LaTeX mathematical notation to plain text that can be parsed
//! by symbolic math engines like SymEngine.

/// Convert LaTeX commands to plain text notation for symbolic parsing.
///
/// Handles: \frac, \sqrt, trig functions, exponent braces, delimiters.
/// The symbolic engine handles implicit multiplication (2x, xy, etc.)
/// so we only need to convert LaTeX syntax to plain math notation.
pub fn convert_latex_to_plain(input: &str) -> String {
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

    for c in s.chars() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_functions() {
        assert_eq!(convert_latex_to_plain("\\cos(x)"), "cos(x)");
        assert_eq!(convert_latex_to_plain("\\sin(x)"), "sin(x)");
        assert_eq!(convert_latex_to_plain("\\tan(x)"), "tan(x)");
    }

    #[test]
    fn test_latex_fractions() {
        assert_eq!(convert_latex_to_plain("\\frac{1}{x}"), "(1)/(x)");
        assert_eq!(convert_latex_to_plain("\\frac{x^2}{2}"), "(x^2)/(2)");
    }

    #[test]
    fn test_latex_delimiters() {
        assert_eq!(convert_latex_to_plain("\\left(x\\right)"), "(x)");
    }

    #[test]
    fn test_latex_cdot_times() {
        assert_eq!(convert_latex_to_plain("2\\cdot x"), "2* x");
        assert_eq!(convert_latex_to_plain("a\\times b"), "a* b");
    }

    #[test]
    fn test_latex_exponents_with_braces() {
        assert_eq!(convert_latex_to_plain("x^{2}"), "x^(2)");
        assert_eq!(convert_latex_to_plain("e^{x+1}"), "e^(x+1)");
    }

    #[test]
    fn test_latex_sqrt_with_braces() {
        assert_eq!(convert_latex_to_plain("\\sqrt{x}"), "sqrt(x)");
        assert_eq!(convert_latex_to_plain("\\sqrt{x+1}"), "sqrt(x+1)");
    }

    #[test]
    fn test_latex_sqrt_implicit() {
        assert_eq!(convert_latex_to_plain("\\sqrt2"), "sqrt(2)");
        assert_eq!(convert_latex_to_plain("\\sqrtx"), "sqrt(x)");
    }

    #[test]
    fn test_nested_fractions() {
        assert_eq!(
            convert_latex_to_plain("\\frac{\\frac{1}{2}}{3}"),
            "((1)/(2))/(3)"
        );
    }
}
