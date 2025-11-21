# Changelog

All notable changes to the amoskeag-rb gem will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-20

### Added
- Initial release of amoskeag-rb gem
- Native Ruby bindings to Amoskeag DSL
- FFI layer using statically compiled Rust library
- Core API methods:
  - `Amoskeag.compile(source, symbols)` - Compile programs with symbol validation
  - `Amoskeag.evaluate(program, data)` - Evaluate compiled programs
  - `Amoskeag.eval(source, data, symbols)` - Compile and evaluate in one step
- Full support for all Amoskeag data types:
  - Numbers, Strings, Booleans, Nil
  - Arrays and Dictionaries
  - Symbols with compile-time validation
- Ruby-friendly API with automatic type conversion
- Thread-safe compiled programs
- Comprehensive error handling with CompileError and EvalError
- Complete documentation and examples
- JSON-based marshalling between Ruby and native code

### Features
- Business Rules Engine capabilities
- Template Engine functionality
- Spreadsheet Formula support
- 60+ standard library functions
- Excel-compatible financial functions
- Safe navigation (nil-safe property access)
- Pure functional evaluation (no side effects)
- Security-first design (immune to code injection)

[0.1.0]: https://github.com/durable-oss/amoskeag/releases/tag/v0.1.0
