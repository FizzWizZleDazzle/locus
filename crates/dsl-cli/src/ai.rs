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

    // Check all variable definitions reference valid samplers or functions
    for (name, def) in &spec.variables {
        let d = def.trim();
        if locus_dsl::sampler::is_sampler(d) {
            // Verify sampler parses
            if let Err(e) = locus_dsl::sampler::sample(d) {
                errors.push(format!("Variable '{name}': {e}"));
            }
        } else if locus_dsl::functions::is_builtin_call(d) {
            // Verify function name exists (don't evaluate — just check name)
            if let Some(paren) = d.find('(') {
                let func = &d[..paren];
                if !locus_dsl::functions::BUILTIN_FUNCTIONS.contains(&func) {
                    errors.push(format!("Variable '{name}': unknown function '{func}'"));
                }
            }
        }
        // Derived expressions validated implicitly by SymEngine at generate time
    }

    // Check template refs exist as variables or display functions
    let var_names: std::collections::HashSet<&str> = spec.variables.keys().map(|s| s.as_str()).collect();
    for field_name in ["question"] {
        let field = match field_name {
            "question" => &spec.question,
            _ => continue,
        };
        // Find all {ref} patterns
        let mut i = 0;
        let bytes = field.as_bytes();
        while i < bytes.len() {
            if bytes[i] == b'{' && (i + 1 >= bytes.len() || bytes[i + 1] != b'{') {
                if let Some(end) = field[i + 1..].find('}') {
                    let ref_content = field[i + 1..i + 1 + end].trim();
                    if !ref_content.is_empty() {
                        // Check if it's a var name or display function
                        let is_var = var_names.contains(ref_content);
                        let is_display = ref_content.contains('(') && ref_content.ends_with(')');
                        let has_operators = ref_content.contains('+') || ref_content.contains('-')
                            || ref_content.contains('*') || ref_content.contains('/');
                        if !is_var && !is_display && !has_operators {
                            errors.push(format!("Question: {{{}}} is not a defined variable or display function", ref_content));
                        }
                    }
                    i = i + 1 + end + 1;
                    continue;
                }
            }
            i += 1;
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

    // Retry HTTP errors with backoff
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
                    eprintln!("    HTTP error, retrying in {}s: {e}", (attempt + 1) * 5);
                    tokio::time::sleep(std::time::Duration::from_secs((attempt + 1) as u64 * 5)).await;
                    continue;
                }
                return Err(format!("HTTP error after 3 attempts: {e}"));
            }
        };

        if resp.status().as_u16() == 429 || resp.status().as_u16() == 529 {
            if attempt < 2 {
                eprintln!("    Rate limited, retrying in {}s", (attempt + 1) * 10);
                tokio::time::sleep(std::time::Duration::from_secs((attempt + 1) as u64 * 10)).await;
                continue;
            }
        }

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

        return Ok(format!("topic:{text}"));
    }

    Err("Unreachable".into())
}

// =============================================================================
// System prompt (static, cached by Anthropic)
// =============================================================================

const SYSTEM_PROMPT: &str = r#"You generate math problem YAML files. Output ONLY valid YAML.

# DESIGN METHOD: WORK BACKWARDS FROM THE ANSWER

ALWAYS design problems answer-first:
1. Pick a CLEAN answer (small integer, simple fraction, clean expression).
2. Work BACKWARDS to build the problem that produces this answer.
3. Choose variable ranges that GUARANTEE the answer works.
4. Constraints should be trivially satisfied by your construction — not aspirational filters.

Example of backward design:
- Want students to solve a quadratic with roots 3 and -5
- Pick r1: integer(-8, 8), r2: integer(-8, 8), constrain r1 != r2
- Build f: (x - r1) * (x - r2), expanded: expand(f)
- The roots are guaranteed by construction — no need for is_integer(roots)

NEVER do forward design:
- Do NOT pick random coefficients then hope the answer is clean
- Do NOT use is_integer(answer) to filter — construct integrality instead
- Do NOT write 4+ constraints to force a specific outcome

# FORMAT

topic: main_topic/subtopic
calculator: none|scientific|graphing
variables: (samplers, derived, or functions)
constraints: (max 3, trivially satisfiable)
question: "Text with {var} and {display_func()} refs"
answer: variable_name
solution: (step-by-step with {var} refs)

# VARIABLES

Samplers: integer(lo, hi)  nonzero(lo, hi)  decimal(lo, hi, places)  choice(a, b, c)  prime(lo, hi)
Derived: f: a * x^n + b
Functions: derivative(expr, var)  integral(expr, var)  solve(expr, var)  expand(expr)  simplify(expr)  evaluate(expr, var, value)  sqrt(x)  abs(x)  floor(x)  ceil(x)  round(x, n)  mod(a, b)  max(a, b)  min(a, b)  sin cos tan asin acos atan log ln exp

# DISPLAY FUNCTIONS (for {ref} in question/solution text)

{var_name}  {derivative_of(f, x)}  {integral_of(f, x)}  {definite_integral_of(f, x, a, b)}
{limit_of(f, x, val)}  {equation(lhs, rhs)}  {system(eq1, eq2)}  {matrix_of(M)}
{det_of(M)}  {vec(v)}  {norm(v)}  {abs_of(x)}  {set_of(s)}  {binomial(n, k)}

# STRICT RULES

1. Inside {} in question/solution: ONLY variable names or display functions above.
   {b*c} is INVALID — define product: b*c as variable, use {product}.
   {sqrt(x)} is INVALID — define val: sqrt(x) as variable, use {val}.

2. The answer field must be a variable name, never an expression.

3. YAML strings with { or : MUST be double-quoted.

4. Never write LaTeX, $, $$, or %. Use display functions for all math notation.

5. Max 3 constraints. Every constraint must pass for >80% of the sample space.
   If you need is_integer, redesign so integrality is guaranteed by construction.

6. No difficulty field (injected by system). Calibrate complexity to the difficulty in the user message.

7. Matrices: use matrix(rows, cols, lo, hi) sampler, never manual [[...]].

8. Before outputting: mentally substitute one concrete value for each sampler and verify all constraints pass and the answer is clean.

9. INTERVALS: If the answer is an interval, define the bounds as variables and build the interval string.
   Example for compound inequality answer (-3, 5]:
     lo: ...computed...
     hi: ...computed...
     answer: "open:lo,closed:hi"   ← WRONG, this is a literal string
   Instead, just use answer_type: tuple and return the bounds, OR avoid interval type entirely.
   For compound inequalities, return the boundary values as a tuple: answer: lo, hi"#;

// =============================================================================
// Examples (selected by topic relevance)
// =============================================================================

// Examples demonstrate backward design: answer chosen first, problem built around it

const EXAMPLE_ARITHMETIC: &str = r#"## Backward design: pick nice denominators, build fractions from them
topic: arithmetic/fractions
calculator: none
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
solution:
  - "Common denominator: {b} x {d} = {den}"
  - "{a}/{b} + {c}/{d} = {answer}"
"#;

const EXAMPLE_CALCULUS: &str = r#"## Backward design: pick simple polynomial, integral is always clean
topic: calculus/derivative_rules
calculator: none
variables:
  a: nonzero(-8, 8)
  n: integer(2, 6)
  f: a * x^n
  answer: derivative(f, x)
question: "Find {derivative_of(f, x)}"
answer: answer
solution:
  - "Apply the power rule to {f}"
  - "{derivative_of(f, x)} = {answer}"
"#;

const EXAMPLE_ALGEBRA: &str = r#"## Backward design: pick roots first, build quadratic from them
## Roots are guaranteed integer by construction — no is_integer constraint needed
topic: algebra1/quadratic_formula
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

const EXAMPLE_GEOMETRY: &str = r#"## Backward design: use known Pythagorean triples via choice()
## Hypotenuse is integer by construction — no is_integer constraint needed
topic: geometry/pythagorean_theorem
calculator: scientific
variables:
  triple: choice(1, 2, 3, 4)
  a: choice(3, 5, 8, 7)
  b: choice(4, 12, 15, 24)
  c: choice(5, 13, 17, 25)
  answer: c
question: "Find the hypotenuse of a right triangle with legs {a} and {b}."
answer: answer
solution:
  - "Pythagorean theorem: a^2 + b^2 = c^2"
  - "c = {answer}"
"#;
