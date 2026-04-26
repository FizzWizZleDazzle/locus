//! Validation pipeline — self-grading + LaTeX checks

use locus_common::grader::{GradeResult, grade_answer};
use locus_common::katex_validate::validate_katex;
use locus_common::{AnswerType, GradingMode};

use crate::ProblemOutput;
use crate::error::DslError;

/// Self-grade: answer_key graded against itself must return Correct.
/// This catches malformed answer_keys that the grader can't parse.
pub fn self_grade(output: &ProblemOutput) -> Result<(), DslError> {
    let answer_type = AnswerType::from_str(&output.answer_type).unwrap_or_default();
    let grading_mode = match output.grading_mode.as_str() {
        "factor" => GradingMode::Factor,
        "expand" => GradingMode::Expand,
        _ => GradingMode::Equivalent,
    };

    // Factor/expand modes: self-grading the expanded form against itself fails
    // because the grader checks if the answer IS factored/expanded.
    // For self-grading, always use Equivalent mode — we just want to verify parseability.
    let self_grade_mode = GradingMode::Equivalent;
    let result = grade_answer(
        &output.answer_key,
        &output.answer_key,
        answer_type,
        self_grade_mode,
    );

    match result {
        GradeResult::Correct => Ok(()),
        other => Err(DslError::SelfGradeFailed {
            result: format!("{:?}", other),
        }),
    }
}

/// Validate generated LaTeX passes KaTeX checks
pub fn check_latex(latex: &str) -> Result<(), DslError> {
    let result = validate_katex(latex);
    if result.has_errors() {
        let errors: Vec<String> = result
            .issues
            .iter()
            .filter(|i| i.severity == locus_common::katex_validate::Severity::Error)
            .map(|i| i.to_string())
            .collect();
        Err(DslError::KatexError(errors.join("; ")))
    } else {
        Ok(())
    }
}
