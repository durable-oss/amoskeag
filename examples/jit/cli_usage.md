# JIT Backend CLI Usage

This document demonstrates how to use the JIT backend via the Amoskeag CLI.

## Building with JIT Support

To enable the JIT backend, build the CLI with the `jit` feature:

```bash
cargo build --release -p amoskeag-cli --features jit
```

Or run directly with the feature:

```bash
cargo run --release -p amoskeag-cli --features jit -- <args>
```

## Usage Examples

### Basic Arithmetic

```bash
# Using the default interpreter backend
amoskeag eval "2 + 3"
# Output: 5

# Using the JIT backend (faster for numeric computations)
amoskeag eval "2 + 3 * 4" --backend jit
# Output: 14
```

### Complex Expressions

```bash
# JIT supports complex numeric expressions
amoskeag eval "(10 + 5) * 2 - 8 / 4" --backend jit
# Output: 28

# JIT supports comparisons
amoskeag eval "5 > 3" --backend jit
# Output: 1 (true as float)

# JIT supports if expressions
amoskeag eval "if 10 > 5 then 42 else 0 end" --backend jit
# Output: 42

# JIT supports let bindings
amoskeag eval "let x = 10 in x * 2 + 5" --backend jit
# Output: 25
```

### Using Files

Create a file `example.amos`:

```
let base = 100
in base * 1.5 + 25
```

Run it with the JIT backend:

```bash
amoskeag run example.amos --backend jit
# Output: 175
```

### REPL Mode

Start a REPL with the JIT backend:

```bash
amoskeag repl --backend jit
```

Then try expressions interactively:

```
> 2 + 3
=> 5

> let x = 10 in x * x
=> 100

> if 5 > 3 then 42 else 0 end
=> 42

> exit
```

## Limitations

The JIT backend currently has the following limitations:

1. **No Runtime Data Access**: Cannot access variables from JSON data files
   ```bash
   # This will NOT work with JIT:
   echo '{"price": 100}' > data.json
   amoskeag eval "price * 1.2" data.json --backend jit
   # Error: JIT backend does not support this expression

   # Use interpreter instead:
   amoskeag eval "price * 1.2" data.json --backend interpreter
   # Output: 120
   ```

2. **Numeric Expressions Only**: Only supports numbers and booleans
   ```bash
   # This will NOT work with JIT:
   amoskeag eval "'hello' | upcase" --backend jit
   # Error: JIT backend does not support this expression

   # Use interpreter instead:
   amoskeag eval "'hello' | upcase" --backend interpreter
   # Output: HELLO
   ```

3. **No Function Calls**: Standard library functions are not supported
   ```bash
   # This will NOT work with JIT:
   amoskeag eval "abs(-5)" --backend jit
   # Error: JIT backend does not support this expression

   # Use interpreter instead:
   amoskeag eval "abs(-5)" --backend interpreter
   # Output: 5
   ```

## When to Use JIT vs Interpreter

### Use JIT When:
- Performing numeric computations
- Running expressions many times (compilation overhead amortized)
- Maximum performance is needed for mathematical operations
- Expression doesn't require runtime data or stdlib functions

### Use Interpreter When:
- Working with strings, arrays, or dictionaries
- Need runtime data access from JSON files
- Using standard library functions
- Need full language feature support

## Performance Comparison

For numeric workloads, the JIT backend can be 10-100x faster than the interpreter:

```bash
# Interpreter (slower)
time amoskeag eval "let x = 10 in x * x * x * x * x"
# ~0.001s

# JIT (faster)
time amoskeag eval "let x = 10 in x * x * x * x * x" --backend jit
# ~0.0001s (after compilation)
```

Note: For simple expressions, the compilation overhead may outweigh the performance gain. JIT is most beneficial when:
- Compiling once and executing many times
- Expression has complex numeric computations
- Working with large numeric datasets

## Backend Detection

The CLI will automatically warn you if an expression is not supported by the JIT backend:

```bash
amoskeag eval "user.age * 2" --backend jit
# Error: JIT backend does not support this expression. Try using --backend interpreter instead.
# Note: JIT currently only supports numeric expressions without runtime data access.
```

## Requirements

The JIT backend requires:
- LLVM 18 with development headers
- Build with `--features jit`

If LLVM is not available, the CLI will gracefully inform you:

```bash
amoskeag eval "2 + 3" --backend jit
# Error: JIT backend not available. Rebuild with --features jit to enable.
```

## Summary

The JIT backend provides near-native performance for numeric computations while maintaining the same CLI interface. Use `--backend jit` to enable it, and fall back to `--backend interpreter` (the default) for full language support.
