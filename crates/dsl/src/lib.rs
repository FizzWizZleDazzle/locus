//! LocusDSL — Problem generation from YAML definitions
//!
//! AI describes problems in structured YAML. This crate handles all
//! computation, LaTeX formatting, validation, and grading.
//!
//! See `docs/DSL_SPEC.md` for the full specification.

pub mod answer;
pub mod constraints;
pub mod display;
pub mod error;
pub mod format;
pub mod functions;
pub mod gpu;
pub mod latex;
pub mod resolver;
pub mod sampler;
pub mod spec;
pub mod template;
pub mod validate;

pub mod diagram;

use error::DslError;
use spec::{ProblemSpec, Variant};

pub use gpu::{Executor, enumerate as enumerate_problems};

/// Parse a YAML string into a ProblemSpec
pub fn parse(yaml: &str) -> Result<ProblemSpec, DslError> {
    spec::parse_yaml(yaml)
}

/// Generate one problem from an explicit variant. No validation pass.
pub fn generate_fast(spec: &ProblemSpec, variant: &Variant) -> Result<ProblemOutput, DslError> {
    let vars = resolver::resolve(&variant.variables, &variant.constraints)?;
    let question_latex = template::render(&variant.question, &vars)?;
    let answer_key = answer::format(&vars, &variant.answer, variant.answer_type.as_deref())?;
    let solution_latex = variant
        .solution
        .as_ref()
        .map(|steps| template::render_steps(steps, &vars))
        .transpose()?;

    let answer_type = answer::infer_type(&answer_key, variant.answer_type.as_deref());
    let difficulty = variant.difficulty.as_ref().unwrap_or(&spec.difficulty);

    let question_image_url = match &variant.diagram {
        Some(d) => diagram::render(d, &vars)?,
        None => String::new(),
    };

    Ok(ProblemOutput {
        question_latex,
        answer_key,
        solution_latex: solution_latex.unwrap_or_default(),
        difficulty: sampler::sample_difficulty(difficulty)?,
        main_topic: spec.topic.main.clone(),
        subtopic: spec.topic.sub.clone(),
        grading_mode: variant.mode.clone().unwrap_or_else(|| "equivalent".into()),
        answer_type,
        calculator_allowed: spec.calculator.clone().unwrap_or_else(|| "none".into()),
        question_image_url,
        time_limit_seconds: spec.time,
        variant_name: variant.name.clone(),
    })
}

/// Generate a problem with full validation (self-grade + KaTeX check).
pub fn generate(spec: &ProblemSpec, variant: &Variant) -> Result<ProblemOutput, DslError> {
    let output = generate_fast(spec, variant)?;

    validate::self_grade(&output)?;

    if let Some(ref fmt) = variant.format {
        if !format::check_format(fmt, &output.answer_key)? {
            return Err(error::DslError::Evaluation(format!(
                "Answer '{}' does not satisfy format '{}'",
                output.answer_key, fmt
            )));
        }
    }

    validate::check_latex(&output.question_latex)?;

    Ok(output)
}

/// Pick a random variant and generate one problem (with validation). Used by
/// callers that don't care which variant — the renderer samples uniformly.
pub fn generate_random(spec: &ProblemSpec) -> Result<ProblemOutput, DslError> {
    let variant = pick_variant(spec)?;
    generate(spec, variant)
}

/// Pick a random variant and generate one problem without validation.
pub fn generate_random_fast(spec: &ProblemSpec) -> Result<ProblemOutput, DslError> {
    let variant = pick_variant(spec)?;
    generate_fast(spec, variant)
}

fn pick_variant(spec: &ProblemSpec) -> Result<&Variant, DslError> {
    if spec.variants.is_empty() {
        return Err(DslError::InvalidSampler(
            "ProblemSpec has no variants".into(),
        ));
    }
    let idx = sampler::random_index(spec.variants.len());
    Ok(&spec.variants[idx])
}

/// Ready-to-insert problem data matching the DB schema
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProblemOutput {
    pub question_latex: String,
    pub answer_key: String,
    pub solution_latex: String,
    pub difficulty: i32,
    pub main_topic: String,
    pub subtopic: String,
    pub grading_mode: String,
    pub answer_type: String,
    pub calculator_allowed: String,
    pub question_image_url: String,
    pub time_limit_seconds: Option<i32>,
    /// Name of the variant this output was rendered from.
    pub variant_name: String,
}
