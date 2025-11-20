# Amoskeag Language Examples

This directory contains 25 comprehensive examples demonstrating the Amoskeag programming language features and use cases.

## What is Amoskeag?

Amoskeag is a purely functional, statically-validated evaluation language designed for high-security, sandboxed evaluation. It's ideal for:

- **Business Rules Engines**: Complex decision logic with compile-time guarantees
- **Template Engines**: Secure alternative to ERB with more power than Liquid
- **Spreadsheet Formula Engines**: Formal execution model for cell formulas

## Quick Start

Each example directory contains:
- `example.amos` - The Amoskeag source code
- `README.md` - Detailed explanation and expected output

## Examples by Category

### Basic Language Features

| # | Example | Description |
|---|---------|-------------|
| 01 | [hello_world](01_hello_world/) | The simplest program - string literals |
| 02 | [arithmetic](02_arithmetic/) | Basic mathematical operations |
| 03 | [variables](03_variables/) | Let bindings and immutable variables |
| 04 | [conditionals](04_conditionals/) | If/else expressions |
| 16 | [comparisons](16_comparisons/) | Comparison operators (==, !=, <, >, <=, >=) |
| 17 | [logical_operators](17_logical_operators/) | Boolean logic (and, or, not) |

### String Operations

| # | Example | Description |
|---|---------|-------------|
| 05 | [string_operations](05_string_operations/) | String manipulation with pipe operator |
| 15 | [pipe_chaining](15_pipe_chaining/) | Complex multi-step transformations |
| 20 | [string_formatting](20_string_formatting/) | Advanced string formatting and joining |

### Data Structures

| # | Example | Description |
|---|---------|-------------|
| 06 | [array_operations](06_array_operations/) | Array functions (sum, avg, size, first, last) |
| 07 | [dictionary_access](07_dictionary_access/) | Dictionary navigation with dot notation |
| 18 | [nested_data](18_nested_data/) | Deep nested data access |
| 19 | [array_filtering](19_array_filtering/) | Collection transformations and aggregates |

### Symbols & Business Rules

| # | Example | Description |
|---|---------|-------------|
| 08 | [symbols](08_symbols/) | Statically-validated enumeration values |
| 09 | [business_rule_simple](09_business_rule_simple/) | Credit card approval logic |
| 10 | [business_rule_complex](10_business_rule_complex/) | Insurance underwriting with multiple data sources |
| 25 | [validation_rule](25_validation_rule/) | Form validation with detailed error reporting |

### Templates

| # | Example | Description |
|---|---------|-------------|
| 11 | [template_greeting](11_template_greeting/) | Dynamic user greeting |
| 12 | [template_blog_post](12_template_blog_post/) | Blog post rendering with formatting |

### Spreadsheets & Formulas

| # | Example | Description |
|---|---------|-------------|
| 13 | [spreadsheet_formulas](13_spreadsheet_formulas/) | Excel-like CHOOSE and formula composition |
| 21 | [number_rounding](21_number_rounding/) | Math operations and rounding |
| 22 | [date_operations](22_date_operations/) | Date handling and formatting |

### Financial Calculations

| # | Example | Description |
|---|---------|-------------|
| 14 | [financial_calculations](14_financial_calculations/) | Loan payment calculations (PMT function) |
| 23 | [loan_calculator](23_loan_calculator/) | Complete mortgage calculator with analysis |
| 24 | [discount_calculator](24_discount_calculator/) | E-commerce pricing with tiered discounts |

## Key Language Features

### Pure Functional Programming
- No side effects, no mutation, no I/O
- Deterministic evaluation
- Thread-safe and parallelizable
- Referentially transparent

### Static Symbol Validation
```ruby
# Symbols must be declared at compile-time
:approve  # ✓ Valid if in symbols array
:deny     # ✓ Valid if in symbols array
:reject   # ✗ Compile error if not in symbols array
```

### Pipe Operator
```ruby
# Left-to-right data transformation
name | downcase | capitalize
# Equivalent to: capitalize(downcase(name))
```

### Expression-Based
```ruby
# Everything returns a value
let result = if x > 10 then "big" else "small" end
```

### Safe Navigation
```ruby
# Missing keys return nil, don't crash
user.profile.settings.theme  # Returns nil if any part is missing
```

## Language Syntax Overview

### Data Types
- **Number**: 64-bit floating-point
- **String**: UTF-8 immutable strings
- **Boolean**: `true` or `false`
- **Nil**: The `nil` value
- **Array**: `[1, 2, 3]`
- **Dictionary**: `{"key": "value"}`
- **Symbol**: `:symbol_name`

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `and`, `or`, `not`
- **Pipe**: `|` (function chaining)
- **Access**: `.` (dictionary navigation)

### Control Flow
```ruby
# If expressions (must have else)
if condition
  value1
else
  value2
end

# Let bindings
let x = 10
in x * 2
```

## Standard Library Functions

### String Functions
`upcase`, `downcase`, `capitalize`, `strip`, `lstrip`, `rstrip`, `truncate`, `replace`, `split`, `join`

### Numeric Functions
`abs`, `ceil`, `floor`, `round`, `plus`, `minus`, `times`, `divided_by`, `modulo`

### Collection Functions
`sum`, `avg`, `size`, `first`, `last`, `contains`, `sort`, `keys`, `values`

### Financial Functions
`pmt`, `npv`

### Date Functions
`date_now`, `date_format`

### Logic Functions
`choose` (Excel CHOOSE equivalent)

## Security Features

### Immune to SSTI/RCE
Amoskeag is immune by design to Server-Side Template Injection and Remote Code Execution:
- No I/O operations possible
- No system access
- No reflection
- No way to "escape" the sandbox

### Compile-Time Guarantees
- All symbols validated at compile-time
- Function signatures checked
- Type errors caught before execution

### Thread-Safe by Design
- No shared state
- No mutation
- Can evaluate thousands of scripts in parallel

## Use Cases

### 1. Business Rules Engine
```ruby
if applicant.credit_score < 650
  :deny
else if applicant.income < 50000
  :manual_review
else
  :approve
end
```

### 2. Template Engine
```ruby
user.name | capitalize + " - Last login: " + user.last_login | date_format("MMM DD")
```

### 3. Spreadsheet Formulas
```ruby
let growth = choose(scenario, growth_rates)
in revenue * (1 + growth) | round(2)
```

## Running Examples

Each example is a standalone Amoskeag program that can be evaluated with:

```rust
use amoskeag::{compile, evaluate};

// 1. Define the valid symbols (if needed)
let symbols = vec![":approve", ":deny"];

// 2. Compile the script
let compiled = compile(script, &symbols)?;

// 3. Provide data context
let data = json!({
    "applicant": {
        "age": 30,
        "income": 55000
    }
});

// 4. Evaluate
let result = evaluate(&compiled, &data)?;
```

## Learning Path

**Beginners**: Start with examples 01-08 to learn basic syntax and data types.

**Intermediate**: Examples 09-17 cover business logic, symbols, and operators.

**Advanced**: Examples 18-25 demonstrate real-world applications and complex scenarios.

## Additional Resources

- [Main README](../README.md) - Project overview
- [AMOSKEAG.md](../AMOSKEAG.md) - Complete formal specification
- [Lexer](../lib/amoskeag-lexer/) - Tokenization implementation
- [Parser](../lib/amoskeag-parser/) - Syntax analysis implementation

## Contributing

To add a new example:
1. Create a new numbered subdirectory (e.g., `26_new_feature/`)
2. Add `example.amos` with the code
3. Add `README.md` with explanation and expected output
4. Update this README with the new example

## License

MIT OR Apache-2.0
