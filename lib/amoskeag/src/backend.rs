//! Unified backend trait for Amoskeag code generation
//!
//! This module defines a common interface for all Amoskeag backends:
//! - JIT compilation (LLVM)
//! - Transpilation to Python
//! - Transpilation to Ruby
//! - Interpretation (tree-walking evaluator)

pub mod interpreter;

use crate::{CompileError, EvalError};
use amoskeag_parser::Expr;
use amoskeag_stdlib_operators::Value;
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur in any backend
#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Compilation error: {0}")]
    CompileError(#[from] CompileError),

    #[error("Evaluation error: {0}")]
    EvalError(#[from] EvalError),

    #[error("Backend-specific error: {0}")]
    BackendSpecific(String),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
}

/// Result type for backend operations
pub type BackendResult<T> = Result<T, BackendError>;

/// Unified trait for all Amoskeag code generation backends
///
/// This trait allows different backends (JIT, transpilers, interpreters)
/// to provide a consistent interface for compilation and execution.
pub trait Backend {
    /// The type of compiled output this backend produces
    type CompiledOutput;

    /// The type of execution result this backend produces
    type ExecutionResult;

    /// Get the name of this backend
    fn name(&self) -> &str;

    /// Compile an Amoskeag expression
    ///
    /// # Arguments
    ///
    /// * `expr` - The AST expression to compile
    /// * `symbols` - The list of valid symbol literals
    ///
    /// # Returns
    ///
    /// The compiled output in the backend's format
    fn compile(
        &self,
        expr: &Expr,
        symbols: &[&str],
    ) -> BackendResult<Self::CompiledOutput>;

    /// Execute a compiled program
    ///
    /// # Arguments
    ///
    /// * `compiled` - The compiled output from `compile()`
    /// * `data` - The runtime data context
    ///
    /// # Returns
    ///
    /// The execution result in the backend's format
    fn execute(
        &self,
        compiled: &Self::CompiledOutput,
        data: &HashMap<String, Value>,
    ) -> BackendResult<Self::ExecutionResult>;

    /// Compile and execute in one step (convenience method)
    ///
    /// # Arguments
    ///
    /// * `expr` - The AST expression to compile and execute
    /// * `symbols` - The list of valid symbol literals
    /// * `data` - The runtime data context
    ///
    /// # Returns
    ///
    /// The execution result
    fn compile_and_execute(
        &self,
        expr: &Expr,
        symbols: &[&str],
        data: &HashMap<String, Value>,
    ) -> BackendResult<Self::ExecutionResult> {
        let compiled = self.compile(expr, symbols)?;
        self.execute(&compiled, data)
    }

    /// Check if this backend supports a given expression type
    ///
    /// This allows backends to declare feature support upfront
    fn supports(&self, expr: &Expr) -> bool;

    /// Get a description of this backend's capabilities
    fn description(&self) -> &str;
}

/// Metadata about a backend's capabilities
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    /// Backend name
    pub name: String,

    /// Backend description
    pub description: String,

    /// Supported expression types
    pub supported_features: Vec<String>,

    /// Performance characteristics
    pub performance_tier: PerformanceTier,

    /// Whether this backend requires external dependencies
    pub requires_external_deps: bool,
}

/// Performance tier classification for backends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceTier {
    /// Near-native performance (e.g., JIT compilation)
    Native,

    /// Fast interpreted performance
    Fast,

    /// Standard interpreted performance
    Standard,

    /// Code generation for external execution
    Transpiled,
}

impl PerformanceTier {
    /// Get a human-readable description of this tier
    pub fn description(&self) -> &str {
        match self {
            Self::Native => "Near-native performance via LLVM JIT compilation",
            Self::Fast => "Fast tree-walking interpreter with optimizations",
            Self::Standard => "Standard tree-walking interpreter",
            Self::Transpiled => "Code generation for external runtime",
        }
    }
}

/// Registry of available backends
pub struct BackendRegistry {
    backends: HashMap<String, BackendCapabilities>,
}

impl BackendRegistry {
    /// Create a new backend registry
    pub fn new() -> Self {
        Self {
            backends: HashMap::new(),
        }
    }

    /// Register a backend
    pub fn register(&mut self, capabilities: BackendCapabilities) {
        self.backends.insert(capabilities.name.clone(), capabilities);
    }

    /// Get information about a backend
    pub fn get(&self, name: &str) -> Option<&BackendCapabilities> {
        self.backends.get(name)
    }

    /// List all available backends
    pub fn list(&self) -> Vec<&BackendCapabilities> {
        self.backends.values().collect()
    }
}

impl Default for BackendRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_registry() {
        let mut registry = BackendRegistry::new();

        let jit_caps = BackendCapabilities {
            name: "jit".to_string(),
            description: "LLVM JIT compiler".to_string(),
            supported_features: vec!["arithmetic".to_string(), "comparisons".to_string()],
            performance_tier: PerformanceTier::Native,
            requires_external_deps: true,
        };

        registry.register(jit_caps);

        assert!(registry.get("jit").is_some());
        assert_eq!(registry.list().len(), 1);
    }

    #[test]
    fn test_performance_tier_descriptions() {
        assert!(!PerformanceTier::Native.description().is_empty());
        assert!(!PerformanceTier::Fast.description().is_empty());
        assert!(!PerformanceTier::Standard.description().is_empty());
        assert!(!PerformanceTier::Transpiled.description().is_empty());
    }
}
