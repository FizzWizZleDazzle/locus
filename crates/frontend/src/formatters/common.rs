//! Common helpers for LaTeX rendering across formatters

use crate::katex_bindings::{render_math_to_string, render_plain_math_to_string};

/// Render LaTeX math to HTML string (for display mode)
pub fn render_latex(latex: &str) -> Result<String, String> {
    render_math_to_string(latex, false)
}

/// Render plain math notation to HTML string (via Nerdamer preprocessing)
pub fn render_plain(plain: &str) -> Result<String, String> {
    render_plain_math_to_string(plain)
}

/// Wrap raw text in a code tag for display
pub fn render_code(text: &str) -> String {
    format!("<code>{}</code>", text)
}
