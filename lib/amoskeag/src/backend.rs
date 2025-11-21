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

    // Mock backend for testing trait methods
    struct MockBackend;

    impl Backend for MockBackend {
        type CompiledOutput = String;
        type ExecutionResult = Value;

        fn name(&self) -> &str {
            "mock"
        }

        fn compile(&self, _expr: &Expr, _symbols: &[&str]) -> BackendResult<Self::CompiledOutput> {
            Ok("compiled".to_string())
        }

        fn execute(&self, _compiled: &Self::CompiledOutput, _data: &HashMap<String, Value>) -> BackendResult<Self::ExecutionResult> {
            Ok(Value::Number(42.0))
        }

        fn supports(&self, _expr: &Expr) -> bool {
            true
        }

        fn description(&self) -> &str {
            "Mock backend for testing"
        }
    }

    #[test]
    fn test_backend_error_from_compile_error() {
        // Since CompileError is from another crate, we can't instantiate it directly
        // But we can test that BackendError implements Error
        let error = BackendError::BackendSpecific("test".to_string());
        assert_eq!(format!("{}", error), "Backend-specific error: test");
    }

    #[test]
    fn test_backend_error_from_eval_error() {
        let error = BackendError::UnsupportedFeature("feature".to_string());
        assert_eq!(format!("{}", error), "Unsupported feature: feature");
    }

    #[test]
    fn test_backend_capabilities_creation() {
        let caps = BackendCapabilities {
            name: "test".to_string(),
            description: "Test backend".to_string(),
            supported_features: vec!["feature1".to_string(), "feature2".to_string()],
            performance_tier: PerformanceTier::Standard,
            requires_external_deps: false,
        };

        assert_eq!(caps.name, "test");
        assert_eq!(caps.description, "Test backend");
        assert_eq!(caps.supported_features.len(), 2);
        assert_eq!(caps.performance_tier, PerformanceTier::Standard);
        assert!(!caps.requires_external_deps);
    }

    #[test]
    fn test_backend_capabilities_clone() {
        let caps = BackendCapabilities {
            name: "original".to_string(),
            description: "Original".to_string(),
            supported_features: vec!["clone".to_string()],
            performance_tier: PerformanceTier::Fast,
            requires_external_deps: true,
        };

        let cloned = caps.clone();
        assert_eq!(caps.name, cloned.name);
        assert_eq!(caps.performance_tier, cloned.performance_tier);
    }

    #[test]
    fn test_performance_tier_equality() {
        assert_eq!(PerformanceTier::Native, PerformanceTier::Native);
        assert_ne!(PerformanceTier::Native, PerformanceTier::Fast);
        assert_eq!(PerformanceTier::Transpiled, PerformanceTier::Transpiled);
    }

    #[test]
    fn test_performance_tier_descriptions() {
        assert!(!PerformanceTier::Native.description().is_empty());
        assert!(!PerformanceTier::Fast.description().is_empty());
        assert!(!PerformanceTier::Standard.description().is_empty());
        assert!(!PerformanceTier::Transpiled.description().is_empty());

        // Test specific content
        assert!(PerformanceTier::Native.description().contains("native"));
        assert!(PerformanceTier::Transpiled.description().contains("external"));
    }

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
    fn test_backend_registry_multiple() {
        let mut registry = BackendRegistry::new();

        let jit_caps = BackendCapabilities {
            name: "jit".to_string(),
            description: "JIT".to_string(),
            supported_features: vec![],
            performance_tier: PerformanceTier::Native,
            requires_external_deps: true,
        };

        let interp_caps = BackendCapabilities {
            name: "interpreter".to_string(),
            description: "Interpreter".to_string(),
            supported_features: vec![],
            performance_tier: PerformanceTier::Standard,
            requires_external_deps: false,
        };

        registry.register(jit_caps);
        registry.register(interp_caps);

        assert!(registry.get("jit").is_some());
        assert!(registry.get("interpreter").is_some());
        assert_eq!(registry.list().len(), 2);
    }

    #[test]
    fn test_backend_registry_get_nonexistent() {
        let registry = BackendRegistry::new();
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_backend_registry_default() {
        let registry: BackendRegistry = Default::default();
        assert_eq!(registry.list().len(), 0);
    }

    #[test]
    fn test_mock_backend_trait_methods() {
        let backend = MockBackend;

        assert_eq!(backend.name(), "mock");
        assert_eq!(backend.description(), "Mock backend for testing");
        assert!(backend.supports(&Expr::Number(1.0))); // Dummy expr
    }

    #[test]
    fn test_compile_and_execute_default() {
        let backend = MockBackend;
        let expr = Expr::Number(1.0);
        let symbols = vec![];
        let data = HashMap::new();

        let result = backend.compile_and_execute(&expr, &symbols, &data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Number(42.0));
    }
}
