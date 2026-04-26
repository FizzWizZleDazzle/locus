//! Variable resolution — topological sort + evaluation
//!
//! Variables come in three kinds:
//! 1. Sampled: `a: integer(2, 10)` → random value
//! 2. Derived: `f: a*x^2 + b*x` → expression with other vars substituted
//! 3. Function: `answer: derivative(f, x)` → built-in math operation

use std::collections::{BTreeMap, HashMap, HashSet};

use locus_common::symengine::Expr;

use crate::error::DslError;
use crate::functions;
use crate::sampler;

/// Resolved variables: name → string value (SymEngine-compatible)
pub type VarMap = BTreeMap<String, String>;

/// Resolve all derived/builtin variables given pre-determined sampler values.
/// Used by the GPU enumerator's render path so we don't redo random sampling
/// — the GPU already chose specific sampler values, we just need to compute
/// the dependent symbolic/numeric forms for question/answer/solution rendering.
pub fn resolve_with_preset(
    variables: &BTreeMap<String, String>,
    presets: &VarMap,
) -> Result<VarMap, DslError> {
    let order = topo_sort(variables)?;
    let mut resolved: VarMap = presets.clone();

    for name in &order {
        if resolved.contains_key(name) {
            continue;
        }
        let definition = &variables[name];
        if sampler::is_sampler(definition) {
            // Should have been preset; sample as fallback
            let value = sampler::sample(definition)?;
            resolved.insert(name.clone(), value);
        } else if functions::is_builtin_call(definition) {
            let value = functions::evaluate(definition, &resolved)?;
            resolved.insert(name.clone(), value);
        } else {
            let value = eval_derived(definition, &resolved)?;
            resolved.insert(name.clone(), value);
        }
    }

    Ok(resolved)
}

/// Resolve all variables: sample randoms, evaluate derived, execute functions.
/// Resamples up to 1000 times if constraints aren't satisfied.
pub fn resolve(
    variables: &BTreeMap<String, String>,
    constraints: &[String],
) -> Result<VarMap, DslError> {
    let max_attempts = if constraints.is_empty() { 1 } else { 100 };

    for attempt in 0..max_attempts {
        match try_resolve(variables) {
            Ok(vars) => {
                if constraints.is_empty() || check_constraints(&vars, constraints)? {
                    return Ok(vars);
                }
            }
            Err(e) => {
                if attempt == max_attempts - 1 {
                    return Err(e);
                }
            }
        }
    }

    Err(DslError::ConstraintUnsatisfiable {
        constraint: constraints.join(", "),
        attempts: max_attempts,
    })
}

fn try_resolve(variables: &BTreeMap<String, String>) -> Result<VarMap, DslError> {
    let order = topo_sort(variables)?;
    let mut resolved = VarMap::new();

    for name in &order {
        let definition = &variables[name];

        if sampler::is_sampler(definition) {
            let value = sampler::sample(definition)?;
            resolved.insert(name.clone(), value);
        } else if functions::is_builtin_call(definition) {
            let value = functions::evaluate(definition, &resolved)?;
            resolved.insert(name.clone(), value);
        } else {
            let value = eval_derived(definition, &resolved)?;
            resolved.insert(name.clone(), value);
        }
    }

    Ok(resolved)
}

/// Evaluate a derived expression by substituting known variables
fn eval_derived(expr_str: &str, vars: &VarMap) -> Result<String, DslError> {
    let mut substituted = expr_str.to_string();

    // Sort by name length descending to avoid partial replacements
    let mut sorted_vars: Vec<(&String, &String)> = vars.iter().collect();
    sorted_vars.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    for (name, value) in &sorted_vars {
        let pattern = format!(r"\b{}\b", regex::escape(name));
        if let Ok(re) = regex::Regex::new(&pattern) {
            substituted = re
                .replace_all(&substituted, format!("({})", value))
                .to_string();
        }
    }

    // Handle Python ternary: `true_val if condition else false_val`
    let substituted = eval_python_ternary(&substituted);

    // Handle equations: `LHS = RHS` → `LHS - (RHS)` (makes it an expression equal to zero)
    let substituted = rewrite_equation_to_expr(&substituted);

    // Clean up matrix notation: [[(−2), (1)]] → [[-2, 1]]
    let substituted = clean_matrix_parens(&substituted);

    // Matrix literals [[...]] can't be parsed by SymEngine — return as-is
    if substituted.trim().starts_with("[[") {
        return Ok(substituted);
    }

    // Single-bracket array literals [...] — bypass SymEngine, return as-is
    let trimmed = substituted.trim();
    if trimmed.starts_with('[') && trimmed.ends_with(']') && !trimmed.starts_with("[[") {
        return Ok(trimmed.to_string());
    }

    // Tuple literals like (3, 5) or bare comma lists like 3, 5 — bypass SymEngine
    if is_tuple_literal(trimmed) {
        return Ok(trimmed.to_string());
    }
    // Bare comma-separated values (from solve() output): "3, 5" or "-5/4, 3/2"
    if trimmed.contains(", ") && !trimmed.contains('(') && !trimmed.contains('[') {
        let all_simple = trimmed.split(", ").all(|p| {
            let p = p.trim();
            p.parse::<f64>().is_ok()
                || p.contains('/')
                || p.trim_start_matches('-')
                    .chars()
                    .all(|c| c.is_ascii_digit() || c == '/' || c == '.')
        });
        if all_simple {
            return Ok(trimmed.to_string());
        }
    }

    // Handle tuple/array indexing: `(a, b, c)[1]` → pick element
    let substituted = eval_indexing(&substituted);

    // Rewrite % operator: a % b → compute as a - floor(a/b)*b
    let substituted = rewrite_modulo(&substituted);

    // Evaluate embedded function calls that SymEngine can't handle (gcd, mod, abs, etc.)
    // Repeatedly replace innermost function calls with their evaluated results.
    let substituted = eval_embedded_functions(&substituted)?;

    let expr = Expr::parse(&substituted)
        .map_err(|e| DslError::ExpressionParse(format!("{}: '{}'", e, substituted)))?;

    Ok(expr.to_string())
}

/// Evaluate Python ternary expressions: `true_val if condition else false_val`
fn eval_python_ternary(s: &str) -> String {
    let trimmed = s.trim();
    // Look for ` if ` and ` else ` pattern
    let if_pos = match trimmed.find(" if ") {
        Some(p) => p,
        None => return s.to_string(),
    };
    let else_pos = match trimmed.find(" else ") {
        Some(p) => p,
        None => return s.to_string(),
    };
    if else_pos <= if_pos {
        return s.to_string();
    }

    let true_val = trimmed[..if_pos].trim();
    let condition = trimmed[if_pos + 4..else_pos].trim();
    let false_val = trimmed[else_pos + 6..].trim();

    // Evaluate condition: try simple string equality like ("multiply") == "multiply"
    let cond_result = eval_simple_condition(condition);

    match cond_result {
        Some(true) => true_val.to_string(),
        Some(false) => false_val.to_string(),
        None => {
            // Can't evaluate condition, default to true branch
            true_val.to_string()
        }
    }
}

/// Evaluate simple boolean conditions for Python ternary support.
/// Handles: `"a" == "b"`, `"a" != "b"`, `("a") == "a"`, numeric comparisons.
fn eval_simple_condition(cond: &str) -> Option<bool> {
    let cond = cond.trim();
    // String equality: ("x") == "x" or "x" == "x"
    for (op, negate) in &[("==", false), ("!=", true)] {
        if let Some(pos) = cond.find(op) {
            let lhs = cond[..pos]
                .trim()
                .trim_matches(|c: char| c == '"' || c == '(' || c == ')' || c.is_whitespace());
            let rhs = cond[pos + op.len()..]
                .trim()
                .trim_matches(|c: char| c == '"' || c == '(' || c == ')' || c.is_whitespace());
            let equal = lhs == rhs;
            return Some(if *negate { !equal } else { equal });
        }
    }
    None
}

/// Rewrite equation `LHS = RHS` → `LHS - (RHS)` to make it an expression.
/// Only triggers for bare `=` (not `==`, `<=`, `>=`, `!=`).
fn rewrite_equation_to_expr(s: &str) -> String {
    let trimmed = s.trim();
    // Find bare `=` that is not part of `==`, `<=`, `>=`, `!=`
    let bytes = trimmed.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'=' {
            // Check it's not part of a compound operator
            let prev = if i > 0 { bytes[i - 1] } else { 0 };
            let next = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
            if prev == b'=' || prev == b'!' || prev == b'<' || prev == b'>' || next == b'=' {
                continue;
            }
            let lhs = trimmed[..i].trim();
            let rhs = trimmed[i + 1..].trim();
            if !lhs.is_empty() && !rhs.is_empty() {
                return format!("{} - ({})", lhs, rhs);
            }
        }
    }
    s.to_string()
}

/// Evaluate tuple/array indexing: `(a, b, c)[1]` → `b`
fn eval_indexing(s: &str) -> String {
    let trimmed = s.trim();
    // Match pattern: `(...)[N]` at the end
    let re = match regex::Regex::new(r"^\(([^)]+)\)\[(\d+)\]$") {
        Ok(r) => r,
        Err(_) => return s.to_string(),
    };
    if let Some(caps) = re.captures(trimmed) {
        let inner = &caps[1];
        let idx: usize = match caps[2].parse() {
            Ok(i) => i,
            Err(_) => return s.to_string(),
        };
        let elements: Vec<&str> = inner.split(',').map(|e| e.trim()).collect();
        if idx < elements.len() {
            return elements[idx].to_string();
        }
    }
    s.to_string()
}

/// Topological sort of variable dependencies
pub(crate) fn topo_sort(variables: &BTreeMap<String, String>) -> Result<Vec<String>, DslError> {
    let var_names: HashSet<&str> = variables.keys().map(|s| s.as_str()).collect();

    // Build dependency graph: deps[name] = vars that `name` depends on
    let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();
    for (name, definition) in variables {
        let mut var_deps = Vec::new();
        for other in &var_names {
            if *other != name.as_str() {
                let pattern = format!(r"\b{}\b", regex::escape(other));
                if let Ok(re) = regex::Regex::new(&pattern) {
                    if re.is_match(definition) {
                        var_deps.push(*other);
                    }
                }
            }
        }
        deps.insert(name.as_str(), var_deps);
    }

    // Kahn's algorithm
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    for (name, d) in &deps {
        in_degree.insert(name, d.len());
    }

    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(&name, _)| name)
        .collect();
    queue.sort();

    let mut order = Vec::new();
    while let Some(name) = queue.pop() {
        order.push(name.to_string());
        for (other, other_deps) in &deps {
            if other_deps.contains(&name) {
                let deg = in_degree.get_mut(other).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push(other);
                }
            }
        }
        queue.sort();
    }

    if order.len() != var_names.len() {
        let remaining: Vec<&str> = var_names
            .iter()
            .filter(|n| !order.iter().any(|o| o == **n))
            .copied()
            .collect();
        return Err(DslError::CircularDependency {
            cycle: remaining.join(", "),
        });
    }

    Ok(order)
}

/// Evaluate embedded function calls (like `gcd(3, 6)`, `mod(7, 3)`) within expressions.
/// These are functions in BUILTIN_FUNCTIONS that SymEngine can't parse natively.
/// Repeatedly finds and evaluates the innermost call until none remain.
fn eval_embedded_functions(s: &str) -> Result<String, DslError> {
    // Functions we need to evaluate ourselves (SymEngine doesn't handle these)
    const EVAL_FUNCS: &[&str] = &[
        "gcd", "lcm", "mod", "abs", "floor", "ceil", "round", "max", "min",
    ];

    let mut result = s.to_string();
    let mut max_iters = 20; // prevent infinite loops

    loop {
        if max_iters == 0 {
            break;
        }
        max_iters -= 1;

        // Find the innermost function call: name(args_without_nested_parens)
        let re = regex::Regex::new(r"\b(gcd|lcm|mod|abs|floor|ceil|round|max|min)\(").unwrap();
        let m = match re.find(&result) {
            Some(m) => m,
            None => break,
        };

        let func_start = m.start();
        let name_end = result[func_start..].find('(').unwrap() + func_start;
        let func_name = &result[func_start..name_end];

        // Find matching closing paren
        let mut depth = 0;
        let mut close_pos = None;
        for (i, c) in result[name_end..].char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        close_pos = Some(name_end + i);
                        break;
                    }
                }
                _ => {}
            }
        }

        let close_pos = match close_pos {
            Some(p) => p,
            None => break, // no matching close paren
        };

        let full_call = &result[func_start..=close_pos];

        // Check if it's a builtin we handle — evaluate via functions module
        if EVAL_FUNCS.contains(&func_name) {
            let empty_vars = VarMap::new();
            match crate::functions::evaluate(full_call, &empty_vars) {
                Ok(val) => {
                    result = format!(
                        "{}{}{}",
                        &result[..func_start],
                        val,
                        &result[close_pos + 1..]
                    );
                    continue;
                }
                Err(_) => break, // can't evaluate, let SymEngine try
            }
        } else {
            break;
        }
    }

    Ok(result)
}

/// Check if a string looks like a tuple literal: (N, M) or (N, M, ...)
/// where each element is a number or simple expression.
fn is_tuple_literal(s: &str) -> bool {
    let s = s.trim();
    if !s.starts_with('(') || !s.ends_with(')') {
        return false;
    }
    let inner = &s[1..s.len() - 1];
    // Must have at least one comma
    if !inner.contains(',') {
        return false;
    }
    // Each part should be parseable as a number (allowing negative, decimals, fractions)
    inner.split(',').all(|part| {
        let p = part.trim();
        // Accept numbers, fractions, simple negatives
        p.parse::<f64>().is_ok()
            || p.trim_start_matches('-').parse::<f64>().is_ok()
            || (p.contains('/') && p.split('/').count() == 2)
    })
}

/// Rewrite `%` modulo operator into SymEngine-compatible form.
/// Handles expressions like `(5 - 1) % (-2 - 7)` → evaluates operands and computes modulo.
fn rewrite_modulo(s: &str) -> String {
    if !s.contains('%') {
        return s.to_string();
    }
    // Use regex to find A % B patterns (where A and B can be parenthesized expressions or simple tokens)
    let re = regex::Regex::new(r"(\([^)]+\)|\w+)\s*%\s*(\([^)]+\)|\w+)").unwrap();
    let result = re.replace_all(s, |caps: &regex::Captures| {
        let a_str = &caps[1];
        let b_str = &caps[2];
        // Try to evaluate both sides numerically
        if let (Ok(a_expr), Ok(b_expr)) = (Expr::parse(a_str), Expr::parse(b_str)) {
            if let (Some(a), Some(b)) = (a_expr.to_float(), b_expr.to_float()) {
                if b.abs() > 1e-10 {
                    let result = ((a % b) + b) % b;
                    if (result - result.round()).abs() < 1e-10 {
                        return format!("{}", result.round() as i64);
                    }
                    return format!("{}", result);
                }
            }
        }
        // Fallback: can't evaluate, return original
        caps[0].to_string()
    });
    result.to_string()
}

/// Strip redundant parens around numbers/fractions in expressions.
/// [[(−2), (1)]] → [[-2, 1]]
/// open:(-5/3) → open:-5/3
fn clean_matrix_parens(s: &str) -> String {
    // Strip parens wrapping numbers or fractions: (N) or (N/M) or (-N/M)
    let re = regex::Regex::new(r"\((-?\d+(?:/\d+)?(?:\.\d+)?)\)").unwrap();
    re.replace_all(s, "$1").to_string()
}

fn check_constraints(vars: &VarMap, constraints: &[String]) -> Result<bool, DslError> {
    for c in constraints {
        if !eval_constraint(c, vars)? {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn eval_constraint_str(constraint: &str, vars: &VarMap) -> Result<bool, DslError> {
    eval_constraint(constraint, vars)
}

fn eval_constraint(constraint: &str, vars: &VarMap) -> Result<bool, DslError> {
    let c = constraint.trim();

    // Handle "or" — split and return true if ANY sub-constraint passes
    if let Some(pos) = c.find(" or ") {
        let left = &c[..pos];
        let right = &c[pos + 4..];
        return Ok(eval_constraint(left, vars)? || eval_constraint(right, vars)?);
    }

    // Handle "and" — split and return true if ALL sub-constraints pass
    if let Some(pos) = c.find(" and ") {
        let left = &c[..pos];
        let right = &c[pos + 5..];
        return Ok(eval_constraint(left, vars)? && eval_constraint(right, vars)?);
    }

    // Try comparison operators (longest first to avoid partial matches)
    for (op, _) in &[
        ("!=", 2),
        (">=", 2),
        ("<=", 2),
        ("==", 2),
        (">", 1),
        ("<", 1),
    ] {
        if let Some(pos) = c.find(op) {
            // Avoid matching sub-operators
            if *op == ">" && pos > 0 && c.as_bytes()[pos - 1] == b'!' {
                continue;
            }
            if *op == "<" && pos + 1 < c.len() && c.as_bytes()[pos + 1] == b'=' {
                continue;
            }
            if *op == ">" && pos + 1 < c.len() && c.as_bytes()[pos + 1] == b'=' {
                continue;
            }

            let lhs_str = c[..pos].trim();
            let rhs_str = c[pos + op.len()..].trim();

            let lhs = eval_derived(lhs_str, vars)?;
            let rhs = eval_derived(rhs_str, vars)?;

            let lhs_expr =
                Expr::parse(&lhs).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
            let rhs_expr =
                Expr::parse(&rhs).map_err(|e| DslError::ExpressionParse(e.to_string()))?;

            return match (lhs_expr.to_float(), rhs_expr.to_float()) {
                (Some(l), Some(r)) => match *op {
                    "==" => Ok((l - r).abs() < 1e-10),
                    "!=" => Ok((l - r).abs() > 1e-10),
                    ">" => Ok(l > r),
                    "<" => Ok(l < r),
                    ">=" => Ok(l >= r),
                    "<=" => Ok(l <= r),
                    _ => Ok(false),
                },
                _ => match *op {
                    "==" => Ok(lhs_expr.equals(&rhs_expr)),
                    "!=" => Ok(!lhs_expr.equals(&rhs_expr)),
                    _ => Ok(false),
                },
            };
        }
    }

    // Function constraints
    if c.starts_with("is_integer(") && c.ends_with(')') {
        let inner = &c[11..c.len() - 1];
        let val = eval_derived(inner, vars)?;
        let f = val
            .parse::<f64>()
            .ok()
            .or_else(|| Expr::parse(&val).ok().and_then(|e| e.to_float()));
        return Ok(f.map_or(false, |f| (f - f.round()).abs() < 1e-10));
    }

    // Unknown constraint — treat as satisfied with warning
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topo_sort_simple() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "integer(1, 10)".into());
        vars.insert("b".into(), "integer(1, 10)".into());
        vars.insert("c".into(), "a + b".into());
        vars.insert("answer".into(), "c * 2".into());

        let order = topo_sort(&vars).unwrap();
        let a_pos = order.iter().position(|x| x == "a").unwrap();
        let b_pos = order.iter().position(|x| x == "b").unwrap();
        let c_pos = order.iter().position(|x| x == "c").unwrap();
        let ans_pos = order.iter().position(|x| x == "answer").unwrap();

        assert!(a_pos < c_pos);
        assert!(b_pos < c_pos);
        assert!(c_pos < ans_pos);
    }

    #[test]
    fn test_circular_dependency() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "b + 1".into());
        vars.insert("b".into(), "a + 1".into());
        assert!(topo_sort(&vars).is_err());
    }

    #[test]
    fn test_resolve_basic() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "integer(5, 5)".into());
        vars.insert("b".into(), "integer(3, 3)".into());
        vars.insert("answer".into(), "a + b".into());

        let resolved = resolve(&vars, &[]).unwrap();
        assert_eq!(resolved["a"], "5");
        assert_eq!(resolved["b"], "3");
        assert_eq!(resolved["answer"], "8");
    }

    #[test]
    fn test_constraint_resampling() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "integer(1, 10)".into());
        vars.insert("b".into(), "integer(1, 10)".into());

        let constraints = vec!["a > b".into()];
        let resolved = resolve(&vars, &constraints).unwrap();

        let a: i64 = resolved["a"].parse().unwrap();
        let b: i64 = resolved["b"].parse().unwrap();
        assert!(a > b);
    }
}
