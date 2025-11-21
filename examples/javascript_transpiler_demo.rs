//! JavaScript Transpiler Demo
//!
//! This example demonstrates how to use the JavaScript transpiler to convert
//! Amoskeag expressions into executable JavaScript code.
//!
//! Run with: cargo run --example javascript_transpiler_demo

use amoskeag_transpiler_javascript::{transpile_source, TranspileConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Amoskeag JavaScript Transpiler Demo");
    println!("====================================\n");

    // Example 1: Simple arithmetic
    let example1 = "2 + 3 * 4";
    println!("Example 1: Arithmetic");
    println!("Amoskeag: {}", example1);
    println!("JavaScript:");
    let js1 = transpile_source(example1, None)?;
    println!("{}\n", js1);

    // Example 2: Conditional expression
    let example2 = "if driver.age > 16 :continue else :deny end";
    println!("Example 2: Conditional");
    println!("Amoskeag: {}", example2);
    println!("JavaScript:");
    let js2 = transpile_source(example2, None)?;
    println!("{}\n", js2);

    // Example 3: String manipulation
    let example3 = "'hello' | upcase";
    println!("Example 3: Pipe operator");
    println!("Amoskeag: {}", example3);
    println!("JavaScript:");
    let js3 = transpile_source(example3, None)?;
    println!("{}\n", js3);

    // Example 4: Let binding
    let example4 = "let x = 10 in x * 2 + 5";
    println!("Example 4: Let binding");
    println!("Amoskeag: {}", example4);
    println!("JavaScript:");
    let js4 = transpile_source(example4, None)?;
    println!("{}\n", js4);

    // Example 5: Array operations
    let example5 = "[1, 2, 3, 4, 5] | sum";
    println!("Example 5: Array operations");
    println!("Amoskeag: {}", example5);
    println!("JavaScript:");
    let js5 = transpile_source(example5, None)?;
    println!("{}\n", js5);

    // Example 6: Complex nested expression
    let example6 = r#"
        let discount = if customer.vip true else false end in
        if discount
          price * 0.9
        else
          price
        end
    "#;
    println!("Example 6: Complex expression");
    println!("Amoskeag: {}", example6.trim());
    println!("JavaScript:");
    let js6 = transpile_source(example6, None)?;
    println!("{}\n", js6);

    // Example 7: Custom configuration (CommonJS instead of ES6)
    println!("Example 7: CommonJS module format");
    let config = TranspileConfig {
        use_es6_modules: false,
        ..Default::default()
    };
    let js7 = transpile_source("42", Some(config))?;
    println!("{}\n", js7);

    Ok(())
}
