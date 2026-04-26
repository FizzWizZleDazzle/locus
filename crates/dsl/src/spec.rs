//! YAML problem spec — deserialization and top-level validation
//!
//! Schema is variants-only: top-level holds metadata (`topic`, `difficulty`,
//! `calculator`, `time`) plus a non-empty `variants` list. Each variant is
//! self-contained — no inheritance, no top-level body fields. A single-variant
//! YAML is fine; the wrapper is the canonical form regardless of count.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use crate::error::DslError;

/// Top-level problem definition
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProblemSpec {
    /// Topic in "main/sub" format
    #[serde(deserialize_with = "deserialize_topic")]
    pub topic: Topic,

    /// Difficulty label or range (variants may override)
    pub difficulty: Difficulty,

    /// Calculator policy
    pub calculator: Option<String>,

    /// Expected solve time in seconds
    pub time: Option<i32>,

    /// Problem variants — must contain at least one entry.
    pub variants: Vec<Variant>,
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

/// A single problem variant — fully self-contained.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,

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

    /// Grading mode (default: equivalent)
    pub mode: Option<String>,

    /// Format check — tag name or predicate expression.
    pub format: Option<String>,

    /// Solution steps
    pub solution: Option<Vec<String>>,

    /// Diagram specification — rendered to compressed SVG at generation time.
    /// See `crate::diagram` and `docs/DSL_SPEC.md` §11.
    pub diagram: Option<crate::diagram::spec::DiagramSpec>,

    /// Variant-level difficulty override
    pub difficulty: Option<Difficulty>,
}

/// Parse YAML string into a ProblemSpec
pub fn parse_yaml(yaml: &str) -> Result<ProblemSpec, DslError> {
    let spec: ProblemSpec = serde_yaml::from_str(yaml)?;
    validate_spec(&spec)?;
    Ok(spec)
}

/// Validate top-level spec fields
fn validate_spec(spec: &ProblemSpec) -> Result<(), DslError> {
    if let Some(ref calc) = spec.calculator {
        match calc.as_str() {
            "none" | "scientific" | "graphing" => {}
            other => {
                return Err(DslError::InvalidSampler(format!(
                    "Unknown calculator type: '{other}'"
                )));
            }
        }
    }

    if spec.variants.is_empty() {
        return Err(DslError::InvalidSampler(
            "Problem must declare at least one variant in `variants:`".into(),
        ));
    }

    let mut seen = BTreeSet::new();
    for v in &spec.variants {
        if v.name.is_empty() {
            return Err(DslError::InvalidSampler(
                "Variant `name` must be non-empty".into(),
            ));
        }
        if !seen.insert(v.name.clone()) {
            return Err(DslError::InvalidSampler(format!(
                "Duplicate variant name: '{}'",
                v.name
            )));
        }
        if let Some(ref mode) = v.mode {
            match mode.as_str() {
                "equivalent" => {}
                other => {
                    return Err(DslError::InvalidSampler(format!(
                        "Unknown grading mode in variant '{}': '{other}'",
                        v.name
                    )));
                }
            }
        }
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
