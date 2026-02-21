//! Matrix formatter
//!
//! Converts nested array notation to LaTeX bmatrix

use super::common::render_latex;

/// Convert matrix array notation to LaTeX bmatrix
///
/// # Format
/// - Input: "[[3, 4], [5, 6]]"
/// - Output: LaTeX bmatrix with proper row/column formatting
///
/// # Examples
/// ```ignore
/// format_matrix("[[3, 4], [5, 6]]")
/// // -> \begin{bmatrix} 3 & 4 \\ 5 & 6 \end{bmatrix}
/// ```
pub fn format_matrix(answer_key: &str) -> Result<String, String> {
    // "[[3, 4], [5, 6]]" - convert to LaTeX matrix
    // Remove outer brackets and split by rows
    let inner = answer_key
        .trim()
        .strip_prefix("[[")
        .and_then(|s| s.strip_suffix("]]"))
        .unwrap_or(answer_key);

    // Split into rows by "], ["
    let rows: Vec<&str> = inner.split("], [").collect();

    // Convert each row: "3, 4" -> "3 & 4"
    let latex_rows: Vec<String> = rows.iter().map(|row| row.replace(",", " &")).collect();

    let matrix_latex = format!(
        "\\begin{{bmatrix}} {} \\end{{bmatrix}}",
        latex_rows.join(" \\\\ ")
    );

    render_latex(&matrix_latex)
}
