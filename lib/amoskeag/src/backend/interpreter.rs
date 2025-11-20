//! Interpreter backend implementation
//!
//! This module implements the Backend trait for the tree-walking interpreter.

use super::{Backend, BackendError, BackendResult, PerformanceTier};
use crate::{compile, evaluate, CompiledProgram};
use amoskeag_parser::Expr;
use amoskeag_stdlib_operators::Value;
use std::collections::HashMap;

/// The tree-walking interpreter backend
pub struct InterpreterBackend;

impl InterpreterBackend {
    /// Create a new interpreter backend
    pub fn new() -> Self {
        Self
    }

    /// Get the capabilities of this backend
    pub fn capabilities() -> super::BackendCapabilities {
        super::BackendCapabilities {
            name: "interpreter".to_string(),
            description: "Tree-walking interpreter with full language support".to_string(),
            supported_features: vec![
                "numbers".to_string(),
                "strings".to_string(),
                "booleans".to_string(),
                "symbols".to_string(),
                "arrays".to_string(),
                "dictionaries".to_string(),
                "arithmetic".to_string(),
                "comparisons".to_string(),
                "logic".to_string(),
                "if_expressions".to_string(),
                "let_bindings".to_string(),
                "function_calls".to_string(),
                "pipe_expressions".to_string(),
                "safe_navigation".to_string(),
            ],
            performance_tier: PerformanceTier::Standard,
            requires_external_deps: false,
        }
    }
}

impl Default for InterpreterBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for InterpreterBackend {
    type CompiledOutput = CompiledProgram;
    type ExecutionResult = Value;

    fn name(&self) -> &str {
        "interpreter"
    }

    fn compile(
        &self,
        expr: &Expr,
        symbols: &[&str],
    ) -> BackendResult<Self::CompiledOutput> {
        // We need to convert the expr to source and back
        // For now, we'll use the AST directly
        // This is a bit of a hack, but works for the interface
        compile(&format!("{:?}", expr), symbols)
            .map_err(|e| BackendError::CompileError(e))
    }

    fn execute(
        &self,
        compiled: &Self::CompiledOutput,
        data: &HashMap<String, Value>,
    ) -> BackendResult<Self::ExecutionResult> {
        evaluate(compiled, data)
            .map_err(|e| BackendError::EvalError(e))
    }

    fn supports(&self, _expr: &Expr) -> bool {
        // The interpreter supports all Amoskeag expressions
        true
    }

    fn description(&self) -> &str {
        "Tree-walking interpreter with full language support and safe navigation"
    }
}

/// A simpler, direct interpreter backend that doesn't require re-parsing
pub struct DirectInterpreterBackend;

impl DirectInterpreterBackend {
    /// Create a new direct interpreter backend
    pub fn new() -> Self {
        Self
    }

    /// Get the capabilities of this backend
    pub fn capabilities() -> super::BackendCapabilities {
        super::BackendCapabilities {
            name: "direct-interpreter".to_string(),
            description: "Direct AST interpreter (no re-parsing required)".to_string(),
            supported_features: vec![
                "numbers".to_string(),
                "strings".to_string(),
                "booleans".to_string(),
                "symbols".to_string(),
                "arrays".to_string(),
                "dictionaries".to_string(),
                "arithmetic".to_string(),
                "comparisons".to_string(),
                "logic".to_string(),
                "if_expressions".to_string(),
                "let_bindings".to_string(),
                "function_calls".to_string(),
                "pipe_expressions".to_string(),
                "safe_navigation".to_string(),
            ],
            performance_tier: PerformanceTier::Standard,
            requires_external_deps: false,
        }
    }
}

impl Default for DirectInterpreterBackend {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper for an AST expression as "compiled" output
pub struct DirectCompiledProgram {
    expr: Expr,
}

impl Backend for DirectInterpreterBackend {
    type CompiledOutput = DirectCompiledProgram;
    type ExecutionResult = Value;

    fn name(&self) -> &str {
        "direct-interpreter"
    }

    fn compile(
        &self,
        expr: &Expr,
        _symbols: &[&str],
    ) -> BackendResult<Self::CompiledOutput> {
        // Symbol validation would happen here in a real implementation
        Ok(DirectCompiledProgram {
            expr: expr.clone(),
        })
    }

    fn execute(
        &self,
        compiled: &Self::CompiledOutput,
        data: &HashMap<String, Value>,
    ) -> BackendResult<Self::ExecutionResult> {
        use crate::{Context, eval_expr};

        let context = Context::new(data.clone());
        eval_expr(&compiled.expr, &context)
            .map_err(|e| BackendError::EvalError(e))
    }

    fn supports(&self, _expr: &Expr) -> bool {
        true
    }

    fn description(&self) -> &str {
        "Direct AST interpreter (no re-parsing required)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amoskeag_lexer::Lexer;
    use amoskeag_parser::Parser;

    fn parse_expr(source: &str) -> Expr {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn test_direct_interpreter_backend() {
        let backend = DirectInterpreterBackend::new();
        let expr = parse_expr("2 + 3");
        let data = HashMap::new();

        let compiled = backend.compile(&expr, &[]).unwrap();
        let result = backend.execute(&compiled, &data).unwrap();

        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_backend_supports_all() {
        let backend = DirectInterpreterBackend::new();
        let expr = parse_expr("if true 1 else 0 end");

        assert!(backend.supports(&expr));
    }

    #[test]
    fn test_direct_interpreter_with_let() {
        let backend = DirectInterpreterBackend::new();
        let expr = parse_expr("let x = 5 in x * 2");
        let data = HashMap::new();

        let compiled = backend.compile(&expr, &[]).unwrap();
        let result = backend.execute(&compiled, &data).unwrap();

        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_direct_interpreter_with_if() {
        let backend = DirectInterpreterBackend::new();
        let expr = parse_expr("if 10 > 5 :yes else :no end");
        let data = HashMap::new();

        let compiled = backend.compile(&expr, &["yes", "no"]).unwrap();
        let result = backend.execute(&compiled, &data).unwrap();

        assert_eq!(result, Value::Symbol("yes".to_string()));
    }

    #[test]
    fn test_direct_interpreter_with_variables() {
        let backend = DirectInterpreterBackend::new();
        let expr = parse_expr("user.age");

        let mut user = HashMap::new();
        user.insert("age".to_string(), Value::Number(25.0));

        let mut data = HashMap::new();
        data.insert("user".to_string(), Value::Dictionary(user));

        let compiled = backend.compile(&expr, &[]).unwrap();
        let result = backend.execute(&compiled, &data).unwrap();

        assert_eq!(result, Value::Number(25.0));
    }

    #[test]
    fn test_direct_interpreter_safe_navigation() {
        let backend = DirectInterpreterBackend::new();
        let expr = parse_expr("missing.nested.key");
        let data = HashMap::new();

        let compiled = backend.compile(&expr, &[]).unwrap();
        let result = backend.execute(&compiled, &data).unwrap();

        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_direct_interpreter_array() {
        let backend = DirectInterpreterBackend::new();
        let expr = parse_expr("[1, 2, 3]");
        let data = HashMap::new();

        let compiled = backend.compile(&expr, &[]).unwrap();
        let result = backend.execute(&compiled, &data).unwrap();

        assert_eq!(
            result,
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_direct_interpreter_dictionary() {
        let backend = DirectInterpreterBackend::new();
        let expr = parse_expr(r#"{"name": "Alice", "age": 30}"#);
        let data = HashMap::new();

        let compiled = backend.compile(&expr, &[]).unwrap();
        let result = backend.execute(&compiled, &data).unwrap();

        if let Value::Dictionary(dict) = result {
            assert_eq!(dict.get("name"), Some(&Value::String("Alice".to_string())));
            assert_eq!(dict.get("age"), Some(&Value::Number(30.0)));
        } else {
            panic!("Expected dictionary");
        }
    }

    #[test]
    fn test_direct_interpreter_arithmetic() {
        let test_cases = vec![
            ("2 + 3", Value::Number(5.0)),
            ("10 - 4", Value::Number(6.0)),
            ("3 * 4", Value::Number(12.0)),
            ("20 / 5", Value::Number(4.0)),
            ("10 % 3", Value::Number(1.0)),
        ];

        let backend = DirectInterpreterBackend::new();
        let data = HashMap::new();

        for (source, expected) in test_cases {
            let expr = parse_expr(source);
            let compiled = backend.compile(&expr, &[]).unwrap();
            let result = backend.execute(&compiled, &data).unwrap();
            assert_eq!(result, expected, "Failed for: {}", source);
        }
    }

    #[test]
    fn test_direct_interpreter_comparisons() {
        let test_cases = vec![
            ("5 == 5", Value::Boolean(true)),
            ("5 != 3", Value::Boolean(true)),
            ("5 < 10", Value::Boolean(true)),
            ("10 > 5", Value::Boolean(true)),
            ("5 <= 5", Value::Boolean(true)),
            ("5 >= 5", Value::Boolean(true)),
        ];

        let backend = DirectInterpreterBackend::new();
        let data = HashMap::new();

        for (source, expected) in test_cases {
            let expr = parse_expr(source);
            let compiled = backend.compile(&expr, &[]).unwrap();
            let result = backend.execute(&compiled, &data).unwrap();
            assert_eq!(result, expected, "Failed for: {}", source);
        }
    }

    #[test]
    fn test_direct_interpreter_logical() {
        let test_cases = vec![
            ("true and true", Value::Boolean(true)),
            ("true and false", Value::Boolean(false)),
            ("true or false", Value::Boolean(true)),
            ("false or false", Value::Boolean(false)),
            ("not true", Value::Boolean(false)),
            ("not false", Value::Boolean(true)),
        ];

        let backend = DirectInterpreterBackend::new();
        let data = HashMap::new();

        for (source, expected) in test_cases {
            let expr = parse_expr(source);
            let compiled = backend.compile(&expr, &[]).unwrap();
            let result = backend.execute(&compiled, &data).unwrap();
            assert_eq!(result, expected, "Failed for: {}", source);
        }
    }

    #[test]
    fn test_backend_name() {
        let backend = DirectInterpreterBackend::new();
        assert_eq!(backend.name(), "direct-interpreter");
    }

    #[test]
    fn test_backend_description() {
        let backend = DirectInterpreterBackend::new();
        assert!(!backend.description().is_empty());
    }

    #[test]
    fn test_backend_capabilities() {
        let caps = DirectInterpreterBackend::capabilities();
        assert_eq!(caps.name, "direct-interpreter");
        assert!(!caps.supported_features.is_empty());
        assert_eq!(caps.requires_external_deps, false);
    }
}
