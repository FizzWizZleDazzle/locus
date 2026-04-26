//! End-to-end tests: YAML → parse → generate → validate
//!
//! Each test parses a YAML string, generates N problems, and verifies:
//! - Parse succeeds
//! - Generation succeeds (variables resolve, constraints satisfied)
//! - Self-grading passes (answer_key grades as Correct)
//! - KaTeX validation passes (question_latex renders correctly)
//! - Format checks pass (if specified)
//! - Answer type is correctly inferred
//! - Generated values are within expected ranges

use locus_dsl::{ProblemOutput, generate_random, parse};

/// Wrap a legacy flat-body YAML into the new variants-only form. Splits the
/// top-level keys into header (topic/difficulty/calculator/time) and body
/// (everything else, which becomes the single variant). Leaves text indented
/// under continuation keys intact.
fn wrap_legacy(yaml: &str) -> String {
    const HEADER_KEYS: &[&str] = &["topic:", "difficulty:", "calculator:", "time:"];
    let mut header_lines: Vec<&str> = Vec::new();
    let mut body_lines: Vec<&str> = Vec::new();
    let mut in_header = false;
    let mut started_body = false;

    for line in yaml.lines() {
        if line.trim().is_empty() {
            if started_body {
                body_lines.push("");
            }
            continue;
        }
        let starts_top_level =
            !line.starts_with(' ') && !line.starts_with('\t') && line.contains(':');
        if starts_top_level {
            in_header = HEADER_KEYS.iter().any(|k| line.trim_start().starts_with(k));
            if !in_header {
                started_body = true;
            }
        }
        if in_header {
            header_lines.push(line);
        } else {
            body_lines.push(line);
        }
    }

    let mut out = String::new();
    for h in header_lines {
        out.push_str(h);
        out.push('\n');
    }
    out.push_str("variants:\n");
    out.push_str("  - name: t\n");
    for b in body_lines {
        if b.is_empty() {
            out.push('\n');
        } else {
            out.push_str("    ");
            out.push_str(b);
            out.push('\n');
        }
    }
    out
}

/// Helper: parse + generate N, assert all succeed, return results
fn gen_problems(yaml: &str, n: usize) -> Vec<ProblemOutput> {
    let wrapped = wrap_legacy(yaml);
    let spec = parse(&wrapped).unwrap_or_else(|e| panic!("Parse failed: {e}\nYAML:\n{wrapped}"));
    (0..n)
        .map(|i| {
            generate_random(&spec)
                .unwrap_or_else(|e| panic!("Generation {i} failed: {e}\nYAML:\n{wrapped}"))
        })
        .collect()
}

/// Helper: parse should fail. Wraps the YAML the same way real specs are
/// wrapped, so missing-field tests exercise the variant-level requirement.
fn parse_fails(yaml: &str) {
    let wrapped = wrap_legacy(yaml);
    assert!(
        parse(&wrapped).is_err(),
        "Expected parse failure for:\n{wrapped}"
    );
}

// ============================================================================
// Basic pipeline
// ============================================================================

#[test]
fn e2e_arithmetic_addition() {
    let problems = gen_problems(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: integer(10, 99)
  b: integer(10, 99)
  answer: a + b
question: What is {a} + {b}?
answer: answer
"#,
        20,
    );

    for p in &problems {
        assert_eq!(p.main_topic, "arithmetic");
        assert_eq!(p.subtopic, "addition");
        assert_eq!(p.answer_type, "numeric");
        assert!(p.question_latex.contains('+'));
        assert!(p.question_latex.contains('$'));
        let ans: i64 = p.answer_key.parse().expect("answer should be integer");
        assert!((20..=198).contains(&ans));
    }
}

#[test]
fn e2e_arithmetic_fractions() {
    let problems = gen_problems(
        r#"
topic: arithmetic/fractions
difficulty: easy
variables:
  a: integer(1, 5)
  b: integer(6, 9)
  c: integer(1, 5)
  d: integer(6, 9)
  num: a*d + c*b
  den: b*d
  answer: num/den
constraints:
  - b != d
question: "Add: {a}/{b} + {c}/{d}"
answer: answer
"#,
        10,
    );

    for p in &problems {
        // Answer is either a fraction (expression) or simplifies to integer (numeric)
        assert!(
            p.answer_type == "expression" || p.answer_type == "numeric",
            "Expected expression or numeric, got: {}",
            p.answer_type
        );
        assert!(
            p.answer_key.contains('/') || p.answer_key.parse::<i64>().is_ok(),
            "Expected fraction or integer, got: {}",
            p.answer_key
        );
    }
}

// ============================================================================
// Algebra — solve, expand, constraints
// ============================================================================

#[test]
fn e2e_linear_equations() {
    let problems = gen_problems(
        r#"
topic: algebra1/two_step_equations
difficulty: easy
variables:
  a: nonzero(-8, 8)
  b: integer(-10, 10)
  answer: integer(-10, 10)
  rhs: a * answer + b
constraints:
  - a != 1
  - a != -1
question: "Solve for x: {equation(a*x + b, rhs)}"
answer: answer
"#,
        20,
    );

    for p in &problems {
        assert!(p.question_latex.contains('='));
        assert!(p.question_latex.contains('x'));
        let ans: i64 = p.answer_key.parse().expect("answer should be integer");
        assert!((-10..=10).contains(&ans));
    }
}

#[test]
fn e2e_quadratic_solve() {
    let problems = gen_problems(
        r#"
topic: algebra1/quadratic_formula
difficulty: medium
variables:
  r1: integer(-8, 8)
  r2: integer(-8, 8)
  f: (x - r1) * (x - r2)
  expanded: expand(f)
  answer: solve(expanded, x)
constraints:
  - r1 != r2
  - r1 < r2
question: "Solve {equation(expanded, 0)} for x."
answer: answer
answer_type: set
"#,
        20,
    );

    for p in &problems {
        assert_eq!(p.answer_type, "set");
        assert!(
            p.answer_key.contains(','),
            "Set answer should have comma: {}",
            p.answer_key
        );
        assert!(p.question_latex.contains('='));
    }
}

// ============================================================================
// Calculus — derivative, integral
// ============================================================================

#[test]
fn e2e_derivative() {
    let problems = gen_problems(
        r#"
topic: calculus/derivative_rules
difficulty: medium
variables:
  a: nonzero(-8, 8)
  n: integer(2, 6)
  f: a * x^n
  answer: derivative(f, x)
question: "Find {derivative_of(f, x)}"
answer: answer
"#,
        20,
    );

    for p in &problems {
        assert_eq!(p.answer_type, "expression");
        assert!(
            p.answer_key.contains('x'),
            "Derivative should contain x: {}",
            p.answer_key
        );
        // Check display function rendered
        assert!(
            p.question_latex.contains("\\frac{d}{dx}"),
            "Should have d/dx: {}",
            p.question_latex
        );
    }
}

#[test]
fn e2e_definite_integral() {
    let problems = gen_problems(
        r#"
topic: calculus/definite_integrals
difficulty: medium
variables:
  a: nonzero(-5, 5)
  n: integer(1, 4)
  f: a * x^n
  lo: integer(0, 3)
  hi: integer(4, 8)
  F: integral(f, x)
  F_hi: evaluate(F, x, hi)
  F_lo: evaluate(F, x, lo)
  answer: F_hi - F_lo
constraints:
  - lo < hi
  - is_integer(answer)
question: "Evaluate {definite_integral_of(f, x, lo, hi)}"
answer: answer
"#,
        20,
    );

    for p in &problems {
        // Answer should be numeric (integer per constraint)
        assert!(
            p.answer_key.parse::<i64>().is_ok() || p.answer_key.parse::<f64>().is_ok(),
            "Definite integral should be numeric: {}",
            p.answer_key
        );
        assert!(
            p.question_latex.contains("\\int"),
            "Should have integral sign: {}",
            p.question_latex
        );
    }
}

// ============================================================================
// Geometry — sqrt, constraints
// ============================================================================

#[test]
fn e2e_pythagorean() {
    let problems = gen_problems(
        r#"
topic: geometry/pythagorean_theorem
difficulty: easy
calculator: scientific
variables:
  a: choice(3, 5, 6, 7, 8, 9)
  b: choice(4, 8, 9, 12, 15)
  a2: a^2
  b2: b^2
  sum: a2 + b2
  answer: sqrt(sum)
constraints:
  - a < b
  - is_integer(answer)
question: "Find the hypotenuse of a right triangle with legs {a} and {b}."
answer: answer
"#,
        20,
    );

    for p in &problems {
        assert_eq!(p.calculator_allowed, "scientific");
        let ans: i64 = p.answer_key.parse().expect("Hypotenuse should be integer");
        assert!(ans > 0);
    }
}

// ============================================================================
// Precalculus — function composition, expand
// ============================================================================

#[test]
fn e2e_function_composition() {
    let problems = gen_problems(
        r#"
topic: precalculus/function_composition
difficulty: medium
variables:
  a: nonzero(-5, 5)
  b: integer(-8, 8)
  c: nonzero(-5, 5)
  d: integer(-8, 8)
  f: a*x + b
  g: c*x + d
  fog: a*(c*x + d) + b
  answer: expand(fog)
question: "Let f(x) = {f} and g(x) = {g}. Find (f o g)(x) in expanded form."
answer: answer
"#,
        20,
    );

    for p in &problems {
        assert_eq!(p.answer_type, "expression");
        assert!(p.answer_key.contains('x'));
    }
}

// ============================================================================
// Format checking
// ============================================================================

#[test]
fn e2e_format_factored() {
    let problems = gen_problems(
        r#"
topic: algebra1/factoring_gcf
difficulty: easy
variables:
  gcf: choice(2, 3, 4, 5)
  c1: integer(1, 4)
  c2: integer(1, 4)
  a: gcf * c1
  b: gcf * c2
  f: a*x + b
  answer: gcf * (c1*x + c2)
constraints:
  - c1 != c2
question: "Factor out the GCF: {f}"
answer: answer
format: factored
"#,
        20,
    );

    for p in &problems {
        // Answer passed format check — verify it's actually factored
        // (contains * and parens, not just a sum)
        assert!(
            p.answer_key.contains('(') || !p.answer_key.contains('+'),
            "Factored answer should have parens or be monomial: {}",
            p.answer_key
        );
    }
}

#[test]
fn e2e_format_expanded() {
    let problems = gen_problems(
        r#"
topic: algebra1/polynomial_operations
difficulty: easy
variables:
  a: nonzero(-4, 4)
  b: integer(-5, 5)
  c: nonzero(-4, 4)
  d: integer(-5, 5)
  f: (a*x + b) * (c*x + d)
  answer: expand(f)
constraints:
  - a*c != 0
question: "Expand {f}"
answer: answer
format: expanded
"#,
        20,
    );

    for p in &problems {
        assert_eq!(p.answer_type, "expression");
    }
}

// ============================================================================
// Constraints
// ============================================================================

#[test]
fn e2e_constraints_nonzero() {
    let problems = gen_problems(
        r#"
topic: algebra1/one_step_equations
difficulty: easy
variables:
  a: nonzero(-10, 10)
  answer: integer(-10, 10)
  b: a * answer
constraints:
  - answer != 0
  - a != 1
  - a != -1
question: "Solve for x: {equation(a*x, b)}"
answer: answer
"#,
        50,
    );

    for p in &problems {
        let ans: i64 = p.answer_key.parse().unwrap();
        assert_ne!(ans, 0, "Constraint answer != 0 violated");
    }
}

#[test]
fn e2e_constraints_and_or() {
    let problems = gen_problems(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: integer(1, 100)
  b: integer(1, 100)
  answer: a + b
constraints:
  - a > 10 or b > 10
  - a < 50 and b < 50
question: What is {a} + {b}?
answer: answer
"#,
        50,
    );

    for p in &problems {
        // Can't easily check a,b individually from output,
        // but answer range should be 12..99
        let ans: i64 = p.answer_key.parse().unwrap();
        assert!(ans >= 12 && ans <= 98, "Answer out of range: {ans}");
    }
}

// ============================================================================
// Display functions
// ============================================================================

#[test]
fn e2e_display_equation() {
    let problems = gen_problems(
        r#"
topic: algebra1/one_step_equations
difficulty: easy
variables:
  a: nonzero(-5, 5)
  b: integer(-10, 10)
question: "Solve: {equation(a*x, b)}"
answer: b
"#,
        10,
    );

    for p in &problems {
        // equation() should produce "lhs = rhs" with $ delimiters
        assert!(
            p.question_latex.contains('='),
            "equation() should have =: {}",
            p.question_latex
        );
        assert!(
            p.question_latex.contains('$'),
            "Should have $ delimiters: {}",
            p.question_latex
        );
    }
}

#[test]
fn e2e_display_integral() {
    let problems = gen_problems(
        r#"
topic: calculus/antiderivatives
difficulty: easy
variables:
  a: nonzero(-5, 5)
  f: a * x^2
  answer: integral(f, x)
question: "Find {integral_of(f, x)}"
answer: answer
"#,
        10,
    );

    for p in &problems {
        assert!(
            p.question_latex.contains("\\int"),
            "integral_of should produce \\int: {}",
            p.question_latex
        );
        assert!(
            p.question_latex.contains("dx"),
            "Should have dx: {}",
            p.question_latex
        );
    }
}

#[test]
fn e2e_display_limit() {
    let problems = gen_problems(
        r#"
topic: calculus/continuity
difficulty: easy
variables:
  a: nonzero(-5, 5)
  f: a * x^2 + 1
  val: integer(1, 5)
  answer: evaluate(f, x, val)
question: "Evaluate {limit_of(f, x, val)}"
answer: answer
"#,
        10,
    );

    for p in &problems {
        assert!(
            p.question_latex.contains("\\lim"),
            "limit_of should produce \\lim: {}",
            p.question_latex
        );
        assert!(
            p.question_latex.contains("\\to"),
            "Should have \\to: {}",
            p.question_latex
        );
    }
}

// ============================================================================
// Solution steps
// ============================================================================

#[test]
fn e2e_solution_steps() {
    let problems = gen_problems(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: integer(10, 99)
  b: integer(10, 99)
  answer: a + b
question: What is {a} + {b}?
answer: answer
solution:
  - Add {a} and {b}
  - "Answer: {answer}"
"#,
        5,
    );

    for p in &problems {
        assert!(!p.solution_latex.is_empty(), "Solution should not be empty");
        assert!(
            p.solution_latex.contains('$'),
            "Solution should have rendered math"
        );
        assert!(
            p.solution_latex.contains('\n'),
            "Solution should have multiple steps"
        );
    }
}

// ============================================================================
// Difficulty
// ============================================================================

#[test]
fn e2e_difficulty_labels() {
    for (label, lo, hi) in &[
        ("very_easy", 800, 1000),
        ("easy", 1000, 1200),
        ("medium", 1200, 1400),
        ("hard", 1400, 1600),
        ("very_hard", 1600, 1800),
        ("competition", 1800, 2200),
    ] {
        let yaml = format!(
            r#"
topic: arithmetic/addition
difficulty: {label}
variables:
  a: integer(1, 10)
  b: integer(1, 10)
  answer: a + b
question: What is {{a}} + {{b}}?
answer: answer
"#
        );
        let problems = gen_problems(&yaml, 10);
        for p in &problems {
            assert!(
                p.difficulty >= *lo && p.difficulty <= *hi,
                "{label}: difficulty {} not in [{lo}, {hi}]",
                p.difficulty
            );
        }
    }
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn e2e_single_variable() {
    let problems = gen_problems(
        r#"
topic: arithmetic/multiplication
difficulty: easy
variables:
  answer: integer(1, 100)
question: "What is {answer}?"
answer: answer
"#,
        10,
    );

    for p in &problems {
        let ans: i64 = p.answer_key.parse().unwrap();
        assert!((1..=100).contains(&ans));
    }
}

#[test]
fn e2e_negative_numbers() {
    let problems = gen_problems(
        r#"
topic: algebra1/one_step_equations
difficulty: easy
variables:
  a: integer(-20, -1)
  b: integer(-20, -1)
  answer: a + b
question: What is {a} + {b}?
answer: answer
"#,
        20,
    );

    for p in &problems {
        let ans: i64 = p.answer_key.parse().unwrap();
        assert!(ans < 0, "Sum of negatives should be negative: {ans}");
    }
}

#[test]
fn e2e_large_expression() {
    let problems = gen_problems(
        r#"
topic: algebra1/polynomial_operations
difficulty: medium
variables:
  a: nonzero(-3, 3)
  b: nonzero(-3, 3)
  c: nonzero(-3, 3)
  f: a*x^3 + b*x^2 + c*x
  answer: derivative(f, x)
question: "Find {derivative_of(f, x)}"
answer: answer
"#,
        10,
    );

    for p in &problems {
        assert!(
            p.answer_key.contains('x'),
            "Derivative should have x: {}",
            p.answer_key
        );
    }
}

// ============================================================================
// Parse errors — should fail gracefully
// ============================================================================

#[test]
fn parse_error_missing_topic() {
    parse_fails(
        r#"
difficulty: easy
variables:
  a: integer(1, 10)
question: What is {a}?
answer: a
"#,
    );
}

#[test]
fn parse_error_invalid_topic() {
    parse_fails(
        r#"
topic: invalid
variables:
  a: integer(1, 10)
question: What is {a}?
answer: a
"#,
    );
}

#[test]
fn parse_error_missing_question() {
    parse_fails(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: integer(1, 10)
answer: a
"#,
    );
}

#[test]
fn parse_error_missing_answer() {
    parse_fails(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: integer(1, 10)
question: What is {a}?
"#,
    );
}

#[test]
fn parse_error_invalid_mode() {
    parse_fails(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: integer(1, 10)
question: What is {a}?
answer: a
mode: invalid_mode
"#,
    );
}

// ============================================================================
// Generation errors — should fail at generate time
// ============================================================================

#[test]
fn gen_error_undefined_variable() {
    // Template references undefined var → should fail
    let spec = parse(&wrap_legacy(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: integer(1, 10)
question: "What is {a} + {b}?"
answer: a
"#,
    ))
    .unwrap();
    assert!(
        generate_random(&spec).is_err(),
        "Should fail: {{b}} is undefined in template"
    );
}

#[test]
fn gen_error_unsatisfiable_constraint() {
    let spec = parse(&wrap_legacy(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: integer(5, 5)
constraints:
  - a > 10
question: What is {a}?
answer: a
"#,
    ))
    .unwrap();
    assert!(generate_random(&spec).is_err());
}

#[test]
fn gen_error_circular_dependency() {
    let spec = parse(&wrap_legacy(
        r#"
topic: arithmetic/addition
difficulty: easy
variables:
  a: b + 1
  b: a + 1
question: What is {a}?
answer: a
"#,
    ))
    .unwrap();
    assert!(generate_random(&spec).is_err());
}

// ============================================================================
// All existing problem files
// ============================================================================

#[test]
fn e2e_all_problem_files_parse() {
    let problem_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("problems");

    let mut files = Vec::new();
    collect_yaml_files(&problem_dir, &mut files);

    assert!(
        !files.is_empty(),
        "No problem files found in {}",
        problem_dir.display()
    );

    let mut parse_ok = 0;
    let mut parse_fail = 0;
    for file in &files {
        let yaml = std::fs::read_to_string(file).unwrap();
        match parse(&yaml) {
            Ok(_) => parse_ok += 1,
            Err(e) => {
                eprintln!("Parse failed: {}: {e}", file.display());
                parse_fail += 1;
            }
        }
    }
    assert_eq!(
        parse_fail,
        0,
        "{parse_fail}/{} files failed to parse",
        files.len()
    );
}

#[test]
fn e2e_handwritten_problem_files_generate() {
    let problem_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("problems");

    let mut files = Vec::new();
    collect_yaml_files(&problem_dir, &mut files);

    // Only test handwritten files for full generation (AI files skip generation validation)
    let handwritten: Vec<_> = files
        .iter()
        .filter(|f| f.file_name().map_or(false, |n| n == "handwritten.yaml"))
        .collect();

    assert!(!handwritten.is_empty(), "No handwritten files found");

    for file in &handwritten {
        let yaml = std::fs::read_to_string(file).unwrap();
        let spec =
            parse(&yaml).unwrap_or_else(|e| panic!("Parse failed for {}: {e}", file.display()));
        for i in 0..5 {
            generate_random(&spec)
                .unwrap_or_else(|e| panic!("Generation {i} failed for {}: {e}", file.display()));
        }
    }
}

fn collect_yaml_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                collect_yaml_files(&p, out);
            } else if p.extension().map_or(false, |e| e == "yaml") {
                out.push(p);
            }
        }
    }
}

// ============================================================================
// LaTeX rendering quality (regression suite for the prompt-quality overhaul)
// ============================================================================

/// Generate `n` outputs, assert no banned token appears in question or solution.
/// Patterns are common bug signatures from the earlier regex-based renderer.
fn assert_no_render_leaks(yaml: &str, n: usize) {
    let problems = gen_problems(yaml, n);
    let banned: &[(&str, &str)] = &[
        (
            r"\*\*",
            "raw `**` — SymEngine native printer should emit `^{}`",
        ),
        (r"\bsqrt\(", "raw `sqrt(` — should render as \\sqrt{}"),
        (
            r"\binfinity\b",
            "literal word `infinity` — should be \\infty",
        ),
        (
            r"\bderivative_of\(",
            "literal display-fn name `derivative_of(`",
        ),
        (r"\bintegral_of\(", "literal display-fn name `integral_of(`"),
        (r"\bevaluate\(", "literal display-fn name `evaluate(`"),
        (r"\blimit_of\(", "literal display-fn name `limit_of(`"),
        (r"\bdet_of\(", "literal display-fn name `det_of(`"),
        (r"\bmatrix_of\(", "literal display-fn name `matrix_of(`"),
    ];
    for (i, p) in problems.iter().enumerate() {
        let combined = format!("{}\n{}", p.question_latex, p.solution_latex);
        for (pat, why) in banned {
            let re = regex::Regex::new(pat).unwrap();
            assert!(
                !re.is_match(&combined),
                "iteration {i}: matched banned pattern `{pat}` ({why})\nrender: {combined}"
            );
        }
    }
}

#[test]
fn render_no_leaks_derivative_with_sqrt() {
    // Was producing `21sqrt(x) + 35x**(9/2)` — should be \sqrt and \frac now.
    let yaml = r#"
topic: calculus/derivative_rules
difficulty: hard
variables:
  a: nonzero(-6, 6)
  b: nonzero(-8, 8)
  n: integer(3, 6)
  f: a * x^n + b * sqrt(x)
  answer: derivative(f, x)
question: "Find {derivative_of(f, x)}"
answer: answer
"#;
    assert_no_render_leaks(yaml, 5);
}

#[test]
fn render_limit_at_infinity_emits_latex_infinity() {
    let yaml = r#"
topic: calculus/limits_at_infinity
difficulty: very_easy
variables:
  a: nonzero(-5, 5)
  c: nonzero(-5, 5)
  f: (a*x) / (c*x)
  answer: a/c
question: "Find {limit_of(f, x, infinity)}"
answer: answer
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        assert!(
            p.question_latex.contains(r"\infty"),
            "expected \\infty in {}",
            p.question_latex
        );
        assert!(
            !p.question_latex.contains("infinity"),
            "literal `infinity` leaked: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_math_wraps_inequality_in_latex() {
    // `{math(x < c)}` lets the AI typeset inline math without falling back to
    // bare-text `x < c`, which the platform doesn't render as LaTeX.
    let yaml = r#"
topic: arithmetic/inequality
difficulty: easy
variables:
  c: integer(2, 5)
  answer: c
question: "Find x such that {math(x < c)}."
answer: answer
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        // SymEngine's Latex printer emits `<` for StrictLessThan
        assert!(
            p.question_latex.contains("<"),
            "expected `<` in: {}",
            p.question_latex
        );
        // The math expression is wrapped in $...$ by the template renderer
        assert!(
            p.question_latex.contains("$x < "),
            "expected math-mode `x < ...`: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_math_wraps_equality_and_renders_greek() {
    // SymEngine doesn't parse `lambda = 3`, so {math()} must split on `=`
    // and render each side. Otherwise `lambda = -2` came out as plain text.
    let yaml = r#"
topic: differential_equations/test
difficulty: easy
variables:
  lam: integer(2, 5)
  answer: lam
question: "Eigenvalue is {math(lambda = lam)}."
answer: answer
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        assert!(
            p.question_latex.contains(r"\lambda"),
            "expected `\\lambda` in: {}",
            p.question_latex
        );
        assert!(
            !p.question_latex.contains(" lambda "),
            "literal `lambda` leaked: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_math_wraps_inequality_with_geq() {
    let yaml = r#"
topic: arithmetic/inequality
difficulty: easy
variables:
  c: integer(2, 5)
  answer: c
question: "Find {math(x >= c)}."
answer: answer
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        assert!(
            p.question_latex.contains(r"\geq"),
            "expected `\\geq` in: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_math_wraps_function_notation() {
    let yaml = r#"
topic: arithmetic/composition
difficulty: easy
variables:
  c: integer(2, 5)
  answer: c
question: "Compute {math(f(c))}."
answer: answer
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        assert!(
            p.question_latex.contains(r"f\left("),
            "expected LaTeX function notation in: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_matrix_cells_resolve_per_cell() {
    // Earlier output: `\begin{pmatrix} (2) & (1) \\ (0) & (-1) \end{pmatrix}` —
    // the whole `[[...]]` text was passed through unchanged.
    // Now each cell goes through expr_to_latex independently.
    let yaml = r#"
topic: linear_algebra/matrix_operations
difficulty: easy
variables:
  a: integer(1, 3)
  b: integer(1, 3)
  c: integer(1, 3)
  d: integer(1, 3)
  answer: a*d - b*c
question: "Find the determinant of {matrix_of([[a, b], [c, d]])}"
answer: answer
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        assert!(
            p.question_latex.contains(r"\begin{pmatrix}"),
            "expected pmatrix in: {}",
            p.question_latex
        );
        // No leftover unresolved variable names from the literal `[[a, b], [c, d]]`
        assert!(
            !regex::Regex::new(r"\b[a-d]\b")
                .unwrap()
                .is_match(&p.question_latex.replace(r"\begin", "").replace(r"\end", "")),
            "raw single-letter sampler name leaked into render: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_det_of_uses_vmatrix() {
    let yaml = r#"
topic: linear_algebra/determinants
difficulty: easy
variables:
  a: integer(1, 3)
  b: integer(1, 3)
  c: integer(1, 3)
  d: integer(1, 3)
  answer: a*d - b*c
question: "Compute {det_of([[a, b], [c, d]])}"
answer: answer
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        assert!(
            p.question_latex.contains(r"\begin{vmatrix}"),
            "expected vmatrix in: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_vec_emits_column_pmatrix() {
    let yaml = r#"
topic: linear_algebra/vectors
difficulty: easy
variables:
  a: integer(1, 5)
  b: integer(1, 5)
  answer: a + b
question: "Find the magnitude of {vec([a, b])}"
answer: answer
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        assert!(
            p.question_latex.contains(r"\begin{pmatrix}"),
            "expected column pmatrix in: {}",
            p.question_latex
        );
        // Rows separated by \\ — column vector has exactly one cell per row
        assert!(
            p.question_latex.contains(r"\\"),
            "expected row separator: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_nested_display_funcs_dispatch() {
    // `{equation(derivative_of(x, t), expr)}` was leaving the inner call literal:
    //   `\begin{cases} derivative(x, t) = ... \end{cases}`
    // Recursive arg resolution should now dispatch the inner display fn.
    let yaml = r#"
topic: differential_equations/systems_of_odes
difficulty: very_easy
variables:
  a: nonzero(-3, 3)
  b: nonzero(-3, 3)
  rhs: a*x + b*y
question: "{equation(derivative_of(x, t), rhs)}"
answer: a
"#;
    let problems = gen_problems(yaml, 3);
    for p in &problems {
        assert!(
            !p.question_latex.contains("derivative_of("),
            "literal display fn in: {}",
            p.question_latex
        );
        assert!(
            !p.question_latex.contains("derivative("),
            "literal `derivative(` in: {}",
            p.question_latex
        );
        assert!(
            p.question_latex.contains(r"\frac{d}{dt}"),
            "expected \\frac{{d}}{{dt}} in: {}",
            p.question_latex
        );
    }
}

#[test]
fn render_evaluate_returns_concrete_value() {
    // `{evaluate(f, x, c)}` should compute. Earlier bug returned the unevaluated form.
    let yaml = r#"
topic: arithmetic/addition
difficulty: easy
variables:
  c: choice(2, 3, 4)
  g: x^2 + c
  answer: c
question: "Compute {evaluate(g, x, c)}"
answer: answer
"#;
    let problems = gen_problems(yaml, 5);
    for p in &problems {
        assert!(
            !p.question_latex.contains("x^"),
            "evaluate didn't substitute: {}",
            p.question_latex
        );
        assert!(
            !p.question_latex.contains("evaluate("),
            "literal display-fn leaked: {}",
            p.question_latex
        );
    }
}
