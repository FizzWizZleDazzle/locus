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

/// Resolve all variables: sample randoms, evaluate derived, execute functions.
/// Resamples up to 1000 times if constraints aren't satisfied.
pub fn resolve(
    variables: &BTreeMap<String, String>,
    constraints: &[String],
) -> Result<VarMap, DslError> {
    let max_attempts = if constraints.is_empty() { 1 } else { 1000 };

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

    let expr = Expr::parse(&substituted)
        .map_err(|e| DslError::ExpressionParse(format!("{}: '{}'", e, substituted)))?;

    Ok(expr.to_string())
}

/// Topological sort of variable dependencies
fn topo_sort(variables: &BTreeMap<String, String>) -> Result<Vec<String>, DslError> {
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

fn check_constraints(vars: &VarMap, constraints: &[String]) -> Result<bool, DslError> {
    for c in constraints {
        if !eval_constraint(c, vars)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn eval_constraint(constraint: &str, vars: &VarMap) -> Result<bool, DslError> {
    let c = constraint.trim();

    // Try comparison operators (longest first to avoid partial matches)
    for (op, _) in &[("!=", 2), (">=", 2), ("<=", 2), ("==", 2), (">", 1), ("<", 1)] {
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
        return Ok(val.parse::<f64>().map_or(false, |f| (f - f.round()).abs() < 1e-10));
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
