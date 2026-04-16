//! Template interpolation — {var} and {display()} substitution in question/solution text

use locus_common::symengine::Expr;

use crate::display;
use crate::error::DslError;
use crate::resolver::VarMap;

/// Render a template string, replacing `{var}` refs and `{display_func(args)}` calls.
///
/// - `{var_name}` → evaluate variable, convert to LaTeX, wrap in `$...$`
/// - `{display_func(args)}` → call display function, output formatted LaTeX
/// - `{{var_name}}` → display mode (centered, `$$...$$`)
/// - Plain text passes through unchanged
pub fn render(template: &str, vars: &VarMap) -> Result<String, DslError> {
    // Strip any $ or $$ delimiters from the template before processing refs.
    // AI sometimes includes LaTeX delimiters despite being told not to.
    let template = strip_dollar_signs(template);
    let mut result = String::with_capacity(template.len());
    // Work with byte offsets — safe as long as we check char boundaries
    let bytes = template.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let display_mode = i + 1 < bytes.len() && bytes[i + 1] == b'{';
            let start = if display_mode { i + 2 } else { i + 1 };

            let close = if display_mode { "}}" } else { "}" };
            if let Some(end_rel) = template[start..].find(close) {
                let content = &template[start..start + end_rel];
                if content.trim().is_empty() {
                    i = start + end_rel + close.len();
                    continue;
                }
                let rendered = render_ref(content.trim(), vars)?;

                if display_mode {
                    result.push_str(&format!("$${}$$", rendered));
                } else {
                    result.push_str(&format!("${}$", rendered));
                }
                i = start + end_rel + close.len();
            } else {
                result.push('{');
                i += 1;
            }
        } else {
            // Push full UTF-8 char
            let c = template[i..].chars().next().unwrap();
            result.push(c);
            i += c.len_utf8();
        }
    }

    Ok(result)
}

/// Render solution steps (one per line)
pub fn render_steps(steps: &[String], vars: &VarMap) -> Result<String, DslError> {
    let rendered: Result<Vec<String>, _> = steps.iter().map(|s| render(s, vars)).collect();
    Ok(rendered?.join("\n"))
}

/// Render a single reference: either a variable name or a display function call
fn render_ref(content: &str, vars: &VarMap) -> Result<String, DslError> {
    // Check if it's a display function call: name(args)
    if let Some(paren) = content.find('(') {
        if content.ends_with(')') {
            let func_name = &content[..paren];
            let args_str = &content[paren + 1..content.len() - 1];
            return display::render_display_func(func_name, args_str, vars);
        }
    }

    // Simple variable reference
    if let Some(value) = vars.get(content) {
        return expr_to_latex(value);
    }

    // Fallback: try evaluating as expression with variable substitution
    // Handles cases like {a*b} or {n-1} that AI writes despite being told not to
    let mut substituted = content.to_string();
    let mut sorted: Vec<(&String, &String)> = vars.iter().collect();
    sorted.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    for (name, value) in &sorted {
        let pattern = format!(r"\b{}\b", regex::escape(name));
        if let Ok(re) = regex::Regex::new(&pattern) {
            substituted = re.replace_all(&substituted, format!("({})", value)).to_string();
        }
    }
    // If substitution changed something, try to render it
    if substituted != content {
        return expr_to_latex(&substituted);
    }

    Err(DslError::TemplateRef {
        name: content.to_string(),
        field: "question/solution".to_string(),
    })
}

/// Strip `$` and `$$` delimiters from template text.
/// AI sometimes wraps expressions in `$...$` or `$$...$$` despite being told not to.
/// We only strip bare `$` signs that appear outside of `{...}` refs.
fn strip_dollar_signs(s: &str) -> String {
    // Replace $$ first, then $
    let mut result = s.to_string();
    // Remove standalone $$ that are not part of template refs
    result = result.replace("$$", "");
    // Remove standalone $ that are not part of template refs
    result = result.replace('$', "");
    result
}

/// Convert a SymEngine expression string to LaTeX
pub fn expr_to_latex(expr_str: &str) -> Result<String, DslError> {
    // Try parsing as SymEngine expression for proper LaTeX formatting
    match Expr::parse(expr_str) {
        Ok(expr) => {
            let se_str = expr.to_string();
            // Convert SymEngine output to LaTeX
            Ok(symengine_to_latex(&se_str))
        }
        Err(_) => {
            // If parse fails, return as-is (might be a word answer, etc.)
            Ok(expr_str.to_string())
        }
    }
}

/// Convert SymEngine string representation to LaTeX notation
fn symengine_to_latex(s: &str) -> String {
    let mut result = s.to_string();

    // x**2 → x^{2}
    let re_pow = regex::Regex::new(r"(\w)\*\*(\d+)").unwrap();
    result = re_pow.replace_all(&result, r"$1^{$2}").to_string();

    // More complex powers: (expr)**n
    let re_pow2 = regex::Regex::new(r"\)\*\*(\d+)").unwrap();
    result = re_pow2.replace_all(&result, r")^{$1}").to_string();

    // Fractions: keep as-is for now (SymEngine outputs a/b which KaTeX handles)
    // Multiplication: remove explicit * between number and variable
    let re_mul = regex::Regex::new(r"(\d)\*([a-zA-Z])").unwrap();
    result = re_mul.replace_all(&result, r"$1$2").to_string();

    // pi → \pi
    result = result.replace("pi", r"\pi");

    result
}

/// Find closing delimiter, respecting nested braces
fn find_closing(s: &str, close: &str) -> Option<usize> {
    let mut depth = 0;
    let chars: Vec<char> = s.chars().collect();
    let close_chars: Vec<char> = close.chars().collect();

    for i in 0..chars.len() {
        if chars[i] == '{' {
            depth += 1;
        } else if chars[i] == '}' {
            if depth > 0 {
                depth -= 1;
            } else {
                // Check if this matches the close pattern
                if i + close_chars.len() <= chars.len() {
                    let candidate: String = chars[i..i + close_chars.len()].iter().collect();
                    if candidate == close {
                        return Some(i);
                    }
                }
                return Some(i);
            }
        }
    }
    None
}
