//! Validation pipeline — 6 layers from DSL spec

use crate::ProblemOutput;
use crate::error::DslError;

/// Self-grade: answer_key must grade correctly against itself
pub fn self_grade(_output: &ProblemOutput) -> Result<(), DslError> {
    // TODO: use locus_common::grader::grade_answer
    Ok(())
}

/// Validate generated LaTeX via katex_validate
pub fn check_latex(_latex: &str) -> Result<(), DslError> {
    // TODO: use locus_common::katex_validate::validate_katex
    Ok(())
}
