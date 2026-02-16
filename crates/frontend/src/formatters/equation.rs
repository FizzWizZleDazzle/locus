//! Equation formatter
//!
//! Converts plain notation equations to LaTeX without expansion

use super::common::render_latex;

/// Format equation for display without symbolic expansion
///
/// Converts plain notation operators to LaTeX:
/// - ** -> ^ (exponentiation)
/// - * -> \cdot (multiplication)
///
/// # Examples
/// ```ignore
/// format_equation("x**2 = 4") // -> "x^2 = 4"
/// format_equation("2*x = 6") // -> "2 \cdot x = 6"
/// ```
pub fn format_equation(answer_key: &str) -> Result<String, String> {
    // For equations, convert plain notation to LaTeX manually to avoid Nerdamer expanding
    // Convert ** to ^ and render directly as LaTeX
    let latex = answer_key
        .replace("**", "^")
        .replace("*", "\\cdot ");

    // Render the LaTeX directly without Nerdamer processing
    render_latex(&latex)
}
