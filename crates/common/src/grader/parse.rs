//! Shared parsing utilities for type-specific graders.

/// Split a string on a delimiter, respecting `()`, `[]`, `{}` nesting depth.
///
/// For example, `split_top_level("1+2, (3, 4), 5", ',')` returns `["1+2", "(3, 4)", "5"]`.
pub fn split_top_level(input: &str, delimiter: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;

    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' => {
                depth += 1;
                current.push(ch);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                current.push(ch);
            }
            c if c == delimiter && depth == 0 => {
                parts.push(current.trim().to_string());
                current = String::new();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() || !parts.is_empty() {
        parts.push(trimmed);
    }

    parts
}

/// Split an equation on a standalone `=` (not `<=`, `>=`, `==`).
///
/// Returns `(lhs, rhs)` or an error if no valid `=` found or multiple `=`.
pub fn split_equation(input: &str) -> Result<(String, String), String> {
    let chars: Vec<char> = input.chars().collect();
    let mut eq_positions = Vec::new();
    let mut depth = 0i32;

    for i in 0..chars.len() {
        match chars[i] {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            '=' if depth == 0 => {
                // Check it's not <=, >=, or ==
                let prev = if i > 0 { Some(chars[i - 1]) } else { None };
                let next = if i + 1 < chars.len() {
                    Some(chars[i + 1])
                } else {
                    None
                };
                if prev != Some('<')
                    && prev != Some('>')
                    && prev != Some('!')
                    && prev != Some('=')
                    && next != Some('=')
                {
                    eq_positions.push(i);
                }
            }
            _ => {}
        }
    }

    if eq_positions.len() != 1 {
        return Err(format!(
            "Expected exactly one '=' in equation, found {}",
            eq_positions.len()
        ));
    }

    let pos = eq_positions[0];
    let lhs = chars[..pos].iter().collect::<String>().trim().to_string();
    let rhs = chars[pos + 1..]
        .iter()
        .collect::<String>()
        .trim()
        .to_string();

    if lhs.is_empty() || rhs.is_empty() {
        return Err("Empty side in equation".into());
    }

    Ok((lhs, rhs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_top_level_simple() {
        let parts = split_top_level("1, 2, 3", ',');
        assert_eq!(parts, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_split_top_level_nested() {
        let parts = split_top_level("(1, 2), 3, (4, 5)", ',');
        assert_eq!(parts, vec!["(1, 2)", "3", "(4, 5)"]);
    }

    #[test]
    fn test_split_top_level_no_delimiter() {
        let parts = split_top_level("abc", ',');
        assert_eq!(parts, vec!["abc"]);
    }

    #[test]
    fn test_split_equation_basic() {
        let (lhs, rhs) = split_equation("y = 2*x + 3").unwrap();
        assert_eq!(lhs, "y");
        assert_eq!(rhs, "2*x + 3");
    }

    #[test]
    fn test_split_equation_ignores_le_ge() {
        // Should not split on <= or >=
        let result = split_equation("x <= 5");
        assert!(result.is_err());
    }

    #[test]
    fn test_split_equation_ignores_double_eq() {
        let result = split_equation("x == 5");
        assert!(result.is_err());
    }
}
