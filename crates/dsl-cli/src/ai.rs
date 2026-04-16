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

    // Full generation test — this catches constraint, symengine, template, and grading errors
    match locus_dsl::generate(&spec) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(vec![format!("Generation error: {e}")]),
    }

    // Below checks are redundant now but kept as fallback
    #[allow(unreachable_code)]
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

# GOLDEN RULE

Design BACKWARDS from the answer. Pick clean answer first, build problem around it.
Every variable is either a sampler OR a simple formula from other variables.
The answer must be GUARANTEED correct by construction — never by filtering.

# YAML STRUCTURE

```
topic: main/sub
calculator: none
variables:
  name: value
constraints:
  - simple_condition
question: "text with {var} refs"
answer: variable_name
solution:
  - "step with {var} refs"
```

# ALLOWED SAMPLERS

integer(lo, hi)          — random int in [lo, hi]
nonzero(lo, hi)          — int excluding 0
choice(a, b, c, ...)     — pick from list
prime(lo, hi)            — random prime
decimal(lo, hi, places)  — decimal with N places

# ALLOWED FUNCTIONS IN VARIABLES

derivative(expr, var)    — differentiate
integral(expr, var)      — antiderivative (polynomial only)
expand(expr)             — expand products
solve(expr, var)         — roots of expr=0 (linear/quadratic only, returns comma-separated roots)
evaluate(expr, var, val) — substitute var=val (EXACTLY 3 args, returns a number)
sqrt(expr)               — square root
abs(expr)                — absolute value
floor(x)  ceil(x)  round(x, n)  mod(a, b)  max(a, b)  min(a, b)
sin(x)  cos(x)  tan(x)  log(x)  ln(x)  exp(x)

# ALLOWED IN {refs} IN QUESTION/SOLUTION TEXT

{variable_name}                           — renders as $value$
{derivative_of(f, x)}                     — renders d/dx[f]
{integral_of(f, x)}                       — renders ∫f dx
{definite_integral_of(f, x, a, b)}        — renders ∫_a^b f dx
{limit_of(f, x, val)}                     — renders lim
{equation(lhs, rhs)}                      — renders lhs = rhs (EXACTLY 2 args)
{system(eq1, eq2)}                        — renders cases

# BANNED — DO NOT USE ANY OF THESE

- y', y'', y''' (prime notation) — SymEngine cannot parse these
- = sign in variable definitions — variables are expressions, not equations
- if/else, implies, ternary — no Python syntax
- % operator — use mod(a, b) instead
- ∞, ∪, ≤, ≥ or any unicode math symbols
- [index] notation like result[0] — no array indexing
- evaluate() with more than 3 args — chain calls instead
- equation() with more than 2 args
- gcd(), mod(), abs(), is_integer() in CONSTRAINTS
- Constraints with divisibility (mod(x, y) == 0)
- Constraints comparing computed values (left_side == right_side)
- answer_type values other than: numeric, expression, tuple, set, boolean, word
- Invented display functions not in the list above
- Comments in the variables section
- $, $$, or any LaTeX markup
- Lists or arrays as variable values: [a, b, c]
- Equations as variable values: x^2 + y^2 = 4

# CONSTRAINTS — MAXIMUM 2, TRIVIALLY SATISFIABLE

ONLY these patterns are allowed:
  a != b          (two samplers differ)
  a < b           (ordering)
  a > 0           (positive)

If you need the answer to be an integer, CONSTRUCT it as an integer by design.
If you need gcd(a,b)==1, use prime() samplers or choice() with coprime values.
If you need divisibility, compute: answer: integer(...), then rhs: a * answer.

# MULTI-VARIABLE EVALUATE

To evaluate f(x,y) at x=2, y=3:
  step1: evaluate(f, x, 2)
  step2: evaluate(step1, y, 3)

NEVER: evaluate(f, x, 2, y, 3)

# COMPOUND INEQUALITIES

Do NOT use answer_type: interval or inequality.
Return boundary values as a tuple: answer: lo, hi (answer_type: tuple)

# DIFFERENTIAL EQUATIONS

Do NOT use y' or y'' notation. Use derivative(y, x) for y' and define:
  char_eq: r^2 + a*r + b     (characteristic equation in terms of r)
  roots: solve(char_eq, r)

# SYSTEMS OF EQUATIONS

Pick x and y values first, compute RHS from them:
  x: integer(-5, 5)
  y: integer(-5, 5)
  a1: nonzero(-5, 5)
  b1: nonzero(-5, 5)
  c1: a1*x + b1*y
  a2: nonzero(-5, 5)
  b2: nonzero(-5, 5)
  c2: a2*x + b2*y
  answer: x, y
Constraint: a1*b2 != a2*b1 (ensures unique solution — ALWAYS satisfiable with these ranges)

# FACTORING

Build the factored form first, expand for the question:
  r1: integer(-8, 8)
  r2: integer(-8, 8)
  factored: (x - r1)*(x - r2)
  expanded: expand(factored)
  answer: solve(expanded, x)
  answer_type: set
Constraint: r1 != r2

For GCF factoring, build from GCF:
  gcf: choice(2, 3, 4, 5)
  c1: integer(1, 6)
  c2: integer(1, 6)
  f: gcf*c1*x + gcf*c2
  answer: gcf*(c1*x + c2)

# GRAPHING / SLOPES

Pick slope and point, compute other values:
  m: nonzero(-5, 5)
  x1: integer(-4, 4)
  y1: integer(-6, 6)
  b: y1 - m*x1
  answer: m

# AREA BETWEEN CURVES / DEFINITE INTEGRALS

Compute step by step:
  f: a*x^n
  F: integral(f, x)
  F_hi: evaluate(F, x, hi)
  F_lo: evaluate(F, x, lo)
  answer: F_hi - F_lo
NEVER: evaluate(F, x, hi) - evaluate(F, x, lo) as one expression

# SEQUENCES

Pick first term and common difference/ratio, compute directly:
  a1: integer(2, 10)
  d: nonzero(-5, 5)
  n: integer(5, 15)
  answer: a1 + (n - 1)*d

# PERCENTAGES / GROWTH

Use simple multiplication:
  original: choice(100, 200, 250, 400, 500)
  rate: choice(10, 15, 20, 25, 30)
  increase: original * rate / 100
  answer: original + increase

# DIFFICULTY CALIBRATION (no difficulty field — injected by system)

very_easy/easy: 1 step, single-digit or small two-digit numbers
medium: 2-3 steps, moderate numbers
hard/very_hard: multi-step, fractions, negative numbers
competition: creative setup, elegant backward construction

# VERIFICATION

Before outputting, mentally substitute the MIDPOINT of every sampler range and trace through:
1. Do all variables resolve to concrete values?
2. Do all constraints pass?
3. Is the answer a clean number or simple expression?
4. Do all {refs} in question/solution match defined variable names?
If ANY check fails, redesign from scratch."#;

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
