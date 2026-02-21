//! Answer formatters for display
//!
//! This module provides formatters for converting internal answer representations
//! to human-readable HTML for display.
//!
//! # Architecture
//!
//! Each answer type has its own formatter module:
//! - `interval` - Interval notation: (a, b], [a, b), etc.
//! - `set` - Set notation: {a, b, c}
//! - `matrix` - Matrix notation: [[a,b],[c,d]]
//! - `multi_part` - Multi-part answers with labeled parts
//! - `equation` - Equations without symbolic expansion
//! - `inequality` - Inequalities with proper comparison symbols
//! - `common` - Shared LaTeX rendering helpers
//!
//! # Pipeline
//!
//! 1. User input (LaTeX or MathJSON) → preprocess → plain notation
//! 2. Grading system works with plain notation internally
//! 3. Display: plain notation → formatter → LaTeX → KaTeX → HTML

use locus_common::AnswerType;

mod common;
mod equation;
mod inequality;
mod interval;
mod matrix;
mod multi_part;
mod set;

#[cfg(test)]
mod tests;

use common::{render_code, render_plain};

/// Format answer_key for display based on answer_type
///
/// This is the main entry point for answer formatting. It delegates to
/// type-specific formatters and handles fallback cases.
pub fn format_answer_for_display(
    answer_key: &str,
    answer_type: AnswerType,
) -> Result<String, String> {
    match answer_type {
        AnswerType::Interval => interval::format_interval(answer_key),
        AnswerType::Set => set::format_set(answer_key),
        AnswerType::Tuple => {
            // "3, 2" -> "(3, 2)"
            let latex = format!("({})", answer_key);
            common::render_latex(&latex)
        }
        AnswerType::List => {
            // "-2, 2" -> "[-2, 2]"
            let latex = format!("[{}]", answer_key);
            common::render_latex(&latex)
        }
        AnswerType::MultiPart => multi_part::format_multi_part(answer_key),
        AnswerType::Boolean | AnswerType::Word => {
            // Display as-is in code tag
            Ok(render_code(answer_key))
        }
        AnswerType::Inequality => inequality::format_inequality(answer_key),
        AnswerType::Matrix => matrix::format_matrix(answer_key),
        AnswerType::Equation => equation::format_equation(answer_key),
        AnswerType::Expression | AnswerType::Numeric => {
            // Render as math (via Nerdamer for nice formatting)
            render_plain(answer_key)
        }
    }
}
