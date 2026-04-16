//! Display functions — formatted LaTeX output for question/solution text
//!
//! These convert variable references into properly formatted LaTeX notation.
//! AI uses these in question text: `{derivative_of(f, x)}` → `\frac{d}{dx}[...]`

use crate::error::DslError;
use crate::resolver::VarMap;
use crate::template::expr_to_latex;

/// Render a display function call to LaTeX
pub fn render_display_func(
    name: &str,
    args_str: &str,
    vars: &VarMap,
) -> Result<String, DslError> {
    let args: Vec<&str> = split_display_args(args_str);
    let resolved: Vec<String> = args
        .iter()
        .map(|a| {
            let trimmed = a.trim();
            // Direct variable lookup
            if let Some(val) = vars.get(trimmed) {
                return expr_to_latex(val).unwrap_or_else(|_| trimmed.to_string());
            }
            // Try evaluating as expression with variable substitution
            let mut substituted = trimmed.to_string();
            let mut sorted: Vec<(&String, &String)> = vars.iter().collect();
            sorted.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
            for (name, value) in &sorted {
                let pattern = format!(r"\b{}\b", regex::escape(name));
                if let Ok(re) = regex::Regex::new(&pattern) {
                    substituted = re.replace_all(&substituted, format!("({})", value)).to_string();
                }
            }
            expr_to_latex(&substituted).unwrap_or_else(|_| substituted)
        })
        .collect();

    match name {
        "derivative_of" | "derivative" => df_derivative_of(&resolved),
        "nth_derivative_of" => df_nth_derivative_of(&resolved),
        "partial_of" => df_partial_of(&resolved),
        "integral_of" => df_integral_of(&resolved),
        "definite_integral_of" => df_definite_integral_of(&resolved),
        "limit_of" => df_limit_of(&resolved),
        "sum_of" => df_sum_of(&resolved),
        "product_of" => df_product_of(&resolved),
        "equation" => df_equation(&resolved),
        "system" => df_system(&resolved),
        "matrix_of" => df_matrix_of(&resolved),
        "det_of" => df_det_of(&resolved),
        "vec" => df_vec(&resolved),
        "norm" => df_norm(&resolved),
        "abs_of" | "abs" => df_abs_of(&resolved),
        "set_of" => df_set_of(&resolved),
        "display" => df_display(&resolved),
        "binomial" => df_binomial(&resolved),
        // Aliases for common AI-generated display function names
        "evaluate" => df_evaluate(&resolved),
        "sqrt" => df_func_notation(r"\sqrt", &resolved),
        "simplify" => df_evaluate(&resolved),
        "round" => df_evaluate(&resolved),
        "log" => df_func_notation(r"\log", &resolved),
        "ln" => df_func_notation(r"\ln", &resolved),
        "sin" => df_func_notation(r"\sin", &resolved),
        "cos" => df_func_notation(r"\cos", &resolved),
        "tan" => df_func_notation(r"\tan", &resolved),
        "asin" => df_func_notation(r"\arcsin", &resolved),
        "acos" => df_func_notation(r"\arccos", &resolved),
        "atan" => df_func_notation(r"\arctan", &resolved),
        "floor" => df_floor_ceil(r"\lfloor", r"\rfloor", &resolved),
        "ceil" => df_floor_ceil(r"\lceil", r"\rceil", &resolved),
        "exp" => df_func_notation(r"\exp", &resolved),
        "mod" => df_mod(&resolved),
        "expand" => df_evaluate(&resolved),
        "sqrt_of" => df_func_notation(r"\sqrt", &resolved),
        "eval" => df_evaluate(&resolved),
        _ => Err(DslError::TemplateDisplayFn {
            name: name.to_string(),
            field: "question/solution".to_string(),
        }),
    }
}

fn df_derivative_of(args: &[String]) -> Result<String, DslError> {
    check_arity("derivative_of", args, 2)?;
    Ok(format!(r"\frac{{d}}{{d{}}}\left[{}\right]", args[1], args[0]))
}

fn df_nth_derivative_of(args: &[String]) -> Result<String, DslError> {
    check_arity("nth_derivative_of", args, 3)?;
    Ok(format!(
        r"\frac{{d^{{{}}}}}{{d{}^{{{}}}}}\left[{}\right]",
        args[2], args[1], args[2], args[0]
    ))
}

fn df_partial_of(args: &[String]) -> Result<String, DslError> {
    check_arity("partial_of", args, 2)?;
    Ok(format!(
        r"\frac{{\partial}}{{\partial {}}}\left[{}\right]",
        args[1], args[0]
    ))
}

fn df_integral_of(args: &[String]) -> Result<String, DslError> {
    check_arity("integral_of", args, 2)?;
    Ok(format!(r"\int {} \, d{}", args[0], args[1]))
}

fn df_definite_integral_of(args: &[String]) -> Result<String, DslError> {
    check_arity("definite_integral_of", args, 4)?;
    Ok(format!(
        r"\int_{{{}}}^{{{}}} {} \, d{}",
        args[2], args[3], args[0], args[1]
    ))
}

fn df_limit_of(args: &[String]) -> Result<String, DslError> {
    check_arity("limit_of", args, 3)?;
    Ok(format!(
        r"\lim_{{{} \to {}}} {}",
        args[1], args[2], args[0]
    ))
}

fn df_sum_of(args: &[String]) -> Result<String, DslError> {
    check_arity("sum_of", args, 4)?;
    Ok(format!(
        r"\sum_{{{}={}}}^{{{}}} {}",
        args[1], args[2], args[3], args[0]
    ))
}

fn df_product_of(args: &[String]) -> Result<String, DslError> {
    check_arity("product_of", args, 4)?;
    Ok(format!(
        r"\prod_{{{}={}}}^{{{}}} {}",
        args[1], args[2], args[3], args[0]
    ))
}

fn df_equation(args: &[String]) -> Result<String, DslError> {
    if args.len() < 2 {
        return Err(DslError::FunctionArity {
            name: "equation".into(),
            expected: 2,
            got: args.len(),
        });
    }
    // Join all args with " = " for multi-part equations like equation(a, b, c)
    Ok(args.join(" = "))
}

fn df_system(args: &[String]) -> Result<String, DslError> {
    if args.is_empty() {
        return Err(DslError::TemplateDisplayFn {
            name: "system".into(),
            field: "requires at least 1 equation".into(),
        });
    }
    let lines = args.join(r" \\ ");
    Ok(format!(r"\begin{{cases}} {} \end{{cases}}", lines))
}

fn df_matrix_of(args: &[String]) -> Result<String, DslError> {
    check_arity("matrix_of", args, 1)?;
    // Parse [[a, b], [c, d]] format → \begin{pmatrix} a & b \\ c & d \end{pmatrix}
    let m = &args[0];
    if m.starts_with("[[") && m.ends_with("]]") {
        let inner = &m[1..m.len() - 1]; // strip outer []
        let rows: Vec<&str> = inner.split("], [").collect();
        let latex_rows: Vec<String> = rows
            .iter()
            .map(|row| {
                let clean = row.trim_matches(|c| c == '[' || c == ']');
                clean
                    .split(',')
                    .map(|e| e.trim())
                    .collect::<Vec<_>>()
                    .join(" & ")
            })
            .collect();
        Ok(format!(
            r"\begin{{pmatrix}} {} \end{{pmatrix}}",
            latex_rows.join(r" \\ ")
        ))
    } else {
        // Already formatted or single value
        Ok(args[0].clone())
    }
}

fn df_det_of(args: &[String]) -> Result<String, DslError> {
    check_arity("det_of", args, 1)?;
    let matrix = df_matrix_of(args)?;
    // Replace pmatrix with vmatrix for determinant notation
    Ok(matrix
        .replace("pmatrix", "vmatrix"))
}

fn df_vec(args: &[String]) -> Result<String, DslError> {
    check_arity("vec", args, 1)?;
    Ok(format!(r"\vec{{{}}}", args[0]))
}

fn df_norm(args: &[String]) -> Result<String, DslError> {
    check_arity("norm", args, 1)?;
    Ok(format!(r"\|\vec{{{}}}\|", args[0]))
}

fn df_abs_of(args: &[String]) -> Result<String, DslError> {
    check_arity("abs_of", args, 1)?;
    Ok(format!(r"|{}|", args[0]))
}

fn df_set_of(args: &[String]) -> Result<String, DslError> {
    check_arity("set_of", args, 1)?;
    Ok(format!(r"\lbrace {} \rbrace", args[0]))
}

fn df_display(args: &[String]) -> Result<String, DslError> {
    check_arity("display", args, 1)?;
    // Display mode is handled by the template renderer (double braces)
    Ok(args[0].clone())
}

fn df_binomial(args: &[String]) -> Result<String, DslError> {
    check_arity("binomial", args, 2)?;
    Ok(format!(r"\binom{{{}}}{{{}}}", args[0], args[1]))
}

/// Display evaluate: substitute and render the result.
/// Accepts `evaluate(expr, var, val)` or `evaluate(expr, var1, val1, var2, val2, ...)`
/// For display purposes, just render the first arg (the expression) as LaTeX.
fn df_evaluate(args: &[String]) -> Result<String, DslError> {
    if args.is_empty() {
        return Err(DslError::TemplateDisplayFn {
            name: "evaluate".into(),
            field: "requires at least 1 arg".into(),
        });
    }
    // Render the expression (first arg) as LaTeX
    Ok(args[0].clone())
}

/// Display a math function in LaTeX notation: \func{arg} or \func(arg1, arg2)
fn df_func_notation(latex_cmd: &str, args: &[String]) -> Result<String, DslError> {
    if args.is_empty() {
        return Err(DslError::TemplateDisplayFn {
            name: latex_cmd.into(),
            field: "requires at least 1 arg".into(),
        });
    }
    if latex_cmd == r"\sqrt" {
        // sqrt uses \sqrt{...} notation
        Ok(format!(r"{}{{{}}}", latex_cmd, args[0]))
    } else if args.len() == 1 {
        Ok(format!(r"{}\left({}\right)", latex_cmd, args[0]))
    } else {
        // e.g. log(x, base) → \log_{base}(x)
        if latex_cmd == r"\log" && args.len() == 2 {
            Ok(format!(r"\log_{{{}}}\left({}\right)", args[1], args[0]))
        } else {
            let joined = args.join(", ");
            Ok(format!(r"{}\left({}\right)", latex_cmd, joined))
        }
    }
}

/// Display floor/ceil with bracket notation: ⌊x⌋ or ⌈x⌉
fn df_floor_ceil(open: &str, close: &str, args: &[String]) -> Result<String, DslError> {
    if args.len() != 1 {
        return Err(DslError::TemplateDisplayFn {
            name: "floor/ceil".into(),
            field: "requires 1 arg".into(),
        });
    }
    Ok(format!("{} {} {}", open, args[0], close))
}

fn df_mod(args: &[String]) -> Result<String, DslError> {
    check_arity("mod", args, 2)?;
    Ok(format!(r"{} \bmod {}", args[0], args[1]))
}

fn check_arity(name: &str, args: &[String], expected: usize) -> Result<(), DslError> {
    if args.len() != expected {
        Err(DslError::FunctionArity {
            name: name.to_string(),
            expected,
            got: args.len(),
        })
    } else {
        Ok(())
    }
}

/// Split comma-separated args respecting nested parentheses
fn split_display_args(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in s.char_indices() {
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
