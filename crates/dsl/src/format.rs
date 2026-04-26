//! Format checking — structural predicates on answer expressions
//!
//! Two layers:
//!   1. Known tags (e.g. "factored") expand to predicate expressions
//!   2. Raw predicate expressions evaluated directly
//!
//! Predicates use existing math functions + tree inspection primitives.
//! Checked AFTER equivalence — both must pass.

use locus_common::symengine::Expr;

use crate::error::DslError;

/// Check if an answer expression satisfies a format requirement.
/// `format_spec` is either a known tag or a raw predicate expression.
/// `answer_str` is the SymEngine-compatible answer string.
pub fn check_format(format_spec: &str, answer_str: &str) -> Result<bool, DslError> {
    let predicate = expand_tag(format_spec.trim());
    eval_predicate(&predicate, answer_str)
}

/// Expand known tags to predicate expressions. Unknown strings pass through.
fn expand_tag(tag: &str) -> String {
    match tag {
        "factored" => "expand(answer) != answer".into(),
        "expanded" => "expand(answer) == answer".into(),
        "simplified" => "simplify(answer) == answer".into(),
        "reduced_fraction" => "gcd(numerator(answer), denominator(answer)) == 1".into(),
        // Pass through — treat as raw predicate
        other => other.to_string(),
    }
}

/// Evaluate a predicate expression against an answer.
/// Supports: comparisons, `and`, `or`, function calls on `answer`.
fn eval_predicate(predicate: &str, answer_str: &str) -> Result<bool, DslError> {
    let p = predicate.trim();

    // Handle `and`
    if let Some(pos) = find_logical_op(p, " and ") {
        let left = &p[..pos];
        let right = &p[pos + 5..];
        return Ok(eval_predicate(left, answer_str)? && eval_predicate(right, answer_str)?);
    }

    // Handle `or`
    if let Some(pos) = find_logical_op(p, " or ") {
        let left = &p[..pos];
        let right = &p[pos + 4..];
        return Ok(eval_predicate(left, answer_str)? || eval_predicate(right, answer_str)?);
    }

    // Handle comparisons: ==, !=
    // Use SymEngine structural equality, not string comparison
    if let Some(pos) = p.find(" != ") {
        let lhs = eval_expr(&p[..pos], answer_str)?;
        let rhs = eval_expr(&p[pos + 4..], answer_str)?;
        let lhs_expr = parse_expr(&lhs)?;
        let rhs_expr = parse_expr(&rhs)?;
        return Ok(!lhs_expr.equals(&rhs_expr));
    }
    if let Some(pos) = p.find(" == ") {
        let lhs = eval_expr(&p[..pos], answer_str)?;
        let rhs = eval_expr(&p[pos + 4..], answer_str)?;
        let lhs_expr = parse_expr(&lhs)?;
        let rhs_expr = parse_expr(&rhs)?;
        return Ok(lhs_expr.equals(&rhs_expr));
    }
    if let Some(pos) = p.find(" >= ") {
        let lhs = eval_expr_float(&p[..pos], answer_str)?;
        let rhs = eval_expr_float(&p[pos + 4..], answer_str)?;
        return Ok(lhs >= rhs);
    }
    if let Some(pos) = p.find(" <= ") {
        let lhs = eval_expr_float(&p[..pos], answer_str)?;
        let rhs = eval_expr_float(&p[pos + 4..], answer_str)?;
        return Ok(lhs <= rhs);
    }
    if let Some(pos) = p.find(" > ") {
        let lhs = eval_expr_float(&p[..pos], answer_str)?;
        let rhs = eval_expr_float(&p[pos + 3..], answer_str)?;
        return Ok(lhs > rhs);
    }
    if let Some(pos) = p.find(" < ") {
        let lhs = eval_expr_float(&p[..pos], answer_str)?;
        let rhs = eval_expr_float(&p[pos + 3..], answer_str)?;
        return Ok(lhs < rhs);
    }

    // No comparison operator — might be a bare tag like "factored"
    let expanded = expand_tag(p);
    if expanded != p {
        // It was a known tag — recurse with expanded form
        return eval_predicate(&expanded, answer_str);
    }

    Err(DslError::Evaluation(format!(
        "Format predicate has no comparison operator: '{p}'"
    )))
}

/// Find a logical operator position, respecting parentheses depth
fn find_logical_op(s: &str, op: &str) -> Option<usize> {
    let mut depth = 0;
    let bytes = s.as_bytes();
    let op_bytes = op.as_bytes();

    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0
            && i + op_bytes.len() <= bytes.len()
            && &bytes[i..i + op_bytes.len()] == op_bytes
        {
            return Some(i);
        }
    }
    None
}

/// Evaluate an expression side of a predicate.
/// Substitutes `answer` with the actual answer, then evaluates via SymEngine.
/// Returns the string representation for comparison.
fn eval_expr(expr: &str, answer_str: &str) -> Result<String, DslError> {
    let e = expr.trim();

    // Direct values
    if e == "answer" {
        return Ok(answer_str.to_string());
    }

    // Numeric literal
    if let Ok(_) = e.parse::<f64>() {
        return Ok(e.to_string());
    }

    // Function calls: func(answer) or func(answer, arg)
    if let Some(paren) = e.find('(') {
        if e.ends_with(')') {
            let func = &e[..paren];
            let args_str = &e[paren + 1..e.len() - 1];

            // Substitute `answer` in args
            let args_resolved = args_str.replace("answer", answer_str);

            return match func {
                "expand" => {
                    let expr = parse_expr(&args_resolved)?;
                    Ok(expr.expand().to_string())
                }
                "simplify" => {
                    // SymEngine expand is closest to simplify
                    let expr = parse_expr(&args_resolved)?;
                    Ok(expr.expand().to_string())
                }
                "factor" => {
                    // Stub: return as-is (SymEngine C API lacks factor)
                    let expr = parse_expr(&args_resolved)?;
                    Ok(expr.to_string())
                }
                "numerator" => extract_fraction_part(&args_resolved, true),
                "denominator" => extract_fraction_part(&args_resolved, false),
                "gcd" => {
                    let parts: Vec<&str> = split_args(&args_resolved);
                    if parts.len() != 2 {
                        return Err(DslError::Evaluation("gcd requires 2 args".into()));
                    }
                    let a = eval_to_int(parts[0], answer_str)?;
                    let b = eval_to_int(parts[1], answer_str)?;
                    Ok(gcd(a.unsigned_abs(), b.unsigned_abs()).to_string())
                }
                "count" => {
                    let parts: Vec<&str> = split_args(&args_resolved);
                    if parts.len() != 2 {
                        return Err(DslError::Evaluation(
                            "count(func, expr) requires 2 args".into(),
                        ));
                    }
                    let func_name = parts[0].trim();
                    let expr_str = parts[1].trim();
                    let count = count_occurrences(expr_str, func_name);
                    Ok(count.to_string())
                }
                "degree" => {
                    let parts: Vec<&str> = split_args(&args_resolved);
                    if parts.len() != 2 {
                        return Err(DslError::Evaluation(
                            "degree(expr, var) requires 2 args".into(),
                        ));
                    }
                    let deg = compute_degree(parts[0].trim(), parts[1].trim())?;
                    Ok(deg.to_string())
                }
                _ => {
                    // Try as SymEngine function
                    let full = format!("{}({})", func, args_resolved);
                    let expr = parse_expr(&full)?;
                    Ok(expr.to_string())
                }
            };
        }
    }

    // Try parsing as SymEngine expression with answer substituted
    let substituted = e.replace("answer", answer_str);
    let expr = parse_expr(&substituted)?;
    Ok(expr.to_string())
}

/// Like eval_expr but returns f64
fn eval_expr_float(expr: &str, answer_str: &str) -> Result<f64, DslError> {
    let s = eval_expr(expr, answer_str)?;
    let parsed = parse_expr(&s)?;
    parsed
        .to_float()
        .ok_or_else(|| DslError::Evaluation(format!("Can't evaluate '{s}' to float")))
}

fn parse_expr(s: &str) -> Result<Expr, DslError> {
    Expr::parse(s.trim()).map_err(|e| DslError::ExpressionParse(format!("{e}: '{s}'")))
}

/// Extract numerator or denominator from a rational expression
fn extract_fraction_part(expr_str: &str, numerator: bool) -> Result<String, DslError> {
    let expr_str = expr_str.trim();

    // Try parsing as "a/b"
    if let Some(slash) = expr_str.find('/') {
        // Make sure it's top-level (not inside parens)
        let mut depth = 0;
        for (i, c) in expr_str.chars().enumerate() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '/' if depth == 0 && i == slash => {
                    return if numerator {
                        Ok(expr_str[..i].trim().to_string())
                    } else {
                        Ok(expr_str[i + 1..].trim().to_string())
                    };
                }
                _ => {}
            }
        }
    }

    // SymEngine string: check for Rational representation
    let expr = parse_expr(expr_str)?;
    let s = expr.to_string();

    if let Some(slash) = s.find('/') {
        if numerator {
            Ok(s[..slash].trim().to_string())
        } else {
            Ok(s[slash + 1..].trim().to_string())
        }
    } else {
        // Integer — numerator is itself, denominator is 1
        if numerator {
            Ok(s)
        } else {
            Ok("1".to_string())
        }
    }
}

/// Evaluate expression to integer
fn eval_to_int(expr: &str, answer_str: &str) -> Result<i64, DslError> {
    let s = eval_expr(expr, answer_str)?;
    s.parse::<i64>().or_else(|_| {
        // Try float → int
        s.parse::<f64>()
            .map(|f| f.round() as i64)
            .map_err(|_| DslError::Evaluation(format!("Can't evaluate '{s}' to integer")))
    })
}

/// Count occurrences of a function name in an expression string
fn count_occurrences(expr_str: &str, func_name: &str) -> usize {
    let pattern = format!("{}(", func_name);
    expr_str.matches(&pattern).count()
}

/// Compute polynomial degree by repeated differentiation
fn compute_degree(expr_str: &str, var: &str) -> Result<i32, DslError> {
    let expr = parse_expr(expr_str)?;
    let mut current = expr;
    let mut degree = 0;

    for _ in 0..100 {
        // Check if expression is zero (constant w.r.t. var)
        let val1 = current.subs_float(var, 1.0).to_float().unwrap_or(f64::NAN);
        let val2 = current.subs_float(var, 2.0).to_float().unwrap_or(f64::NAN);
        let val3 = current.subs_float(var, 3.0).to_float().unwrap_or(f64::NAN);

        // If all equal, no more var dependence
        if val1.is_finite()
            && val2.is_finite()
            && val3.is_finite()
            && (val1 - val2).abs() < 1e-10
            && (val2 - val3).abs() < 1e-10
        {
            break;
        }

        current = current
            .diff(var)
            .map_err(|e| DslError::Evaluation(e.to_string()))?;
        degree += 1;
    }

    Ok(degree)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn split_args(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in s.chars().enumerate() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                result.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = s[start..].trim();
    if !last.is_empty() {
        result.push(last);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factored_tag() {
        // 3*(x+2) is factored — expand would give 3*x + 6 (different)
        assert!(check_format("factored", "3*(x+2)").unwrap());
    }

    #[test]
    fn test_not_factored() {
        // 3*x + 6 is already expanded — expand gives same thing
        assert!(!check_format("factored", "3*x + 6").unwrap());
    }

    #[test]
    fn test_expanded_tag() {
        assert!(check_format("expanded", "3*x + 6").unwrap());
    }

    #[test]
    fn test_not_expanded() {
        assert!(!check_format("expanded", "3*(x+2)").unwrap());
    }

    #[test]
    fn test_raw_predicate() {
        // degree(answer, x) == 2
        assert!(check_format("degree(answer, x) == 2", "x**2 + 3*x + 1").unwrap());
    }

    #[test]
    fn test_raw_predicate_fails() {
        assert!(!check_format("degree(answer, x) == 2", "x**3 + 1").unwrap());
    }

    #[test]
    fn test_count_log() {
        assert!(check_format("count(log, answer) == 1", "log(x**2)").unwrap());
    }

    #[test]
    fn test_reduced_fraction() {
        assert!(check_format("reduced_fraction", "3/7").unwrap());
    }

    #[test]
    fn test_not_reduced_fraction() {
        assert!(!check_format("reduced_fraction", "6/14").unwrap());
    }

    #[test]
    fn test_compound_and() {
        assert!(check_format("expanded and degree(answer, x) == 2", "x**2 + 3*x + 1").unwrap());
    }

    #[test]
    fn test_compound_or() {
        // Either factored OR degree <= 1
        assert!(
            check_format(
                "factored or degree(answer, x) <= 1",
                "3*x + 6" // not factored, but degree 1
            )
            .unwrap()
        );
    }
}
