//! Interval notation formatter
//!
//! Converts internal format (open:1,closed:7) to display notation: (1, 7]

use super::common::{render_latex, render_code};

/// Format interval from internal notation to display
///
/// # Format
/// - Input: "open:1,closed:7" or "closed:-2,open:4"
/// - Output: "(1, 7]" or "[-2, 4)"
///
/// # Examples
/// ```ignore
/// format_interval("open:1,closed:7") // -> "(1, 7]"
/// format_interval("closed:-2,open:4") // -> "[-2, 4)"
/// ```
pub fn format_interval(answer_key: &str) -> Result<String, String> {
    // Parse internal format: "open:1,closed:7" -> "(1, 7]"
    let parts: Vec<&str> = answer_key.split(',').collect();
    if parts.len() != 2 {
        return Ok(render_code(answer_key));
    }

    let (left_bracket, left_val) = if let Some(val) = parts[0].strip_prefix("open:") {
        ("(", val)
    } else if let Some(val) = parts[0].strip_prefix("closed:") {
        ("[", val)
    } else {
        return Ok(render_code(answer_key));
    };

    let (right_val, right_bracket) = if let Some(val) = parts[1].strip_prefix("open:") {
        (val, ")")
    } else if let Some(val) = parts[1].strip_prefix("closed:") {
        (val, "]")
    } else {
        return Ok(render_code(answer_key));
    };

    let latex = format!("{}{}, {}{}", left_bracket, left_val, right_val, right_bracket);
    render_latex(&latex)
}
