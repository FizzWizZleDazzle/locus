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

    Ok(merge_adjacent_math_regions(&result))
}

/// Post-process rendered template to merge math runs into single `$...$`
/// regions. After interpolation, each `{var}` produces its own `$..$`, with
/// surrounding math glue (`=`, `<`, `f(`, `)`, etc.) sitting between them as
/// plain text. KaTeX renders those gaps in the body font, which looks broken
/// next to the styled math.
///
/// Strategy: tokenize the rendered string into a sequence of spans —
/// `Math(content)`, `Glue(content)` (math-like text, OK to absorb),
/// `Prose(content)` (anything else, must stand alone) — then collapse any
/// adjacent run of Math/Glue spans into a single Math span.
fn merge_adjacent_math_regions(s: &str) -> String {
    let spans = tokenize_spans(s);
    let collapsed = collapse_math_runs(spans);
    collapsed.into_iter().map(span_to_string).collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Span<'a> {
    /// Inline math: `$..$`. The body excludes the delimiters.
    Math(String),
    /// Display math: `$$..$$`. Stands alone — never absorbed into adjacent runs.
    Display(&'a str),
    /// Math-like text between `$..$` regions. Absorbed when sandwiched.
    Glue(&'a str),
    /// English prose. Forces a Math run to end.
    Prose(&'a str),
}

fn tokenize_spans(s: &str) -> Vec<Span<'_>> {
    let mut spans = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut text_start = 0;

    fn flush_text<'a>(out: &mut Vec<Span<'a>>, s: &'a str, lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }
        let chunk = &s[lo..hi];
        if is_math_glue(chunk) {
            out.push(Span::Glue(chunk));
        } else {
            out.push(Span::Prose(chunk));
        }
    }

    while i < bytes.len() {
        if bytes[i] == b'$' {
            // Display mode `$$..$$`?
            if i + 1 < bytes.len() && bytes[i + 1] == b'$' {
                if let Some(rel) = s[i + 2..].find("$$") {
                    flush_text(&mut spans, s, text_start, i);
                    spans.push(Span::Display(&s[i..i + 2 + rel + 2]));
                    i = i + 2 + rel + 2;
                    text_start = i;
                    continue;
                }
                // No closing — bail; treat the rest as prose
                break;
            }
            // Inline `$..$`?
            if let Some(rel) = s[i + 1..].find('$') {
                flush_text(&mut spans, s, text_start, i);
                let body = &s[i + 1..i + 1 + rel];
                spans.push(Span::Math(body.to_string()));
                i = i + 1 + rel + 1;
                text_start = i;
                continue;
            }
            // Unbalanced — treat literally
            i += 1;
        } else {
            i += 1;
        }
    }
    flush_text(&mut spans, s, text_start, bytes.len());
    spans
}

fn collapse_math_runs(spans: Vec<Span<'_>>) -> Vec<Span<'_>> {
    let mut out: Vec<Span<'_>> = Vec::with_capacity(spans.len());
    for span in spans {
        match span {
            Span::Math(body) => {
                // Absorb everything from the most recent "anchor" — either a
                // previous Math span or the start of the run after a Prose /
                // Display boundary — through the trailing Glue spans into one
                // continuous math region. This handles both:
                //   `f($x$)`     → leading `f(` Glue + Math `x` + trailing `)` Glue
                //   `$x$ = $5$`  → Math `x` + glue `=` + Math `5`
                let anchor = find_math_anchor(&out);
                let mut merged = String::new();
                let drained: Vec<Span<'_>> = out.drain(anchor..).collect();
                for s in drained {
                    match s {
                        Span::Math(b) => merged.push_str(&b),
                        Span::Glue(g) => merged.push_str(g),
                        Span::Prose(_) | Span::Display(_) => unreachable!(
                            "find_math_anchor returned an index past prose/display"
                        ),
                    }
                }
                merged.push_str(&body);
                out.push(Span::Math(merged));
            }
            other => out.push(other),
        }
    }

    // After the main pass: if the tail is `Math, Glue, Glue, ...`, fold the
    // trailing Glue into the Math region. (Trailing Prose stays prose.)
    while out.len() >= 2 {
        let last = out.len() - 1;
        if matches!(out[last], Span::Glue(_)) {
            // Find a preceding Math (skipping intermediate Glues — there
            // shouldn't be any after the main pass, but be defensive).
            let mut j = last;
            while j > 0 && matches!(out[j - 1], Span::Glue(_)) {
                j -= 1;
            }
            if j > 0 && matches!(out[j - 1], Span::Math(_)) {
                let glue_tail: String = out
                    .drain(j..)
                    .map(|s| match s {
                        Span::Glue(g) => g.to_string(),
                        _ => String::new(),
                    })
                    .collect();
                let math_idx = j - 1;
                if let Span::Math(prev) = &out[math_idx] {
                    let merged = format!("{prev}{glue_tail}");
                    out[math_idx] = Span::Math(merged);
                }
                continue;
            }
        }
        break;
    }
    out
}

/// Returns the index from which we should start absorbing into a new Math span.
///
/// - After a `Prose` boundary, anchor is `prose_idx + 1`, so any Glue that
///   followed the prose gets folded into the new Math run.
/// - After a `Display` boundary, anchor is `spans.len()` — display math
///   `$$..$$` produces a hard separator. Leading Glue between the display
///   region and the next inline Math must stay separate, otherwise the inline
///   `$..$` collides with the display closing `$$` to form `$$$`, which KaTeX
///   parses as `$$..$$` + a stray `$`.
/// - With no prior boundary, anchor is `0` so the new Math absorbs any leading
///   Glue (e.g., `f(` before the first `$x$`).
fn find_math_anchor(spans: &[Span<'_>]) -> usize {
    for (i, span) in spans.iter().enumerate().rev() {
        match span {
            Span::Prose(_) => return i + 1,
            Span::Display(_) => return spans.len(),
            _ => {}
        }
    }
    0
}

fn span_to_string(span: Span<'_>) -> String {
    match span {
        Span::Math(body) => format!("${body}$"),
        Span::Display(s) => s.to_string(),
        Span::Glue(s) | Span::Prose(s) => s.to_string(),
    }
}

/// True if `s` contains only characters/tokens that belong inside a math
/// region: operators, single-letter math symbols, function-call shapes,
/// digits, parens, whitespace. Multi-letter words (English prose) bail out.
fn is_math_glue(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        // Pure whitespace — definitely safe to merge.
        return true;
    }
    // Letter-runs are OK only if they look like a function name followed by
    // `(`. So we walk through and accept tokens.
    let bytes = trimmed.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            // Math operators and structural tokens. Note: `.` and `,` are
            // intentionally NOT here. A trailing period in `Find $x$.` should
            // close the sentence, not get pulled into the math region as a
            // bogus decimal point. Same for sentence commas like
            // `Find $x$, then $y$`.
            '+' | '-' | '*' | '/' | '=' | '<' | '>' | '^' | '_' |
            '(' | ')' | '[' | ']' | '{' | '}' | '!' | ' ' | '\t' => i += 1,
            // Backslash → LaTeX command. Allow `\cdot`, `\le`, `\to`, etc.
            '\\' => {
                i += 1;
                while i < bytes.len() && (bytes[i] as char).is_ascii_alphabetic() {
                    i += 1;
                }
            }
            // Digit run
            d if d.is_ascii_digit() => {
                while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                    i += 1;
                }
            }
            // Letter run: accept as math glue only if it's a single letter
            // (variable like `x`, `y`) or a function-call shape (`f(...)`).
            // Multi-letter words are prose; bail.
            l if l.is_ascii_alphabetic() => {
                let start = i;
                while i < bytes.len() && (bytes[i] as char).is_ascii_alphabetic() {
                    i += 1;
                }
                let word_len = i - start;
                if word_len == 1 {
                    // single-letter math symbol — fine
                } else if i < bytes.len() && bytes[i] == b'(' {
                    // function-call shape `name(...)` — treat as math token.
                    // We don't need to balance the paren: subsequent `(`/`)`/letters
                    // get accepted by their respective branches.
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
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

/// Convert a SymEngine expression string to LaTeX via SymEngine's native printer.
/// Handles sqrt, Pow, Rational, Derivative, Integral, Abs, sin/cos/log/exp, oo/pi/E/I.
pub fn expr_to_latex(expr_str: &str) -> Result<String, DslError> {
    let s = expr_str.trim();

    // Common-case shortcuts that SymEngine refuses to parse
    if s.is_empty() {
        return Ok(String::new());
    }

    // Normalize aliases SymEngine doesn't accept as built-in literals.
    // Without this, `infinity` parses as a free symbol named "infinity"
    // and renders literally instead of \infty.
    let s = normalize_aliases(s);

    match Expr::parse(&s) {
        Ok(expr) => Ok(post_process_latex(&expr.to_latex())),
        Err(_) => {
            // Word answer, identifier with hyphens, etc — passthrough
            Ok(s.to_string())
        }
    }
}

/// Translate user-written aliases to the spellings SymEngine recognizes.
fn normalize_aliases(s: &str) -> String {
    // Word-boundary replacements only — don't mangle `infinity_score` etc.
    let re_inf = regex::Regex::new(r"\b(infinity|Infinity|INF|Inf)\b").unwrap();
    re_inf.replace_all(s, "oo").to_string()
}

/// Light cosmetic touch-ups on SymEngine's LaTeX:
///   Abs\left(x\right) → \left|x\right|
fn post_process_latex(s: &str) -> String {
    let abs_re = regex::Regex::new(r"Abs\\left\((.*?)\\right\)").unwrap();
    abs_re.replace_all(s, r"\left|$1\right|").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_joins_math_around_equals() {
        // The bug: `$f(x)$ = $5$` rendered with `=` in body font.
        // Expected: one merged math region.
        assert_eq!(
            merge_adjacent_math_regions("Find $f(x)$ = $5$"),
            "Find $f(x) = 5$"
        );
    }

    #[test]
    fn merge_joins_inequality() {
        assert_eq!(
            merge_adjacent_math_regions("when $x$ < $3$"),
            "when $x < 3$"
        );
    }

    #[test]
    fn merge_joins_function_notation() {
        assert_eq!(
            merge_adjacent_math_regions("$f$($x$)"),
            "$f(x)$"
        );
    }

    #[test]
    fn merge_joins_multiple_runs() {
        assert_eq!(
            merge_adjacent_math_regions("$x$ + $y$ = $z$"),
            "$x + y = z$"
        );
    }

    #[test]
    fn merge_does_not_join_across_words() {
        // English word `for` is not math glue — leave the regions separate.
        assert_eq!(
            merge_adjacent_math_regions("$x$ for $y$"),
            "$x$ for $y$"
        );
    }

    #[test]
    fn merge_does_not_join_across_display_mode() {
        // `$$..$$` regions stand alone; never merge through them.
        assert_eq!(
            merge_adjacent_math_regions("$x$ + $$y + z$$ + $w$"),
            "$x$ + $$y + z$$ + $w$"
        );
    }

    #[test]
    fn merge_passes_through_text_with_no_math() {
        assert_eq!(merge_adjacent_math_regions("plain text"), "plain text");
    }

    #[test]
    fn merge_handles_negative_numbers() {
        assert_eq!(
            merge_adjacent_math_regions("$x$ = $-3$"),
            "$x = -3$"
        );
    }

    #[test]
    fn merge_keeps_trailing_period_outside_math() {
        // `Find $x = 3$.` — the period closes the sentence, not the equation.
        assert_eq!(
            merge_adjacent_math_regions("Find $x$ = $3$."),
            "Find $x = 3$."
        );
    }

    #[test]
    fn merge_keeps_sentence_comma_outside_math() {
        assert_eq!(
            merge_adjacent_math_regions("Find $x$, then $y$"),
            "Find $x$, then $y$"
        );
    }

    #[test]
    fn merge_handles_function_call_in_glue() {
        // `f($x$) = $5$` — `f(` is a function-call shape and should count as glue.
        assert_eq!(
            merge_adjacent_math_regions("f($x$) = $5$"),
            "$f(x) = 5$"
        );
    }
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
