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

// pub mod diagram;  // TODO: Typst + circuitikz integration

use error::DslError;
use spec::ProblemSpec;

/// Parse a YAML string into a ProblemSpec
pub fn parse(yaml: &str) -> Result<ProblemSpec, DslError> {
    spec::parse_yaml(yaml)
}

/// Generate without validation. For batch generation of pre-validated YAMLs.
pub fn generate_fast(spec: &ProblemSpec) -> Result<ProblemOutput, DslError> {
    let vars = resolver::resolve(&spec.variables, &spec.constraints)?;
    let question_latex = template::render(&spec.question, &vars)?;
    let answer_key = answer::format(&vars, &spec.answer, spec.answer_type.as_deref())?;
    let solution_latex = spec
        .solution
        .as_ref()
        .map(|steps| template::render_steps(steps, &vars))
        .transpose()?;

    let answer_type = answer::infer_type(&answer_key, spec.answer_type.as_deref());

    Ok(ProblemOutput {
        question_latex,
        answer_key,
        solution_latex: solution_latex.unwrap_or_default(),
        difficulty: sampler::sample_difficulty(&spec.difficulty)?,
        main_topic: spec.topic.main.clone(),
        subtopic: spec.topic.sub.clone(),
        grading_mode: spec.mode.clone().unwrap_or_else(|| "equivalent".into()),
        answer_type,
        calculator_allowed: spec.calculator.clone().unwrap_or_else(|| "none".into()),
        question_image: String::new(),
        time_limit_seconds: spec.time,
    })
}

/// Generate a problem with full validation (self-grade + KaTeX check).
pub fn generate(spec: &ProblemSpec) -> Result<ProblemOutput, DslError> {
    let output = generate_fast(spec)?;

    validate::self_grade(&output)?;

    if let Some(ref fmt) = spec.format {
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
    pub question_image: String,
    pub time_limit_seconds: Option<i32>,
}
