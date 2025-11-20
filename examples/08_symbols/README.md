# Symbols

Using symbols as statically-validated enumeration values.

## Demonstrates
- Symbol literals (`:symbol_name`)
- Static symbol validation at compile-time
- Symbols as return values

## Expected Output
```
:approved
```

## Key Concepts
- Symbols are prefixed with `:` (colon)
- Symbols must be declared in the `symbols` array at compile-time
- Any symbol not in the array will cause a compile-time error
- Symbols are ideal for state machines and decision outputs
- More efficient than strings for enumerations

## Valid Symbols for this Example
The host must provide these symbols at compile-time:
```
[":approved", ":waiting", ":denied"]
```

If the code used `:rejected` instead, it would fail compilation unless `:rejected` was in the symbols array.

## Symbol Naming
- Can use bare identifiers: `:continue`
- Can use quoted strings for special chars: `:"test.something"`
