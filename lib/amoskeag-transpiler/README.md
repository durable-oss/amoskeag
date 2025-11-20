# Amoskeag Transpiler

A transpiler that converts Amoskeag AST to Rust source code.

## Overview

The `amoskeag-transpiler` crate provides functionality to transpile Amoskeag expressions into equivalent Rust code. The generated code uses the `amoskeag-stdlib-operators` and `amoskeag-stdlib-functions` libraries to maintain semantic equivalence with the interpreted version.

## Features

- **Complete AST Coverage**: Supports all Amoskeag expression types including:
  - Literals (numbers, strings, booleans, nil, symbols)
  - Arrays and dictionaries
  - Variables with dot navigation
  - Function calls
  - Let bindings
  - If expressions
  - Binary and unary operations
  - Pipe expressions

- **Type-Safe Code Generation**: Generated Rust code uses the same Value enum and functions as the interpreter

- **Configurable Output**: Control type checking, comments, and indentation in generated code

## Usage

```rust
use amoskeag_transpiler::Transpiler;
use amoskeag_parser::{Expr, Parser};
use amoskeag_lexer::Lexer;

// Parse Amoskeag source code
let source = "driver.age > 16";
let mut lexer = Lexer::new(source);
let tokens = lexer.tokenize().unwrap();
let mut parser = Parser::new(tokens);
let ast = parser.parse().unwrap();

// Transpile to Rust
let mut transpiler = Transpiler::new();
let rust_code = transpiler.transpile(&ast).unwrap();

println!("{}", rust_code);
```

This will generate Rust code like:

```rust
use amoskeag_stdlib_operators::{Value, *};
use amoskeag_stdlib_functions::*;
use std::collections::HashMap;

pub fn evaluate(context: &HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(greater_than(&{let mut current = context.get("driver").cloned().unwrap_or(Value::Nil); current = match current { Value::Dictionary(ref map) => map.get("age").cloned().unwrap_or(Value::Nil), _ => Value::Nil }; current}, &Value::Number(16))?)
}
```

## Configuration

You can customize the transpiler output:

```rust
use amoskeag_transpiler::{Transpiler, TranspilerConfig};

let config = TranspilerConfig {
    type_checking: true,     // Enable runtime type checking
    add_comments: true,      // Add explanatory comments
    indent: "    ".to_string(), // Use 4 spaces for indentation
};

let mut transpiler = Transpiler::with_config(config);
```

## Examples

### Simple Arithmetic

**Amoskeag:**
```
2 + 3 * 4
```

**Generated Rust:**
```rust
add(&Value::Number(2), &multiply(&Value::Number(3), &Value::Number(4))?)?)
```

### Function Calls

**Amoskeag:**
```
upcase('hello')
```

**Generated Rust:**
```rust
upcase(&Value::String("hello".to_string()))?
```

### Pipe Expression

**Amoskeag:**
```
'hello' | upcase
```

**Generated Rust:**
```rust
upcase(&Value::String("hello".to_string()))?
```

### If Expression

**Amoskeag:**
```
if driver.age > 16
    :continue
else
    :deny
end
```

**Generated Rust:**
```rust
{ let cond_value = greater_than(&{...}, &Value::Number(16))?;
  let is_truthy = match cond_value { Value::Boolean(b) => b, Value::Nil => false, _ => true };
  if is_truthy { Value::Symbol("continue".to_string()) }
  else { Value::Symbol("deny".to_string()) }
}
```

## Integration

The transpiler is designed to work seamlessly with other Amoskeag crates:

- **amoskeag-lexer**: Tokenizes source code
- **amoskeag-parser**: Parses tokens into AST
- **amoskeag-transpiler**: Converts AST to Rust code (this crate)
- **amoskeag-stdlib-operators**: Runtime operators used by generated code
- **amoskeag-stdlib-functions**: Runtime functions used by generated code

## Use Cases

- **Ahead-of-Time Compilation**: Compile Amoskeag rules to native Rust code for maximum performance
- **Code Generation**: Generate Rust implementations from Amoskeag specifications
- **Static Analysis**: Inspect the structure of Amoskeag programs through generated code
- **Optimization**: Apply Rust compiler optimizations to Amoskeag logic

## Limitations

- The generated code is not always idiomatic Rust (it mirrors the interpreter's semantics)
- Some runtime checks remain in the generated code for safety
- The output is designed to be compiled, not necessarily human-readable

## License

MIT OR Apache-2.0
