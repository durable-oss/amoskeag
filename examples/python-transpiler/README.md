# Amoskeag to Python Transpiler Examples

This directory contains examples of using the Amoskeag to Python transpiler.

## Overview

The `amoskeag-python-transpiler` crate allows you to convert Amoskeag programs into executable Python code. This is useful for:

- **Integration**: Running Amoskeag logic in Python-based systems
- **Performance**: Pre-compiling rules for faster execution
- **Deployment**: Creating standalone Python scripts from Amoskeag programs

## Example: Business Rule Transpilation

### Amoskeag Source (`underwriting_rule.amoskeag`)

```amoskeag
if driver.age > 16
  :continue
else
  :deny
end
```

### Transpiled Python (`underwriting_rule.py`)

Run the transpiler example to generate Python code:

```bash
cargo run --example transpile_rule
```

This will generate a Python file that can be executed standalone.

## Example: Using the Transpiler Programmatically

```rust
use amoskeag_python_transpiler::{transpile_source, TranspileConfig};

fn main() {
    let amoskeag_code = r#"
        if driver.age > 16
          :continue
        else
          :deny
        end
    "#;

    let python_code = transpile_source(amoskeag_code, None).unwrap();
    println!("{}", python_code);
}
```

## Running the Generated Python

The generated Python code includes an `evaluate` function that takes a data dictionary:

```python
# Generated Python code can be executed like this:
data = {
    "driver": {
        "age": 25
    }
}

result = evaluate(data)
print(result)  # Output: ":continue"
```

## Advanced Usage

### Custom Configuration

```rust
use amoskeag_python_transpiler::TranspileConfig;

let config = TranspileConfig {
    indent: "  ".to_string(),  // Use 2 spaces
    include_runtime_imports: true,
    type_hints: true,
};

let python_code = transpile_source(source, Some(config)).unwrap();
```

### Using the Runtime Module

For complex projects, you can use the standalone runtime module:

```python
# my_app.py
from amoskeag_runtime import get_nested, is_truthy

# Your transpiled code here...
```

Copy `runtime.py` from the `amoskeag-python-transpiler` crate to your Python project.
