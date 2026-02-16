//! Inequality formatter
//!
//! Converts comparison operators to LaTeX symbols

use super::common::render_latex;

/// Format inequality with proper LaTeX comparison symbols
///
/// Converts operators:
/// - >= -> \geq
/// - <= -> \leq
/// - > -> >
/// - < -> <
/// - ** -> ^ (exponentiation)
///
/// # Examples
/// ```ignore
/// format_inequality("x > -4") // -> "x > -4"
/// format_inequality("y >= 2") // -> "y \geq 2"
/// ```
pub fn format_inequality(answer_key: &str) -> Result<String, String> {
    // "x > -4" - convert to LaTeX without Nerdamer (it evaluates comparisons to boolean)
    let latex = answer_key
        .replace("**", "^")
        .replace(">=", r"\geq ")
        .replace("<=", r"\leq ")
        .replace(">", r" > ")
        .replace("<", r" < ");

    render_latex(&latex)
}
