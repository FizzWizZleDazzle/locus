//! Display functions — formatted LaTeX output for question/solution text
//!
//! These convert variable references into properly formatted LaTeX notation.
//! AI uses these in question text: `{derivative_of(f, x)}` → `\frac{d}{dx}[...]`

use crate::error::DslError;
use crate::resolver::VarMap;
use crate::template::expr_to_latex;

/// Render a display function call to LaTeX
pub fn render_display_func(name: &str, args_str: &str, vars: &VarMap) -> Result<String, DslError> {
    let args: Vec<&str> = split_display_args(args_str);

    // `evaluate` is special: it needs the raw substituted expression text so it
    // can hand it to the SymEngine builtin and compute a concrete value, rather
    // than rendering the unevaluated form like every other display function.
    if name == "evaluate" || name == "eval" {
        return df_evaluate_compute(&args, vars);
    }

    // Matrix-shaped display funcs need cell-by-cell rendering of the raw arg.
    // The default resolver would mangle `[[a, b], [c, d]]` by feeding the whole
    // thing to SymEngine (which doesn't understand list-of-lists syntax) and
    // returning either an error or a corrupted approximation.
    if let Some(env) = matrix_environment(name) {
        return df_matrix_render(env, args_str, vars);
    }
    if name == "vec" {
        return df_vec_column(args_str, vars);
    }
    // `{math(expr)}` is a generic inline-math wrapper for content the AI would
    // otherwise leave as plain text — `f(x)`, `x < c`, `x >= 0`, etc. The arg
    // is variable-substituted and parsed through SymEngine, so inequalities,
    // function notation, and equality all render in LaTeX.
    if name == "math" {
        return df_math_inline(args_str, vars);
    }

    let resolved: Vec<String> = args
        .iter()
        .map(|a| resolve_arg_to_latex(a.trim(), vars))
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
        "sqrt" => df_func_notation(r"\sqrt", &resolved),
        "simplify" => df_passthrough(&resolved),
        "round" => df_passthrough(&resolved),
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
        "expand" => df_passthrough(&resolved),
        "sqrt_of" => df_func_notation(r"\sqrt", &resolved),
        _ => Err(DslError::TemplateDisplayFn {
            name: name.to_string(),
            field: "question/solution".to_string(),
        }),
    }
}

fn df_derivative_of(args: &[String]) -> Result<String, DslError> {
    check_arity("derivative_of", args, 2)?;
    Ok(format!(
        r"\frac{{d}}{{d{}}}\left[{}\right]",
        args[1], args[0]
    ))
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
    Ok(format!(r"\lim_{{{} \to {}}} {}", args[1], args[2], args[0]))
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

/// Map a display-fn name to the LaTeX matrix environment it should produce.
fn matrix_environment(name: &str) -> Option<&'static str> {
    match name {
        "matrix_of" => Some("pmatrix"),
        "det_of" => Some("vmatrix"),
        _ => None,
    }
}

/// Render `matrix_of(var)` or literal `matrix_of([[expr, ...], ...])`.
///
/// Each cell is independently variable-substituted and LaTeX-converted so
/// inner expressions like `a - lambda` come out as `2 - \lambda`, not as the
/// raw substitution text or a corrupted SymEngine partial-parse.
fn df_matrix_render(env: &str, args_str: &str, vars: &VarMap) -> Result<String, DslError> {
    let arg = args_str.trim();

    // Resolve a variable reference like `matrix_of(A)` to its definition text.
    let resolved = if let Some(val) = vars.get(arg) {
        val.clone()
    } else {
        arg.to_string()
    };

    let cells = parse_matrix_literal(&resolved).ok_or_else(|| DslError::TemplateDisplayFn {
        name: env.to_string(),
        field: format!("expected [[expr, ...], ...] form, got: {arg}"),
    })?;

    let latex_rows: Vec<String> = cells
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| resolve_arg_to_latex(cell, vars))
                .collect::<Vec<_>>()
                .join(" & ")
        })
        .collect();

    Ok(format!(
        r"\begin{{{env}}} {body} \end{{{env}}}",
        env = env,
        body = latex_rows.join(r" \\ "),
    ))
}

/// `{math(expr)}` — variable-substitute and render any math expression as
/// LaTeX. Works for inequalities (`x < c`), equalities (`f(x) = 5`), function
/// notation (`f(x)`), and plain expressions. The result does NOT wrap itself
/// in `$...$` — the surrounding `{...}` template ref does that, so the output
/// is `$<latex>$` in the question text.
///
/// SymEngine's parser handles `<`, `<=`, `>`, `>=` natively but rejects `=` as
/// an Equality token. To support `{math(lambda = 3)}`, we split on top-level
/// comparison operators here, render each side via SymEngine, and join with
/// the appropriate LaTeX symbol.
fn df_math_inline(args_str: &str, vars: &VarMap) -> Result<String, DslError> {
    let raw = resolve_arg_to_raw(args_str.trim(), vars);
    if let Some((lhs, op_latex, rhs)) = split_top_level_comparison(&raw) {
        let lhs_latex = expr_to_latex(lhs.trim()).unwrap_or_else(|_| lhs.trim().to_string());
        let rhs_latex = expr_to_latex(rhs.trim()).unwrap_or_else(|_| rhs.trim().to_string());
        return Ok(format!("{lhs_latex} {op_latex} {rhs_latex}"));
    }
    expr_to_latex(&raw)
}

/// Find the first top-level (paren-depth-zero) comparison operator and split
/// `expr` into `(lhs, latex_op, rhs)`. Returns None if no comparison is found.
///
/// Iterates by char_indices so multi-byte UTF-8 characters (e.g. `±` written
/// by an AI that ignored the unicode banlist) don't trigger byte-boundary
/// panics when slicing.
fn split_top_level_comparison(expr: &str) -> Option<(&str, &str, &str)> {
    let mut depth = 0;
    let chars: Vec<(usize, char)> = expr.char_indices().collect();
    for (idx, &(byte_pos, c)) in chars.iter().enumerate() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            _ if depth == 0 => {
                // Two-char operators first — ASCII only, so byte offsets line up.
                if let Some(&(next_byte_pos, next_c)) = chars.get(idx + 1) {
                    let pair_end = next_byte_pos + next_c.len_utf8();
                    if c.is_ascii() && next_c.is_ascii() {
                        let pair = &expr[byte_pos..pair_end];
                        let latex = match pair {
                            "<=" => Some("\\leq"),
                            ">=" => Some("\\geq"),
                            "==" => Some("="),
                            "!=" => Some("\\neq"),
                            _ => None,
                        };
                        if let Some(op) = latex {
                            return Some((&expr[..byte_pos], op, &expr[pair_end..]));
                        }
                    }
                }
                let single = match c {
                    '=' => Some("="),
                    '<' => Some("<"),
                    '>' => Some(">"),
                    _ => None,
                };
                if let Some(op) = single {
                    return Some((&expr[..byte_pos], op, &expr[byte_pos + c.len_utf8()..]));
                }
            }
            _ => {}
        }
    }
    None
}

/// Render `vec([a, b, c])` as a column vector (1-column pmatrix). Falls back
/// to `\vec{x}` for a bare variable name.
fn df_vec_column(args_str: &str, vars: &VarMap) -> Result<String, DslError> {
    let arg = args_str.trim();
    if let Some(row) = parse_vector_literal(arg) {
        let cells: Vec<String> = row.iter().map(|c| resolve_arg_to_latex(c, vars)).collect();
        return Ok(format!(
            r"\begin{{pmatrix}} {} \end{{pmatrix}}",
            cells.join(r" \\ ")
        ));
    }
    // Single identifier — render as \vec{var}
    Ok(format!(r"\vec{{{}}}", resolve_arg_to_latex(arg, vars)))
}

/// Parse `[[expr, expr], [expr, expr]]` into a 2D Vec of cell expressions.
/// Respects nested parens/brackets so `[[a - lambda, b], [c, d]]` works.
fn parse_matrix_literal(s: &str) -> Option<Vec<Vec<String>>> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return None;
    }
    let inner = &s[1..s.len() - 1].trim();
    let rows = split_top_level(inner, ',', '[', ']');
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let cells = parse_vector_literal(row.trim())?;
        out.push(cells);
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

/// Parse `[a, b, c]` into a Vec of cell expression strings.
fn parse_vector_literal(s: &str) -> Option<Vec<String>> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return None;
    }
    let inner = &s[1..s.len() - 1];
    Some(split_top_level(inner, ',', '[', ']'))
}

/// Split on `sep` only when paren/bracket depth is zero.
fn split_top_level(s: &str, sep: char, open: char, close: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut depth = 0;
    for c in s.chars() {
        if c == open || c == '(' {
            depth += 1;
            buf.push(c);
        } else if c == close || c == ')' {
            depth -= 1;
            buf.push(c);
        } else if c == sep && depth == 0 {
            out.push(buf.trim().to_string());
            buf.clear();
        } else {
            buf.push(c);
        }
    }
    if !buf.trim().is_empty() {
        out.push(buf.trim().to_string());
    }
    out
}

// Kept only as a fallback for older callers that pass a pre-resolved single
// LaTeX string. The main pipeline now uses df_matrix_render directly.
fn df_matrix_of(args: &[String]) -> Result<String, DslError> {
    check_arity("matrix_of", args, 1)?;
    Ok(args[0].clone())
}

fn df_det_of(args: &[String]) -> Result<String, DslError> {
    check_arity("det_of", args, 1)?;
    Ok(args[0].clone())
}

// Fallback for pre-resolved single-arg vec(); main path is df_vec_column.
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

/// Display passthrough: takes already-LaTeX'd arg, returns it unchanged.
/// Used for `{expand(f)}` etc where the surrounding template already wants
/// the variable's natural rendering — the expansion happened in the variable
/// definition, not at display time.
fn df_passthrough(args: &[String]) -> Result<String, DslError> {
    if args.is_empty() {
        return Err(DslError::TemplateDisplayFn {
            name: "passthrough".into(),
            field: "requires at least 1 arg".into(),
        });
    }
    Ok(args[0].clone())
}

/// `{evaluate(expr, var, val)}` — substitute `var=val` in `expr` and render the
/// resulting closed-form expression in LaTeX.
///
/// Receives args in raw (post-variable-substitution) form so it can hand them
/// to the SymEngine builtin. Returning the unevaluated expression — as the old
/// `df_evaluate` did — caused the bug where solution text showed
/// `(3)^2 + 3 = $3 + x^2$` instead of `12`.
fn df_evaluate_compute(args: &[&str], vars: &VarMap) -> Result<String, DslError> {
    if args.len() < 3 || (args.len() - 1) % 2 != 0 {
        return Err(DslError::FunctionArity {
            name: "evaluate".into(),
            expected: 3,
            got: args.len(),
        });
    }

    // Build a synthetic builtin call and let crates::functions::evaluate run it.
    // This sidesteps duplicating the substitution logic and inherits all the
    // numeric-vs-symbolic handling for free.
    let raw_args: Vec<String> = args
        .iter()
        .map(|a| resolve_arg_to_raw(a.trim(), vars))
        .collect();
    let synthetic_call = format!("evaluate({})", raw_args.join(", "));
    let result = crate::functions::evaluate(&synthetic_call, vars)?;
    expr_to_latex(&result)
}

/// Substitute variable refs in `arg` and return the post-substitution string
/// without LaTeX conversion. Used by `df_evaluate_compute`.
fn resolve_arg_to_raw(arg: &str, vars: &VarMap) -> String {
    if let Some(val) = vars.get(arg) {
        return val.clone();
    }
    let mut out = arg.to_string();
    let mut sorted: Vec<(&String, &String)> = vars.iter().collect();
    sorted.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    for (name, value) in &sorted {
        let pattern = format!(r"\b{}\b", regex::escape(name));
        if let Ok(re) = regex::Regex::new(&pattern) {
            let wrapped = wrap_for_substitution(value);
            out = re.replace_all(&out, wrapped.as_str()).to_string();
        }
    }
    out
}

/// Wrap a substituted value in parens only when needed for safe interpolation.
/// A naked positive integer or single identifier needs no parens; this avoids
/// the `(2)*x + (2)*y` ugliness when substitution output is rendered without
/// passing through SymEngine for canonicalization.
fn wrap_for_substitution(value: &str) -> String {
    let v = value.trim();
    if needs_parens_when_substituted(v) {
        format!("({v})")
    } else {
        v.to_string()
    }
}

fn needs_parens_when_substituted(v: &str) -> bool {
    if v.is_empty() {
        return false;
    }
    // Pure integer / decimal literal — never wrap
    if v.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return false;
    }
    // Negative literals get wrapped to avoid `a - -3` → `a--3` quirks
    if v.starts_with('-') {
        return true;
    }
    // Single identifier — no parens
    if v.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return false;
    }
    // Compound expression with top-level `+`/`-` → wrap. Multiplication-only
    // expressions like `3*x` don't need wrapping in product context.
    let mut depth = 0;
    for c in v.chars() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            '+' | '-' if depth == 0 => return true,
            _ => {}
        }
    }
    false
}

/// Substitute variable refs in `arg`, then convert the result to LaTeX.
/// The default arg-resolution path for every display func except `evaluate`.
fn resolve_arg_to_latex(arg: &str, vars: &VarMap) -> String {
    if let Some(val) = vars.get(arg) {
        return expr_to_latex(val).unwrap_or_else(|_| arg.to_string());
    }
    // Recursively dispatch nested display-func calls like
    // `equation(derivative_of(x, t), a*x + b*y)`. Without this, the inner
    // `derivative_of(x, t)` would be handed to SymEngine as a free symbol
    // and rendered literally.
    if let Some((inner_name, inner_args)) = parse_display_call(arg) {
        if is_display_func_name(inner_name) {
            return render_display_func(inner_name, inner_args, vars)
                .unwrap_or_else(|_| arg.to_string());
        }
    }
    let raw = resolve_arg_to_raw(arg, vars);
    expr_to_latex(&raw).unwrap_or(raw)
}

/// If `s` parses as a top-level function call `name(args)`, return the parts.
fn parse_display_call(s: &str) -> Option<(&str, &str)> {
    let s = s.trim();
    if !s.ends_with(')') {
        return None;
    }
    let open = s.find('(')?;
    let name = s[..open].trim();
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    // Verify the closing paren matches the opening one — bails on `(a)+(b)` etc.
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 && i != s.len() - 1 {
                    return None;
                }
            }
            _ => {}
        }
    }
    Some((name, &s[open + 1..s.len() - 1]))
}

/// True if `name` is one of the registered display-function dispatch keys.
fn is_display_func_name(name: &str) -> bool {
    matches!(
        name,
        "derivative_of"
            | "derivative"
            | "nth_derivative_of"
            | "partial_of"
            | "integral_of"
            | "definite_integral_of"
            | "limit_of"
            | "sum_of"
            | "product_of"
            | "equation"
            | "system"
            | "matrix_of"
            | "det_of"
            | "vec"
            | "norm"
            | "abs_of"
            | "abs"
            | "set_of"
            | "display"
            | "binomial"
            | "evaluate"
            | "eval"
            | "sqrt"
            | "sqrt_of"
            | "simplify"
            | "round"
            | "expand"
            | "log"
            | "ln"
            | "sin"
            | "cos"
            | "tan"
            | "asin"
            | "acos"
            | "atan"
            | "floor"
            | "ceil"
            | "exp"
            | "mod"
            | "math"
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitution_parens_pure_integer_no_wrap() {
        assert!(!needs_parens_when_substituted("3"));
        assert!(!needs_parens_when_substituted("123"));
        assert!(!needs_parens_when_substituted("1.5"));
    }

    #[test]
    fn substitution_parens_identifier_no_wrap() {
        assert!(!needs_parens_when_substituted("x"));
        assert!(!needs_parens_when_substituted("foo_bar"));
    }

    #[test]
    fn substitution_parens_negative_wraps() {
        // `a + -3` would render strangely; force `a + (-3)`.
        assert!(needs_parens_when_substituted("-3"));
        assert!(needs_parens_when_substituted("-x"));
    }

    #[test]
    fn substitution_parens_top_level_addition_wraps() {
        assert!(needs_parens_when_substituted("a + b"));
        assert!(needs_parens_when_substituted("x - 1"));
    }

    #[test]
    fn substitution_parens_multiplication_doesnt_wrap() {
        // `3*x` substituted into a sum stays unambiguous: `a + 3*x` is fine.
        assert!(!needs_parens_when_substituted("3*x"));
    }

    #[test]
    fn substitution_parens_nested_addition_inside_parens_doesnt_count() {
        // The top-level operator is `*`, the `+` lives inside parens.
        assert!(!needs_parens_when_substituted("(a + b)*c"));
    }

    #[test]
    fn split_comparison_handles_multibyte_chars() {
        // Regression: `±` is multi-byte UTF-8; byte slicing would panic if we
        // computed pair offsets without char_indices.
        assert_eq!(split_top_level_comparison("(-(1)) ± 7)/2"), None);
        assert_eq!(
            split_top_level_comparison("x ± 1 < 5"),
            Some(("x ± 1 ", "<", " 5"))
        );
    }

    #[test]
    fn split_comparison_finds_equals() {
        assert_eq!(
            split_top_level_comparison("lambda = 3"),
            Some(("lambda ", "=", " 3"))
        );
    }

    #[test]
    fn split_comparison_two_char_ops() {
        assert_eq!(
            split_top_level_comparison("x >= 0"),
            Some(("x ", "\\geq", " 0"))
        );
    }

    #[test]
    fn parse_display_call_recognizes_simple_form() {
        assert_eq!(parse_display_call("foo(a, b)"), Some(("foo", "a, b")));
        assert_eq!(
            parse_display_call("derivative_of(f, x)"),
            Some(("derivative_of", "f, x"))
        );
    }

    #[test]
    fn parse_display_call_rejects_non_calls() {
        assert_eq!(parse_display_call("a + b"), None);
        assert_eq!(parse_display_call("(a)+(b)"), None); // not a single trailing call
        assert_eq!(parse_display_call("3"), None);
        assert_eq!(parse_display_call(""), None);
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
