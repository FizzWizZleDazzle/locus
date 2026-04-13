//! Built-in math functions — dispatch to SymEngine operations

use locus_common::symengine::Expr;

use crate::error::DslError;
use crate::resolver::VarMap;

const BUILTIN_FUNCTIONS: &[&str] = &[
    "derivative",
    "integral",
    "definite_integral",
    "solve",
    "factor",
    "expand",
    "simplify",
    "evaluate",
    "abs",
    "gcd",
    "lcm",
    "det",
    "inverse",
    "transpose",
    "eigenvalues",
    "rank",
    "dot",
    "cross",
    "magnitude",
    "limit",
    "partial",
    "gradient",
    "round",
    "floor",
    "ceil",
    "max",
    "min",
    "mod",
    "sqrt",
    "sin",
    "cos",
    "tan",
    "asin",
    "acos",
    "atan",
    "log",
    "ln",
    "exp",
];

/// Check if a definition string is a built-in function call
pub fn is_builtin_call(definition: &str) -> bool {
    let def = definition.trim();
    if let Some(paren) = def.find('(') {
        let name = &def[..paren];
        BUILTIN_FUNCTIONS.contains(&name)
    } else {
        false
    }
}

/// Evaluate a built-in function call, substituting variables from VarMap.
/// Returns the result as a string.
pub fn evaluate(definition: &str, vars: &VarMap) -> Result<String, DslError> {
    let def = definition.trim();
    let paren = def.find('(').ok_or_else(|| {
        DslError::UnknownFunction {
            name: def.to_string(),
        }
    })?;

    if !def.ends_with(')') {
        return Err(DslError::ExpressionParse(format!(
            "Missing closing paren: {def}"
        )));
    }

    let name = &def[..paren];
    let args_str = &def[paren + 1..def.len() - 1];

    // Split args on commas, but respect nested parens
    let args = split_args(args_str);

    // Resolve each arg: if it's a variable name, substitute its value
    let resolved_args: Vec<String> = args
        .iter()
        .map(|arg| {
            let a = arg.trim();
            if let Some(val) = vars.get(a) {
                val.clone()
            } else {
                a.to_string()
            }
        })
        .collect();

    match name {
        "derivative" | "partial" => fn_derivative(&resolved_args),
        "expand" => fn_expand(&resolved_args),
        "factor" => fn_factor(&resolved_args),
        "simplify" => fn_simplify(&resolved_args),
        "evaluate" => fn_evaluate(&resolved_args),
        "solve" => fn_solve(&resolved_args),
        "abs" => fn_simple_func("Abs", &resolved_args),
        "sqrt" => fn_simple_func("sqrt", &resolved_args),
        "sin" => fn_simple_func("sin", &resolved_args),
        "cos" => fn_simple_func("cos", &resolved_args),
        "tan" => fn_simple_func("tan", &resolved_args),
        "asin" => fn_simple_func("asin", &resolved_args),
        "acos" => fn_simple_func("acos", &resolved_args),
        "atan" => fn_simple_func("atan", &resolved_args),
        "log" | "ln" => fn_simple_func("log", &resolved_args),
        "exp" => fn_simple_func("exp", &resolved_args),
        "round" => fn_round(&resolved_args),
        "max" => fn_max(&resolved_args),
        "min" => fn_min(&resolved_args),
        _ => Err(DslError::UnknownFunction {
            name: name.to_string(),
        }),
    }
}

fn fn_derivative(args: &[String]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::FunctionArity {
            name: "derivative".into(),
            expected: 2,
            got: args.len(),
        });
    }
    let expr = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    let result = expr
        .diff(&args[1])
        .map_err(|e| DslError::Evaluation(e.to_string()))?;
    Ok(result.to_string())
}

fn fn_expand(args: &[String]) -> Result<String, DslError> {
    if args.len() != 1 {
        return Err(DslError::FunctionArity {
            name: "expand".into(),
            expected: 1,
            got: args.len(),
        });
    }
    let expr = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    Ok(expr.expand().to_string())
}

fn fn_factor(args: &[String]) -> Result<String, DslError> {
    // SymEngine doesn't have a direct factor() — use expand as placeholder
    // TODO: implement proper factoring
    fn_simplify(args)
}

fn fn_simplify(args: &[String]) -> Result<String, DslError> {
    if args.len() != 1 {
        return Err(DslError::FunctionArity {
            name: "simplify".into(),
            expected: 1,
            got: args.len(),
        });
    }
    let expr = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    // SymEngine's expand is the closest to simplify we have
    Ok(expr.expand().to_string())
}

fn fn_evaluate(args: &[String]) -> Result<String, DslError> {
    if args.len() != 3 {
        return Err(DslError::FunctionArity {
            name: "evaluate".into(),
            expected: 3,
            got: args.len(),
        });
    }
    let expr = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    let val: f64 = args[2]
        .parse()
        .map_err(|_| DslError::Evaluation(format!("Can't parse '{}' as float", args[2])))?;
    let result = expr.subs_float(&args[1], val);
    // Try to get a clean numeric result
    if let Some(f) = result.to_float() {
        if (f - f.round()).abs() < 1e-10 {
            Ok(format!("{}", f.round() as i64))
        } else {
            Ok(format!("{}", f))
        }
    } else {
        Ok(result.to_string())
    }
}

fn fn_solve(args: &[String]) -> Result<String, DslError> {
    // SymEngine C API doesn't expose solve() directly
    // TODO: implement via SymEngine's solve or use polynomial root finding
    Err(DslError::UnknownFunction {
        name: "solve (not yet implemented)".into(),
    })
}

fn fn_simple_func(se_name: &str, args: &[String]) -> Result<String, DslError> {
    if args.len() != 1 {
        return Err(DslError::FunctionArity {
            name: se_name.into(),
            expected: 1,
            got: args.len(),
        });
    }
    let expr_str = format!("{}({})", se_name, args[0]);
    let expr = Expr::parse(&expr_str).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    Ok(expr.to_string())
}

fn fn_round(args: &[String]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::FunctionArity {
            name: "round".into(),
            expected: 2,
            got: args.len(),
        });
    }
    let expr = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    let places: u32 = args[1]
        .parse()
        .map_err(|_| DslError::Evaluation(format!("round: can't parse places '{}'", args[1])))?;
    if let Some(f) = expr.to_float() {
        let scale = 10f64.powi(places as i32);
        Ok(format!(
            "{:.prec$}",
            (f * scale).round() / scale,
            prec = places as usize
        ))
    } else {
        Ok(expr.to_string())
    }
}

fn fn_max(args: &[String]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::FunctionArity {
            name: "max".into(),
            expected: 2,
            got: args.len(),
        });
    }
    let a = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    let b = Expr::parse(&args[1]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    match (a.to_float(), b.to_float()) {
        (Some(fa), Some(fb)) => {
            if fa >= fb {
                Ok(args[0].clone())
            } else {
                Ok(args[1].clone())
            }
        }
        _ => Err(DslError::Evaluation("max: can't compare symbolically".into())),
    }
}

fn fn_min(args: &[String]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::FunctionArity {
            name: "min".into(),
            expected: 2,
            got: args.len(),
        });
    }
    let a = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    let b = Expr::parse(&args[1]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    match (a.to_float(), b.to_float()) {
        (Some(fa), Some(fb)) => {
            if fa <= fb {
                Ok(args[0].clone())
            } else {
                Ok(args[1].clone())
            }
        }
        _ => Err(DslError::Evaluation("min: can't compare symbolically".into())),
    }
}

/// Split comma-separated args respecting nested parentheses
fn split_args(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0;

    for c in s.chars() {
        match c {
            '(' | '[' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                args.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.trim().is_empty() {
        args.push(current.trim().to_string());
    }

    args
}
