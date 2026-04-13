//! Error types for the DSL parser

#[derive(Debug, thiserror::Error)]
pub enum DslError {
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Invalid topic '{0}' — must be 'main/sub' format")]
    InvalidTopic(String),

    #[error("Unknown topic '{main}/{sub}'")]
    UnknownTopic { main: String, sub: String },

    #[error("Variable '{name}' not found")]
    UndefinedVariable { name: String },

    #[error("Circular dependency in variables: {cycle}")]
    CircularDependency { cycle: String },

    #[error("Unknown function '{name}'")]
    UnknownFunction { name: String },

    #[error("Function '{name}' expects {expected} args, got {got}")]
    FunctionArity {
        name: String,
        expected: usize,
        got: usize,
    },

    #[error("Constraint unsatisfiable after {attempts} attempts: {constraint}")]
    ConstraintUnsatisfiable { constraint: String, attempts: usize },

    #[error("Expression parse error: {0}")]
    ExpressionParse(String),

    #[error("Evaluation error: {0}")]
    Evaluation(String),

    #[error("Template error: unknown reference '{{{name}}}' in {field}")]
    TemplateRef { name: String, field: String },

    #[error("Template error: unknown display function '{name}' in {field}")]
    TemplateDisplayFn { name: String, field: String },

    #[error("Self-grade failed: grade_answer returned {result}")]
    SelfGradeFailed { result: String },

    #[error("KaTeX validation error: {0}")]
    KatexError(String),

    #[error("Invalid difficulty '{0}'")]
    InvalidDifficulty(String),

    #[error("Invalid sampler '{0}'")]
    InvalidSampler(String),

    #[error("Diagram error: {0}")]
    DiagramError(String),

    #[error("Answer type mismatch: expected {expected}, got {got}")]
    AnswerTypeMismatch { expected: String, got: String },
}
