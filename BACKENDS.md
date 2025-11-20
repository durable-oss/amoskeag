# Amoskeag Backend Architecture

This document describes the unified backend architecture that allows Amoskeag to support multiple execution and code generation backends through a common interface.

## Overview

Amoskeag now supports multiple backends for executing and transpiling programs:

| Backend | Type | Performance | Dependencies | Status |
|---------|------|-------------|--------------|--------|
| **Interpreter** | Tree-walking evaluator | Standard | None | ✅ Complete |
| **JIT Compiler** | LLVM-based compilation | Near-native | LLVM 18 | ✅ Numeric expressions |
| **Python Transpiler** | Code generation | Transpiled | Python runtime | ✅ Complete |
| **Ruby Transpiler** | Code generation | Transpiled | Ruby runtime | ✅ Complete |

## Unified Backend Trait

All backends implement the `Backend` trait, which provides a consistent interface:

```rust
pub trait Backend {
    type CompiledOutput;
    type ExecutionResult;

    fn name(&self) -> &str;
    fn compile(&self, expr: &Expr, symbols: &[&str]) -> BackendResult<Self::CompiledOutput>;
    fn execute(&self, compiled: &Self::CompiledOutput, data: &HashMap<String, Value>) -> BackendResult<Self::ExecutionResult>;
    fn compile_and_execute(&self, expr: &Expr, symbols: &[&str], data: &HashMap<String, Value>) -> BackendResult<Self::ExecutionResult>;
    fn supports(&self, expr: &Expr) -> bool;
    fn description(&self) -> &str;
}
```

## Integration Opportunities

### 1. Shared Testing Infrastructure

All backends can be tested using the same test cases to ensure consistent behavior:

```rust
use amoskeag::backend::Backend;

fn test_backend<B: Backend>(backend: &B) {
    let expr = parse_expr("2 + 3 * 4");
    let result = backend.compile_and_execute(&expr, &[], &HashMap::new()).unwrap();
    // Assert result is correct across all backends
}
```

### 2. Backend Selection via CLI

Users can choose which backend to use at runtime:

```bash
# Use interpreter (default)
amoskeag eval program.amk

# Use JIT compiler for performance
amoskeag eval --backend=jit program.amk

# Transpile to Python
amoskeag transpile --target=python program.amk > program.py

# Transpile to Ruby
amoskeag transpile --target=ruby program.amk > program.rb
```

### 3. Performance Benchmarking

Compare backend performance using the same workloads:

```rust
use std::time::Instant;

fn benchmark_backends(expr: &Expr) {
    // Test interpreter
    let start = Instant::now();
    interpreter.compile_and_execute(expr, &[], &data).unwrap();
    println!("Interpreter: {:?}", start.elapsed());

    // Test JIT
    let start = Instant::now();
    jit.compile_and_execute(expr, &[], &data).unwrap();
    println!("JIT: {:?}", start.elapsed());
}
```

### 4. Hybrid Execution Strategy

Automatically select the best backend based on expression characteristics:

```rust
fn choose_backend(expr: &Expr) -> Box<dyn Backend> {
    if is_numeric_heavy(expr) && jit.supports(expr) {
        Box::new(JitBackend::new())
    } else {
        Box::new(InterpreterBackend::new())
    }
}
```

### 5. Cross-Backend Validation

Use multiple backends to validate correctness:

```rust
fn validate_expression(expr: &Expr) -> bool {
    let interpreter_result = interpreter.execute(expr, &data);
    let jit_result = jit.execute(expr, &data);

    // Ensure both backends produce the same result
    interpreter_result == jit_result
}
```

## Backend Capabilities

### Interpreter

**Strengths:**
- Full language support
- No external dependencies
- Easy to debug
- Safe navigation

**Limitations:**
- Slower performance for compute-heavy workloads
- No native code generation

**Use Cases:**
- Development and debugging
- Embedded systems
- Simple evaluations
- Safe, sandboxed execution

### JIT Compiler (LLVM)

**Strengths:**
- Near-native performance
- LLVM optimizations
- Low-level code generation

**Limitations:**
- Requires LLVM 18 installation
- Currently limited to numeric expressions
- Larger binary size

**Use Cases:**
- Performance-critical applications
- Numeric computations
- High-throughput processing
- Real-time systems

### Python Transpiler

**Strengths:**
- Full language support
- Python ecosystem integration
- Readable output code
- Type hints

**Limitations:**
- Requires Python runtime
- Performance depends on Python
- Additional translation overhead

**Use Cases:**
- Python integration
- Data science workflows
- Gradual migration
- Debugging and inspection

### Ruby Transpiler

**Strengths:**
- Full language support
- Ruby ecosystem integration
- Natural syntax mapping
- Idiomatic Ruby output

**Limitations:**
- Requires Ruby runtime
- Performance depends on Ruby

**Use Cases:**
- Ruby on Rails integration
- Scripting and automation
- Template engines
- Legacy system integration

## Roadmap

### Short Term

- [ ] Complete JIT support for all expression types
- [ ] Add string and array support to JIT
- [ ] Implement function call dispatch in JIT
- [ ] Create unified test suite for all backends
- [ ] Add CLI backend selection

### Medium Term

- [ ] Implement WASM backend
- [ ] Add JavaScript transpiler
- [ ] Create bytecode interpreter
- [ ] Optimize interpreter with inline caching
- [ ] Add ahead-of-time (AOT) compilation

### Long Term

- [ ] GPU acceleration for array operations
- [ ] SIMD optimizations in JIT
- [ ] Distributed execution backend
- [ ] Query optimization for database integration

## Example: Using Multiple Backends

```rust
use amoskeag::backend::{Backend, BackendRegistry};
use amoskeag::backend::interpreter::DirectInterpreterBackend;
// use amoskeag_jit::JitBackend; // When LLVM is available
// use amoskeag_python_transpiler::PythonBackend;
// use amoskeag_transpiler_ruby::RubyBackend;

fn main() {
    let expr = parse_expr("let x = 10 in x * x");
    let data = HashMap::new();

    // Registry of available backends
    let mut registry = BackendRegistry::new();

    // Interpreter (always available)
    let interpreter = DirectInterpreterBackend::new();
    registry.register(DirectInterpreterBackend::capabilities());
    let result = interpreter.compile_and_execute(&expr, &[], &data).unwrap();
    println!("Interpreter: {:?}", result);

    // JIT (if LLVM is installed)
    #[cfg(feature = "jit")]
    {
        let jit = JitBackend::new();
        registry.register(JitBackend::capabilities());
        let result = jit.compile_and_execute(&expr, &[], &data).unwrap();
        println!("JIT: {:?}", result);
    }

    // List all available backends
    for caps in registry.list() {
        println!("{}: {}", caps.name, caps.description);
    }
}
```

## Contributing

When adding a new backend:

1. Implement the `Backend` trait
2. Provide `BackendCapabilities` metadata
3. Add comprehensive tests
4. Update this documentation
5. Add example usage
6. Benchmark against existing backends

## Performance Guidelines

- Use **Interpreter** for development and debugging
- Use **JIT** for numeric computations when LLVM is available
- Use **Transpilers** for ecosystem integration
- Profile before optimizing - measure actual performance
- Consider compilation overhead in total execution time

## See Also

- [lib/amoskeag/src/backend.rs](lib/amoskeag/src/backend.rs) - Backend trait definition
- [lib/amoskeag-jit/README.md](lib/amoskeag-jit/README.md) - JIT compiler documentation
- [lib/amoskeag-python-transpiler/README.md](lib/amoskeag-python-transpiler/README.md) - Python transpiler docs
- [examples/backend-comparison.rs](examples/backend-comparison.rs) - Backend comparison example
