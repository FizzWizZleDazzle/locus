//! Multi-part answer formatter
//!
//! Parses and formats answers with multiple parts

use super::common::render_latex;

/// Format multi-part answer with labeled parts
///
/// # Format
/// - Input: "tuple:5,-4|||numeric:4"
/// - Output: HTML with each part labeled and formatted
///
/// Each part is formatted as: type:value
/// - tuple:5,-4 -> (5, -4)
/// - set:2,3 -> {2, 3}
/// - list:1,2 -> [1, 2]
/// - numeric:4 -> 4
///
/// # Examples
/// ```ignore
/// format_multi_part("tuple:5,-4|||numeric:4")
/// // -> <div><strong>Part 1:</strong> (5, -4)</div>
/// //    <div><strong>Part 2:</strong> 4</div>
/// ```
pub fn format_multi_part(answer_key: &str) -> Result<String, String> {
    // "tuple:5,-4|||numeric:4" -> render each part as math
    let parts: Vec<&str> = answer_key.split("|||").collect();

    let formatted_parts: Result<Vec<String>, String> = parts.iter().enumerate().map(|(i, part)| {
        if let Some((type_str, value)) = part.split_once(':') {
            let latex = match type_str {
                "tuple" => format!("({})", value),
                "set" => format!("\\lbrace {} \\rbrace", value),
                "list" => format!("[{}]", value),
                _ => value.to_string(),
            };
            let rendered = render_latex(&latex)?;
            Ok(format!("<div><strong>Part {}:</strong> {}</div>", i + 1, rendered))
        } else {
            Ok(format!("<div><strong>Part {}:</strong> <code>{}</code></div>", i + 1, part))
        }
    }).collect();

    Ok(formatted_parts?.join(""))
}
