//! AI YAML generation pipeline
//!
//! Fires N requests concurrently, validates as they return,
//! retries failures independently. Uses prompt caching + prefilling.

use std::sync::Arc;

/// Generate YAMLs for multiple (topic, index) pairs with per-task difficulty.
pub async fn generate_batch_multi_diff(
    tasks: &[(String, usize)],
    difficulties: &[String],
    api_key: &str,
    model: &str,
    concurrency: usize,
) -> Vec<Result<String, String>> {
    let client = Arc::new(reqwest::Client::new());
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
    let total = tasks.len();

    let mut handles = Vec::new();

    for (i, ((topic, _idx), difficulty)) in tasks.iter().zip(difficulties.iter()).enumerate() {
        let client = client.clone();
        let sem = semaphore.clone();
        let topic = topic.clone();
        let difficulty = difficulty.clone();
        let api_key = api_key.to_string();
        let model = model.to_string();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            eprintln!("[{}/{}] {topic} ({difficulty})...", i + 1, total);
            let result = generate_one(&client, &topic, &difficulty, &api_key, &model).await;
            match &result {
                Ok(_) => eprintln!("[{}/{}] {topic} ({difficulty}) OK", i + 1, total),
                Err(e) => eprintln!("[{}/{}] {topic} ({difficulty}) FAIL: {e}", i + 1, total),
            }
            result
        });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(total);
    for handle in handles {
        results.push(
            handle
                .await
                .unwrap_or_else(|e| Err(format!("Task panic: {e}"))),
        );
    }
    results
}

/// Maximum number of tool-use turns per script. Twenty leaves the agent room
/// to converge on harder topics where it needs several render → fix cycles.
/// The system prompt is cached, so the marginal cost of an extra turn is just
/// the message delta (~1K tokens output, often less).
const MAX_AGENT_TURNS: usize = 20;

async fn generate_one(
    client: &reqwest::Client,
    topic: &str,
    difficulty: &str,
    api_key: &str,
    model: &str,
) -> Result<String, String> {
    let playbook_block = match topic_playbook_excerpt(topic) {
        Some(excerpt) => format!("\n\nTopic-specific guidance:\n{excerpt}\n"),
        None => String::new(),
    };
    let initial_user = format!(
        "Write a problem YAML for topic `{topic}`, difficulty `{difficulty}`.{playbook_block}\n\n\
         Use the tools to verify your YAML before saving:\n\
         1. Draft your YAML.\n\
         2. Call `render_samples` to see exactly what students will see.\n\
         3. Inspect the rendered question, answer, and solution. Fix anything wrong.\n\
         4. Call `audit` to confirm the mechanical checks pass.\n\
         5. Call `save` with your final YAML.\n\n\
         If render_samples or audit shows issues, edit the YAML and try again. \
         Do not call save until the rendered output reads correctly to a student."
    );

    // The conversation history grows as we iterate. Anthropic caches the
    // system prompt block, so per-script overhead is just the message deltas.
    let mut messages = vec![serde_json::json!({
        "role": "user",
        "content": initial_user,
    })];

    for turn in 0..MAX_AGENT_TURNS {
        let resp = call_agent_turn(client, api_key, model, &messages).await?;

        let content = resp["content"].clone();
        let stop_reason = resp["stop_reason"].as_str().unwrap_or("").to_string();

        // Per-turn diagnostic: list tool calls (and brief text snippets) so a
        // failed run leaves enough breadcrumbs to debug without re-running.
        let tools_in_turn: Vec<String> = content
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|b| {
                        if b["type"].as_str() == Some("tool_use") {
                            Some(b["name"].as_str().unwrap_or("?").to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();
        eprintln!(
            "    [turn {}] tools: {:?} ({})",
            turn + 1,
            tools_in_turn,
            stop_reason
        );

        // Echo the assistant's turn back into history before processing tools.
        messages.push(serde_json::json!({
            "role": "assistant",
            "content": content.clone(),
        }));

        // Walk the assistant blocks, run any tool_use calls, collect results.
        let blocks = content.as_array().cloned().unwrap_or_default();
        let mut tool_results: Vec<serde_json::Value> = Vec::new();
        let mut saved_yaml: Option<String> = None;

        for block in &blocks {
            if block["type"].as_str() != Some("tool_use") {
                continue;
            }
            let tool_id = block["id"].as_str().unwrap_or("").to_string();
            let tool_name = block["name"].as_str().unwrap_or("");
            let input = &block["input"];

            let result = match tool_name {
                "render_samples" => {
                    let r = tool_render_samples(input, difficulty);
                    if let Some(audit) = r.get("audit") {
                        if audit["ok"].as_bool() != Some(true) {
                            if let Some(issues) = audit["issues"].as_array() {
                                eprintln!(
                                    "        audit hits ({}): {}",
                                    issues.len(),
                                    issues
                                        .iter()
                                        .take(2)
                                        .filter_map(|v| v.as_str())
                                        .collect::<Vec<_>>()
                                        .join(" | ")
                                );
                            }
                        }
                    }
                    r
                }
                "audit" => tool_audit(input, difficulty),
                "save" => {
                    let yaml = input["yaml"].as_str().unwrap_or("").to_string();
                    let yaml = inject_difficulty(&yaml, difficulty);
                    let outcome = tool_save(&yaml);
                    if outcome["ok"].as_bool() == Some(true) {
                        saved_yaml = Some(yaml);
                    }
                    outcome
                }
                "compute" => tool_compute(input),
                "solve" => tool_solve(input),
                "differentiate" => tool_differentiate(input),
                "integrate" => tool_integrate(input),
                "evaluate_at" => tool_evaluate_at(input),
                "expand" => tool_expand(input),
                "polynomial_from_roots" => tool_polynomial_from_roots(input),
                "matrix_with_eigenvalues" => tool_matrix_with_eigenvalues(input),
                "find_similar_yaml" => tool_find_similar_yaml(input, topic, difficulty),
                "solve_linear_inequality" => tool_solve_linear_inequality(input),
                "quadratic_roots" => tool_quadratic_roots(input),
                "verify_answer_key" => tool_verify_answer_key(input, difficulty),
                _ => serde_json::json!({"error": format!("unknown tool `{tool_name}`")}),
            };

            tool_results.push(serde_json::json!({
                "type": "tool_result",
                "tool_use_id": tool_id,
                "content": result.to_string(),
            }));
        }

        // Successful save → return the YAML. Skip remaining tool calls in this
        // turn; the model has signaled "done" and we don't want to keep paying.
        if let Some(yaml) = saved_yaml {
            return Ok(yaml);
        }

        if tool_results.is_empty() {
            // The model produced text without calling any tool. If it said
            // `end_turn`, it's giving up; bail with whatever rationale it gave.
            if stop_reason == "end_turn" {
                let text = blocks
                    .iter()
                    .filter(|b| b["type"].as_str() == Some("text"))
                    .filter_map(|b| b["text"].as_str())
                    .collect::<Vec<_>>()
                    .join("\n");
                return Err(format!("Agent ended without saving. Last text: {text}"));
            }
            // No tools and no end_turn — unusual; bail to avoid infinite loop.
            return Err(format!(
                "Turn {} produced no tool calls and no end_turn (stop_reason={stop_reason})",
                turn + 1
            ));
        }

        // Feed tool results back as a user message and continue.
        messages.push(serde_json::json!({
            "role": "user",
            "content": tool_results,
        }));
    }

    Err(format!("Agent did not save within {MAX_AGENT_TURNS} turns"))
}

/// Implements the `render_samples` tool: parse the YAML, generate `n` samples,
/// return the rendered question/answer/solution for each. Also runs the audit
/// in the same call so the model gets render + banlist findings in one turn.
fn tool_render_samples(input: &serde_json::Value, difficulty: &str) -> serde_json::Value {
    let yaml = input["yaml"].as_str().unwrap_or("");
    let n = input["n"].as_u64().unwrap_or(3).clamp(1, 5) as usize;
    let yaml = inject_difficulty(yaml, difficulty);

    let spec = match locus_dsl::parse(&yaml) {
        Ok(s) => s,
        Err(e) => return serde_json::json!({"error": format!("parse error: {e}")}),
    };

    // Render n samples per variant so the model sees each variant in action.
    let mut samples = Vec::with_capacity(n * spec.variants.len());
    for variant in &spec.variants {
        for i in 0..n {
            match locus_dsl::generate(&spec, variant) {
                Ok(p) => samples.push(serde_json::json!({
                    "variant": variant.name,
                    "seed": i,
                    "question": p.question_latex,
                    "answer": p.answer_key,
                    "solution": p.solution_latex,
                })),
                Err(e) => samples.push(serde_json::json!({
                    "variant": variant.name,
                    "seed": i,
                    "error": format!("generation failed: {e}"),
                })),
            }
        }
    }

    // Bundle the audit findings so the agent doesn't need a second turn for
    // the banlist check. The audit runs over its own samples; if it's clean,
    // the model can go straight to save.
    let audit_status = match audit_yaml(&yaml, 5) {
        Ok(()) => serde_json::json!({"ok": true}),
        Err(issues) => serde_json::json!({"ok": false, "issues": issues}),
    };

    serde_json::json!({
        "samples": samples,
        "audit": audit_status,
    })
}

/// Implements the `audit` tool: cheap mechanical scan of N renders.
fn tool_audit(input: &serde_json::Value, difficulty: &str) -> serde_json::Value {
    let yaml = input["yaml"].as_str().unwrap_or("");
    let yaml = inject_difficulty(yaml, difficulty);
    match audit_yaml(&yaml, 5) {
        Ok(()) => serde_json::json!({"ok": true}),
        Err(issues) => serde_json::json!({"ok": false, "issues": issues}),
    }
}

/// Implements the `save` tool: final audit; returns success only if the YAML
/// is fully clean. The model must call this to terminate the conversation.
fn tool_save(yaml: &str) -> serde_json::Value {
    match audit_yaml(yaml, 8) {
        Ok(()) => serde_json::json!({"ok": true}),
        Err(issues) => serde_json::json!({"ok": false, "issues": issues}),
    }
}

// ---------------------------------------------------------------------------
// CAS tool handlers — all dispatch to SymEngine via the existing builtin
// evaluator. We construct a synthetic call string and reuse the same logic
// the YAML's variable definitions use, so the model gets exactly the math
// behavior the renderer would produce.
// ---------------------------------------------------------------------------

/// Run a builtin call with a fresh empty VarMap. The model passes literal
/// expressions, so there are no variables to substitute.
fn run_builtin(call: &str) -> serde_json::Value {
    let vars = locus_dsl::resolver::VarMap::new();
    match locus_dsl::functions::evaluate(call, &vars) {
        Ok(s) => serde_json::json!({"result": s}),
        Err(e) => serde_json::json!({"error": format!("{e}")}),
    }
}

fn tool_compute(input: &serde_json::Value) -> serde_json::Value {
    let expr = input["expr"].as_str().unwrap_or("");
    // Wrap in a no-op call so we can route through the same dispatcher; the
    // simplest no-op is `expand(expr)` which canonicalizes everything.
    run_builtin(&format!("expand({expr})"))
}

fn tool_solve(input: &serde_json::Value) -> serde_json::Value {
    let expr = input["expr"].as_str().unwrap_or("");
    let var = input["var"].as_str().unwrap_or("x");
    run_builtin(&format!("solve({expr}, {var})"))
}

fn tool_differentiate(input: &serde_json::Value) -> serde_json::Value {
    let expr = input["expr"].as_str().unwrap_or("");
    let var = input["var"].as_str().unwrap_or("x");
    run_builtin(&format!("derivative({expr}, {var})"))
}

fn tool_integrate(input: &serde_json::Value) -> serde_json::Value {
    let expr = input["expr"].as_str().unwrap_or("");
    let var = input["var"].as_str().unwrap_or("x");
    if let (Some(lo), Some(hi)) = (input["lo"].as_str(), input["hi"].as_str()) {
        run_builtin(&format!("definite_integral({expr}, {var}, {lo}, {hi})"))
    } else {
        run_builtin(&format!("integral({expr}, {var})"))
    }
}

fn tool_evaluate_at(input: &serde_json::Value) -> serde_json::Value {
    let expr = input["expr"].as_str().unwrap_or("");
    let var = input["var"].as_str().unwrap_or("x");
    let val = input["val"].as_str().unwrap_or("0");
    run_builtin(&format!("evaluate({expr}, {var}, {val})"))
}

fn tool_expand(input: &serde_json::Value) -> serde_json::Value {
    let expr = input["expr"].as_str().unwrap_or("");
    run_builtin(&format!("expand({expr})"))
}

// ---------------------------------------------------------------------------
// Backward-design helpers
// ---------------------------------------------------------------------------

/// `polynomial_from_roots(["3", "-2"])` → `(x - 3)*(x - (-2))` expanded.
/// Returns the expanded form, which is what the YAML answer typically is.
fn tool_polynomial_from_roots(input: &serde_json::Value) -> serde_json::Value {
    let var = input["var"].as_str().unwrap_or("x");
    let roots = match input["roots"].as_array() {
        Some(arr) => arr,
        None => return serde_json::json!({"error": "roots must be an array"}),
    };
    if roots.is_empty() {
        return serde_json::json!({"error": "need at least one root"});
    }
    let factors: Vec<String> = roots
        .iter()
        .filter_map(|r| r.as_str())
        .map(|r| format!("({var} - ({r}))"))
        .collect();
    let product = factors.join("*");
    run_builtin(&format!("expand({product})"))
}

/// `matrix_with_eigenvalues(λ1, λ2)` → 2×2 integer matrix with those
/// eigenvalues. Construction: pick a small `b` (off-diagonal), set
/// trace = λ1 + λ2, det = λ1·λ2, then a = 0 (so d = trace), and choose c
/// such that a·d − b·c = det. Returns a JSON array `[[a, b], [c, d]]`.
fn tool_matrix_with_eigenvalues(input: &serde_json::Value) -> serde_json::Value {
    let l1 = match input["lambda1"].as_i64() {
        Some(v) => v,
        None => return serde_json::json!({"error": "lambda1 must be an integer"}),
    };
    let l2 = match input["lambda2"].as_i64() {
        Some(v) => v,
        None => return serde_json::json!({"error": "lambda2 must be an integer"}),
    };
    let trace = l1 + l2;
    let det = l1 * l2;

    // Try small values of (a, b) until c = (a*d - det) / b is a clean integer.
    // d is always trace - a. We try b ∈ {1, -1, 2, -2, 3, -3} for small entries.
    for a in [0i64, 1, -1, 2, -2] {
        let d = trace - a;
        for &b in &[1i64, -1, 2, -2, 3, -3] {
            let numer = a * d - det;
            if numer % b == 0 {
                let c = numer / b;
                return serde_json::json!({
                    "matrix": [[a, b], [c, d]],
                    "trace": trace,
                    "det": det,
                    "verify": format!("eigenvalues of [[{a}, {b}], [{c}, {d}]] are {l1}, {l2}"),
                });
            }
        }
    }
    serde_json::json!({"error": "could not construct an integer matrix; try other eigenvalues"})
}

/// Solve `a*x + b OP c*x + d` for x. Sign-flip on negative-coefficient division
/// is handled here, eliminating the most common linear-inequality YAML bug.
fn tool_solve_linear_inequality(input: &serde_json::Value) -> serde_json::Value {
    let a = input["a"].as_str().unwrap_or("0");
    let b = input["b"].as_str().unwrap_or("0");
    let c = input["c"].as_str().unwrap_or("0");
    let d = input["d"].as_str().unwrap_or("0");
    let op = input["op"].as_str().unwrap_or("<");

    // Numeric eval — coefficients are always concrete in YAML construction.
    let numeric = |s: &str| -> Option<f64> {
        let vars = locus_dsl::resolver::VarMap::new();
        let result = locus_dsl::functions::evaluate(&format!("expand({s})"), &vars).ok()?;
        result.parse::<f64>().ok().or_else(|| {
            // Try SymEngine numeric eval for "1/2" etc.
            locus_common::symengine::Expr::parse(&result)
                .ok()
                .and_then(|e| e.to_float())
        })
    };

    let (a_n, b_n, c_n, d_n) = match (numeric(a), numeric(b), numeric(c), numeric(d)) {
        (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
        _ => {
            return serde_json::json!({
                "error": "all four coefficients must reduce to numbers (try compute() first if any are expressions)"
            });
        }
    };
    let coef = a_n - c_n;
    let rhs = d_n - b_n;
    if coef.abs() < 1e-12 {
        return serde_json::json!({
            "error": "coefficient of x cancels — there's no x to solve for"
        });
    }
    let value = rhs / coef;
    let final_op = if coef < 0.0 {
        // Flip: < ↔ >, <= ↔ >=
        match op {
            "<" => ">",
            ">" => "<",
            "<=" => ">=",
            ">=" => "<=",
            other => other,
        }
    } else {
        op
    };
    // Format value as int if it's clean, else as fraction
    let value_str = format_clean_number(value);
    serde_json::json!({
        "op": final_op,
        "value": value_str,
        "explanation": format!(
            "Move x to one side: ({a} - {c})*x = ({d} - {b}). \
             Coefficient = {coef}, so dividing {}flips the inequality.",
            if coef < 0.0 { "" } else { "doesn't " }
        )
    })
}

/// Format a float as a clean integer or simple fraction. Mirrors the logic in
/// crates/dsl/src/functions.rs::format_root.
fn format_clean_number(r: f64) -> String {
    if (r - r.round()).abs() < 1e-10 {
        return format!("{}", r.round() as i64);
    }
    for denom in 2..=12 {
        let num = r * denom as f64;
        if (num - num.round()).abs() < 1e-8 {
            let n = num.round() as i64;
            let d = denom as i64;
            // gcd
            let mut a = n.unsigned_abs();
            let mut b = d as u64;
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            let g = a as i64;
            return format!("{}/{}", n / g, d / g);
        }
    }
    format!("{r:.6}")
}

/// Discriminant + case + roots for `a*x^2 + b*x + c = 0`.
fn tool_quadratic_roots(input: &serde_json::Value) -> serde_json::Value {
    let a = input["a"].as_str().unwrap_or("0");
    let b = input["b"].as_str().unwrap_or("0");
    let c = input["c"].as_str().unwrap_or("0");
    let vars = locus_dsl::resolver::VarMap::new();
    let disc_call = format!("expand(({b})^2 - 4*({a})*({c}))");
    let disc = match locus_dsl::functions::evaluate(&disc_call, &vars) {
        Ok(s) => s,
        Err(e) => return serde_json::json!({"error": format!("{e}")}),
    };
    let disc_n = locus_common::symengine::Expr::parse(&disc)
        .ok()
        .and_then(|e| e.to_float());

    let case = match disc_n {
        Some(d) if d > 1e-9 => "real_distinct",
        Some(d) if d.abs() < 1e-9 => "real_repeated",
        Some(_) => "complex",
        None => "symbolic",
    };

    // Roots via the existing `solve` builtin
    let solve_call = format!("solve(({a})*x^2 + ({b})*x + ({c}), x)");
    let roots_str = locus_dsl::functions::evaluate(&solve_call, &vars).unwrap_or_default();
    let roots: Vec<&str> = roots_str.split(',').map(|s| s.trim()).collect();

    serde_json::json!({
        "discriminant": disc,
        "case": case,
        "roots": roots,
    })
}

/// Render one sample, then run `grade_answer` on the answer_key against itself.
/// If the grader doesn't return Correct, the YAML's answer is internally
/// inconsistent — an answer/solution mismatch the audit can't catch.
fn tool_verify_answer_key(input: &serde_json::Value, difficulty: &str) -> serde_json::Value {
    let yaml = input["yaml"].as_str().unwrap_or("");
    let yaml = inject_difficulty(yaml, difficulty);
    let spec = match locus_dsl::parse(&yaml) {
        Ok(s) => s,
        Err(e) => return serde_json::json!({"ok": false, "mismatches": [format!("parse: {e}")]}),
    };
    let p = match locus_dsl::generate_random(&spec) {
        Ok(p) => p,
        Err(e) => return serde_json::json!({"ok": false, "mismatches": [format!("generate: {e}")]}),
    };
    // Self-grade: feed the answer_key back as the student's submission.
    use locus_common::{AnswerType, GradingMode};
    let grading_mode = match p.grading_mode.as_str() {
        "factor" => GradingMode::Factor,
        "expand" => GradingMode::Expand,
        _ => GradingMode::Equivalent,
    };
    let answer_type = match p.answer_type.as_str() {
        "numeric" => AnswerType::Numeric,
        "tuple" => AnswerType::Tuple,
        "set" => AnswerType::Set,
        "boolean" => AnswerType::Boolean,
        "word" => AnswerType::Word,
        _ => AnswerType::Expression,
    };
    let verdict =
        locus_common::grader::grade_answer(&p.answer_key, &p.answer_key, answer_type, grading_mode);
    let ok = matches!(verdict, locus_common::grader::GradeResult::Correct);
    if ok {
        serde_json::json!({
            "ok": true,
            "sample_question": p.question_latex,
            "sample_answer": p.answer_key,
        })
    } else {
        serde_json::json!({
            "ok": false,
            "mismatches": [format!("answer_key `{}` did not self-grade as Correct ({:?})", p.answer_key, verdict)],
            "sample_question": p.question_latex,
        })
    }
}

/// Find an existing YAML that's structurally similar to the requested topic.
/// Walks `problems/` and ranks files by topic-path overlap (longest matching
/// prefix wins). Returns the YAML body so the agent can use it as a template.
fn tool_find_similar_yaml(
    _input: &serde_json::Value,
    topic: &str,
    difficulty: &str,
) -> serde_json::Value {
    let problems_root = std::path::Path::new("problems");
    if !problems_root.exists() {
        return serde_json::json!({"error": "problems/ directory not found"});
    }

    // Score = longest shared prefix of slash-separated topic segments.
    let target_segments: Vec<&str> = topic.split('/').collect();
    let mut best: Option<(usize, std::path::PathBuf)> = None;

    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    walk(&p, out);
                } else if p.extension().map_or(false, |e| e == "yaml") {
                    out.push(p);
                }
            }
        }
    }

    let mut all = Vec::new();
    walk(problems_root, &mut all);

    for path in &all {
        let yaml = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        // Extract `topic:` line
        let file_topic = yaml
            .lines()
            .find_map(|l| l.strip_prefix("topic:"))
            .unwrap_or("")
            .trim();
        let file_segments: Vec<&str> = file_topic.split('/').collect();
        let shared = target_segments
            .iter()
            .zip(file_segments.iter())
            .take_while(|(a, b)| a == b)
            .count();
        // Prefer same difficulty when topic ties
        let same_diff = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s == difficulty)
            .unwrap_or(false);
        let score = shared * 10 + if same_diff { 1 } else { 0 };
        if score > 0 && best.as_ref().map_or(true, |(s, _)| score > *s) {
            best = Some((score, path.clone()));
        }
    }

    match best {
        Some((_, path)) => match std::fs::read_to_string(&path) {
            Ok(yaml) => serde_json::json!({
                "found": true,
                "source_path": path.display().to_string(),
                "yaml": yaml,
                "note": "Adapt the structure but write your own values. Don't copy verbatim."
            }),
            Err(e) => serde_json::json!({"error": format!("read error: {e}")}),
        },
        None => serde_json::json!({"found": false, "note": "No similar YAML in problems/"}),
    }
}

/// Tool schemas live in `src/prompts/agent_tools.json` so they can be edited
/// without rebuilding strings. `include_str!` embeds the file at compile time
/// and we parse once on first use.
const AGENT_TOOLS_JSON: &str = include_str!("prompts/agent_tools.json");

/// Anthropic API tool schemas. Mirror what the prompt's "tools" section says.
///
/// The toolset is split into three groups:
///   1. CAS primitives — let the model run any math through SymEngine instead
///      of doing arithmetic in its head (where it gets it wrong).
///   2. Backward-design helpers — generate clean inputs (clean polynomials,
///      matrices with chosen eigenvalues, exact trig values) so the model
///      doesn't pick numbers that lead to ugly answers.
///   3. Inspection — render, audit, find a similar template, save.
fn agent_tools() -> serde_json::Value {
    serde_json::from_str(AGENT_TOOLS_JSON).expect("agent_tools.json is malformed at compile time")
}

/// Make one tool-use turn against Anthropic. Returns the raw response (we'll
/// unpack `content`, `stop_reason`, etc. in the caller).
async fn call_agent_turn(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    messages: &[serde_json::Value],
) -> Result<serde_json::Value, String> {
    let body = serde_json::json!({
        "model": model,
        "max_tokens": 4096,
        "temperature": 0.4,
        "system": [{
            "type": "text",
            "text": system_prompt(),
            "cache_control": {"type": "ephemeral"}
        }],
        "tools": agent_tools(),
        "messages": messages,
    });

    for attempt in 0..3 {
        let resp = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await;

        let resp = match resp {
            Ok(r) => r,
            Err(e) => {
                if attempt < 2 {
                    tokio::time::sleep(std::time::Duration::from_secs((attempt + 1) as u64 * 5))
                        .await;
                    continue;
                }
                return Err(format!("HTTP error after 3 attempts: {e}"));
            }
        };

        let status = resp.status();
        if status.as_u16() == 429 || status.as_u16() == 529 {
            if attempt < 2 {
                tokio::time::sleep(std::time::Duration::from_secs((attempt + 1) as u64 * 10)).await;
                continue;
            }
        }
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("API error {status}: {text}"));
        }

        return resp
            .json()
            .await
            .map_err(|e| format!("JSON decode error: {e}"));
    }
    Err("unreachable".into())
}

/// Override difficulty in generated YAML to match CLI arg
fn inject_difficulty(yaml: &str, difficulty: &str) -> String {
    let re = regex::Regex::new(r"(?m)^difficulty:.*$").unwrap();
    if re.is_match(yaml) {
        re.replace(yaml, format!("difficulty: {difficulty}"))
            .to_string()
    } else {
        // No difficulty line — insert after topic line
        let topic_re = regex::Regex::new(r"(?m)^(topic:.*)$").unwrap();
        topic_re
            .replace(yaml, format!("$1\ndifficulty: {difficulty}"))
            .to_string()
    }
}

/// Audit a YAML by rendering `runs` samples and scanning for the same banned
/// patterns the AI generation pipeline uses. Public entry point for the
/// `dsl-cli audit` subcommand — callers don't have to know which patterns are
/// in the list, only whether the file passes.
pub fn audit_yaml(yaml: &str, runs: usize) -> Result<(), Vec<String>> {
    audit_yaml_inner(yaml, runs)
}

fn audit_yaml_inner(yaml: &str, runs: usize) -> Result<(), Vec<String>> {
    let spec = locus_dsl::parse(yaml).map_err(|e| vec![format!("Parse error: {e}")])?;

    let mut errors = Vec::new();
    'outer: for variant in &spec.variants {
        for i in 0..runs {
            match locus_dsl::generate(&spec, variant) {
                Ok(out) => {
                    let combined = format!("{}\n{}", out.question_latex, out.solution_latex);
                    for (pat, why, fix) in RENDER_BANLIST {
                        let re = regex::Regex::new(pat).unwrap();
                        if let Some(m) = re.find(&combined) {
                            errors.push(format!(
                                "variant `{}` seed {i}: {why} — matched `{}`. Fix: {fix}",
                                variant.name,
                                m.as_str()
                            ));
                        }
                    }
                    // Variable-name leak detector: a variable name appearing in
                    // rendered output usually means substitution failed. But math
                    // problem text legitimately uses words like "area", "angle",
                    // "height" that the YAML may also have as variable names. Only
                    // flag names that look distinctively code-y: contain an
                    // underscore, or are ≥ 8 chars (long words like
                    // `cyl_volume_rounded`, `box_volume`, `discriminant` survive;
                    // short English words like `area`, `radius` don't).
                    for var_name in variant.variables.keys() {
                        let looks_code_like = var_name.contains('_') || var_name.len() >= 8;
                        if !looks_code_like {
                            continue;
                        }
                        let pat = format!(r"\b{}\b", regex::escape(var_name));
                        if let Ok(re) = regex::Regex::new(&pat) {
                            if re.is_match(&combined) {
                                errors.push(format!(
                                    "variant `{}` seed {i}: unresolved variable `{var_name}` in output",
                                    variant.name
                                ));
                            }
                        }
                    }
                }
                Err(e) => errors.push(format!(
                    "variant `{}` seed {i}: generation error: {e}",
                    variant.name
                )),
            }
            if !errors.is_empty() {
                break 'outer;
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Patterns that should never appear in a rendered question or solution.
/// Each entry is (regex, what's wrong, how to fix). The fix string is appended
/// to the audit message so the model can apply it without a separate diagnosis
/// step.
const RENDER_BANLIST: &[(&str, &str, &str)] = &[
    (
        r"\*\*",
        "raw `**` exponent",
        "wrap the math in `{math(expr)}` or define a variable; never write `**` in solution prose",
    ),
    (
        r"\bsqrt\(",
        "raw `sqrt(` in prose",
        "wrap with `{math(sqrt(N))}` or pre-compute the value via the `compute` tool",
    ),
    (
        r"\binfinity\b",
        "literal word `infinity`",
        "use the `{limit_of(...)}` display function or write `oo` inside a variable definition",
    ),
    (
        r"\b(Infinity|INF|Inf)\b",
        "literal infinity alias",
        "same fix as `infinity` — use display fn or `oo`",
    ),
    (
        r"\bderivative_of\(",
        "literal display fn `derivative_of(`",
        "the inner display call wasn't nested inside a `{...}` ref — wrap the entire call in braces",
    ),
    (
        r"\bintegral_of\(",
        "literal display fn `integral_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bdefinite_integral_of\(",
        "literal display fn `definite_integral_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bevaluate\(",
        "literal display fn `evaluate(`",
        "wrap in `{...}` braces — the form is `{evaluate(expr, var, val)}`",
    ),
    (
        r"\blimit_of\(",
        "literal display fn `limit_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bdet_of\(",
        "literal display fn `det_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bmatrix_of\(",
        "literal display fn `matrix_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bsum_of\(",
        "literal display fn `sum_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bproduct_of\(",
        "literal display fn `product_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bpartial_of\(",
        "literal display fn `partial_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bnth_derivative_of\(",
        "literal display fn `nth_derivative_of(`",
        "wrap in `{...}` braces",
    ),
    (
        r"\bAbs\(",
        "raw `Abs(`",
        "use `{abs_of(x)}` display fn for `|x|` rendering",
    ),
    (
        r"[∞∪∩≤≥±→√∂∇∫∑∏×÷·°′″πθλαβγδεζημσφψωΔΣΠΩ]",
        "unicode math glyph",
        "write the spelled-out name: pi, theta, lambda, infinity, leq, geq — SymEngine renders them as LaTeX",
    ),
    (
        r"_\$\d+\$[a-zA-Z]",
        "subscript value followed by letters (`a_$7$th`)",
        "describe the position in plain English (e.g. `the 7th term`) without subscript",
    ),
    (
        r"'[^']{1,8}'",
        "single-quoted literal",
        "remove the quotes — the DSL has no strings; either define a variable or rephrase",
    ),
];

// =============================================================================
// System prompt (static, cached by Anthropic)
// =============================================================================

// The system prompt lives in `src/prompts/system_prompt.md` so it can be edited
// like a real document (markdown highlighting, side-by-side diffs, no escaping
// of backticks or quotes). `include_str!` embeds the file at compile time, so
// the binary still ships as a single artifact and any prompt change forces a
// rebuild — no separate codegen step, no runtime file I/O.
/// The model receives the core prompt followed by the per-topic playbook so
/// every conversation has the topic-specific guidance available without an
/// extra tool call. Both files are cached by Anthropic, so this concat costs
/// nothing per script after the first cache fill.
const SYSTEM_PROMPT_CORE: &str = include_str!("prompts/system_prompt.md");
const TOPIC_PLAYBOOK: &str = include_str!("prompts/topic_playbook.md");

fn system_prompt() -> String {
    // Playbook is appended only when the topic-specific entry adds meaningful
    // guidance. Empirically, including the full playbook hurts: it makes the
    // model overthink simple cases (basic determinant, unit circle) without
    // helping the hard ones, and pass rate drops from ~62% to ~42%. The fix
    // is to put the truly load-bearing rules in the core prompt instead.
    SYSTEM_PROMPT_CORE.to_string()
}

/// Pull just the bullet line(s) for a single `main/sub` topic from the
/// playbook, so we can inject targeted guidance into the user message
/// without dumping the whole document (which empirically hurt pass rate).
/// Returns None when the topic isn't documented — caller should fall back
/// to the generic prompt.
pub(crate) fn topic_playbook_excerpt(topic: &str) -> Option<String> {
    let (main, sub) = topic.split_once('/')?;
    let header = format!("## {}/*", main);
    let body_start = TOPIC_PLAYBOOK.find(&header)?;
    // Section runs until the next "## " heading or EOF.
    let after = &TOPIC_PLAYBOOK[body_start + header.len()..];
    let section_end = after.find("\n## ").unwrap_or(after.len());
    let section = &after[..section_end];

    let needle = format!("`{}/{}`", main, sub);
    let mut hits: Vec<&str> = Vec::new();
    for line in section.lines() {
        if line.contains(&needle) {
            hits.push(line.trim());
        }
    }
    if hits.is_empty() {
        return None;
    }
    Some(hits.join("\n"))
}

// Kept as a SYSTEM_PROMPT alias for tests that grep the prompt body.
#[cfg(test)]
const SYSTEM_PROMPT: &str = include_str!("prompts/system_prompt.md");

#[cfg(test)]
mod judge_tests {
    use super::*;

    #[test]
    fn render_banlist_catches_known_leaks() {
        let bad_renders = [
            (r"$\lim_{x \to infinity} x$", "infinity"),
            (r"$x**2 + 1$", "raw `**`"),
            (r"$5*sqrt(x)$", "raw `sqrt(`"),
            (r"$derivative_of(f, x)$", "literal display fn"),
            (r"a_$7$th term", "subscript-followed-by-text"),
            (r"$3 + Abs(y)$", "raw `Abs(`"),
        ];
        for (sample, label) in bad_renders {
            let mut hit = false;
            for (pat, _, _) in RENDER_BANLIST {
                if regex::Regex::new(pat).unwrap().is_match(sample) {
                    hit = true;
                    break;
                }
            }
            assert!(hit, "banlist missed `{label}` in `{sample}`");
        }
    }

    #[test]
    fn system_prompt_includes_critical_rules() {
        // Spot-check that the prompt mentions every non-obvious failure mode
        // we've seen in past generations and the agent-loop tool names.
        let must_mention = [
            "BACKWARD",
            "BANNED",
            "where f(x)",
            "infinity",
            "derivative_of",
            "sqrt(",
            "**",
            "render_samples",
            "save",
            "audit",
            "DISPLAY FUNCTIONS",
            "BUILTIN FUNCTIONS",
        ];
        for needle in must_mention {
            assert!(
                SYSTEM_PROMPT.contains(needle),
                "system prompt missing critical guidance: `{needle}`"
            );
        }
    }

    #[test]
    fn render_banlist_passes_clean_renders() {
        let good_renders = [
            r"$\lim_{x \to \infty} \frac{1}{x}$",
            r"$\sqrt{x} + x^{\frac{7}{2}}$",
            r"$\frac{d}{dx}\left[3 x^2\right]$",
            r"$\begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}$",
        ];
        for sample in good_renders {
            for (pat, why, _) in RENDER_BANLIST {
                let re = regex::Regex::new(pat).unwrap();
                assert!(
                    !re.is_match(sample),
                    "false positive: `{pat}` ({why}) matched clean render `{sample}`"
                );
            }
        }
    }
}
