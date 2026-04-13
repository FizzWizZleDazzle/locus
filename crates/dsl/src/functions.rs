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

    // Resolve each arg: substitute variables, then evaluate expressions
    let resolved_args: Vec<String> = args
        .iter()
        .map(|arg| {
            let a = arg.trim();
            // Direct variable lookup first
            if let Some(val) = vars.get(a) {
                return val.clone();
            }
            // If it contains variable refs, substitute them and evaluate
            let mut substituted = a.to_string();
            let mut sorted_vars: Vec<(&String, &String)> = vars.iter().collect();
            sorted_vars.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
            for (name, value) in &sorted_vars {
                let pattern = format!(r"\b{}\b", regex::escape(name));
                if let Ok(re) = regex::Regex::new(&pattern) {
                    substituted = re.replace_all(&substituted, format!("({})", value)).to_string();
                }
            }
            // Try to evaluate via SymEngine
            match Expr::parse(&substituted) {
                Ok(expr) => expr.to_string(),
                Err(_) => substituted,
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
        "integral" => fn_integral(&resolved_args),
        "definite_integral" => fn_definite_integral(&resolved_args),
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

/// Solve expr = 0 for variable. Handles linear and quadratic.
/// Returns comma-separated roots for set/tuple answers.
fn fn_solve(args: &[String]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::FunctionArity {
            name: "solve".into(),
            expected: 2,
            got: args.len(),
        });
    }

    let expr = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    let var = &args[1];

    // Strategy: evaluate expr at enough points to determine polynomial coefficients
    // then solve analytically

    // Get coefficient by evaluating at specific points
    // For f(x) = ax^2 + bx + c:
    //   f(0) = c
    //   f(1) = a + b + c
    //   f(-1) = a - b + c
    //   f(2) = 4a + 2b + c

    let f = |x: f64| -> Option<f64> {
        expr.subs_float(var, x).to_float()
    };

    let f0 = f(0.0).ok_or_else(|| DslError::Evaluation("Can't evaluate at 0".into()))?;
    let f1 = f(1.0).ok_or_else(|| DslError::Evaluation("Can't evaluate at 1".into()))?;
    let fm1 = f(-1.0).ok_or_else(|| DslError::Evaluation("Can't evaluate at -1".into()))?;
    let f2 = f(2.0).ok_or_else(|| DslError::Evaluation("Can't evaluate at 2".into()))?;

    let c = f0;
    let a = (f1 + fm1 - 2.0 * c) / 2.0;
    let b = f1 - a - c;

    // Check if it's actually quadratic by testing f(3)
    let f3 = f(3.0).ok_or_else(|| DslError::Evaluation("Can't evaluate at 3".into()))?;
    let predicted_f3 = 9.0 * a + 3.0 * b + c;
    let is_quadratic_or_linear = (f3 - predicted_f3).abs() < 1e-8;

    if !is_quadratic_or_linear {
        // Higher degree — try numeric root finding
        return solve_numeric(&expr, var);
    }

    if a.abs() < 1e-10 {
        // Linear: bx + c = 0 → x = -c/b
        if b.abs() < 1e-10 {
            return Err(DslError::Evaluation("No variable in expression".into()));
        }
        let root = -c / b;
        return Ok(format_root(root));
    }

    // Quadratic: ax^2 + bx + c = 0
    let disc = b * b - 4.0 * a * c;
    if disc < -1e-10 {
        return Err(DslError::Evaluation("No real roots".into()));
    }

    if disc.abs() < 1e-10 {
        // One repeated root
        let root = -b / (2.0 * a);
        Ok(format_root(root))
    } else {
        // Two roots
        let sqrt_disc = disc.sqrt();
        let r1 = (-b + sqrt_disc) / (2.0 * a);
        let r2 = (-b - sqrt_disc) / (2.0 * a);
        let (r1, r2) = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        Ok(format!("{}, {}", format_root(r1), format_root(r2)))
    }
}

/// Try to find roots numerically via bisection
fn solve_numeric(expr: &Expr, var: &str) -> Result<String, DslError> {
    // Scan for sign changes in [-100, 100]
    let mut roots = Vec::new();
    let step = 0.5;
    let mut x = -100.0;

    while x < 100.0 {
        let y1 = expr.subs_float(var, x).to_float().unwrap_or(f64::NAN);
        let y2 = expr.subs_float(var, x + step).to_float().unwrap_or(f64::NAN);

        if y1.is_finite() && y2.is_finite() && y1 * y2 < 0.0 {
            // Sign change — bisect to find root
            let root = bisect(expr, var, x, x + step, 50);
            if let Some(r) = root {
                // Avoid duplicates
                if !roots.iter().any(|&existing: &f64| (existing - r).abs() < 1e-6) {
                    roots.push(r);
                }
            }
        }
        x += step;
    }

    if roots.is_empty() {
        Err(DslError::Evaluation("No roots found in [-100, 100]".into()))
    } else {
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(roots.iter().map(|r| format_root(*r)).collect::<Vec<_>>().join(", "))
    }
}

fn bisect(expr: &Expr, var: &str, mut lo: f64, mut hi: f64, max_iter: usize) -> Option<f64> {
    for _ in 0..max_iter {
        let mid = (lo + hi) / 2.0;
        let y_mid = expr.subs_float(var, mid).to_float()?;
        let y_lo = expr.subs_float(var, lo).to_float()?;

        if y_mid.abs() < 1e-12 {
            return Some(mid);
        }
        if y_lo * y_mid < 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Some((lo + hi) / 2.0)
}

fn format_root(r: f64) -> String {
    if (r - r.round()).abs() < 1e-10 {
        format!("{}", r.round() as i64)
    } else {
        // Try common fractions
        for denom in 2..=12 {
            let num = r * denom as f64;
            if (num - num.round()).abs() < 1e-8 {
                let n = num.round() as i64;
                let d = denom as i64;
                let g = gcd_i64(n.unsigned_abs(), d as u64) as i64;
                return format!("{}/{}", n / g, d / g);
            }
        }
        format!("{:.6}", r)
    }
}

/// Indefinite integral via reverse power rule.
/// For polynomial terms: integral of a*x^n = a*x^(n+1)/(n+1)
fn fn_integral(args: &[String]) -> Result<String, DslError> {
    if args.len() != 2 {
        return Err(DslError::FunctionArity {
            name: "integral".into(),
            expected: 2,
            got: args.len(),
        });
    }
    let expr = Expr::parse(&args[0]).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    let var = &args[1];

    // Numeric approach: if f(x) = sum of a_i * x^n_i,
    // integral = sum of a_i * x^(n_i+1) / (n_i+1)
    // Detect degree by evaluating derivative chain
    // integral(f, x) is the F such that F'(x) = f(x)
    //
    // Strategy: build polynomial coefficients, integrate term by term
    // f(x) = c0 + c1*x + c2*x^2 + ... + cn*x^n
    // F(x) = c0*x + c1*x^2/2 + c2*x^3/3 + ... + cn*x^(n+1)/(n+1)

    // Find degree by differentiating until zero
    let mut degree = 0;
    let mut test = expr.clone();
    for d in 0..20 {
        if test.subs_float(var, 1.0).to_float().map_or(true, |v| v.abs() < 1e-12)
            && test.subs_float(var, 2.0).to_float().map_or(true, |v| v.abs() < 1e-12)
        {
            degree = d;
            break;
        }
        test = test.diff(var).map_err(|e| DslError::Evaluation(e.to_string()))?;
        degree = d + 1;
    }

    // Extract coefficients using finite differences
    let mut coeffs = Vec::new();
    let mut remaining = expr.clone();
    for n in (0..=degree).rev() {
        // Coefficient of x^n: differentiate n times, divide by n!
        let mut deriv = expr.clone();
        for _ in 0..n {
            deriv = deriv.diff(var).map_err(|e| DslError::Evaluation(e.to_string()))?;
        }
        let coeff = deriv.subs_float(var, 0.0).to_float().unwrap_or(0.0);
        let factorial: f64 = (1..=n as u64).map(|i| i as f64).product::<f64>().max(1.0);
        coeffs.push(coeff / factorial);
        let _ = remaining; // suppress warning
    }
    coeffs.reverse(); // now coeffs[i] = coefficient of x^i

    // Build antiderivative: c_i * x^(i+1) / (i+1)
    let mut terms = Vec::new();
    for (i, &c) in coeffs.iter().enumerate() {
        if c.abs() < 1e-12 {
            continue;
        }
        let new_exp = i + 1;
        let new_coeff = c / new_exp as f64;
        // Format as clean fraction/integer if possible
        let coeff_str = format_root(new_coeff);
        if new_exp == 1 {
            terms.push(format!("{}*{}", coeff_str, var));
        } else {
            terms.push(format!("{}*{}**{}", coeff_str, var, new_exp));
        }
    }

    if terms.is_empty() {
        return Ok("0".to_string());
    }

    let result_str = terms.join(" + ").replace("+ -", "- ");
    // Parse through SymEngine to clean up
    let result = Expr::parse(&result_str).map_err(|e| DslError::ExpressionParse(e.to_string()))?;
    Ok(result.to_string())
}

/// Definite integral: evaluate antiderivative at bounds
fn fn_definite_integral(args: &[String]) -> Result<String, DslError> {
    if args.len() != 4 {
        return Err(DslError::FunctionArity {
            name: "definite_integral".into(),
            expected: 4,
            got: args.len(),
        });
    }
    // Get antiderivative
    let anti = fn_integral(&args[..2].to_vec())?;
    let anti_expr = Expr::parse(&anti).map_err(|e| DslError::ExpressionParse(e.to_string()))?;

    let lo: f64 = args[2].parse().map_err(|_| DslError::Evaluation(format!("Can't parse lo '{}'", args[2])))?;
    let hi: f64 = args[3].parse().map_err(|_| DslError::Evaluation(format!("Can't parse hi '{}'", args[3])))?;

    let f_hi = anti_expr.subs_float(&args[1], hi).to_float()
        .ok_or_else(|| DslError::Evaluation("Can't evaluate at upper bound".into()))?;
    let f_lo = anti_expr.subs_float(&args[1], lo).to_float()
        .ok_or_else(|| DslError::Evaluation("Can't evaluate at lower bound".into()))?;

    let result = f_hi - f_lo;
    Ok(format_root(result))
}

fn gcd_i64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
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
