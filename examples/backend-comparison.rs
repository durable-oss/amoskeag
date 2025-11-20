//! Backend Comparison Example
//!
//! This example demonstrates how to use different Amoskeag backends
//! and compare their capabilities and outputs.
//!
//! Run with:
//!   cargo run --example backend-comparison

use amoskeag::backend::{interpreter::DirectInterpreterBackend, Backend, BackendRegistry};
use amoskeag_lexer::Lexer;
use amoskeag_parser::Parser;
use amoskeag_stdlib_operators::Value;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("Amoskeag Backend Comparison\n");
    println!("============================\n");

    // Parse an example expression
    let source = "let x = 10 in let y = 5 in x * y + 2";
    println!("Expression: {}\n", source);

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();

    // Prepare runtime data
    let data = HashMap::new();

    // Test interpreter backend
    println!("🔍 Testing Interpreter Backend");
    println!("--------------------------------");
    let interpreter = DirectInterpreterBackend::new();
    println!("Name: {}", interpreter.name());
    println!("Description: {}", interpreter.description());
    println!("Supports expression: {}", interpreter.supports(&expr));

    let start = Instant::now();
    let compiled = interpreter.compile(&expr, &[]).unwrap();
    let compile_time = start.elapsed();

    let start = Instant::now();
    let result = interpreter.execute(&compiled, &data).unwrap();
    let exec_time = start.elapsed();

    println!("Result: {:?}", result);
    println!("Compile time: {:?}", compile_time);
    println!("Execute time: {:?}", exec_time);
    println!();

    // Display backend capabilities
    println!("📋 Backend Registry");
    println!("-------------------");
    let mut registry = BackendRegistry::new();
    registry.register(DirectInterpreterBackend::capabilities());

    for caps in registry.list() {
        println!("\nBackend: {}", caps.name);
        println!("Description: {}", caps.description);
        println!("Performance Tier: {}", caps.performance_tier.description());
        println!("External Dependencies: {}", caps.requires_external_deps);
        println!("Supported Features:");
        for feature in &caps.supported_features {
            println!("  • {}", feature);
        }
    }

    println!("\n============================");
    println!("✓ Comparison complete!");
}

/// Helper function to format execution results
fn format_result(result: &Value) -> String {
    match result {
        Value::Number(n) => format!("{}", n),
        Value::String(s) => format!("\"{}\"", s),
        Value::Boolean(b) => format!("{}", b),
        Value::Symbol(s) => format!(":{}", s),
        Value::Nil => "nil".to_string(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_result).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Dictionary(map) => {
            let items: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{}: {}", k, format_result(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}
