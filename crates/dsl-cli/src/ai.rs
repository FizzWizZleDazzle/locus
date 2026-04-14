//! AI YAML generation pipeline
//!
//! Fires N requests concurrently, validates as they return,
//! retries failures independently. Uses prompt caching + prefilling.

use std::sync::Arc;

const MAX_RETRIES: usize = 3;

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
        results.push(handle.await.unwrap_or_else(|e| Err(format!("Task panic: {e}"))));
    }
    results
}

async fn generate_one(
    client: &reqwest::Client,
    topic: &str,
    difficulty: &str,
    api_key: &str,
    model: &str,
) -> Result<String, String> {
    let mut last_yaml = String::new();
    let mut last_errors = Vec::new();

    let examples = select_examples(topic);

    for attempt in 0..=MAX_RETRIES {
        let user_msg = if attempt == 0 {
            format!(
                "Generate a problem YAML for topic: {topic}, difficulty: {difficulty}\n\n\
                 Make it pedagogically useful with appropriate constraints for clean numbers."
            )
        } else {
            format!(
                "Your YAML for '{topic}' ({difficulty}) had errors:\n\
                 ```yaml\n{last_yaml}\n```\n\
                 Errors:\n{}\n\
                 Fix and output ONLY the corrected YAML.",
                last_errors.iter().map(|e| format!("- {e}")).collect::<Vec<_>>().join("\n")
            )
        };

        let yaml = call_llm(client, api_key, model, &examples, &user_msg).await?;
        let cleaned = extract_yaml(&yaml);

        // Force difficulty to match CLI arg
        let cleaned = inject_difficulty(&cleaned, difficulty);

        match validate_yaml(&cleaned) {
            Ok(()) => return Ok(cleaned),
            Err(errors) => {
                eprintln!("  Attempt {}/{}: {} error(s)", attempt + 1, MAX_RETRIES + 1, errors.len());
                for e in &errors { eprintln!("    - {e}"); }
                last_yaml = cleaned;
                last_errors = errors;
            }
        }
    }

    Err(format!(
        "Failed after {} attempts. Errors: {}",
        MAX_RETRIES + 1,
        last_errors.join("; ")
    ))
}

/// Select 2 relevant examples based on topic
fn select_examples(topic: &str) -> String {
    let main = topic.split('/').next().unwrap_or("");

    let relevant = match main {
        "arithmetic" => EXAMPLE_ARITHMETIC,
        "calculus" | "multivariable_calculus" => EXAMPLE_CALCULUS,
        "algebra1" | "algebra2" => EXAMPLE_ALGEBRA,
        "geometry" => EXAMPLE_GEOMETRY,
        _ => EXAMPLE_ALGEBRA,
    };

    // Always include one contrasting example for variety
    let contrast = if relevant == EXAMPLE_CALCULUS {
        EXAMPLE_GEOMETRY
    } else {
        EXAMPLE_CALCULUS
    };

    format!("{relevant}\n\n{contrast}")
}

/// Override difficulty in generated YAML to match CLI arg
fn inject_difficulty(yaml: &str, difficulty: &str) -> String {
    let re = regex::Regex::new(r"(?m)^difficulty:.*$").unwrap();
    if re.is_match(yaml) {
        re.replace(yaml, format!("difficulty: {difficulty}")).to_string()
    } else {
        // No difficulty line — insert after topic line
        let topic_re = regex::Regex::new(r"(?m)^(topic:.*)$").unwrap();
        topic_re.replace(yaml, format!("$1\ndifficulty: {difficulty}")).to_string()
    }
}

fn validate_yaml(yaml: &str) -> Result<(), Vec<String>> {
    let spec = match locus_dsl::parse(yaml) {
        Ok(s) => s,
        Err(e) => return Err(vec![format!("Parse error: {e}")]),
    };
    let mut errors = Vec::new();
    for i in 0..3 {
        match locus_dsl::generate(&spec) {
            Ok(_) => {}
            Err(e) => errors.push(format!("Generation {}: {e}", i + 1)),
        }
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn extract_yaml(response: &str) -> String {
    let trimmed = response.trim();
    if trimmed.starts_with("```") {
        let lines: Vec<&str> = trimmed.lines().collect();
        let end = if lines.last().map_or(false, |l| l.trim() == "```") {
            lines.len() - 1
        } else {
            lines.len()
        };
        return lines[1..end].join("\n");
    }
    // If response starts with "topic:", it's already clean YAML
    trimmed.to_string()
}

async fn call_llm(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    examples: &str,
    user_message: &str,
) -> Result<String, String> {
    let system_with_examples = format!("{SYSTEM_PROMPT}\n\n# EXAMPLES\n\n{examples}");

    let body = serde_json::json!({
        "model": model,
        "max_tokens": 2048,
        "temperature": 0.4,
        "system": [{
            "type": "text",
            "text": system_with_examples,
            "cache_control": {"type": "ephemeral"}
        }],
        "messages": [
            {"role": "user", "content": user_message},
            {"role": "assistant", "content": "topic:"}
        ]
    });

    let resp = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API error {status}: {text}"));
    }

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse error: {e}"))?;

    let text = json["content"][0]["text"]
        .as_str()
        .ok_or_else(|| "No text in API response".to_string())?;

    // Prepend "topic:" since we prefilled it
    Ok(format!("topic:{text}"))
}

// =============================================================================
// System prompt (static, cached by Anthropic)
// =============================================================================

const SYSTEM_PROMPT: &str = r#"You generate math problem YAML files in LocusDSL format. Output ONLY valid YAML.

# FORMAT

topic: main_topic/subtopic
calculator: none|scientific|graphing
variables: (samplers, derived expressions, or function calls)
constraints: (boolean conditions for clean numbers)
question: "Text with {var} and {display_func()} refs"
answer: variable_name
format: (optional) factored|expanded|simplified|reduced_fraction|"predicate expression"
solution: (step-by-step with {var} refs)

# VARIABLE TYPES

Samplers: integer(lo, hi)  nonzero(lo, hi)  decimal(lo, hi, places)  choice(a, b, c)  prime(lo, hi)  rational(lo, hi, max_denom)
Derived: f: a * x^n + b  (plain math using other vars)
Functions: derivative(expr, var)  integral(expr, var)  solve(expr, var)  expand(expr)  simplify(expr)  evaluate(expr, var, value)  sqrt(x)  abs(x)  sin cos tan asin acos atan log ln exp  round(x, n)  max(a, b)  min(a, b)

# DISPLAY FUNCTIONS (produce LaTeX in question/solution text)

{var}  {derivative_of(f, x)}  {integral_of(f, x)}  {definite_integral_of(f, x, a, b)}  {limit_of(f, x, val)}  {equation(lhs, rhs)}  {system(eq1, eq2)}  {matrix_of(M)}  {det_of(M)}  {vec(v)}  {norm(v)}  {abs_of(x)}  {set_of(s)}  {binomial(n, k)}

Display functions substitute variables — {equation(a*x + b, c)} shows numeric values.

# RULES

1. The answer field must be a variable name. Define intermediate variables for computed answers.
2. YAML strings containing { or : must be quoted.
3. Use display functions instead of LaTeX. Never write \frac, \int, etc.
4. Use constraints to ensure clean answers (is_integer, nonzero denominators, distinct roots).
5. Solution steps should show work, not just state the answer.
   FORMAT FIELD: Use when the question asks for a specific form.
   - Tags: factored, expanded, simplified, reduced_fraction
   - Custom: "degree(answer, x) == 2", "count(log, answer) == 1"
   - When using format: factored, the answer variable must BE the factored expression (e.g. gcf*(c1*x + c2)), not the expanded form.
6. Each problem tests one concept clearly.
7. Constraints support "and" / "or" for compound conditions. Prefer separate lines when possible.
8. Do NOT include a "difficulty" field — it is injected by the system. But calibrate problem complexity to the difficulty level given in the user message:
   - easy: single-step, small numbers, direct application
   - medium: 2-3 steps, moderate numbers, standard techniques
   - hard: multi-step, larger numbers or fractions, combined concepts
   - very_hard: complex setup, non-obvious approach, edge cases
   - competition: creative insight required, elegant solutions

# ANSWER TYPES (auto-detected from value, or set answer_type)

numeric: 42  expression: 3*x+5  tuple: "answer: x, y"  set: answer_type: set  boolean: true/false  interval: open:0,closed:5  matrix: [[1,2],[3,4]]"#;

// =============================================================================
// Examples (selected by topic relevance)
// =============================================================================

const EXAMPLE_ARITHMETIC: &str = r#"topic: arithmetic/fractions
difficulty: easy
calculator: none

variables:
  a: integer(1, 9)
  b: integer(2, 9)
  c: integer(1, 9)
  d: integer(2, 9)
  num: a*d + c*b
  den: b*d
  answer: num/den

constraints:
  - b != d
  - a < b
  - c < d

question: "Add: {a}/{b} + {c}/{d}"
answer: answer
solution:
  - "Common denominator: {b} x {d} = {den}"
  - "{a}/{b} + {c}/{d} = {answer}"
"#;

const EXAMPLE_CALCULUS: &str = r#"topic: calculus/definite_integrals
difficulty: medium
calculator: none

variables:
  a: nonzero(-5, 5)
  n: integer(1, 4)
  f: a * x^n
  lo: integer(0, 3)
  hi: integer(4, 8)
  F: integral(f, x)
  F_hi: evaluate(F, x, hi)
  F_lo: evaluate(F, x, lo)
  result: F_hi - F_lo

constraints:
  - lo < hi
  - is_integer(result)

question: "Evaluate {definite_integral_of(f, x, lo, hi)}"
answer: result
solution:
  - "Antiderivative: {integral_of(f, x)} = {F} + C"
  - "Evaluate from {lo} to {hi}"
  - "= {result}"
"#;

const EXAMPLE_ALGEBRA: &str = r#"topic: algebra1/quadratic_formula
difficulty: medium
calculator: none

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
solution:
  - "Start with {equation(expanded, 0)}"
  - "Factor: {equation(f, 0)}"
  - "x = {r1} or x = {r2}"
"#;

const EXAMPLE_GEOMETRY: &str = r#"topic: geometry/pythagorean_theorem
difficulty: easy
calculator: scientific

variables:
  a: choice(3, 5, 6, 7, 8, 9)
  b: choice(4, 8, 9, 12, 15)
  a2: a^2
  b2: b^2
  sum_sq: a2 + b2
  answer: sqrt(sum_sq)

constraints:
  - a < b
  - is_integer(answer)

question: "Find the hypotenuse of a right triangle with legs {a} and {b}."
answer: answer
solution:
  - "Pythagorean theorem: a^2 + b^2 = c^2"
  - "{a}^2 + {b}^2 = {sum_sq}"
  - "c = {answer}"
"#;
