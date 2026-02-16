//! Set notation formatter
//!
//! Wraps comma-separated values in set braces: {2, 3}

use super::common::render_latex;

/// Format set notation with braces
///
/// # Examples
/// ```ignore
/// format_set("2, 3") // -> "{2, 3}"
/// format_set("-1, 0, 1") // -> "{-1, 0, 1}"
/// ```
pub fn format_set(answer_key: &str) -> Result<String, String> {
    // "2, 3" -> "{2, 3}"
    // Render as LaTeX with \lbrace \rbrace
    let latex = format!("\\lbrace {} \\rbrace", answer_key);
    render_latex(&latex)
}
