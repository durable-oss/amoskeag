# Amoskeag Python Transpiler

A transpiler that converts [Amoskeag](https://github.com/durableprogramming/amoskeag) code into executable Python code.

## Overview

The `amoskeag-python-transpiler` crate provides functionality to transpile Amoskeag programs (a purely functional, statically-validated evaluation language) into Python code that can be executed in standard Python environments.

## Features

- **Full Language Support**: Transpiles all Amoskeag language constructs including:
  - Literals (numbers, strings, booleans, symbols, arrays, dictionaries)
  - Variables and nested field access
  - Binary and unary operations
  - Conditional expressions (if-else)
  - Let bindings (local variables)
  - Function calls (mapped to Python equivalents)
  - Pipe operators (function chaining)

- **Standard Library Mapping**: Automatically maps Amoskeag standard library functions to Python equivalents:
  - String functions (`upcase`, `downcase`, `capitalize`, etc.)
  - Numeric functions (`abs`, `ceil`, `floor`, `round`, etc.)
  - Collection functions (`size`, `first`, `last`, `sum`, `avg`, etc.)
  - Logic functions (`choose`, `if_then_else`, type checking functions)

- **Python Runtime**: Includes helper functions for safe navigation and Amoskeag semantics:
  - Safe dictionary navigation (nil-forgiving access)
  - Truthiness handling (nil and false are falsy, everything else is truthy)

- **Configurable Output**: Control indentation, imports, and type hints

## Usage

### Basic Example

```rust
use amoskeag_python_transpiler::transpile_source;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let amoskeag_code = r#"
        if driver.age > 16
          :continue
        else
          :deny
        end
    "#;

    let python_code = transpile_source(amoskeag_code, None)?;
    println!("{}", python_code);

    Ok(())
}
```

### Custom Configuration

```rust
use amoskeag_python_transpiler::{transpile_source, TranspileConfig};

let config = TranspileConfig {
    indent: "  ".to_string(),          // Use 2 spaces
    include_runtime_imports: true,     // Include imports
    type_hints: true,                  // Add type hints
};

let python_code = transpile_source(source, Some(config))?;
```

### Generated Python Code

The transpiler generates a Python function `evaluate(data)` that takes a dictionary containing the evaluation context:

```python
# Generated code (simplified)
def evaluate(data: Dict[str, Any]) -> Any:
    """Evaluate the Amoskeag program."""
    def _get_nested(obj: Any, *keys: str) -> Any:
        # Safe navigation helper
        ...

    def _is_truthy(val: Any) -> bool:
        # Truthiness checker
        ...

    return (":continue" if _is_truthy(_get_nested(data, "driver", "age") > 16) else ":deny")
```

### Using the Generated Code

```python
# Execute the generated Python
data = {
    "driver": {
        "age": 25
    }
}

result = evaluate(data)
print(result)  # Output: ":continue"
```

## Python Runtime Module

For projects that need to integrate multiple transpiled Amoskeag programs, you can use the standalone `runtime.py` module:

1. Copy `runtime.py` from `lib/amoskeag-python-transpiler/runtime.py` to your Python project
2. Import the helper functions in your code:

```python
from amoskeag_runtime import get_nested, is_truthy

# Your custom Python code here
```

## API Documentation

### `transpile_source`

Transpile Amoskeag source code to Python (convenience function).

```rust
pub fn transpile_source(
    source: &str,
    config: Option<TranspileConfig>,
) -> Result<String, Box<dyn std::error::Error>>
```

### `transpile`

Transpile an Amoskeag AST to Python code.

```rust
pub fn transpile(
    expr: &Expr,
    config: &TranspileConfig,
) -> Result<String, TranspileError>
```

### `TranspileConfig`

Configuration for the transpiler.

```rust
pub struct TranspileConfig {
    /// Indentation string (default: 4 spaces)
    pub indent: String,
    /// Whether to include runtime imports (default: true)
    pub include_runtime_imports: bool,
    /// Whether to generate type hints (default: true)
    pub type_hints: bool,
}
```

## Examples

See `examples/transpile_rule.rs` for a complete example:

```bash
cargo run --example transpile_rule
```

This will:
1. Transpile several Amoskeag programs to Python
2. Save the generated Python files to `examples/python-transpiler/`
3. Display the transpiled code

## Use Cases

- **Integration**: Run Amoskeag business rules in Python-based systems
- **Performance**: Pre-compile rules for faster execution
- **Deployment**: Create standalone Python scripts from Amoskeag programs
- **Testing**: Generate Python code for testing Amoskeag logic
- **Migration**: Gradually migrate from Amoskeag to Python

## Limitations

- Pipe expressions are transformed by the parser before transpilation
- Custom functions defined at runtime are not supported (only standard library)
- Symbol validation happens at Amoskeag compile-time, not Python runtime

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
