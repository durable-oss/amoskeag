# Amoskeag Ruby Gem

Native Ruby bindings for [Amoskeag](https://github.com/durableprogramming/amoskeag) - a secure, purely functional DSL for business rules, templates, and spreadsheet formulas.

## Features

- **Security-First Design**: Immune to code injection attacks (SSTI, RCE)
- **Purely Functional**: No side effects, no mutation, no I/O
- **Static Validation**: Symbols and functions validated at compile-time
- **High Performance**: Native Rust implementation with zero-copy FFI
- **Rich Standard Library**: 60+ built-in functions for strings, numbers, collections, and finance
- **Multiple Use Cases**:
  - Business Rules Engine (insurance, loans, eligibility)
  - Template Engine (secure alternative to ERB)
  - Spreadsheet Formulas (Excel-compatible financial functions)

## Installation

### Prerequisites

- Ruby 2.7 or higher
- Rust and Cargo (for building the native extension)

### Install from source

```bash
gem build amoskeag-rb.gemspec
gem install amoskeag-rb-0.1.0.gem
```

Or add to your Gemfile:

```ruby
gem 'amoskeag-rb', path: 'path/to/amoskeag/ext'
```

## Quick Start

```ruby
require 'amoskeag-rb'

# Basic arithmetic
Amoskeag.eval("2 + 2", {})
# => 4.0

# Using variables
Amoskeag.eval("user.age * 2", { "user" => { "age" => 25 } })
# => 50.0

# String operations with pipe
Amoskeag.eval('"hello world" | upcase', {})
# => "HELLO WORLD"

# Business rules with symbols
code = "if user.age >= 18 :adult else :minor end"
Amoskeag.eval(code, { "user" => { "age" => 25 } }, [:adult, :minor])
# => :adult
```

## Core Concepts

### Compile Once, Evaluate Many

For best performance, compile programs once and evaluate them multiple times:

```ruby
# Compile the program (with symbol validation)
program = Amoskeag.compile(
  "if score >= 90 :A else :B end",
  [:A, :B, :C, :D, :F]
)

# Evaluate with different data
Amoskeag.evaluate(program, { "score" => 95 })  # => :A
Amoskeag.evaluate(program, { "score" => 85 })  # => :B
```

### Data Types

Amoskeag supports seven fundamental types:

```ruby
# Numbers (64-bit float)
Amoskeag.eval("42", {})  # => 42.0

# Strings
Amoskeag.eval('"hello"', {})  # => "hello"

# Booleans
Amoskeag.eval("true and false", {})  # => false

# Nil
Amoskeag.eval("nil", {})  # => nil

# Arrays
Amoskeag.eval("[1, 2, 3]", {})  # => [1.0, 2.0, 3.0]

# Dictionaries (hashes)
Amoskeag.eval('{"name": "Alice", "age": 30}', {})
# => {"name" => "Alice", "age" => 30.0}

# Symbols (validated at compile-time)
Amoskeag.eval(":approved", {}, [:approved, :denied])
# => :approved
```

### Safe Navigation

Missing keys return `nil` instead of raising errors:

```ruby
Amoskeag.eval("user.address.street", { "user" => {} })
# => nil
```

## Examples

### Business Rules Engine

```ruby
# Define eligibility rules
rules = <<~AMOSKEAG
  let applicant = {
    "age": user.age,
    "state": user.state,
    "credit_score": user.credit_score
  }
  in let restricted_states = ["FL", "LA", "TX"]
  in
    if contains(restricted_states, applicant.state)
      :deny
    else if applicant.age < 18
      :deny
    else if applicant.credit_score < 650
      :conditional
    else
      :approve
    end
AMOSKEAG

# Compile once
program = Amoskeag.compile(rules, [:approve, :deny, :conditional])

# Evaluate for different users
user1 = { "age" => 25, "state" => "CA", "credit_score" => 720 }
Amoskeag.evaluate(program, { "user" => user1 })  # => :approve

user2 = { "age" => 30, "state" => "FL", "credit_score" => 750 }
Amoskeag.evaluate(program, { "user" => user2 })  # => :deny
```

### Template Engine

```ruby
template = <<~AMOSKEAG
  "Hello, " + user.name + "! " +
  if user.premium
    "Welcome to Premium. "
  else
    "Upgrade to Premium today! "
  end +
  "You have " + user.messages | string + " new messages."
AMOSKEAG

program = Amoskeag.compile(template, [])

data = {
  "user" => {
    "name" => "Alice",
    "premium" => true,
    "messages" => 5
  }
}

Amoskeag.evaluate(program, data)
# => "Hello, Alice! Welcome to Premium. You have 5 new messages."
```

### Financial Calculations

```ruby
# Monthly payment for a $250,000 loan at 4.5% APR for 30 years
formula = "pmt(rate / 12, years * 12, -principal) | round(2)"
program = Amoskeag.compile(formula, [])

data = {
  "rate" => 0.045,
  "years" => 30,
  "principal" => 250000
}

Amoskeag.evaluate(program, data)
# => 1266.71
```

### Array and Collection Operations

```ruby
# Calculate average of filtered values
code = <<~AMOSKEAG
  let nums = [95, 82, 67, 88, 91, 73]
  in let passing = nums | filter(lambda(x) { x >= 70 })
  in passing | avg | round(2)
AMOSKEAG

Amoskeag.eval(code, {})
# => 85.8
```

## Standard Library

Amoskeag includes 60+ built-in functions organized into categories:

### String Functions
- `upcase`, `downcase`, `capitalize`
- `strip`, `lstrip`, `rstrip`
- `split`, `join`, `replace`
- `truncate`, `prepend`, `append`

### Numeric Functions
- `abs`, `ceil`, `floor`, `round`
- `max`, `min`, `clamp`
- `plus`, `minus`, `times`, `divided_by`

### Collection Functions
- `size`, `first`, `last`, `at`
- `contains`, `sort`, `reverse`
- `sum`, `avg`, `max`, `min`
- `keys`, `values`

### Logic Functions
- `if_then_else`, `choose`
- `is_number`, `is_string`, `is_boolean`, `is_nil`, `is_array`, `is_dictionary`
- `coalesce`, `default`

### Financial Functions (Excel-compatible)
- **Time Value**: `pmt`, `pv`, `fv`, `nper`, `rate`
- **Investment**: `npv`, `irr`, `mirr`
- **Depreciation**: `sln`, `ddb`, `db`
- **Payment Components**: `ipmt`, `ppmt`, `cumipmt`, `cumprinc`

## Error Handling

```ruby
begin
  # Undefined symbol will raise CompileError
  Amoskeag.compile(":invalid_symbol", [:valid, :symbols])
rescue Amoskeag::CompileError => e
  puts "Compilation failed: #{e.message}"
end

begin
  # Missing variable will raise EvalError
  program = Amoskeag.compile("missing_var", [])
  Amoskeag.evaluate(program, {})
rescue Amoskeag::EvalError => e
  puts "Evaluation failed: #{e.message}"
end
```

## Performance Tips

1. **Compile once, evaluate many**: Compilation has overhead, so reuse compiled programs
2. **Use native types**: Ruby Symbols map directly to Amoskeag Symbols with zero overhead
3. **Keep data shallow**: Deep nesting requires more JSON serialization
4. **Avoid string concatenation in loops**: Use `join` function instead

## Thread Safety

Compiled programs are immutable and thread-safe. You can safely evaluate the same program from multiple threads concurrently:

```ruby
program = Amoskeag.compile("x * 2", [])

threads = 10.times.map do |i|
  Thread.new do
    Amoskeag.evaluate(program, { "x" => i })
  end
end

results = threads.map(&:value)
# => [0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0]
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/durableprogramming/amoskeag.git
cd amoskeag

# Build and install the gem
cd ext
gem build amoskeag-rb.gemspec
gem install amoskeag-rb-0.1.0.gem
```

### Development

```bash
# Build the native extension
cd ext/amoskeag
ruby extconf.rb
make

# Run tests (if available)
rake test
```

## Architecture

The gem consists of three layers:

1. **Rust Core** (`lib/amoskeag`): Pure Rust implementation of the Amoskeag compiler and interpreter
2. **FFI Layer** (`ext/amoskeag/src/lib.rs`): C-compatible FFI bindings using static library
3. **Ruby Extension** (`ext/amoskeag/amoskeag_native.c`): Native Ruby C extension
4. **Ruby Wrapper** (`lib/amoskeag-rb.rb`): Idiomatic Ruby API with proper marshalling

Data flows through JSON serialization at the Ruby/C boundary for simplicity and type safety.

## License

MIT

## Contributing

Contributions are welcome! Please see the main [Amoskeag repository](https://github.com/durableprogramming/amoskeag) for contribution guidelines.

## Support

- GitHub Issues: https://github.com/durableprogramming/amoskeag/issues
- Documentation: https://github.com/durableprogramming/amoskeag

## Credits

Amoskeag is built by [Durable Programming](https://github.com/durableprogramming) with a focus on security, correctness, and developer experience.
