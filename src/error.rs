use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SemanticError {
    #[error("Undefined symbol: {name} at {location}")]
    UndefinedSymbol {
        name: String,
        location: SourceLocation,
    },

    #[error(
        "Symbol already defined: {name} at {location}, previously defined at {previous_location}"
    )]
    DuplicateSymbol {
        name: String,
        location: SourceLocation,
        previous_location: SourceLocation,
    },

    #[error("Type mismatch: expected {expected}, found {found} at {location}")]
    TypeMismatch {
        expected: String,
        found: String,
        location: SourceLocation,
    },

    #[error("Invalid scope: {message}")]
    InvalidScope { message: String },

    #[error("Invalid type: {message} at {location}")]
    InvalidType {
        message: String,
        location: SourceLocation,
    },

    #[error("Circular dependency detected: {message}")]
    CircularDependency { message: String },

    #[error("Invalid operation: {message} at {location}")]
    InvalidOperation {
        message: String,
        location: SourceLocation,
    },

    #[error("Inaccessible symbol: {name} is {visibility} at {location}")]
    InaccessibleSymbol {
        name: String,
        visibility: String,
        location: SourceLocation,
    },

    #[error("Invalid assignment: {message} at {location}")]
    InvalidAssignment {
        message: String,
        location: SourceLocation,
    },

    #[error("Parameter mismatch: {message} at {location}")]
    ParameterMismatch {
        message: String,
        location: SourceLocation,
    },

    #[error("Analysis error: {0}")]
    AnalysisError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

pub type Result<T> = std::result::Result<T, SemanticError>;
