use crate::error::{Result, SourceLocation};
use crate::scope::{ScopeKind, ScopeManager};
use crate::symbols::Symbol;
use crate::types::TypeChecker;

/// Main semantic analyzer that processes VB6 code
pub struct SemanticAnalyzer {
    /// Scope manager for symbol resolution
    scope_manager: ScopeManager,

    /// Type checker for type validation
    #[allow(dead_code)]
    type_checker: TypeChecker,

    /// Current file being analyzed
    current_file: Option<String>,

    /// Collected errors
    errors: Vec<crate::error::SemanticError>,

    /// Collected warnings
    warnings: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            scope_manager: ScopeManager::new(),
            type_checker: TypeChecker::new(),
            current_file: None,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Analyze a VB6 project file
    pub fn analyze_project(
        &mut self,
        project: &vb6parse::files::ProjectFile,
    ) -> Result<AnalysisResult> {
        // Create project-level scope
        let _project_scope = self
            .scope_manager
            .push_scope(ScopeKind::Global, project.properties.name.to_string());

        // TODO: Analyze project references and external dependencies

        // Analyze all modules, classes, and forms
        // This would iterate through the parsed structures

        // For now, return empty result
        Ok(AnalysisResult {
            scope_manager: self.scope_manager.clone(),
            errors: self.errors.clone(),
            warnings: self.warnings.clone(),
        })
    }

    /// Analyze a module file
    pub fn analyze_module(&mut self, module: &vb6parse::files::ModuleFile) -> Result<()> {
        self.current_file = Some(module.name.clone());

        // Create module scope
        let _module_scope = self
            .scope_manager
            .push_scope(ScopeKind::Global, module.name.clone());

        // TODO: Process module-level declarations
        // - Process Option Explicit, Option Base, etc.
        // - Process module-level variables
        // - Process constants
        // - Process procedures and functions
        // - Process type definitions
        // - Process enums

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Analyze a class file
    pub fn analyze_class(&mut self, _class: &vb6parse::files::ClassFile) -> Result<()> {
        self.current_file = Some("class".to_string());

        // Create class scope
        let _class_scope = self
            .scope_manager
            .push_scope(ScopeKind::Class, "class".to_string());

        // TODO: Process class members
        // - Process properties
        // - Process methods
        // - Process events
        // - Process implements

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Analyze a form file
    pub fn analyze_form(&mut self, _form: &vb6parse::files::FormFile) -> Result<()> {
        self.current_file = Some("form".to_string());

        // Create form scope (forms are like classes)
        let _form_scope = self
            .scope_manager
            .push_scope(ScopeKind::Class, "form".to_string());

        // TODO: Process form structure
        // - Process controls (add as symbols)
        // - Process event handlers
        // - Process form-level variables

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Add a symbol to the current scope
    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<()> {
        match self.scope_manager.add_symbol(symbol) {
            Ok(()) => Ok(()),
            Err(e) => {
                self.errors.push(e.clone());
                Err(e)
            }
        }
    }

    /// Lookup a symbol
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.scope_manager.lookup(name)
    }

    /// Get the scope manager (for inspection)
    pub fn scope_manager(&self) -> &ScopeManager {
        &self.scope_manager
    }

    /// Get collected errors
    pub fn errors(&self) -> &[crate::error::SemanticError] {
        &self.errors
    }

    /// Get collected warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Add a warning
    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }

    /// Create a source location for current file
    #[allow(dead_code)]
    fn make_location(&self, line: usize, column: usize) -> SourceLocation {
        SourceLocation {
            file: self
                .current_file
                .clone()
                .unwrap_or_else(|| "<unknown>".to_string()),
            line,
            column,
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of semantic analysis
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Final scope manager with all symbols
    pub scope_manager: ScopeManager,

    /// Errors found during analysis
    pub errors: Vec<crate::error::SemanticError>,

    /// Warnings generated
    pub warnings: Vec<String>,
}

impl AnalysisResult {
    /// Check if analysis was successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}
