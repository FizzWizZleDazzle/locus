//! Answer handling — type inference + formatting

use crate::error::DslError;
use crate::resolver::VarMap;

/// Format the answer variable into a grader-compatible answer_key string.
/// `answer_ref` is the variable name (e.g. "answer") or a comma-separated
/// list of names for tuple answers (e.g. "sol_x, sol_y").
pub fn format(
    vars: &VarMap,
    answer_ref: &str,
    _answer_type: Option<&str>,
) -> Result<String, DslError> {
    let ref_trimmed = answer_ref.trim();

    // Comma-separated refs → tuple: "sol_x, sol_y" → "3, -5"
    if ref_trimmed.contains(',') {
        let parts: Result<Vec<String>, _> = ref_trimmed
            .split(',')
            .map(|part| {
                let name = part.trim();
                vars.get(name)
                    .cloned()
                    .ok_or_else(|| DslError::UndefinedVariable {
                        name: name.to_string(),
                    })
            })
            .collect();
        return Ok(parts?.iter().map(|p| normalize_for_grader(p)).collect::<Vec<_>>().join(", "));
    }

    // Single variable ref — normalize SymEngine output for grader
    vars.get(ref_trimmed)
        .map(|v| normalize_for_grader(v))
        .ok_or_else(|| DslError::UndefinedVariable {
            name: ref_trimmed.to_string(),
        })
}

/// Normalize SymEngine output for the grader.
/// SymEngine uses `**` for power, grader expects `^`.
fn normalize_for_grader(value: &str) -> String {
    value.replace("**", "^")
}

/// Infer answer_type from the answer value
pub fn infer_type(answer_key: &str, explicit: Option<&str>) -> String {
    if let Some(t) = explicit {
        return t.to_string();
    }

    let key = answer_key.trim();

    // Boolean
    if matches!(
        key.to_lowercase().as_str(),
        "true" | "false" | "yes" | "no"
    ) {
        return "boolean".into();
    }

    // Numeric: pure number (integer or decimal)
    if key.parse::<f64>().is_ok() {
        return "numeric".into();
    }

    // Set: contains { }
    if key.starts_with('{') && key.ends_with('}') {
        return "set".into();
    }

    // Matrix: [[ ]]
    if key.starts_with("[[") {
        return "matrix".into();
    }

    // Interval: contains "open:" or "closed:"
    if key.contains("open:") || key.contains("closed:") {
        return "interval".into();
    }

    // Tuple: comma-separated values (no brackets)
    if key.contains(',') && !key.contains('[') {
        return "tuple".into();
    }

    // List: [ ]
    if key.starts_with('[') && key.ends_with(']') {
        return "list".into();
    }

    // Equation: contains =
    if key.contains('=') && !key.contains("==") {
        return "equation".into();
    }

    // Default: expression
    "expression".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_format_single() {
        let mut vars = BTreeMap::new();
        vars.insert("answer".into(), "42".into());
        assert_eq!(format(&vars, "answer", None).unwrap(), "42");
    }

    #[test]
    fn test_format_tuple() {
        let mut vars = BTreeMap::new();
        vars.insert("x".into(), "3".into());
        vars.insert("y".into(), "-5".into());
        assert_eq!(format(&vars, "x, y", None).unwrap(), "3, -5");
    }

    #[test]
    fn test_infer_numeric() {
        assert_eq!(infer_type("42", None), "numeric");
        assert_eq!(infer_type("3.14", None), "numeric");
    }

    #[test]
    fn test_infer_expression() {
        assert_eq!(infer_type("3*x + 5", None), "expression");
    }

    #[test]
    fn test_infer_boolean() {
        assert_eq!(infer_type("true", None), "boolean");
        assert_eq!(infer_type("Yes", None), "boolean");
    }

    #[test]
    fn test_infer_tuple() {
        assert_eq!(infer_type("3, -5", None), "tuple");
    }

    #[test]
    fn test_explicit_override() {
        assert_eq!(infer_type("42", Some("expression")), "expression");
    }
}
