# Amoskeag

A purely functional, statically-validated evaluation language designed for high-security, sandboxed evaluation.

## Overview

Amoskeag is a Domain-Specific Language (DSL) engineered from first principles for high-security, sandboxed evaluation. Its core purpose is to evaluate domain-specific logic in a provably safe, deterministic, and efficient manner.

### Key Features

- **Purely Functional**: No side effects, no mutation, no I/O
- **Statically Validated**: Symbols are validated at compile-time
- **Expression-Based**: Everything is an expression that yields a value
- **Secure by Design**: Immune to SSTI and RCE vulnerabilities
- **Developer-Friendly**: Ruby-inspired syntax with functional composition

### Use Cases

1. **Business Rules Engine**: Evaluate complex business logic with compile-time guarantees
2. **Template Engine**: Secure alternative to ERB with more power than Liquid
3. **Spreadsheet Formula Engine**: Formal execution model for cell formulas

## Project Structure

```
amoskeag/
├── lib/
│   ├── amoskeag/                    # Main library (compiler & interpreter)
│   ├── amoskeag-lexer/              # Lexical analysis (tokenization)
│   ├── amoskeag-parser/             # Syntactic analysis using nom
│   ├── amoskeag-stdlib-operators/   # Standard library operators
│   ├── amoskeag-stdlib-functions/   # Standard library functions
│   └── amoskeag-jit/                # JIT compiler using LLVM
├── bin/
│   └── amoskeag-cli/                # Command-line interface
├── Cargo.toml                       # Workspace configuration
└── README.md                        # This file
```

## Building

This project uses [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) for cross-platform builds.

### Prerequisites

Install zigbuild:
```bash
cargo install cargo-zigbuild
```

### Build Commands

```bash
# Standard build
cargo build

# Build with zigbuild
cargo zigbuild --release

# Run tests
cargo test

# Build for specific target
cargo zigbuild --target x86_64-unknown-linux-gnu
```

### Building with JIT Support

The `amoskeag-jit` crate requires LLVM 18 with development headers:

```bash
# Ubuntu/Debian
sudo apt-get install llvm-18-dev

# macOS
brew install llvm@18

# Build with JIT support
cargo build -p amoskeag-jit
```

See [lib/amoskeag-jit/README.md](lib/amoskeag-jit/README.md) for detailed JIT compiler documentation.

## Language Syntax

### Example: Business Rule

```ruby
let app = applicant
let limits = env.underwriting_limits

let state_is_restricted = limits.restricted_states | contains(app.state)

if state_is_restricted
  :deny
else if app.vehicle.value > limits.max_vehicle_value
  :deny
else if app.vehicle.type == 'SPORT' and app.age < 25
  :deny
else
  :approve
end
```

### Example: Template

```ruby
# String manipulation with pipe operator
post.title | upcase | truncate(50)

# Conditional rendering
if user.is_admin
  "Welcome, Admin!"
else
  "Welcome, " + user.name
end
```

### Example: Spreadsheet Formula

```ruby
# Excel-like functions
let growth_rate = choose(B1, B2)
let revenue = B3 * (1 + growth_rate)
revenue | round(2)
```

## Core Language Features

### Data Types

- **Number**: 64-bit floating-point
- **String**: UTF-8 immutable strings
- **Boolean**: `true` or `false`
- **Nil**: The `nil` value
- **Array**: Ordered, immutable list
- **Dictionary**: Immutable key-value map
- **Symbol**: Statically-validated enumeration (e.g., `:approve`, `:deny`)

### Operators

- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `and`, `or`, `not`
- **Pipe**: `|` (function chaining)
- **Access**: `.` (dictionary navigation)

### Keywords

`if`, `else`, `end`, `let`, `in`, `true`, `false`, `nil`, `and`, `or`, `not`

## License

MIT OR Apache-2.0

## Documentation

See [AMOSKEAG.md](./AMOSKEAG.md) for the complete formal specification.
