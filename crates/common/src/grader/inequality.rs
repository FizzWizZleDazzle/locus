//! Inequality grader — parses inequalities, converts to interval representation, then compares.
//!
//! Handles: `x > -4`, `x <= 1`, `-2 < x <= 5`, and flipped forms like `-4 < x`.

use super::interval;
use super::{ExprEngine, GradeResult};

/// Convert an inequality string to interval answer key format, then delegate to interval grader.
pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str) -> GradeResult {
    let key_interval = match inequality_to_interval_key(answer_key) {
        Ok(s) => s,
        Err(e) => return GradeResult::Error(format!("Invalid inequality answer key: {}", e)),
    };

    let user_interval = match inequality_to_interval_key(user_input) {
        Ok(s) => s,
        Err(_) => return GradeResult::Invalid("Could not parse inequality".into()),
    };

    // Now compare as interval format
    // We need to convert both to user-style interval notation and use interval grader
    let user_user_style = interval_key_to_user_notation(&user_interval);

    // Compare using interval grader
    interval::grade::<E>(&user_user_style, &key_interval)
}

/// Parse an inequality and convert to interval key format.
///
/// Examples:
/// - `x > -4`       -> `open:-4,open:inf`
/// - `x <= 1`       -> `open:-inf,closed:1`
/// - `-2 < x <= 5`  -> `open:-2,closed:5`
/// - `-4 < x`       -> `open:-4,open:inf`
fn inequality_to_interval_key(input: &str) -> Result<String, String> {
    let s = input.trim();

    // Try compound inequality first: `a < x <= b` or `a <= x < b` etc.
    if let Some(result) = try_parse_compound(s) {
        return result;
    }

    // Try simple inequality: `x > a`, `x <= a`, `a < x`, etc.
    if let Some(result) = try_parse_simple(s) {
        return result;
    }

    Err(format!("Could not parse inequality: {}", s))
}

/// Try parsing compound inequality like `-2 < x <= 5`
fn try_parse_compound(s: &str) -> Option<Result<String, String>> {
    // Look for patterns with two relational operators
    // Pattern: value op1 var op2 value
    let ops = ["<=", ">=", "<", ">"];

    for op1 in &ops {
        if let Some(pos1) = s.find(op1) {
            let left = s[..pos1].trim();
            let rest = s[pos1 + op1.len()..].trim();

            for op2 in &ops {
                if let Some(pos2) = rest.find(op2) {
                    let var = rest[..pos2].trim();
                    let right = rest[pos2 + op2.len()..].trim();

                    // Check var is a single variable name
                    if !is_variable(var) {
                        continue;
                    }

                    // Determine bound types
                    let left_bound = match *op1 {
                        "<" => "open",
                        "<=" => "closed",
                        ">" => "open",    // reversed: value > var means var < value
                        ">=" => "closed", // reversed
                        _ => continue,
                    };
                    let right_bound = match *op2 {
                        "<" => "open",
                        "<=" => "closed",
                        ">" => "open",
                        ">=" => "closed",
                        _ => continue,
                    };

                    // For `left < var <= right`: left is open lower, right is closed upper
                    // For `left > var >= right`: left is open upper, right is closed lower (reversed)
                    let is_ascending = *op1 == "<" || *op1 == "<=";
                    if is_ascending {
                        return Some(Ok(format!(
                            "{}:{},{}:{}",
                            left_bound, left, right_bound, right
                        )));
                    } else {
                        return Some(Ok(format!(
                            "{}:{},{}:{}",
                            right_bound, right, left_bound, left
                        )));
                    }
                }
            }
        }
    }

    None
}

/// Try parsing simple inequality like `x > -4` or `-4 < x`
fn try_parse_simple(s: &str) -> Option<Result<String, String>> {
    let ops = ["<=", ">=", "<", ">"];

    for op in &ops {
        if let Some(pos) = s.find(op) {
            let left = s[..pos].trim();
            let right = s[pos + op.len()..].trim();

            let (_var, val, var_on_left) = if is_variable(left) {
                (left, right, true)
            } else if is_variable(right) {
                (right, left, false)
            } else {
                continue;
            };

            // Determine the effective operator relative to variable
            let effective_op = if var_on_left {
                *op
            } else {
                // Flip: `val < var` means `var > val`
                match *op {
                    "<" => ">",
                    ">" => "<",
                    "<=" => ">=",
                    ">=" => "<=",
                    _ => continue,
                }
            };

            // Convert to interval
            let interval = match effective_op {
                ">" => format!("open:{},open:inf", val),
                ">=" => format!("closed:{},open:inf", val),
                "<" => format!("open:-inf,open:{}", val),
                "<=" => format!("open:-inf,closed:{}", val),
                _ => continue,
            };

            return Some(Ok(interval));
        }
    }

    None
}

/// Check if a string looks like a single variable name (alphabetic chars only).
fn is_variable(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic() || c == '_')
}

/// Convert interval key format to user-style notation for display (not used in grading directly).
fn interval_key_to_user_notation(key: &str) -> String {
    // Parse as interval key and render as user notation
    let parts: Vec<&str> = key.splitn(2, ',').collect();
    if parts.len() != 2 {
        return key.to_string();
    }

    let (left_type, left_val) = if let Some(v) = parts[0].strip_prefix("open:") {
        ("(", v)
    } else if let Some(v) = parts[0].strip_prefix("closed:") {
        ("[", v)
    } else {
        return key.to_string();
    };

    let (right_type, right_val) = if let Some(v) = parts[1].strip_prefix("open:") {
        (")", v)
    } else if let Some(v) = parts[1].strip_prefix("closed:") {
        ("]", v)
    } else {
        return key.to_string();
    };

    format!("{}{}, {}{}", left_type, left_val, right_val, right_type)
}
