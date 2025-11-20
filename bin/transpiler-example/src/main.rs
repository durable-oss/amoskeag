//! Example: Transpiling Amoskeag to Python
//!
//! This example demonstrates how to use the amoskeag-python-transpiler
//! to convert Amoskeag code into executable Python code.

use amoskeag_python_transpiler::{transpile_source, TranspileConfig};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Amoskeag to Python Transpiler Example ===\n");

    // Example 1: Simple underwriting rule
    println!("Example 1: Simple Underwriting Rule");
    println!("====================================\n");

    let simple_rule = r#"
if driver.age > 16
  :continue
else
  :deny
end
    "#;

    println!("Amoskeag source:");
    println!("{}\n", simple_rule);

    let python_code = transpile_source(simple_rule, None)?;

    println!("Generated Python code:");
    println!("{}\n", python_code);

    // Save to file
    fs::write("examples/python-transpiler/simple_rule.py", &python_code)?;
    println!("Saved to: examples/python-transpiler/simple_rule.py\n");

    // Example 2: Complex rule with let bindings
    println!("\n\nExample 2: Complex Rule with Let Bindings");
    println!("==========================================\n");

    let complex_rule = r#"
let min_age = 16
in
let max_value = 100000
in
  if driver.age < min_age
    :deny
  else if vehicle.value > max_value
    :manual_review
  else if vehicle.type == 'SPORT' and driver.age < 25
    :manual_review
  else
    :approve
  end
    "#;

    println!("Amoskeag source:");
    println!("{}\n", complex_rule);

    let python_code = transpile_source(complex_rule, None)?;

    println!("Generated Python code:");
    println!("{}\n", python_code);

    // Save to file
    fs::write("examples/python-transpiler/complex_rule.py", &python_code)?;
    println!("Saved to: examples/python-transpiler/complex_rule.py\n");

    // Example 3: String manipulation with pipes
    println!("\n\nExample 3: String Manipulation with Pipes");
    println!("==========================================\n");

    let string_example = r#"
name | downcase | truncate(10)
    "#;

    println!("Amoskeag source:");
    println!("{}\n", string_example);

    let python_code = transpile_source(string_example, None)?;

    println!("Generated Python code:");
    println!("{}\n", python_code);

    // Example 4: Custom configuration
    println!("\n\nExample 4: Custom Configuration (2-space indent)");
    println!("=================================================\n");

    let config = TranspileConfig {
        indent: "  ".to_string(), // 2 spaces instead of 4
        include_runtime_imports: true,
        type_hints: true,
    };

    let simple_rule_custom = "if x > 10 :high else :low end";

    let python_code = transpile_source(simple_rule_custom, Some(config))?;

    println!("Generated Python code:");
    println!("{}\n", python_code);

    println!("\n=== All examples completed successfully! ===");
    println!("\nYou can now run the generated Python files:");
    println!("  python examples/python-transpiler/simple_rule.py");
    println!("  python examples/python-transpiler/complex_rule.py");

    Ok(())
}
