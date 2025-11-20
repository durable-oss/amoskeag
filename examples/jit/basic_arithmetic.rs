//! Basic arithmetic JIT compilation example
//!
//! This example demonstrates compiling and executing simple arithmetic
//! expressions using the Amoskeag JIT compiler.
//!
//! Run with:
//!   cargo run --example basic_arithmetic -p amoskeag-jit

use amoskeag_jit::JitCompiler;
use amoskeag_lexer::Lexer;
use amoskeag_parser::Parser;
use inkwell::context::Context;

fn main() {
    println!("Amoskeag JIT Compiler - Basic Arithmetic Examples\n");
    println!("==================================================\n");

    // Create LLVM context
    let context = Context::create();

    // Example 1: Simple addition
    compile_and_run(&context, "2 + 3", "Simple addition");

    // Example 2: Order of operations
    compile_and_run(&context, "2 + 3 * 4", "Order of operations");

    // Example 3: Parentheses
    compile_and_run(&context, "(2 + 3) * 4", "Parentheses");

    // Example 4: Division
    compile_and_run(&context, "100 / 4", "Division");

    // Example 5: Complex expression
    compile_and_run(&context, "(10 + 5) * 2 - 8 / 4", "Complex expression");

    // Example 6: Comparison
    compile_and_run(&context, "5 > 3", "Comparison (returns 1.0 for true)");

    // Example 7: If expression
    compile_and_run(
        &context,
        "if 10 > 5 then 42 else 0 end",
        "If expression",
    );

    // Example 8: Let binding
    compile_and_run(
        &context,
        "let x = 10 in x * x",
        "Let binding (x squared)",
    );

    // Example 9: Nested let bindings
    compile_and_run(
        &context,
        "let x = 5 in let y = 3 in x * y + 2",
        "Nested let bindings",
    );

    // Example 10: Complex conditional
    compile_and_run(
        &context,
        "let age = 25 in if age >= 18 then age * 2 else age / 2 end",
        "Complex conditional with let",
    );

    println!("\n==================================================");
    println!("All examples completed successfully!");
}

fn compile_and_run(context: &Context, source: &str, description: &str) {
    print!("{:40} ", description);

    // Parse the expression
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Lexer error: {}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let expr = match parser.parse() {
        Ok(e) => e,
        Err(e) => {
            println!("❌ Parser error: {}", e);
            return;
        }
    };

    // Create JIT compiler
    let jit = match JitCompiler::new(context) {
        Ok(j) => j,
        Err(e) => {
            println!("❌ JIT error: {}", e);
            return;
        }
    };

    // Compile and run
    match jit.compile_and_run(&expr) {
        Ok(result) => {
            println!("✓ {} = {}", source, result);
        }
        Err(e) => {
            println!("❌ Execution error: {}", e);
        }
    }
}
