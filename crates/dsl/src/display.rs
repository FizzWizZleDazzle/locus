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
    let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
    let resolved: Vec<String> = args
        .iter()
        .map(|a| {
            if let Some(val) = vars.get(*a) {
                expr_to_latex(val).unwrap_or_else(|_| a.to_string())
            } else {
                a.to_string()
            }
        })
        .collect();

    match name {
        "derivative_of" => df_derivative_of(&resolved),
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
        "abs_of" => df_abs_of(&resolved),
        "set_of" => df_set_of(&resolved),
        "display" => df_display(&resolved),
        "binomial" => df_binomial(&resolved),
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
    check_arity("equation", args, 2)?;
    Ok(format!("{} = {}", args[0], args[1]))
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
