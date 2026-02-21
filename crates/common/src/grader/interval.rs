//! Interval grader — compare interval bounds and types.
//!
//! DB answer key format: `open:1,closed:7` means (1, 7].
//! Unions: `open:-inf,closed:0|open:2,open:inf` means (-inf, 0] U (2, inf).
//!
//! User input format: `(1, 7]`, `[-2, 4)`, `(-inf, 3]`.
//! Unions with `U` or `union`.

use super::{ExprEngine, GradeResult, are_equivalent};

#[derive(Debug, Clone, PartialEq)]
enum BoundType {
    Open,
    Closed,
}

#[derive(Debug, Clone)]
struct Bound {
    kind: BoundType,
    value: String, // SymEngine expression string, or "inf"/"-inf"
}

#[derive(Debug, Clone)]
struct Interval {
    left: Bound,
    right: Bound,
}

/// Parse DB answer key format: `open:1,closed:7`
fn parse_key_interval(s: &str) -> Result<Interval, String> {
    let parts: Vec<&str> = s.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err(format!("Expected two bounds separated by comma: {}", s));
    }

    let left = parse_key_bound(parts[0].trim())?;
    let right = parse_key_bound(parts[1].trim())?;
    Ok(Interval { left, right })
}

fn parse_key_bound(s: &str) -> Result<Bound, String> {
    if let Some(val) = s.strip_prefix("open:") {
        Ok(Bound {
            kind: BoundType::Open,
            value: val.trim().to_string(),
        })
    } else if let Some(val) = s.strip_prefix("closed:") {
        Ok(Bound {
            kind: BoundType::Closed,
            value: val.trim().to_string(),
        })
    } else {
        Err(format!(
            "Expected 'open:value' or 'closed:value', got: {}",
            s
        ))
    }
}

/// Parse user input format: `(1, 7]`, `[-2, 4)`, `(-inf, 3]`
fn parse_user_interval(s: &str) -> Result<Interval, String> {
    let s = s.trim();
    if s.len() < 3 {
        return Err("Interval too short".into());
    }

    let first_char = s.chars().next().unwrap();
    let last_char = s.chars().last().unwrap();

    let left_type = match first_char {
        '(' => BoundType::Open,
        '[' => BoundType::Closed,
        _ => {
            return Err(format!(
                "Interval must start with '(' or '[', got '{}'",
                first_char
            ));
        }
    };

    let right_type = match last_char {
        ')' => BoundType::Open,
        ']' => BoundType::Closed,
        _ => {
            return Err(format!(
                "Interval must end with ')' or ']', got '{}'",
                last_char
            ));
        }
    };

    // Strip delimiters
    let inner = &s[1..s.len() - 1];

    // Split on comma — but we need to be careful with negative numbers and expressions.
    // Use a simple approach: find the first comma at depth 0
    let mut depth = 0i32;
    let mut comma_pos = None;
    for (i, ch) in inner.chars().enumerate() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                comma_pos = Some(i);
                break;
            }
            _ => {}
        }
    }

    let comma_pos = comma_pos.ok_or_else(|| "No comma found in interval".to_string())?;

    let left_val = inner[..comma_pos].trim().to_string();
    let right_val = inner[comma_pos + 1..].trim().to_string();

    Ok(Interval {
        left: Bound {
            kind: left_type,
            value: normalize_inf(&left_val),
        },
        right: Bound {
            kind: right_type,
            value: normalize_inf(&right_val),
        },
    })
}

fn normalize_inf(s: &str) -> String {
    let s = s.trim();
    match s {
        "inf" | "infinity" | "oo" | "+inf" | "+infinity" | "+oo" => "inf".to_string(),
        "-inf" | "-infinity" | "-oo" => "-inf".to_string(),
        other => other.to_string(),
    }
}

fn is_inf(s: &str) -> bool {
    matches!(s, "inf" | "-inf")
}

fn bounds_equivalent<E: ExprEngine>(a: &Bound, b: &Bound) -> bool {
    if a.kind != b.kind {
        return false;
    }

    let a_val = normalize_inf(&a.value);
    let b_val = normalize_inf(&b.value);

    // Handle infinity
    if is_inf(&a_val) || is_inf(&b_val) {
        return a_val == b_val;
    }

    // Parse as expressions and check equivalence
    let a_expr = match E::parse(&a_val) {
        Ok(e) => e,
        Err(_) => return false,
    };
    let b_expr = match E::parse(&b_val) {
        Ok(e) => e,
        Err(_) => return false,
    };

    are_equivalent(&a_expr, &b_expr)
}

fn intervals_equivalent<E: ExprEngine>(a: &Interval, b: &Interval) -> bool {
    bounds_equivalent::<E>(&a.left, &b.left) && bounds_equivalent::<E>(&a.right, &b.right)
}

pub fn grade<E: ExprEngine>(user_input: &str, answer_key: &str) -> GradeResult {
    // Split on | for answer key unions
    let key_parts: Vec<&str> = answer_key.split('|').collect();

    // Split on U/union for user input unions
    let user_str = user_input.trim();
    let user_parts: Vec<&str> = if user_str.contains(" U ") || user_str.contains(" union ") {
        // Split on " U " or " union "
        let re_split: Vec<&str> = user_str
            .split(" U ")
            .flat_map(|s| s.split(" union "))
            .collect();
        re_split
    } else {
        vec![user_str]
    };

    if key_parts.len() != user_parts.len() {
        return GradeResult::Incorrect;
    }

    // Parse key intervals
    let key_intervals: Vec<Interval> = match key_parts
        .iter()
        .map(|s| parse_key_interval(s.trim()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(v) => v,
        Err(e) => return GradeResult::Error(format!("Invalid interval answer key: {}", e)),
    };

    // Parse user intervals
    let user_intervals: Vec<Interval> = match user_parts
        .iter()
        .map(|s| parse_user_interval(s.trim()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(v) => v,
        Err(_) => return GradeResult::Invalid("Could not parse interval notation".into()),
    };

    // For single intervals, compare directly
    if key_intervals.len() == 1 {
        if intervals_equivalent::<E>(&user_intervals[0], &key_intervals[0]) {
            return GradeResult::Correct;
        }
        return GradeResult::Incorrect;
    }

    // For unions, match each component (sorted comparison would be ideal,
    // but for simplicity do unordered bipartite matching)
    let mut matched = vec![false; user_intervals.len()];
    for ki in &key_intervals {
        let mut found = false;
        for (j, ui) in user_intervals.iter().enumerate() {
            if !matched[j] && intervals_equivalent::<E>(ki, ui) {
                matched[j] = true;
                found = true;
                break;
            }
        }
        if !found {
            return GradeResult::Incorrect;
        }
    }

    GradeResult::Correct
}
