//! KaTeX rendering validator
//!
//! Statically analyzes LaTeX strings for issues that would cause
//! KaTeX to render incorrectly (red error text, missing content, etc.).
//! Designed to match the KaTeX 0.16.9 configuration used in the frontend.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

/// Severity of a rendering issue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Will definitely render incorrectly (red error text in KaTeX)
    Error,
    /// May render incorrectly depending on context
    Warning,
}

/// A single rendering issue found in a LaTeX string
#[derive(Debug, Clone)]
pub struct KatexIssue {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
    /// Byte offset into the *math content* (inside delimiters) where the issue was found, if applicable
    pub position: Option<usize>,
}

impl std::fmt::Display for KatexIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sev = match self.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARN",
        };
        if let Some(pos) = self.position {
            write!(f, "[{}] {} (at byte {}): {}", sev, self.code, pos, self.message)
        } else {
            write!(f, "[{}] {}: {}", sev, self.code, self.message)
        }
    }
}

/// Result of validating a LaTeX string
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub issues: Vec<KatexIssue>,
}

impl ValidationResult {
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warning)
    }

    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == Severity::Error).count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == Severity::Warning).count()
    }
}

/// Validate and attempt to fix a LaTeX string. Returns the corrected string
/// if any fixes were applied, or `None` if unfixable (corrupted data) or already valid.
pub fn validate_and_fix(input: &str) -> (ValidationResult, Option<String>) {
    let result = validate_katex(input);
    if result.is_ok() {
        return (result, None);
    }

    let codes: Vec<&str> = result.issues.iter().map(|i| i.code).collect();

    // Unfixable: corrupted data must be deleted and regenerated
    if codes.contains(&"factory-code-leak") || codes.contains(&"malformed-data") {
        return (result, None);
    }

    let mut fixed = input.to_string();

    // Fix 1: Literal \n → space
    if codes.contains(&"literal-backslash-n") {
        fixed = fix_literal_backslash_n(&fixed);
    }

    // Fix 2: \$ → $ (escaped dollars used as math delimiters)
    if codes.contains(&"escaped-dollar-as-delim") {
        fixed = fixed.replace("\\$", "$");
    }

    // Fix 3: Newlines inside display blocks
    if codes.contains(&"newline-in-display") {
        fixed = fix_newlines_in_display(&fixed);
    }

    // Fix 4: Bare \begin{env}...\end{env} outside delimiters → wrap in $$...$$
    if codes.contains(&"bare-environment") {
        fixed = fix_bare_environments(&fixed);
    }

    // Fix 5: Missing delimiters — wrap individual math spans in $...$
    if codes.contains(&"missing-delimiters") {
        fixed = fix_missing_delimiters(&fixed);
    }

    if fixed != input {
        (result, Some(fixed))
    } else {
        (result, None)
    }
}

/// Prepare LaTeX content for rendering. Applies all safe auto-fixes:
/// - `\$` → `$` (escaped dollars used as math delimiters)
/// - `<br>` → space (HTML tags in content)
/// - Literal `\n` → space (double-escaped newlines, preserving `\nabla` etc.)
/// - Strip newlines inside `$$...$$` and `\[...\]` display blocks
/// - Wrap pure LaTeX (starting with `\`) in `$...$`
///
/// Used by the frontend `LatexRenderer` before calling `set_inner_html`.
pub fn prepare_for_rendering(content: &str) -> String {
    let mut result = content.trim().to_string();

    // Fix \$ → $
    if result.contains("\\$") {
        result = result.replace("\\$", "$");
    }

    // Fix <br> tags
    if result.contains("<br>") {
        result = result.replace("<br>", " ");
    }

    // Fix literal \n text
    result = fix_literal_backslash_n(&result);

    // Strip newlines inside display blocks
    result = fix_newlines_in_display(&result);

    // Wrap pure LaTeX in $...$
    if !result.contains('$') && !result.contains("\\(") && !result.contains("\\[") {
        let trimmed = result.trim();
        if trimmed.starts_with('\\') {
            return format!("${}$", trimmed);
        }
    }

    result
}

// ============================================================================
// Auto-fix functions (used by both validate_and_fix and prepare_for_rendering)
// ============================================================================

fn fix_literal_backslash_n(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'\\' && bytes[i + 1] == b'n' {
            // Check if this is a LaTeX \n... command
            let rest = &input[i + 1..];
            let is_latex_cmd = rest.len() > 1
                && rest.as_bytes()[1].is_ascii_alphabetic()
                && KNOWN_N_COMMANDS.iter().any(|cmd| rest.starts_with(cmd));
            let is_double_bs = i > 0 && bytes[i - 1] == b'\\';

            if is_latex_cmd || is_double_bs {
                result.push(bytes[i]);
            } else {
                result.push(b' ');
                i += 2;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }

    String::from_utf8(result).unwrap_or_else(|_| input.to_string())
}

/// Known LaTeX commands starting with \n (to avoid replacing them)
const KNOWN_N_COMMANDS: &[&str] = &[
    "nabla", "neq", "neg", "not", "nu", "newcommand", "newline", "nolimits",
    "notin", "nmid", "nless", "ngtr", "ncong", "nparallel", "nleq", "ngeq",
    "nprec", "nsucc", "nsim", "nexists", "nonumber", "notag", "ni", "natural",
];

fn fix_newlines_in_display(input: &str) -> String {
    let mut result = input.to_string();

    // Strip newlines inside $$...$$
    while let Some(start) = result.find("$$") {
        if let Some(end) = result[start + 2..].find("$$") {
            let abs_end = start + 2 + end;
            let content = &result[start + 2..abs_end];
            if content.contains('\n') {
                let fixed_content = content.replace('\n', " ");
                result = format!("{}$${}$${}", &result[..start], fixed_content, &result[abs_end + 2..]);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Strip newlines inside \[...\]
    let mut search = 0;
    while let Some(start) = result[search..].find("\\[") {
        let abs_start = search + start;
        if let Some(end) = result[abs_start + 2..].find("\\]") {
            let abs_end = abs_start + 2 + end;
            let content = &result[abs_start + 2..abs_end];
            if content.contains('\n') {
                let fixed_content = content.replace('\n', " ");
                result = format!("{}\\[{}\\]{}", &result[..abs_start], fixed_content, &result[abs_end + 2..]);
            }
            search = abs_start + 2 + end + 2;
        } else {
            break;
        }
    }

    result
}

fn fix_bare_environments(input: &str) -> String {
    let mut result = input.to_string();

    // Find \begin{env} and its matching \end{env}
    if let Some(cap) = BEGIN_RE.captures(&result) {
        let env_name = cap.get(1).unwrap().as_str();
        let begin_start = cap.get(0).unwrap().start();

        // Find matching \end{env_name}
        let end_pattern = format!("\\end{{{}}}", env_name);
        if let Some(end_pos) = result[begin_start..].find(&end_pattern) {
            let abs_end = begin_start + end_pos + end_pattern.len();

            // Check if already inside delimiters
            let before = &result[..begin_start];
            let mut in_math = false;
            let bytes = before.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] == b'$' && (i == 0 || bytes[i - 1] != b'\\') {
                    in_math = !in_math;
                    if i + 1 < bytes.len() && bytes[i + 1] == b'$' { i += 1; }
                }
                i += 1;
            }

            if !in_math {
                let inner = result[begin_start..abs_end].replace('\n', " ");
                result = format!("{}$${}$${}", &result[..begin_start], inner, &result[abs_end..]);
            }
        }
    }

    result
}

fn fix_missing_delimiters(input: &str) -> String {
    // Wrap content starting with \ in $...$  (LatexRenderer already does this,
    // but we make it explicit for consistency)
    let trimmed = input.trim();
    if trimmed.starts_with('\\') && !trimmed.contains('$') {
        return format!("${}$", trimmed);
    }
    // For mixed text + math, we'd need the span-level wrapper which is complex.
    // Return as-is for now — the LatexRenderer + global auto-render handles most cases.
    input.to_string()
}

// ============================================================================
// Supported / Unsupported command lists
// ============================================================================

lazy_static! {
    /// KaTeX 0.16.9 supported environments
    static ref SUPPORTED_ENVS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        // Matrix family
        for e in &["matrix", "pmatrix", "bmatrix", "vmatrix", "Vmatrix", "Bmatrix", "smallmatrix",
                    "matrix*", "pmatrix*", "bmatrix*", "vmatrix*", "Vmatrix*", "Bmatrix*"] {
            s.insert(*e);
        }
        // Alignment
        for e in &["equation", "equation*", "align", "align*", "gather", "gather*",
                    "alignat", "alignat*", "split", "gathered", "aligned", "alignedat"] {
            s.insert(*e);
        }
        // Cases
        for e in &["cases", "rcases", "dcases", "drcases"] {
            s.insert(*e);
        }
        // Array & misc
        for e in &["array", "darray", "CD", "subarray"] {
            s.insert(*e);
        }
        s
    };

    /// LaTeX commands that are NOT supported by KaTeX and will render as red text
    static ref UNSUPPORTED_COMMANDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for cmd in &[
            // Environments (would fail in \begin)
            "eqnarray", "eqnarray*", "multline", "multline*", "flalign", "flalign*",
            // Table/array features
            "multicolumn", "cline", "vline", "hfill", "hfil",
            // Macros
            "DeclareMathOperator", "newenvironment", "renewenvironment",
            // References
            "label", "ref", "eqref", "autoref", "nameref",
            // Layout
            "setlength", "rotatebox", "scalebox", "sideset", "displaylines",
            "buildrel", "shoveleft", "shoveright", "strut", "mbox",
            // Annotation
            "cancelto",
            // Fonts
            "textsc", "textsl", "sc", "bfseries", "em",
            // Conditionals
            "if", "else", "fi", "ifx", "or", "expandafter",
            // Package loading (MathJax-specific)
            "require",
            // Misc
            "bbox", "unicode", "uproot", "leftroot", "iddots",
            // siunitx
            "SI", "si",
        ] {
            s.insert(*cmd);
        }
        s
    };

    /// Commands that require exactly 2 brace-delimited arguments
    static ref TWO_ARG_COMMANDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for cmd in &["frac", "dfrac", "tfrac", "cfrac", "binom", "dbinom", "tbinom",
                      "overset", "underset", "stackrel", "textcolor", "colorbox"] {
            s.insert(*cmd);
        }
        s
    };

    static ref CMD_RE: Regex = Regex::new(r"\\([a-zA-Z]+\*?)").unwrap();
    static ref BEGIN_RE: Regex = Regex::new(r"\\begin\{([^}]+)\}").unwrap();
    static ref END_RE: Regex = Regex::new(r"\\end\{([^}]+)\}").unwrap();
    static ref DOUBLE_SCRIPT_RE: Regex = Regex::new(r"[^^_](\^[^{])\^|[^^_](_[^{])_").unwrap();
}

// ============================================================================
// Main validation entry point
// ============================================================================

/// Validate a `question_latex` or `solution_latex` string for KaTeX rendering issues.
///
/// Checks content *after* `prepare_for_rendering()` is applied, so it only
/// flags issues that will actually be visible to users.
pub fn validate_katex(input: &str) -> ValidationResult {
    let mut issues = Vec::new();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return ValidationResult { issues };
    }

    // Apply the same fixes the renderer applies, then validate what's left
    let prepared = prepare_for_rendering(trimmed);
    let checked = prepared.trim();

    // 1. Check for corrupted data (survives prepare_for_rendering)
    check_corrupted_data(checked, &mut issues);

    // 2. Check for bare \begin{env} outside delimiters
    check_bare_environments_in_input(checked, &mut issues);

    // 3. Check for literal \n text (should be gone after prepare, but verify)
    check_literal_backslash_n(checked, &mut issues);

    // 4. Check delimiter format
    check_delimiters(checked, &mut issues);

    // 5. Extract math segments and validate each
    let segments = extract_math_segments(checked);

    if segments.is_empty() && !checked.contains('$') && !checked.starts_with('\\') {
        // Pure text, no math — that's fine for word problems
        return ValidationResult { issues };
    }

    for (math, offset) in &segments {
        check_brace_balance(math, *offset, &mut issues);
        check_left_right_pairing(math, *offset, &mut issues);
        check_begin_end_matching(math, *offset, &mut issues);
        check_unsupported_commands(math, *offset, &mut issues);
        check_command_arity(math, *offset, &mut issues);
        check_double_scripts(math, *offset, &mut issues);
        check_bare_percent(math, *offset, &mut issues);
        check_empty_groups(math, *offset, &mut issues);
        check_trailing_double_backslash(math, *offset, &mut issues);
    }

    ValidationResult { issues }
}

// ============================================================================
// Delimiter checks
// ============================================================================

fn check_delimiters(input: &str, issues: &mut Vec<KatexIssue>) {
    // Count $ signs (not \$)
    let dollars: Vec<usize> = input
        .char_indices()
        .filter(|&(i, c)| c == '$' && (i == 0 || input.as_bytes()[i - 1] != b'\\'))
        .map(|(i, _)| i)
        .collect();

    // Check for \( \) and \[ \] delimiters (handled by global auto-render)
    let has_paren_delim = input.contains("\\(") && input.contains("\\)");
    let has_bracket_delim = input.contains("\\[") && input.contains("\\]");

    // Check for \$ used as math delimiters (should be $ without backslash)
    if input.contains("\\$") {
        // Strategy: find pairs of \$...\$ and check if the content between them
        // looks like math (contains operators, variables, LaTeX, etc.) vs currency.
        // Currency: \$500, \$1,234.56 — just a number after \$
        // Math delimiters: \$x + y\$, \$\frac{1}{2}\$, \$7:23\$
        let parts: Vec<&str> = input.split("\\$").collect();
        if parts.len() >= 3 {
            // Check odd-indexed parts (content between \$ pairs)
            for i in (1..parts.len()).step_by(2) {
                let between = parts[i].trim();
                // Currency: just digits, commas, dots, spaces
                let is_currency = between.chars().all(|c| c.is_ascii_digit() || c == ',' || c == '.' || c == ' ');
                if !is_currency && !between.is_empty() {
                    issues.push(KatexIssue {
                        severity: Severity::Error,
                        code: "escaped-dollar-as-delim",
                        message: "\\$ used as math delimiter — should be $ without backslash. \
                                  KaTeX sees \\$ as a literal dollar sign, not a delimiter."
                            .to_string(),
                        position: input.find("\\$").map(|p| p),
                    });
                    break;
                }
            }
        }

        // Even for currency \$, the backslash renders visibly — should just be $
        // e.g. "costs \$500" renders as "costs \$500" instead of "costs $500"
        if !issues.iter().any(|i| i.code == "escaped-dollar-as-delim") {
            issues.push(KatexIssue {
                severity: Severity::Warning,
                code: "escaped-dollar-literal",
                message: "\\$ renders as visible backslash-dollar (\\$) — \
                          use plain $ for currency display"
                    .to_string(),
                position: input.find("\\$").map(|p| p),
            });
        }
    }

    if dollars.is_empty() {
        if has_paren_delim || has_bracket_delim {
            // Valid alternative delimiters — not missing.
            // But check for newlines inside \[...\] blocks (breaks innerHTML rendering)
            check_newlines_in_display_blocks(input, issues);
            return;
        }

        // No delimiters at all — check if it looks like it should have them
        // LatexRenderer auto-wraps content starting with \ in $...$
        if looks_like_math(input) && !input.trim().starts_with('\\') {
            issues.push(KatexIssue {
                severity: Severity::Warning,
                code: "missing-delimiters",
                message: format!(
                    "Content appears to contain math but has no $ delimiters. \
                     The LatexRenderer will only auto-wrap if content starts with \\. \
                     Found: {:?}",
                    truncate(input, 60)
                ),
                position: None,
            });
        }
        return;
    }

    // Check for unmatched $ (odd count, ignoring $$)
    // Build a list of delimiter tokens
    let mut i = 0;
    let bytes = input.as_bytes();
    let mut delim_stack: Vec<(usize, bool)> = Vec::new(); // (position, is_display)

    while i < bytes.len() {
        if bytes[i] == b'$' && (i == 0 || bytes[i - 1] != b'\\') {
            let is_display = i + 1 < bytes.len() && bytes[i + 1] == b'$';
            if let Some((_, last_display)) = delim_stack.last() {
                if *last_display == is_display {
                    delim_stack.pop(); // matched
                } else {
                    // Mismatched: e.g., opening $$ but closing $
                    delim_stack.push((i, is_display));
                }
            } else {
                delim_stack.push((i, is_display));
            }
            i += if is_display { 2 } else { 1 };
        } else {
            i += 1;
        }
    }

    if !delim_stack.is_empty() {
        for (pos, is_display) in &delim_stack {
            let delim_type = if *is_display { "$$" } else { "$" };
            issues.push(KatexIssue {
                severity: Severity::Error,
                code: "unmatched-delimiter",
                message: format!("Unmatched {} delimiter", delim_type),
                position: Some(*pos),
            });
        }
    }

    // Check for empty math: $$ (inline empty, not display mode $$...$$)
    // This regex finds $$ that isn't part of $$...$$ display mode
    for (i, _) in input.match_indices("$$") {
        // Check if this is actually display-mode: look for matching $$
        // A simple heuristic: if the content between two $$ is non-empty, it's display mode
        // If we find $$ immediately (empty), it's an empty inline math
        if i + 2 < input.len() && input.as_bytes().get(i + 2) == Some(&b'$') {
            // $$$ — ambiguous, likely an error
            issues.push(KatexIssue {
                severity: Severity::Error,
                code: "ambiguous-dollars",
                message: "Three consecutive $ signs — ambiguous delimiter boundary".to_string(),
                position: Some(i),
            });
        }
    }

    // Check for newlines inside $$...$$ display blocks
    check_newlines_in_display_blocks(input, issues);

    // Check for bare LaTeX commands outside $ delimiters
    // e.g. "Find \frac{dy}{dx} using: $5x + 3y = -18$"
    //       ^^^^^^^^^^^^^^^^^^^^^ this part has no delimiters
    check_bare_latex_outside_delimiters(input, issues);
}

/// Check for LaTeX commands that appear outside any $ delimiter pair
fn check_bare_latex_outside_delimiters(input: &str, issues: &mut Vec<KatexIssue>) {
    let bytes = input.as_bytes();
    let mut in_math = false;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'$' && (i == 0 || bytes[i - 1] != b'\\') {
            in_math = !in_math;
            if i + 1 < bytes.len() && bytes[i + 1] == b'$' {
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }

        // Check for LaTeX command outside math mode
        if !in_math && bytes[i] == b'\\' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_alphabetic() {
            // Extract the command name
            let cmd_start = i + 1;
            let mut cmd_end = cmd_start;
            while cmd_end < bytes.len() && bytes[cmd_end].is_ascii_alphabetic() {
                cmd_end += 1;
            }
            let cmd = &input[cmd_start..cmd_end];

            // Check if it's a math command (not a text command like \text)
            let math_commands = [
                "frac", "dfrac", "tfrac", "sqrt", "sin", "cos", "tan", "log", "ln",
                "int", "sum", "prod", "lim", "nabla", "mathbf", "vec", "langle",
                "rangle", "cdot", "times", "left", "right", "begin", "displaystyle",
            ];
            if math_commands.contains(&cmd) {
                issues.push(KatexIssue {
                    severity: Severity::Warning,
                    code: "bare-latex-outside-delimiters",
                    message: format!(
                        "\\{} appears outside $ delimiters — won't render as math",
                        cmd
                    ),
                    position: Some(i),
                });
                return; // Report once
            }
        }
        i += 1;
    }
}

/// Check if a string looks like it contains math (heuristic)
fn looks_like_math(s: &str) -> bool {
    // LaTeX commands
    s.contains("\\frac") || s.contains("\\sqrt") || s.contains("\\sin")
        || s.contains("\\cos") || s.contains("\\sum") || s.contains("\\int")
        || s.contains("\\lim") || s.contains("\\begin{") || s.contains("\\text{")
        || s.contains("\\mathbf") || s.contains("\\nabla") || s.contains("\\vec")
        || s.contains("\\langle") || s.contains("\\cdot")
        // Subscript/superscript patterns
        || s.contains('^') || s.contains("_{")
        // Unicode math symbols
        || s.contains('ℝ') || s.contains('ℤ') || s.contains('ℂ') || s.contains('ℕ')
        || s.contains('ℚ') || s.contains('∈') || s.contains('∞') || s.contains('∑')
        || s.contains('∫') || s.contains('π')
}

/// Check for corrupted data: factory code leaks and malformed matrix notation.
/// These are data quality issues, not rendering issues — the problems need deletion/regeneration.
fn check_corrupted_data(input: &str, issues: &mut Vec<KatexIssue>) {
    // Julia DiagramObj code leaked into question_latex (~2500 problems in production)
    if input.contains("DiagramObj(") || input.contains("Dict{Symbol") || input.contains("stroke_width") {
        issues.push(KatexIssue {
            severity: Severity::Error,
            code: "factory-code-leak",
            message: "Raw Julia/factory code in question_latex — problem must be deleted and regenerated"
                .to_string(),
            position: input.find("DiagramObj(").or_else(|| input.find("Dict{")).map(|p| p),
        });
    }

    // Malformed matrix notation: [[-, 3, /, /, 1]] (~15500 problems in production)
    // This is garbled fraction data from a factory script bug
    if input.contains("/, /,") || input.contains("/, /, ") {
        issues.push(KatexIssue {
            severity: Severity::Error,
            code: "malformed-data",
            message: "Garbled matrix/fraction data ([[-, N, /, /, 1]] pattern) — \
                      problem must be deleted and regenerated"
                .to_string(),
            position: input.find("/, /").map(|p| p),
        });
    }

    // HTML tags in content (e.g. <br> from a broken pipeline)
    if input.contains("<br>") || input.contains("<br/>") || input.contains("<br />") {
        issues.push(KatexIssue {
            severity: Severity::Error,
            code: "html-in-content",
            message: "HTML tags (<br>) in LaTeX content — should use LaTeX line breaks (\\\\) instead"
                .to_string(),
            position: input.find("<br").map(|p| p),
        });
    }

    // Double-slash // inside matrix notation [[..., //, ...]] is malformed
    // But standalone // like "-3//4" is legitimate fraction notation in some contexts
    if input.contains("//") && !input.contains("http") && input.contains("[[") {
        issues.push(KatexIssue {
            severity: Severity::Warning,
            code: "double-slash-fraction",
            message: "// in matrix notation — likely a malformed fraction"
                .to_string(),
            position: input.find("//").map(|p| p),
        });
    }
}

/// Check for \begin{env} blocks that are outside any math delimiter.
/// These need to be wrapped in $$...$$ to render.
fn check_bare_environments_in_input(input: &str, issues: &mut Vec<KatexIssue>) {
    for cap in BEGIN_RE.captures_iter(input) {
        let env_name = cap.get(1).unwrap().as_str();
        let pos = cap.get(0).unwrap().start();
        let before = &input[..pos];

        // Check if this \begin is inside any delimiter by tracking open/close state
        let mut in_math = false;
        let bytes = before.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'$' && (i == 0 || bytes[i - 1] != b'\\') {
                // Toggle math mode (handles both $ and $$)
                if i + 1 < bytes.len() && bytes[i + 1] == b'$' {
                    in_math = !in_math;
                    i += 2;
                } else {
                    in_math = !in_math;
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        let in_paren = before.matches("\\(").count() > before.matches("\\)").count();
        let in_bracket = before.matches("\\[").count() > before.matches("\\]").count();

        if !in_math && !in_paren && !in_bracket {
            issues.push(KatexIssue {
                severity: Severity::Error,
                code: "bare-environment",
                message: format!(
                    "\\begin{{{}}} is not inside any math delimiter — \
                     wrap in $$...$$",
                    env_name
                ),
                position: Some(pos),
            });
            return; // Only report once
        }
    }
}

/// Check for literal `\n` text in content (double-escaped newlines from factory pipeline).
/// These render as visible "\n" text instead of line breaks.
fn check_literal_backslash_n(input: &str, issues: &mut Vec<KatexIssue>) {
    let bytes = input.as_bytes();
    for i in 0..bytes.len().saturating_sub(1) {
        if bytes[i] == b'\\' && bytes[i + 1] == b'n' {
            // Make sure this isn't part of a LaTeX command like \nabla, \neq, \not, \nu, etc.
            let is_cmd = if i + 2 < bytes.len() {
                bytes[i + 2].is_ascii_alphabetic()
            } else {
                false
            };
            // Also skip if preceded by another backslash (\\n = LaTeX linebreak + n)
            let is_double_backslash = i > 0 && bytes[i - 1] == b'\\';

            if !is_cmd && !is_double_backslash {
                issues.push(KatexIssue {
                    severity: Severity::Warning,
                    code: "literal-backslash-n",
                    message: "Literal \\n text found — renders as visible text instead of a \
                              line break. Likely a double-escaping bug in the factory pipeline."
                        .to_string(),
                    position: Some(i),
                });
                // Only report once per problem
                return;
            }
        }
    }
}

/// Check for newlines inside display math blocks ($$...$$ or \[...\])
/// When these are set via innerHTML with <br> tags, the newlines break KaTeX parsing.
fn check_newlines_in_display_blocks(input: &str, issues: &mut Vec<KatexIssue>) {
    // Check $$...$$ blocks
    let mut search_from = 0;
    while let Some(start) = input[search_from..].find("$$") {
        let abs_start = search_from + start;
        if let Some(end) = input[abs_start + 2..].find("$$") {
            let content = &input[abs_start + 2..abs_start + 2 + end];
            if content.contains('\n') {
                issues.push(KatexIssue {
                    severity: Severity::Error,
                    code: "newline-in-display",
                    message: "Newline inside $$...$$ block — when rendered via innerHTML \
                              with <br> tags, KaTeX will fail to parse this. \
                              Remove newlines or use a single-line format."
                        .to_string(),
                    position: Some(abs_start),
                });
            }
            search_from = abs_start + 2 + end + 2;
        } else {
            break;
        }
    }

    // Check \[...\] blocks
    search_from = 0;
    while let Some(start) = input[search_from..].find("\\[") {
        let abs_start = search_from + start;
        if let Some(end) = input[abs_start + 2..].find("\\]") {
            let content = &input[abs_start + 2..abs_start + 2 + end];
            if content.contains('\n') {
                issues.push(KatexIssue {
                    severity: Severity::Error,
                    code: "newline-in-display",
                    message: "Newline inside \\[...\\] block — when rendered via innerHTML \
                              with <br> tags, KaTeX will fail to parse this. \
                              Remove newlines or use a single-line format."
                        .to_string(),
                    position: Some(abs_start),
                });
            }
            search_from = abs_start + 2 + end + 2;
        } else {
            break;
        }
    }
}

// ============================================================================
// Extract math segments from delimited content
// ============================================================================

/// Extract (math_content, byte_offset) pairs from a string with $ delimiters
fn extract_math_segments(input: &str) -> Vec<(String, usize)> {
    let mut segments = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'$' && (i == 0 || bytes[i - 1] != b'\\') {
            let is_display = i + 1 < bytes.len() && bytes[i + 1] == b'$';
            let start = if is_display { i + 2 } else { i + 1 };
            let close_pat = if is_display { "$$" } else { "$" };

            // Find matching close
            if let Some(rel_end) = find_closing_delimiter(&input[start..], close_pat) {
                let end = start + rel_end;
                let content = &input[start..end];
                if !content.trim().is_empty() {
                    segments.push((content.to_string(), start));
                }
                i = end + close_pat.len();
            } else {
                // No closing delimiter — already flagged by check_delimiters
                break;
            }
        } else {
            i += 1;
        }
    }

    // Also extract \(...\) and \[...\] segments
    let mut search_from = 0;
    for (open, close) in &[("\\(", "\\)"), ("\\[", "\\]")] {
        search_from = 0;
        while let Some(start_rel) = input[search_from..].find(open) {
            let abs_start = search_from + start_rel + open.len();
            if let Some(end_rel) = input[abs_start..].find(close) {
                let content = &input[abs_start..abs_start + end_rel];
                if !content.trim().is_empty() {
                    segments.push((content.to_string(), abs_start));
                }
                search_from = abs_start + end_rel + close.len();
            } else {
                break;
            }
        }
    }

    // If no segments found but content starts with \, the LatexRenderer wraps it
    if segments.is_empty() && input.trim().starts_with('\\') {
        segments.push((input.trim().to_string(), 0));
    }

    segments
}

fn find_closing_delimiter(s: &str, delim: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let delim_bytes = delim.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if i + delim_bytes.len() <= bytes.len()
            && &bytes[i..i + delim_bytes.len()] == delim_bytes
            && (i == 0 || bytes[i - 1] != b'\\')
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

// ============================================================================
// Structural checks (run on each math segment)
// ============================================================================

fn check_brace_balance(math: &str, offset: usize, issues: &mut Vec<KatexIssue>) {
    let mut depth: i32 = 0;
    let bytes = math.as_bytes();

    for i in 0..bytes.len() {
        if bytes[i] == b'{' && (i == 0 || bytes[i - 1] != b'\\') {
            depth += 1;
        } else if bytes[i] == b'}' && (i == 0 || bytes[i - 1] != b'\\') {
            depth -= 1;
            if depth < 0 {
                issues.push(KatexIssue {
                    severity: Severity::Error,
                    code: "unmatched-brace",
                    message: "Unexpected closing brace } without matching opening brace".to_string(),
                    position: Some(offset + i),
                });
                return;
            }
        }
    }

    if depth > 0 {
        issues.push(KatexIssue {
            severity: Severity::Error,
            code: "unmatched-brace",
            message: format!("{} unclosed {{ brace(s)", depth),
            position: None,
        });
    }
}

fn check_left_right_pairing(math: &str, offset: usize, issues: &mut Vec<KatexIssue>) {
    let mut left_count = 0i32;

    // Track positions for better error messages
    let mut last_left_pos = 0;

    for m in Regex::new(r"\\(left|right|middle)").unwrap().find_iter(math) {
        let cmd = &math[m.start() + 1..m.end()];
        match cmd {
            "left" => {
                left_count += 1;
                last_left_pos = m.start();
            }
            "right" => {
                left_count -= 1;
                if left_count < 0 {
                    issues.push(KatexIssue {
                        severity: Severity::Error,
                        code: "unmatched-right",
                        message: "\\right without matching \\left".to_string(),
                        position: Some(offset + m.start()),
                    });
                    return;
                }
            }
            "middle" => {
                if left_count == 0 {
                    issues.push(KatexIssue {
                        severity: Severity::Error,
                        code: "orphan-middle",
                        message: "\\middle must appear between \\left and \\right".to_string(),
                        position: Some(offset + m.start()),
                    });
                }
            }
            _ => {}
        }
    }

    if left_count > 0 {
        issues.push(KatexIssue {
            severity: Severity::Error,
            code: "unmatched-left",
            message: format!("{} \\left without matching \\right", left_count),
            position: Some(offset + last_left_pos),
        });
    }
}

fn check_begin_end_matching(math: &str, offset: usize, issues: &mut Vec<KatexIssue>) {
    let mut stack: Vec<(String, usize)> = Vec::new();

    // Collect all \begin and \end
    for cap in BEGIN_RE.captures_iter(math) {
        let env_name = cap.get(1).unwrap().as_str().to_string();
        let pos = cap.get(0).unwrap().start();

        // Check if environment is supported
        if !SUPPORTED_ENVS.contains(env_name.as_str()) {
            issues.push(KatexIssue {
                severity: Severity::Error,
                code: "unsupported-env",
                message: format!(
                    "Environment '{}' is not supported by KaTeX",
                    env_name
                ),
                position: Some(offset + pos),
            });
        }

        stack.push((env_name, pos));
    }

    // Now check \end tags match in order
    let begins: Vec<(String, usize)> = stack;
    let mut remaining_begins: Vec<&(String, usize)> = begins.iter().collect();

    for cap in END_RE.captures_iter(math) {
        let env_name = cap.get(1).unwrap().as_str();
        let pos = cap.get(0).unwrap().start();

        if let Some(last) = remaining_begins.last() {
            if last.0 != env_name {
                issues.push(KatexIssue {
                    severity: Severity::Error,
                    code: "env-mismatch",
                    message: format!(
                        "\\end{{{}}} does not match \\begin{{{}}}",
                        env_name, last.0
                    ),
                    position: Some(offset + pos),
                });
            }
            remaining_begins.pop();
        } else {
            issues.push(KatexIssue {
                severity: Severity::Error,
                code: "orphan-end",
                message: format!("\\end{{{}}} without matching \\begin", env_name),
                position: Some(offset + pos),
            });
        }
    }

    for (env_name, pos) in &remaining_begins {
        issues.push(KatexIssue {
            severity: Severity::Error,
            code: "orphan-begin",
            message: format!("\\begin{{{}}} without matching \\end", env_name),
            position: Some(offset + *pos),
        });
    }
}

fn check_unsupported_commands(math: &str, offset: usize, issues: &mut Vec<KatexIssue>) {
    for cap in CMD_RE.captures_iter(math) {
        let cmd = cap.get(1).unwrap().as_str();
        let pos = cap.get(0).unwrap().start();

        // Skip \begin, \end, \left, \right — handled separately
        if cmd == "begin" || cmd == "end" || cmd == "left" || cmd == "right" || cmd == "middle" {
            continue;
        }

        if UNSUPPORTED_COMMANDS.contains(cmd) {
            issues.push(KatexIssue {
                severity: Severity::Error,
                code: "unsupported-cmd",
                message: format!(
                    "\\{} is not supported by KaTeX — will render as red text",
                    cmd
                ),
                position: Some(offset + pos),
            });
        }
    }
}

fn check_command_arity(math: &str, offset: usize, issues: &mut Vec<KatexIssue>) {
    for cap in CMD_RE.captures_iter(math) {
        let cmd = cap.get(1).unwrap().as_str();
        let cmd_end = cap.get(0).unwrap().end();

        if !TWO_ARG_COMMANDS.contains(cmd) {
            continue;
        }

        // After the command, expect two brace groups: {arg1}{arg2}
        let rest = &math[cmd_end..];
        let rest_trimmed = rest.trim_start();

        if rest_trimmed.is_empty() || !rest_trimmed.starts_with('{') {
            issues.push(KatexIssue {
                severity: Severity::Error,
                code: "missing-args",
                message: format!(
                    "\\{} requires 2 brace-delimited arguments but first {{ is missing",
                    cmd
                ),
                position: Some(offset + cmd_end),
            });
            continue;
        }

        // Find end of first brace group
        if let Some(first_end) = find_brace_group_end(rest_trimmed) {
            let after_first = rest_trimmed[first_end..].trim_start();
            if after_first.is_empty() || !after_first.starts_with('{') {
                issues.push(KatexIssue {
                    severity: Severity::Error,
                    code: "missing-args",
                    message: format!(
                        "\\{} requires 2 arguments but only 1 brace group found",
                        cmd
                    ),
                    position: Some(offset + cmd_end),
                });
            }
        }
    }
}

/// Find the end of a brace group starting at `{`, returns position after the closing `}`
fn find_brace_group_end(s: &str) -> Option<usize> {
    if !s.starts_with('{') {
        return None;
    }
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '{' if i == 0 || s.as_bytes()[i - 1] != b'\\' => depth += 1,
            '}' if i == 0 || s.as_bytes()[i - 1] != b'\\' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
    }
    None
}

fn check_double_scripts(math: &str, offset: usize, issues: &mut Vec<KatexIssue>) {
    // Detect x^a^b or x_a_b (double superscript/subscript without braces)
    let bytes = math.as_bytes();
    let mut last_script: Option<(u8, usize)> = None; // ('^' or '_', position)
    let mut consumed_arg = false; // whether the single-char arg after a script has been consumed
    let mut in_brace = 0i32;

    for i in 0..bytes.len() {
        match bytes[i] {
            b'{' if i == 0 || bytes[i - 1] != b'\\' => {
                in_brace += 1;
                last_script = None; // inside braces, reset
            }
            b'}' if i == 0 || bytes[i - 1] != b'\\' => {
                in_brace -= 1;
            }
            b'^' | b'_' if in_brace == 0 => {
                if let Some((prev_char, _prev_pos)) = last_script {
                    if prev_char == bytes[i] {
                        let script_type = if bytes[i] == b'^' { "superscript" } else { "subscript" };
                        issues.push(KatexIssue {
                            severity: Severity::Error,
                            code: "double-script",
                            message: format!(
                                "Double {} (e.g. x{}a{}b) — use braces: x{}{{a{}b}}",
                                script_type,
                                bytes[i] as char, bytes[i] as char,
                                bytes[i] as char, bytes[i] as char,
                            ),
                            position: Some(offset + i),
                        });
                        last_script = None;
                        consumed_arg = false;
                        continue;
                    }
                }
                // Check if next char is { — if so, the script has a brace group (ok)
                if i + 1 < bytes.len() && bytes[i + 1] == b'{' {
                    last_script = None;
                } else {
                    last_script = Some((bytes[i], i));
                    consumed_arg = false;
                }
            }
            b' ' | b'\\' | b'+' | b'-' | b'=' | b',' | b'(' | b')' | b'|' | b'&'
                if in_brace == 0 =>
            {
                // Operators, delimiters, and commands reset — new expression context
                last_script = None;
                consumed_arg = false;
            }
            _ if in_brace == 0 => {
                if last_script.is_some() && !consumed_arg {
                    // This char is the single-char argument to the script (e.g. the '2' in x^2)
                    consumed_arg = true;
                }
            }
            _ => {}
        }
    }
}

fn check_bare_percent(math: &str, offset: usize, issues: &mut Vec<KatexIssue>) {
    for (i, c) in math.char_indices() {
        if c == '%' && (i == 0 || math.as_bytes()[i - 1] != b'\\') {
            issues.push(KatexIssue {
                severity: Severity::Error,
                code: "bare-percent",
                message: "Bare % character — KaTeX does not support LaTeX comments. Use \\% for a literal percent sign".to_string(),
                position: Some(offset + i),
            });
        }
    }
}

fn check_empty_groups(math: &str, offset: usize, issues: &mut Vec<KatexIssue>) {
    // Check for \frac{}{} or \sqrt{} — likely mistakes
    for cap in CMD_RE.captures_iter(math) {
        let cmd = cap.get(1).unwrap().as_str();
        let cmd_end = cap.get(0).unwrap().end();

        if cmd == "sqrt" {
            let rest = &math[cmd_end..];
            let rest_trimmed = rest.trim_start();
            if rest_trimmed.starts_with("{}") {
                issues.push(KatexIssue {
                    severity: Severity::Warning,
                    code: "empty-sqrt",
                    message: "\\sqrt{} has empty content — likely a mistake".to_string(),
                    position: Some(offset + cmd_end),
                });
            }
        }

        if TWO_ARG_COMMANDS.contains(cmd) {
            let rest = &math[cmd_end..];
            let rest_trimmed = rest.trim_start();
            // Check for two consecutive empty brace groups
            if rest_trimmed.starts_with("{}{}") {
                issues.push(KatexIssue {
                    severity: Severity::Warning,
                    code: "empty-args",
                    message: format!("\\{} has empty arguments — likely a mistake", cmd),
                    position: Some(offset + cmd_end),
                });
            }
        }
    }
}

fn check_trailing_double_backslash(math: &str, _offset: usize, issues: &mut Vec<KatexIssue>) {
    // Check for trailing \\ at the end of environments (creates empty row)
    // Only warn inside environments
    for cap in BEGIN_RE.captures_iter(math) {
        let env_name = cap.get(1).unwrap().as_str();
        let begin_end = cap.get(0).unwrap().end();

        // Find matching \end
        let end_pattern = format!("\\end{{{}}}", env_name);
        if let Some(end_pos) = math[begin_end..].find(&end_pattern) {
            let env_content = &math[begin_end..begin_end + end_pos];
            let trimmed = env_content.trim_end();
            if trimmed.ends_with(r"\\") {
                issues.push(KatexIssue {
                    severity: Severity::Warning,
                    code: "trailing-newline",
                    message: format!(
                        "Trailing \\\\ at end of {} environment creates an empty row",
                        env_name
                    ),
                    position: None,
                });
            }
        }
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        // Find a valid char boundary at or before max_len
        let mut end = max_len;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", &s[..end])
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to check if a specific issue code was found
    fn has_issue(result: &ValidationResult, code: &str) -> bool {
        result.issues.iter().any(|i| i.code == code)
    }

    fn has_error(result: &ValidationResult, code: &str) -> bool {
        result.issues.iter().any(|i| i.code == code && i.severity == Severity::Error)
    }

    fn has_warning(result: &ValidationResult, code: &str) -> bool {
        result.issues.iter().any(|i| i.code == code && i.severity == Severity::Warning)
    }

    // ====================================================================
    // Valid inputs — should produce no issues
    // ====================================================================

    #[test]
    fn valid_simple_inline() {
        let r = validate_katex("$2x + 3x$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_mixed_text_and_math() {
        let r = validate_katex("Simplify: $2x + 3x$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_display_mode() {
        let r = validate_katex("$$\\frac{x^2 - 9}{x + 3}$$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_fraction() {
        let r = validate_katex("$\\frac{1}{x}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_nested_fractions() {
        let r = validate_katex("$\\frac{\\frac{1}{2}}{\\frac{3}{4}}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_sqrt() {
        let r = validate_katex("$\\sqrt{x^2 + 1}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_nth_root() {
        let r = validate_katex("$\\sqrt[3]{8}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_left_right() {
        let r = validate_katex("$\\left(\\frac{1}{2}\\right)$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_left_right_invisible() {
        let r = validate_katex("$\\left.\\frac{dy}{dx}\\right|_{x=0}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_trig() {
        let r = validate_katex("$\\sin^2(x) + \\cos^2(x) = 1$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_matrix() {
        let r = validate_katex("$\\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\end{pmatrix}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_cases() {
        let r = validate_katex("$f(x) = \\begin{cases} x & x \\geq 0 \\\\ -x & x < 0 \\end{cases}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_align() {
        let r = validate_katex("$$\\begin{align} x &= 1 \\\\ y &= 2 \\end{align}$$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_text_command() {
        let r = validate_katex("$\\text{Factor: } x^2 - 4$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_word_problem() {
        let r = validate_katex("A car travels 60 mph for 3 hours. How far does it go?");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_pure_latex_no_dollars() {
        // LatexRenderer wraps this in $ automatically
        let r = validate_katex("\\frac{1}{2}");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_multiple_math_segments() {
        let r = validate_katex("If $x = 3$ and $y = 4$, find $x + y$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_escaped_braces() {
        let r = validate_katex("$\\lbrace 1, 2, 3 \\rbrace$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_subscript_superscript() {
        let r = validate_katex("$x_1^{2} + x_2^{3}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_implicit_diff_factory() {
        // Actual pattern from calculus factory scripts
        let r = validate_katex("Find $\\frac{dy}{dx}$ for $x^2+y^2=25$ at $(3,4)$.");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_trig_identity_factory() {
        // Actual pattern from trig identity factory scripts
        let r = validate_katex("Simplify: $5\\sin^2(2x) + 5\\cos^2(2x)$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_solution_steps() {
        // Solution with multiple math segments
        let r = validate_katex("Use $\\sin^2 + \\cos^2 = 1$: $= 5$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_percent_escaped() {
        let r = validate_katex("$15\\%$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_color() {
        let r = validate_katex("$\\color{red}{x^2}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_operatorname() {
        let r = validate_katex("$\\operatorname{lcm}(a, b)$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_boxed() {
        let r = validate_katex("$\\boxed{x = 5}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Delimiter errors
    // ====================================================================

    #[test]
    fn error_unmatched_dollar_open() {
        let r = validate_katex("Simplify: $x + 2");
        assert!(has_error(&r, "unmatched-delimiter"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_unmatched_dollar_close() {
        let r = validate_katex("Simplify: x + 2$");
        assert!(has_error(&r, "unmatched-delimiter"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_unmatched_display_dollar() {
        let r = validate_katex("$$\\frac{1}{2}$");
        assert!(has_issue(&r, "unmatched-delimiter") || has_issue(&r, "ambiguous-dollars"),
                "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Missing delimiters
    // ====================================================================

    #[test]
    fn warn_math_without_delimiters() {
        let r = validate_katex("Simplify: x^2 + 3x");
        assert!(has_warning(&r, "missing-delimiters"), "issues: {:?}", r.issues);
    }

    #[test]
    fn no_warn_plain_word_problem() {
        let r = validate_katex("How many apples are in the basket?");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Brace balance
    // ====================================================================

    #[test]
    fn error_unclosed_brace() {
        let r = validate_katex("$\\frac{x{$");
        assert!(has_error(&r, "unmatched-brace"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_extra_closing_brace() {
        let r = validate_katex("$x^{2}}$");
        assert!(has_error(&r, "unmatched-brace"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_deeply_nested_unclosed() {
        let r = validate_katex("$\\frac{\\frac{1}{2}{3}$");
        assert!(has_error(&r, "unmatched-brace") || has_error(&r, "missing-args"),
                "issues: {:?}", r.issues);
    }

    // ====================================================================
    // \left / \right pairing
    // ====================================================================

    #[test]
    fn error_left_without_right() {
        let r = validate_katex("$\\left(\\frac{1}{2}$");
        assert!(has_error(&r, "unmatched-left"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_right_without_left() {
        let r = validate_katex("$\\frac{1}{2}\\right)$");
        assert!(has_error(&r, "unmatched-right"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_middle_without_left() {
        let r = validate_katex("$x \\middle| y$");
        assert!(has_error(&r, "orphan-middle"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // \begin / \end matching
    // ====================================================================

    #[test]
    fn error_begin_without_end() {
        let r = validate_katex("$\\begin{pmatrix} 1 & 2$");
        assert!(has_error(&r, "orphan-begin"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_end_without_begin() {
        let r = validate_katex("$1 & 2 \\end{pmatrix}$");
        assert!(has_error(&r, "orphan-end"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_mismatched_envs() {
        let r = validate_katex("$\\begin{pmatrix} 1 & 2 \\end{bmatrix}$");
        assert!(has_error(&r, "env-mismatch"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_unsupported_environment() {
        let r = validate_katex("$\\begin{eqnarray} x &=& 1 \\end{eqnarray}$");
        assert!(has_error(&r, "unsupported-env"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_tabular_env() {
        let r = validate_katex("$\\begin{tabular}{cc} a & b \\end{tabular}$");
        assert!(has_error(&r, "unsupported-env"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Unsupported commands
    // ====================================================================

    #[test]
    fn error_declare_math_operator() {
        let r = validate_katex("$\\DeclareMathOperator{\\lcm}{lcm}$");
        assert!(has_error(&r, "unsupported-cmd"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_label() {
        let r = validate_katex("$x = 1 \\label{eq:one}$");
        assert!(has_error(&r, "unsupported-cmd"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_textsc() {
        let r = validate_katex("$\\textsc{hello}$");
        assert!(has_error(&r, "unsupported-cmd"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_mbox() {
        let r = validate_katex("$\\mbox{text}$");
        assert!(has_error(&r, "unsupported-cmd"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_require() {
        let r = validate_katex("$\\require{cancel}$");
        assert!(has_error(&r, "unsupported-cmd"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Command arity
    // ====================================================================

    #[test]
    fn error_frac_one_arg() {
        let r = validate_katex("$\\frac{x}$");
        assert!(has_error(&r, "missing-args"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_frac_no_args() {
        let r = validate_katex("$\\frac x$");
        assert!(has_error(&r, "missing-args"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_binom_one_arg() {
        let r = validate_katex("$\\binom{n}$");
        assert!(has_error(&r, "missing-args"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_overset_one_arg() {
        let r = validate_katex("$\\overset{a}$");
        assert!(has_error(&r, "missing-args"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Double scripts
    // ====================================================================

    #[test]
    fn error_double_superscript() {
        let r = validate_katex("$x^2^3$");
        assert!(has_error(&r, "double-script"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_double_subscript() {
        let r = validate_katex("$x_1_2$");
        assert!(has_error(&r, "double-script"), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_mixed_super_sub() {
        // x^2_3 is valid (superscript then subscript)
        let r = validate_katex("$x^2_3$");
        assert!(!has_error(&r, "double-script"), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_braced_scripts() {
        let r = validate_katex("$x^{2^3}$");
        assert!(!has_error(&r, "double-script"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Bare percent
    // ====================================================================

    #[test]
    fn error_bare_percent() {
        let r = validate_katex("$x = 50%$");
        assert!(has_error(&r, "bare-percent"), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_escaped_percent_in_test() {
        let r = validate_katex("$50\\%$");
        assert!(!has_issue(&r, "bare-percent"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Empty groups
    // ====================================================================

    #[test]
    fn warn_empty_sqrt() {
        let r = validate_katex("$\\sqrt{}$");
        assert!(has_warning(&r, "empty-sqrt"), "issues: {:?}", r.issues);
    }

    #[test]
    fn warn_empty_frac() {
        let r = validate_katex("$\\frac{}{}$");
        assert!(has_warning(&r, "empty-args"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Trailing double backslash in environments
    // ====================================================================

    #[test]
    fn warn_trailing_newline_in_matrix() {
        let r = validate_katex("$\\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\\\ \\end{pmatrix}$");
        assert!(has_warning(&r, "trailing-newline"), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_no_trailing_newline() {
        let r = validate_katex("$\\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\end{pmatrix}$");
        assert!(!has_issue(&r, "trailing-newline"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Real-world factory patterns (regression tests)
    // ====================================================================

    #[test]
    fn factory_exponent_rules() {
        let r = validate_katex("Simplify: $x^{3} \\cdot x^{4}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_factoring() {
        let r = validate_katex("Factor out the greatest common factor: $6x^2 + 9x$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_derivative() {
        let r = validate_katex("Find $\\frac{d}{dx}\\left(x^3 + 2x\\right)$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_solution_multiline() {
        let r = validate_katex(
            "Product rule: $x^a \\cdot x^b = x^{a+b}$"
        );
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_determinant() {
        let r = validate_katex(
            "If $A = \\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\end{pmatrix}$, find $\\det(A)$"
        );
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_set_notation() {
        let r = validate_katex("$\\lbrace 1, 2, 3 \\rbrace$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_integral() {
        let r = validate_katex("Evaluate: $\\int_0^1 x^2 \\, dx$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_limit() {
        let r = validate_katex("$\\lim_{x \\to \\infty} \\frac{1}{x}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_summation() {
        let r = validate_katex("$\\sum_{n=1}^{\\infty} \\frac{1}{n^2}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_inequality_solution() {
        let r = validate_katex("$2x + 3 \\leq 7$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_compound_inequality() {
        let r = validate_katex("Solve: $-3 < 2x + 1 \\leq 5$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_absolute_value() {
        let r = validate_katex("$|x - 3| = 5$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn factory_binomial() {
        let r = validate_katex("$\\binom{n}{k} = \\frac{n!}{k!(n-k)!}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Edge cases
    // ====================================================================

    #[test]
    fn empty_input() {
        let r = validate_katex("");
        assert!(r.is_ok());
    }

    #[test]
    fn whitespace_only() {
        let r = validate_katex("   ");
        assert!(r.is_ok());
    }

    #[test]
    fn single_dollar() {
        let r = validate_katex("$");
        assert!(has_error(&r, "unmatched-delimiter"), "issues: {:?}", r.issues);
    }

    #[test]
    fn dollar_in_word_problem() {
        // "$5" as in money — this would be parsed as math delimiter start
        // This is a known edge case: the validator flags it because $ starts math mode
        let r = validate_katex("A toy costs $5. How much for 3?");
        // This will likely have issues since $5. How much for 3?$ looks like broken math
        assert!(r.has_errors() || r.has_warnings(),
                "Dollar sign in text should be flagged: {:?}", r.issues);
    }

    #[test]
    fn valid_complex_nested() {
        let r = validate_katex(
            "$\\frac{\\sqrt{x^{2}+1}}{\\left(\\frac{1}{x}\\right)^{3}}$"
        );
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // \( \) and \[ \] delimiter support
    // ====================================================================

    #[test]
    fn valid_paren_delimiters() {
        let r = validate_katex("Solve for \\(x\\): \\(\\frac{4 + 6x}{-7} = 8\\)");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_bracket_delimiters() {
        let r = validate_katex("Solve for \\(x\\): \\[6(x + 6) - 4(x + 2) = 64\\]");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_mixed_dollar_and_paren() {
        // Both $ and \( work via auto-render — not an error
        let r = validate_katex("Solve for \\(x\\): $2^{-6 - 4x} = 2^{-21 - x}$");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_nabla_with_paren_delim() {
        // \( \) delimiters around \nabla — renders fine via global auto-render
        let r = validate_katex("Find \\(\\nabla f\\) at \\((1, 2)\\).");
        assert!(r.is_ok(), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Newlines inside display blocks (innerHTML <br> issue)
    // ====================================================================

    #[test]
    fn newline_in_display_auto_fixed() {
        // prepare_for_rendering strips newlines, so validate_katex sees clean content
        let r = validate_katex("Text:\n$$\\begin{cases}\nx = 1 \\\\\ny = 2\n\\end{cases}$$");
        assert!(!has_issue(&r, "newline-in-display"), "issues: {:?}", r.issues);
    }

    #[test]
    fn newline_in_bracket_auto_fixed() {
        let r = validate_katex("Define:\n\\[\nr_1 = \\frac{1}{x}\n\\]");
        assert!(!has_issue(&r, "newline-in-display"), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_display_no_newlines() {
        let r = validate_katex("$$\\begin{cases} x = 1 \\\\ y = 2 \\end{cases}$$");
        assert!(!has_issue(&r, "newline-in-display"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Unicode math without delimiters
    // ====================================================================

    #[test]
    fn warn_unicode_math_no_delimiters() {
        let r = validate_katex("Let V = ℝ^{5} and consider vectors");
        assert!(has_warning(&r, "missing-delimiters"), "issues: {:?}", r.issues);
    }

    #[test]
    fn warn_subscript_brace_no_delimiters() {
        let r = validate_katex("v_{1} = [[3], [2]]");
        assert!(has_warning(&r, "missing-delimiters"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Literal \n text (double-escaped newlines)
    // ====================================================================

    #[test]
    fn literal_backslash_n_auto_fixed() {
        // prepare_for_rendering replaces literal \n, so validator sees clean content
        let r = validate_katex("Consider the system:\\n\\n$$x = 1$$");
        assert!(!has_issue(&r, "literal-backslash-n"), "issues: {:?}", r.issues);
    }

    #[test]
    fn no_warn_latex_commands_with_n() {
        // \nabla, \neq, \nu, \not — these are LaTeX commands, not escaped newlines
        let r = validate_katex("$\\nabla f = 0$");
        assert!(!has_issue(&r, "literal-backslash-n"), "issues: {:?}", r.issues);
    }

    #[test]
    fn no_warn_linebreak_n() {
        // \\n inside math is a LaTeX linebreak + n — not a literal \n
        let r = validate_katex("$a \\\\n b$");
        assert!(!has_issue(&r, "literal-backslash-n"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // Corrupted data (factory code leaks, malformed matrices)
    // ====================================================================

    #[test]
    fn error_factory_code_leak() {
        let r = validate_katex("$(-4x + 35)°$ and DiagramObj(600, 400, 40, ...)");
        assert!(has_error(&r, "factory-code-leak"), "issues: {:?}", r.issues);
    }

    #[test]
    fn error_malformed_matrix_data() {
        let r = validate_katex("$A = [[-, 3, /, /, 1], [0, /, /, 1]]$");
        assert!(has_error(&r, "malformed-data"), "issues: {:?}", r.issues);
    }

    #[test]
    fn valid_normal_matrix() {
        let r = validate_katex("$A = \\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\end{pmatrix}$");
        assert!(!has_issue(&r, "malformed-data"), "issues: {:?}", r.issues);
        assert!(!has_issue(&r, "factory-code-leak"), "issues: {:?}", r.issues);
    }

    // ====================================================================
    // validate_and_fix — auto-correction tests
    // ====================================================================

    #[test]
    fn fix_escaped_dollar_delimiters() {
        // validate_katex runs on prepared content, so \$ is already fixed
        // validate_and_fix should report no issues since prepare handles it
        let (result, _) = validate_and_fix("Simplify: \\$\\frac{1}{2}\\$");
        assert!(result.is_ok(), "should be clean after auto-fix: {:?}", result.issues);
    }

    #[test]
    fn fix_literal_backslash_n_text() {
        // prepare_for_rendering handles \n, so validator sees clean content
        let (result, _) = validate_and_fix("Question:\\n\\n$$x = 1$$");
        assert!(result.is_ok(), "should be clean after auto-fix: {:?}", result.issues);
    }

    #[test]
    fn fix_newlines_in_display_block() {
        // prepare_for_rendering strips newlines in display blocks
        let (result, _) = validate_and_fix("Text:\n$$\\begin{cases}\nx = 1\n\\end{cases}$$");
        assert!(result.is_ok(), "should be clean after auto-fix: {:?}", result.issues);
    }

    #[test]
    fn fix_bare_environment() {
        let (_, fix) = validate_and_fix("Solve:\n\\begin{align*}\nx &= 1\n\\end{align*}");
        let fixed = fix.unwrap();
        assert!(fixed.contains("$$"), "should wrap in $$: {}", fixed);
    }

    #[test]
    fn no_fix_for_corrupted_data() {
        let (result, fix) = validate_and_fix("$(-4x + 35)°$ DiagramObj(600, 400)");
        assert!(result.has_errors());
        assert!(fix.is_none(), "corrupted data should return None");
    }

    #[test]
    fn no_fix_for_valid_input() {
        let (result, fix) = validate_and_fix("$\\frac{1}{2}$");
        assert!(result.is_ok());
        assert!(fix.is_none());
    }

    #[test]
    fn fix_pure_latex_no_delimiters() {
        let (_, fix) = validate_and_fix("\\frac{1}{2}");
        // LatexRenderer handles this, but explicit wrapping is fine too
        // The function should return the input unchanged since starts_with(\) is handled
        // by LatexRenderer already
        assert!(fix.is_none() || fix.unwrap() == "$\\frac{1}{2}$");
    }
}
