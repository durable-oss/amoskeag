# Amoskeag SAST

Static Application Security Testing (SAST) for the Amoskeag language.

## Overview

The `amoskeag-sast` crate provides comprehensive static analysis tools for Amoskeag programs, including:

- **Error Detection**: Identifies programming errors such as division by zero, type mismatches, unreachable code, unused variables, and more.
- **Algebraic Analysis**: Uses constraint solving and symbolic execution to find input parameters that could trigger errors.
- **Range Analysis**: Tracks possible value ranges for expressions to detect potential issues.
- **Data Flow Analysis**: Analyzes how data flows through the program to identify issues like undefined variables and dead code.

## Features

### Error Detection

The SAST analyzer can detect:

- **Division by Zero**: Both constant and potential (variable-based)
- **Array Out of Bounds**: Invalid array indices
- **Type Mismatches**: Operations on incompatible types
- **Unreachable Code**: Code that can never be executed
- **Unused Variables**: Variables that are defined but never used
- **Integer Overflow**: Arithmetic operations that exceed limits
- **Null Dereference**: Potential null pointer access
- **Security Issues**: Duplicate dictionary keys, suspicious patterns

### Algebraic Analysis

Uses constraint solving to find inputs that trigger errors:

- **Symbolic Execution**: Tracks symbolic values through the program
- **Constraint Solving**: Finds satisfying assignments for error conditions
- **Path Sensitivity**: Analyzes different execution paths
- **Example Generation**: Provides concrete examples of problematic inputs

### Range Analysis

Tracks value ranges through arithmetic operations:

- **Interval Arithmetic**: Computes ranges for expressions
- **Zero Detection**: Identifies when divisors could be zero
- **Overflow Detection**: Detects when operations exceed bounds

### Data Flow Analysis

Builds data flow graphs to understand variable usage:

- **Definition-Use Chains**: Tracks where variables are defined and used
- **Liveness Analysis**: Determines which variables are live at each point
- **Reaching Definitions**: Finds which definitions reach each use

## Usage

### Basic Analysis

```rust
use amoskeag_parser::parse;
use amoskeag_sast::analyze;

let expr = parse("10 / x").unwrap();
let result = analyze(&expr, &[]);

// Check for errors
for error in &result.errors {
    println!("{}: {}", error.severity, error.message);
    if let Some(suggestion) = &error.suggestion {
        println!("  Suggestion: {}", suggestion);
    }
}

// Check for vulnerable inputs
for vuln in &result.vulnerable_inputs {
    println!("Vulnerability: {}", vuln.description);
    println!("  Example input: {:?}", vuln.example_input);
}

// View statistics
println!("Critical errors: {}", result.statistics.critical_errors);
println!("Warnings: {}", result.statistics.warnings);
```

### JSON Output

```rust
use amoskeag_parser::parse;
use amoskeag_sast::SastAnalyzer;

let expr = parse("10 / 0").unwrap();
let mut analyzer = SastAnalyzer::new();
let json = analyzer.analyze_json(&expr, &[]).unwrap();
println!("{}", json);
```

### Advanced Usage

```rust
use amoskeag_parser::parse;
use amoskeag_sast::{
    SastAnalyzer, ErrorDetector, AlgebraicAnalyzer,
    RangeAnalyzer, DataFlowAnalyzer
};
use std::collections::HashMap;

let expr = parse("let x = 10 in x / y").unwrap();

// Individual analyzers
let mut error_detector = ErrorDetector::new();
let mut range_analyzer = RangeAnalyzer::new();
let mut algebraic_analyzer = AlgebraicAnalyzer::new();
let mut data_flow_analyzer = DataFlowAnalyzer::new();

// Perform analyses
let ranges = range_analyzer.analyze(&expr, &HashMap::new());
let errors = error_detector.detect(&expr, &[], &ranges);
let vulnerabilities = algebraic_analyzer.analyze(&expr, &[], &ranges);
let data_flow = data_flow_analyzer.analyze(&expr);

// Use results
println!("Found {} errors", errors.len());
println!("Found {} vulnerabilities", vulnerabilities.len());
println!("Data flow has {} nodes", data_flow.nodes.len());
```

## Error Severity Levels

- **Critical**: Errors that will definitely cause runtime failures
- **Warning**: Potential issues that may cause errors under certain conditions
- **Info**: Code quality issues and best practice violations

## Error Categories

- `DivisionByZero`: Division or modulo by zero
- `TypeMismatch`: Type errors and incompatible operations
- `UndefinedVariable`: Use of undefined variables
- `UnreachableCode`: Code that cannot be executed
- `UnusedVariable`: Variables defined but never used
- `ArrayOutOfBounds`: Invalid array access
- `IntegerOverflow`: Arithmetic overflow
- `NullDereference`: Potential null access
- `Complexity`: Performance or complexity warnings
- `Security`: Security-related issues

## Integration

The SAST crate integrates seamlessly with other Amoskeag crates:

- Uses `amoskeag-parser` for AST representation
- Can be integrated into `amoskeag` compilation pipeline
- Provides analysis before backend execution
- Supports all Amoskeag language constructs

## Example Output

```json
{
  "errors": [
    {
      "severity": "Critical",
      "message": "Division by zero",
      "category": "DivisionByZero",
      "location": "division by zero",
      "suggestion": "Replace divisor with a non-zero value or add a check"
    }
  ],
  "vulnerable_inputs": [
    {
      "error_type": "DivisionByZero",
      "description": "Division by zero when x = 0",
      "example_input": {
        "x": 0.0
      },
      "location": "division by x",
      "severity": "Critical"
    }
  ],
  "statistics": {
    "total_expressions": 3,
    "critical_errors": 1,
    "warnings": 0,
    "info_issues": 0,
    "vulnerable_patterns": 1
  }
}
```

## Future Enhancements

- SMT solver integration for more powerful constraint solving
- Interprocedural analysis for function calls
- Taint analysis for tracking untrusted input
- More sophisticated symbolic execution
- Integration with CI/CD pipelines
- SARIF output format support

## License

MIT OR Apache-2.0
