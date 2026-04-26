//! Symbolic hoisting: pre-compute SymEngine ops once per YAML so the per-combo
//! work is pure integer arithmetic on samplers.
//!
//! Strategy
//! --------
//! 1. Walk variables in topological order. Treat each free sampler as a
//!    placeholder symbol bearing its own name.
//! 2. For arithmetic derived vars, build the SymEngine `Expr` by substituting
//!    dependencies' string forms.
//! 3. For builtin calls (`derivative`, `expand`, `evaluate`, …), dispatch
//!    symbolically using SymEngine FFI methods.
//! 4. Each var's resulting `Expr` is in placeholder symbols matching the
//!    sampler names. If the answer (and constraint vars) reduce to integer
//!    arithmetic in samplers only, the bytecode VM can evaluate per combo.
//!
//! Vars whose `Expr` still references non-sampler symbols (typically `x`, `y`)
//! are kept symbolic — the GPU enumerator skips them, and the CPU render pass
//! recomputes them via `resolver::resolve_with_preset`.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use locus_common::symengine::Expr;

#[derive(Debug, thiserror::Error)]
pub enum HoistError {
    #[error("unsupported builtin '{0}' for hoisting")]
    UnsupportedBuiltin(String),
    #[error("symengine error: {0}")]
    SymEngine(String),
    #[error("circular dependency: {0}")]
    Circular(String),
    #[error("non-integer rational in '{0}'")]
    Rational(String),
}

/// One entry per variable.
pub struct HoistedExpr {
    /// Symbolic SymEngine expression in sampler placeholder names.
    pub expr: Expr,
    /// Free symbols. If a strict subset of `sampler_names`, the var is
    /// integer-evaluable per combo; otherwise it's symbolic-with-x and skipped.
    pub free: BTreeSet<String>,
}

pub struct HoistResult {
    pub sampler_names: BTreeSet<String>,
    pub by_var: HashMap<String, HoistedExpr>,
}

/// Hoist as many vars as possible into symbolic Expr form.
///
/// Returns one entry per variable whose symbolic value could be computed.
/// Vars whose builtin op we can't hoist (`solve`, `integrate`, …) are simply
/// absent from `by_var`.
pub fn try_hoist(variables: &BTreeMap<String, String>) -> Result<HoistResult, HoistError> {
    let order =
        crate::resolver::topo_sort(variables).map_err(|e| HoistError::Circular(e.to_string()))?;

    let mut sym_table: HashMap<String, Expr> = HashMap::new();
    let mut sampler_names: BTreeSet<String> = BTreeSet::new();
    let mut by_var: HashMap<String, HoistedExpr> = HashMap::new();

    for name in &order {
        let def = variables[name].trim();

        let expr = if crate::sampler::is_sampler(def) {
            sampler_names.insert(name.clone());
            Expr::parse(name).map_err(|e| HoistError::SymEngine(e.to_string()))?
        } else if crate::functions::is_builtin_call(def) {
            match sym_eval_builtin(def, &sym_table) {
                Ok(e) => e,
                Err(_) => continue, // can't hoist this var; skip
            }
        } else {
            let substituted = substitute_var_refs(def, &sym_table);
            match Expr::parse(&substituted) {
                Ok(e) => e,
                Err(_) => continue,
            }
        };

        let free: BTreeSet<String> = expr.free_symbols().into_iter().collect();
        sym_table.insert(name.clone(), expr.clone());
        by_var.insert(name.clone(), HoistedExpr { expr, free });
    }

    Ok(HoistResult {
        sampler_names,
        by_var,
    })
}

/// Substitute variable references in `s` with their Expr.to_string() forms.
/// Longest-name-first to avoid prefix collisions.
fn substitute_var_refs(s: &str, sym_table: &HashMap<String, Expr>) -> String {
    let mut sorted: Vec<&String> = sym_table.keys().collect();
    sorted.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut out = s.to_string();
    for name in sorted {
        let pat = format!(r"\b{}\b", regex::escape(name));
        if let Ok(re) = regex::Regex::new(&pat) {
            let value = sym_table[name].to_string();
            out = re.replace_all(&out, format!("({})", value)).to_string();
        }
    }
    out
}

/// Symbolically evaluate a builtin call. Returns `UnsupportedBuiltin` if we
/// don't have a symbolic implementation.
fn sym_eval_builtin(def: &str, sym_table: &HashMap<String, Expr>) -> Result<Expr, HoistError> {
    let (name, args) = parse_call(def)?;
    let resolved_args: Vec<String> = args
        .iter()
        .map(|a| substitute_var_refs(a, sym_table))
        .collect();

    match name {
        "derivative" | "partial" => {
            if resolved_args.len() != 2 {
                return Err(HoistError::UnsupportedBuiltin(format!(
                    "{name} expects (f, var)"
                )));
            }
            let f =
                Expr::parse(&resolved_args[0]).map_err(|e| HoistError::SymEngine(e.to_string()))?;
            let var = resolved_args[1]
                .trim()
                .trim_matches(|c| c == '(' || c == ')');
            f.diff(var)
                .map_err(|e| HoistError::SymEngine(e.to_string()))
        }
        "expand" | "simplify" => {
            if resolved_args.len() != 1 {
                return Err(HoistError::UnsupportedBuiltin(format!(
                    "{name} expects (f)"
                )));
            }
            let f =
                Expr::parse(&resolved_args[0]).map_err(|e| HoistError::SymEngine(e.to_string()))?;
            Ok(f.expand())
        }
        "evaluate" => {
            if resolved_args.len() < 3 || (resolved_args.len() - 1) % 2 != 0 {
                return Err(HoistError::UnsupportedBuiltin(
                    "evaluate expects (f, var, val [, var, val…])".into(),
                ));
            }
            let mut e = Expr::parse(&resolved_args[0])
                .map_err(|err| HoistError::SymEngine(err.to_string()))?;
            for pair in resolved_args[1..].chunks(2) {
                let var = pair[0].trim().trim_matches(|c| c == '(' || c == ')');
                let val =
                    Expr::parse(&pair[1]).map_err(|err| HoistError::SymEngine(err.to_string()))?;
                e = e.subs_expr(var, &val);
            }
            Ok(e)
        }
        // Other ops: leave to runtime SymEngine via legacy fallback.
        _ => Err(HoistError::UnsupportedBuiltin(name.into())),
    }
}

fn parse_call(def: &str) -> Result<(&str, Vec<&str>), HoistError> {
    let paren = def
        .find('(')
        .ok_or_else(|| HoistError::UnsupportedBuiltin(format!("not a call: {def}")))?;
    if !def.ends_with(')') {
        return Err(HoistError::UnsupportedBuiltin(format!("missing ): {def}")));
    }
    let name = &def[..paren];
    let inner = &def[paren + 1..def.len() - 1];
    Ok((name, split_top_args(inner)))
}

/// Split args by top-level commas (ignore commas inside nested parens).
fn split_top_args(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    let mut start = 0;
    for (i, &c) in bytes.iter().enumerate() {
        match c {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b',' if depth == 0 => {
                out.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < s.len() {
        out.push(s[start..].trim());
    }
    out
}

/// Convert a SymEngine expression to a string parseable by our bytecode
/// compiler. Replaces `**` (Python pow) with `^` and rejects rationals
/// (we don't yet have rational arithmetic in the VM).
pub fn expr_to_bytecode_input(expr: &Expr) -> Result<String, HoistError> {
    let s = expr.to_string();
    let normalized = s.replace("**", "^");
    if has_rational_constant(&normalized) {
        return Err(HoistError::Rational(s));
    }
    Ok(normalized)
}

/// Check whether the SymEngine string contains a rational constant of the
/// form `(a/b)` where a, b are integers (possibly with sign), e.g. `(1/2)`,
/// `(-3/2)`. We only flag those because integer division in our VM would
/// silently truncate.
fn has_rational_constant(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'(' {
            let mut j = i + 1;
            // optional sign
            if j < bytes.len() && (bytes[j] == b'-' || bytes[j] == b'+') {
                j += 1;
            }
            let num_start = j;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            if j > num_start && j < bytes.len() && bytes[j] == b'/' {
                let denom_start = j + 1;
                let mut k = denom_start;
                while k < bytes.len() && bytes[k].is_ascii_digit() {
                    k += 1;
                }
                if k > denom_start && k < bytes.len() && bytes[k] == b')' {
                    return true;
                }
            }
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hoist_arithmetic() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "integer(1, 5)".into());
        vars.insert("b".into(), "a + 3".into());
        let h = try_hoist(&vars).unwrap();
        assert!(h.by_var.contains_key("a"));
        assert!(h.by_var.contains_key("b"));
        let b = &h.by_var["b"];
        assert_eq!(b.expr.to_string(), "3 + a");
    }

    #[test]
    fn hoist_derivative_evaluate() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "integer(1, 3)".into());
        vars.insert("b".into(), "integer(1, 3)".into());
        vars.insert("f".into(), "a*x^2 + b*x".into());
        vars.insert("fp".into(), "derivative(f, x)".into());
        vars.insert("ans".into(), "evaluate(fp, x, 2)".into());

        let h = try_hoist(&vars).unwrap();
        assert!(h.by_var.contains_key("ans"));
        let ans = &h.by_var["ans"];
        // 2*a*x + b at x=2 → 4*a + b. Free symbols = {a, b}.
        let s = ans.expr.to_string();
        assert!(s.contains("4*a"));
        assert!(s.contains("b"));
        assert_eq!(ans.free, ["a", "b"].iter().map(|s| s.to_string()).collect());
    }

    #[test]
    fn rational_is_rejected() {
        let mut vars = BTreeMap::new();
        vars.insert("a".into(), "integer(1, 5)".into());
        vars.insert("b".into(), "(a + 1)/2".into());
        let h = try_hoist(&vars).unwrap();
        let b = &h.by_var["b"];
        // b's Expr is `(1/2)*(1 + a)` — has rational
        let r = expr_to_bytecode_input(&b.expr);
        assert!(r.is_err());
    }
}
