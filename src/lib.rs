pub mod analyzer;
pub mod error;
pub mod resolution;
pub mod scope;
pub mod symbols;
pub mod types;

// Re-export core types
pub use analyzer::SemanticAnalyzer;
pub use error::{Result, SemanticError};
pub use resolution::NameResolver;
pub use scope::{Scope, ScopeKind, ScopeManager};
pub use symbols::{Symbol, SymbolKind, SymbolTable};
pub use types::{TypeChecker, TypeInfo, TypeKind};

/// Version of the semantic analysis library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
