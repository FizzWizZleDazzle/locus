//! AI YAML generation pipeline
//!
//! Fires N requests concurrently, validates as they return,
//! retries failures independently. No waiting in sequence.

use std::sync::Arc;

const MAX_RETRIES: usize = 3;

/// Generate multiple problem YAMLs concurrently.
/// Returns vec of (index, Result<yaml, error>).
pub async fn generate_batch(
    topic: &str,
    difficulty: &str,
    api_key: &str,
    model: &str,
    count: usize,
    concurrency: usize,
) -> Vec<Result<String, String>> {
    let client = Arc::new(reqwest::Client::new());
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));

    let mut handles = Vec::new();

    for i in 0..count {
        let client = client.clone();
        let sem = semaphore.clone();
        let topic = topic.to_string();
        let difficulty = difficulty.to_string();
        let api_key = api_key.to_string();
        let model = model.to_string();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            eprintln!("[{}/{}] Starting...", i + 1, count);

            let result = generate_one(&client, &topic, &difficulty, &api_key, &model).await;

            match &result {
                Ok(_) => eprintln!("[{}/{}] OK", i + 1, count),
                Err(e) => eprintln!("[{}/{}] Failed: {}", i + 1, count, e),
            }
            result
        });

        handles.push(handle);
    }

    let mut results = Vec::with_capacity(count);
    for handle in handles {
        results.push(handle.await.unwrap_or_else(|e| Err(format!("Task panic: {e}"))));
    }
    results
}

/// Single YAML generation with retry loop
async fn generate_one(
    client: &reqwest::Client,
    topic: &str,
    difficulty: &str,
    api_key: &str,
    model: &str,
) -> Result<String, String> {
    let mut last_yaml = String::new();
    let mut last_errors = Vec::new();

    for attempt in 0..=MAX_RETRIES {
        let prompt = if attempt == 0 {
            build_initial_prompt(topic, difficulty)
        } else {
            build_retry_prompt(topic, difficulty, &last_yaml, &last_errors)
        };

        let yaml = call_llm(client, api_key, model, &prompt).await?;
        let cleaned = extract_yaml(&yaml);

        match validate_yaml(&cleaned) {
            Ok(()) => return Ok(cleaned),
            Err(errors) => {
                eprintln!(
                    "  Attempt {}/{}: {} error(s)",
                    attempt + 1,
                    MAX_RETRIES + 1,
                    errors.len()
                );
                for e in &errors {
                    eprintln!("    - {e}");
                }
                last_yaml = cleaned;
                last_errors = errors;
            }
        }
    }

    Err(format!(
        "Failed after {} attempts. Last errors: {}",
        MAX_RETRIES + 1,
        last_errors.join("; ")
    ))
}

fn build_initial_prompt(topic: &str, difficulty: &str) -> String {
    format!(
        "{SYSTEM_PROMPT}\n\n\
         Generate a problem YAML for:\n\
         - Topic: {topic}\n\
         - Difficulty: {difficulty}\n\n\
         Output ONLY the YAML, nothing else."
    )
}

fn build_retry_prompt(topic: &str, difficulty: &str, yaml: &str, errors: &[String]) -> String {
    format!(
        "{SYSTEM_PROMPT}\n\n\
         Your previous YAML for topic '{topic}' (difficulty: {difficulty}) had errors:\n\
         ```yaml\n{yaml}\n```\n\n\
         Errors:\n{error_list}\n\n\
         Fix these errors and output ONLY the corrected YAML, nothing else.",
        error_list = errors
            .iter()
            .map(|e| format!("- {e}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
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
            Err(e) => errors.push(format!("Generation attempt {}: {e}", i + 1)),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn extract_yaml(response: &str) -> String {
    let trimmed = response.trim();
    if trimmed.starts_with("```") {
        let lines: Vec<&str> = trimmed.lines().collect();
        let start = 1; // skip ```yaml line
        let end = if lines.last().map_or(false, |l| l.trim() == "```") {
            lines.len() - 1
        } else {
            lines.len()
        };
        return lines[start..end].join("\n");
    }
    trimmed.to_string()
}

async fn call_llm(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    let body = serde_json::json!({
        "model": model,
        "max_tokens": 2048,
        "messages": [{
            "role": "user",
            "content": prompt
        }]
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

    json["content"][0]["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "No text in API response".to_string())
}

const SYSTEM_PROMPT: &str = r#"You are a math problem generator. You output YAML files in the LocusDSL format.

RULES:
- Output ONLY valid YAML. No explanations, no markdown, no commentary.
- NEVER write LaTeX. Use {var} refs and {display_func()} for all math rendering.
- All math expressions use plain notation: *, ^, /, +, - (compatible with SymEngine).
- Variables are either samplers or derived expressions. Never write code.
- Strings containing : or { must be quoted in YAML.

SAMPLER TYPES:
  integer(lo, hi)       # random integer in [lo, hi]
  nonzero(lo, hi)       # integer excluding 0
  decimal(lo, hi, n)    # decimal with n places
  choice(a, b, c)       # pick from list
  prime(lo, hi)         # random prime
  rational(lo, hi, max) # random fraction

BUILT-IN FUNCTIONS (for derived variables):
  derivative(expr, var)         # differentiate
  integral(expr, var)           # antiderivative
  evaluate(expr, var, value)    # plug in value
  solve(expr, var)              # find roots (linear/quadratic)
  expand(expr)                  # expand expression
  simplify(expr)                # simplify
  sqrt(expr)                    # square root
  abs(expr)                     # absolute value
  sin, cos, tan, asin, acos, atan, log, ln, exp  # standard functions
  round(expr, places)           # round decimal
  max(a, b), min(a, b)          # min/max

DISPLAY FUNCTIONS (for question/solution text):
  {var_name}                        # inline math: $...$
  {derivative_of(f, x)}            # d/dx[f]
  {integral_of(f, x)}              # ∫ f dx
  {definite_integral_of(f, x, a, b)} # ∫_a^b f dx
  {limit_of(f, x, val)}            # lim_{x→val} f
  {equation(lhs, rhs)}             # lhs = rhs
  {system(eq1, eq2)}               # system of equations
  {matrix_of(M)}                   # pmatrix
  {det_of(M)}                      # |M|
  {vec(v)}                         # vector arrow
  {abs_of(x)}                      # |x|
  {set_of(s)}                      # {s}
  {binomial(n, k)}                 # (n choose k)

ANSWER:
  - "answer" field is a variable name (not an expression)
  - For multi-value answers, use intermediate variables, then: "answer: var_name"
  - For tuples: "answer: x, y" (comma-separated variable names)
  - Parser auto-detects answer_type, or override with answer_type field
  - If the answer is a computed expression (e.g. num/den), define it as a variable first

CONSTRAINTS:
  - List boolean conditions: a != 0, b > a, is_integer(answer), etc.
  - Parser resamples up to 1000 times to satisfy all constraints

IMPORTANT:
  - Every expression in a display function or equation() gets variable substitution.
    So {equation(a*x + b, c)} will substitute a, b, c with their numeric values.
  - If you need to show intermediate computations, define them as variables.
  - Strings in YAML that contain { or : MUST be quoted.

STRUCTURE:
```yaml
topic: main_topic/subtopic
difficulty: easy|medium|hard|very_hard|competition
calculator: none|scientific|graphing

variables:
  name1: sampler_or_expression
  name2: sampler_or_expression
  answer: derived_or_function

constraints:
  - condition1
  - condition2

question: "Text with {var} refs and {display_func()} calls"

answer: answer_variable_name
mode: equivalent

solution:
  - "Step 1 with {var} refs"
  - "Step 2 with {display_func()} calls"
```

EXAMPLE 1 (arithmetic):
```yaml
topic: arithmetic/addition
difficulty: easy
calculator: none

variables:
  a: integer(10, 99)
  b: integer(10, 99)
  answer: a + b

question: What is {a} + {b}?

answer: answer

solution:
  - Add {a} and {b}
  - Answer is {answer}
```

EXAMPLE 2 (calculus):
```yaml
topic: calculus/derivative_rules
difficulty: medium
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
```

EXAMPLE 3 (algebra with solve):
```yaml
topic: algebra1/quadratic_formula
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
```

EXAMPLE 4 (definite integral):
```yaml
topic: calculus/definite_integrals
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
  answer: F_hi - F_lo

constraints:
  - lo < hi
  - is_integer(answer)

question: "Evaluate {definite_integral_of(f, x, lo, hi)}"

answer: answer

solution:
  - "Find antiderivative: {integral_of(f, x)} = {F} + C"
  - "Evaluate from {lo} to {hi}"
  - "= {answer}"
```

VALID TOPICS:
arithmetic/{addition,subtraction,multiplication,long_division,fractions,decimals,mixed_numbers,order_of_operations,percentages,ratios_proportions}
algebra1/{one_step_equations,two_step_equations,multi_step_equations,linear_inequalities,compound_inequalities,exponent_rules,polynomial_operations,factoring_gcf,factoring_trinomials,quadratic_formula,graphing_lines,slope_and_intercept,systems_substitution,systems_elimination}
algebra2/{complex_number_operations,complex_number_equations,exponential_equations,exponential_growth_decay,logarithm_properties,logarithmic_equations,radical_expressions,radical_equations,rational_expressions,rational_equations,arithmetic_sequences,geometric_sequences}
geometry/{angle_relationships,triangle_properties,triangle_congruence,similar_triangles,pythagorean_theorem,right_triangle_trig,perimeter,area_of_polygons,circle_theorems,arc_length_sectors,surface_area,volume,coordinate_geometry}
precalculus/{domain_and_range,function_composition,inverse_functions,transformations,unit_circle,graphing_trig,trig_identities,sum_difference_formulas,inverse_trig_functions,law_of_sines_cosines,vector_operations,dot_cross_product,polar_coordinates,polar_curves}
calculus/{continuity,lhopitals_rule,limits_at_infinity,derivative_rules,chain_rule,implicit_differentiation,related_rates,curve_sketching,optimization,antiderivatives,u_substitution,integration_by_parts,definite_integrals,area_between_curves,volumes_of_revolution}
multivariable_calculus/{partial_derivatives,gradient,directional_derivatives,lagrange_multipliers,double_integrals,triple_integrals,change_of_variables,line_integrals,greens_theorem,stokes_divergence}
linear_algebra/{matrix_arithmetic,matrix_inverses,determinants,row_reduction,eigenvalues,diagonalization,vector_spaces,subspaces,linear_independence,linear_transformations}
differential_equations/{separable_equations,first_order_linear,exact_equations,homogeneous_equations,second_order_constant,characteristic_equation,undetermined_coefficients,variation_of_parameters,laplace_transforms,systems_of_odes}
"#;
