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

    // Remove \left and \right delimiters first (keep what follows, e.g. \left( → ()
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

    // Convert matrix environments before other replacements
    result = convert_matrix_envs(&result);

    // Remove LaTeX function backslashes: \sin -> sin, \cos -> cos, etc.
    // Inverse trig (must come before \sin/\cos/\tan to avoid partial matches)
    result = result.replace("\\arcsin", "arcsin");
    result = result.replace("\\arccos", "arccos");
    result = result.replace("\\arctan", "arctan");
    // Hyperbolic (must come before \sin/\cos/\tan)
    result = result.replace("\\sinh", "sinh");
    result = result.replace("\\cosh", "cosh");
    result = result.replace("\\tanh", "tanh");
    // Standard trig
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

    // Comparison operators
    result = result.replace("\\le", "<=");
    result = result.replace("\\ge", ">=");
    result = result.replace("\\ne", "!=");

    // Greek/special symbols
    result = result.replace("\\pi", "pi");
    result = result.replace("\\theta", "theta");
    result = result.replace("\\infty", "oo");

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

    // ========================================================================
    // DELIMITER PROTECTION ALGORITHM
    // ========================================================================
    // Problem: LaTeX commands like \lbrace must produce literal { in output,
    //          but braces are also used for grouping (e.g., x^{2})
    //
    // Solution: Three-pass replacement using control character placeholders
    //
    // Pass 1: Protect explicit delimiters
    //   Replace \lbrace → \x01LBRACE\x01 (protect from next step)
    //   Replace \rbrace → \x01RBRACE\x01
    //   Replace \lbrack → \x01LBRACK\x01
    //   Replace \rbrack → \x01RBRACK\x01
    //
    // Pass 2: Convert grouping braces to parentheses
    //   All remaining { → ( (for exponent grouping, etc.)
    //   All remaining } → )
    //   Example: x^{2} → x^(2)
    //
    // Pass 3: Restore explicit delimiters
    //   \x01LBRACE\x01 → { (explicit brace for sets)
    //   \x01RBRACE\x01 → }
    //   \x01LBRACK\x01 → [ (explicit bracket for lists)
    //   \x01RBRACK\x01 → ]
    //
    // This ensures:
    //   \lbrace 2, 3 \rbrace  →  {2, 3}     (set notation)
    //   x^{2}                 →  x^(2)       (exponent grouping)
    //   \frac{1}{x}           →  (1)/(x)     (fraction grouping)
    // ========================================================================

    // Pass 1: Protect explicit brace/bracket commands from conversion
    // MathQuill outputs \{ and \} for literal braces (sets)
    result = result.replace("\\{", "\x01LBRACE\x01");
    result = result.replace("\\}", "\x01RBRACE\x01");
    result = result.replace("\\lbrace", "\x01LBRACE\x01");
    result = result.replace("\\rbrace", "\x01RBRACE\x01");
    result = result.replace("\\lbrack", "\x01LBRACK\x01");
    result = result.replace("\\rbrack", "\x01RBRACK\x01");

    // Pass 2: Convert any remaining unmatched braces to parens (for grouping)
    result = result.replace('{', "(");
    result = result.replace('}', ")");

    // Pass 3: Restore explicit braces and brackets
    result = result.replace("\x01LBRACE\x01", "{");
    result = result.replace("\x01RBRACE\x01", "}");
    result = result.replace("\x01LBRACK\x01", "[");
    result = result.replace("\x01RBRACK\x01", "]");

    // Add explicit multiplication for implicit multiplication cases
    // This handles: )( -> )*(, )x -> )*x, 2( -> 2*(, x( -> x*(
    result = add_explicit_multiplication(&result);

    result
}

/// Add explicit multiplication operators where they're implied.
///
/// Handles cases like:
/// - Adjacent parentheses: (x+1)(x-1) -> (x+1)*(x-1)
/// - Number before paren: 2(x+1) -> 2*(x+1)
/// - Single variable before paren: x(x+1) -> x*(x+1)
/// - Paren before variable: (x+1)y -> (x+1)*y
///
/// Does NOT add multiplication after function names (sin, cos, sqrt, etc.)
fn add_explicit_multiplication(input: &str) -> String {
    const FUNCTIONS: &[&str] = &[
        "sin", "cos", "tan", "sec", "csc", "cot", "asin", "acos", "atan", "arcsin", "arccos",
        "arctan", "sinh", "cosh", "tanh", "ln", "log", "exp", "abs", "sqrt",
    ];

    let mut result = String::with_capacity(input.len() * 2);
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        result.push(chars[i]);

        // Check if we need to insert * after this character
        if i + 1 < chars.len() {
            let current = chars[i];
            let next = chars[i + 1];

            let needs_mult = match (current, next) {
                // )( -> )*(
                (')', '(') => true,
                // digit followed by ( or letter
                (c, '(') if c.is_numeric() => true,
                (c, n) if c.is_numeric() && n.is_alphabetic() => true,
                // ) followed by letter
                (')', c) if c.is_alphabetic() => true,
                // letter followed by ( -> check if it's a function name
                (c, '(') if c.is_alphabetic() => {
                    // Look backward to get the full word before (
                    let word_start = result
                        .rfind(|c: char| !c.is_alphabetic())
                        .map(|p| p + 1)
                        .unwrap_or(0);
                    let word = &result[word_start..];
                    // Only add * if it's NOT a known function
                    !FUNCTIONS.contains(&word)
                }
                _ => false,
            };

            if needs_mult {
                result.push('*');
            }
        }
    }

    result
}

/// Convert LaTeX matrix environments to [[row1],[row2]] notation.
///
/// Handles \begin{pmatrix}, \begin{bmatrix}, \begin{matrix}, \begin{vmatrix}.
/// Rows are separated by \\ and cells by &.
/// Example: \begin{pmatrix}1&2\\3&4\end{pmatrix} → [[1,2],[3,4]]
fn convert_matrix_envs(input: &str) -> String {
    let mut result = input.to_string();

    for env in &["pmatrix", "bmatrix", "matrix", "vmatrix"] {
        let begin_tag = format!("\\begin{{{}}}", env);
        let end_tag = format!("\\end{{{}}}", env);

        while let Some(start) = result.find(&begin_tag) {
            if let Some(end_rel) = result[start..].find(&end_tag) {
                let content_start = start + begin_tag.len();
                let content_end = start + end_rel;
                let content = result[content_start..content_end].trim().to_string();

                // Parse rows (split by \\) and cells (split by &)
                let rows: Vec<String> = content
                    .split("\\\\")
                    .map(|row| {
                        let cells: Vec<String> = row
                            .split('&')
                            .map(|cell| cell.trim().to_string())
                            .collect();
                        format!("[{}]", cells.join(","))
                    })
                    .collect();

                let matrix_str = format!("[{}]", rows.join(","));
                let replace_end = content_end + end_tag.len();
                result.replace_range(start..replace_end, &matrix_str);
            } else {
                break; // No matching \end tag
            }
        }
    }

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

    #[test]
    fn test_implicit_multiplication_parentheses() {
        assert_eq!(convert_latex_to_plain("(x+1)(x-1)"), "(x+1)*(x-1)");
        assert_eq!(convert_latex_to_plain("(x+2)(x+3)"), "(x+2)*(x+3)");
        assert_eq!(convert_latex_to_plain("(2x+1)(x+3)"), "(2*x+1)*(x+3)");
    }

    #[test]
    fn test_implicit_multiplication_number_paren() {
        assert_eq!(convert_latex_to_plain("2(x+1)"), "2*(x+1)");
        assert_eq!(convert_latex_to_plain("3(a+b)"), "3*(a+b)");
    }

    #[test]
    fn test_implicit_multiplication_paren_var() {
        assert_eq!(convert_latex_to_plain("(x+1)y"), "(x+1)*y");
        assert_eq!(convert_latex_to_plain("(a+b)c"), "(a+b)*c");
    }

    #[test]
    fn test_implicit_multiplication_var_paren() {
        assert_eq!(convert_latex_to_plain("x(y+1)"), "x*(y+1)");
        assert_eq!(convert_latex_to_plain("a(b+c)"), "a*(b+c)");
    }

    // === MathQuill-specific conversions ===

    #[test]
    fn test_escaped_braces() {
        // MathQuill outputs \{ and \} for sets
        assert_eq!(convert_latex_to_plain("\\{1, 2, 3\\}"), "{1, 2, 3}");
        assert_eq!(
            convert_latex_to_plain("\\left\\{1, 2\\right\\}"),
            "{1, 2}"
        );
    }

    #[test]
    fn test_inverse_trig() {
        assert_eq!(convert_latex_to_plain("\\arcsin(x)"), "arcsin(x)");
        assert_eq!(convert_latex_to_plain("\\arccos(x)"), "arccos(x)");
        assert_eq!(convert_latex_to_plain("\\arctan(x)"), "arctan(x)");
    }

    #[test]
    fn test_hyperbolic_trig() {
        assert_eq!(convert_latex_to_plain("\\sinh(x)"), "sinh(x)");
        assert_eq!(convert_latex_to_plain("\\cosh(x)"), "cosh(x)");
        assert_eq!(convert_latex_to_plain("\\tanh(x)"), "tanh(x)");
    }

    #[test]
    fn test_greek_symbols() {
        assert_eq!(convert_latex_to_plain("\\pi"), "pi");
        assert_eq!(convert_latex_to_plain("2\\pi"), "2*pi");
        assert_eq!(convert_latex_to_plain("\\theta"), "theta");
        assert_eq!(convert_latex_to_plain("\\infty"), "oo");
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(convert_latex_to_plain("x\\le5"), "x<=5");
        assert_eq!(convert_latex_to_plain("x\\ge-3"), "x>=-3");
        assert_eq!(convert_latex_to_plain("x\\ne0"), "x!=0");
    }

    #[test]
    fn test_matrix_pmatrix() {
        assert_eq!(
            convert_latex_to_plain("\\begin{pmatrix}1&2\\\\3&4\\end{pmatrix}"),
            "[[1,2],[3,4]]"
        );
    }

    #[test]
    fn test_matrix_bmatrix() {
        assert_eq!(
            convert_latex_to_plain("\\begin{bmatrix}a&b\\\\c&d\\end{bmatrix}"),
            "[[a,b],[c,d]]"
        );
    }

    #[test]
    fn test_matrix_3x3() {
        assert_eq!(
            convert_latex_to_plain("\\begin{pmatrix}1&0&0\\\\0&1&0\\\\0&0&1\\end{pmatrix}"),
            "[[1,0,0],[0,1,0],[0,0,1]]"
        );
    }

    #[test]
    fn test_interval_bracket_types() {
        // MathQuill outputs \left( and \right] etc., after \left/\right removal:
        assert_eq!(convert_latex_to_plain("\\left(1, 7\\right]"), "(1, 7]");
        assert_eq!(convert_latex_to_plain("\\left[-2, 4\\right)"), "[-2, 4)");
        assert_eq!(convert_latex_to_plain("\\left[0, \\infty\\right)"), "[0, oo)");
    }
}
