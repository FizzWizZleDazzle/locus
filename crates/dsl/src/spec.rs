//! YAML problem spec — deserialization and top-level validation

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::error::DslError;

/// Top-level problem definition
#[derive(Debug, Clone, Deserialize)]
pub struct ProblemSpec {
    /// Topic in "main/sub" format
    #[serde(deserialize_with = "deserialize_topic")]
    pub topic: Topic,

    /// Difficulty label or range
    pub difficulty: Difficulty,

    /// Calculator policy
    pub calculator: Option<String>,

    /// Expected solve time in seconds
    pub time: Option<i32>,

    /// Variable definitions (ordered map preserves declaration order)
    pub variables: BTreeMap<String, String>,

    /// Constraints that must hold
    #[serde(default)]
    pub constraints: Vec<String>,

    /// Question text with {var} and {display()} refs
    pub question: String,

    /// Variable name pointing to the answer
    pub answer: String,

    /// Override answer type (auto-inferred if omitted)
    pub answer_type: Option<String>,

    /// Grading mode
    pub mode: Option<String>,

    /// Solution steps
    pub solution: Option<Vec<String>>,

    /// Diagram specification
    pub diagram: Option<serde_yaml::Value>,

    /// Problem variants (if present, overrides variables/question/answer per variant)
    pub variants: Option<Vec<Variant>>,
}

/// Parsed topic
#[derive(Debug, Clone)]
pub struct Topic {
    pub main: String,
    pub sub: String,
}

/// Difficulty specification
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Difficulty {
    /// Label: "easy", "medium", "hard", etc.
    Label(String),
    /// Explicit range: "1200-1400" parsed as string
    Range(String),
    /// Single value
    Value(i32),
}

/// A variant within a problem definition
#[derive(Debug, Clone, Deserialize)]
pub struct Variant {
    pub name: String,
    pub variables: Option<BTreeMap<String, String>>,
    pub constraints: Option<Vec<String>>,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub answer_type: Option<String>,
    pub mode: Option<String>,
    pub difficulty: Option<Difficulty>,
    pub solution: Option<Vec<String>>,
    pub diagram: Option<serde_yaml::Value>,
}

/// Parse YAML string into a ProblemSpec
pub fn parse_yaml(yaml: &str) -> Result<ProblemSpec, DslError> {
    let spec: ProblemSpec = serde_yaml::from_str(yaml)?;
    validate_spec(&spec)?;
    Ok(spec)
}

/// Validate top-level spec fields
fn validate_spec(spec: &ProblemSpec) -> Result<(), DslError> {
    // Validate calculator
    if let Some(ref calc) = spec.calculator {
        match calc.as_str() {
            "none" | "scientific" | "graphing" => {}
            other => {
                return Err(DslError::InvalidSampler(format!(
                    "Unknown calculator type: '{other}'"
                )))
            }
        }
    }

    // Validate mode
    if let Some(ref mode) = spec.mode {
        match mode.as_str() {
            "equivalent" | "factor" | "expand" => {}
            other => {
                return Err(DslError::InvalidSampler(format!(
                    "Unknown grading mode: '{other}'"
                )))
            }
        }
    }

    // Validate answer references a variable
    if !spec.variables.contains_key(&spec.answer)
        && spec.answer != "answer"
        && !spec.answer.contains(',')
    {
        // Could be a comma-separated tuple like "sol_x, sol_y"
        // or a literal — let the resolver handle it
    }

    Ok(())
}

fn deserialize_topic<'de, D>(deserializer: D) -> Result<Topic, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let parts: Vec<&str> = s.splitn(2, '/').collect();
    if parts.len() != 2 {
        return Err(serde::de::Error::custom(format!(
            "topic must be 'main/sub' format, got: '{s}'"
        )));
    }
    Ok(Topic {
        main: parts[0].to_string(),
        sub: parts[1].to_string(),
    })
}
