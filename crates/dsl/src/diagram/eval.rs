//! Numeric evaluation for diagram specs.
//!
//! Diagram fields like `radius: r`, `at: 3`, `incline_angle: theta` may be
//! literals, variable references, or full expressions. `eval_num` resolves
//! any of those to an `f64` against the resolved variant `VarMap`.

use locus_common::symengine::Expr;

use crate::error::DslError;
use crate::resolver::VarMap;

/// Evaluate an expression string against `vars`, returning a numeric value.
pub fn eval_num(expr: &str, vars: &VarMap) -> Result<f64, DslError> {
    let trimmed = expr.trim();
    if let Ok(n) = trimmed.parse::<f64>() {
        return Ok(n);
    }
    if let Some(v) = vars.get(trimmed) {
        if let Ok(n) = v.parse::<f64>() {
            return Ok(n);
        }
    }
    let mut e = Expr::parse(trimmed)
        .map_err(|err| DslError::ExpressionParse(format!("{err}: '{trimmed}'")))?;
    for (name, value) in vars {
        if let Ok(v) = value.parse::<f64>() {
            e = e.subs_float(name, v);
        }
    }
    e.to_float().ok_or_else(|| {
        DslError::Evaluation(format!("diagram value not numeric: '{trimmed}'"))
    })
}

/// Evaluate optional value, returning `Ok(None)` when absent.
pub fn eval_num_opt(expr: Option<&str>, vars: &VarMap) -> Result<Option<f64>, DslError> {
    expr.map(|e| eval_num(e, vars)).transpose()
}
