# Comparison Operators

Testing all comparison operators in Amoskeag.

## Demonstrates
- Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Number comparisons
- String comparisons
- Boolean results

## Expected Output
```json
{
  "equal": true,
  "not_equal": true,
  "less_than": true,
  "greater_than": true,
  "less_or_equal": true,
  "greater_or_equal": true,
  "string_equal": true,
  "string_not_equal": true
}
```

## Comparison Operators
| Operator | Meaning |
|----------|---------|
| `==` | Equal to |
| `!=` | Not equal to |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less than or equal |
| `>=` | Greater than or equal |

## Type Support
- **Numbers**: All comparisons supported
- **Strings**: Lexicographic comparison
- **Booleans**: `==` and `!=` only
- **Nil**: `==` and `!=` only
- **Arrays/Dicts**: `==` for structural equality

## Operator Precedence
From lowest to highest:
1. `or`
2. `and`
3. `not`
4. `==`, `!=`, `<`, `>`, `<=`, `>=`
5. `+`, `-`
6. `*`, `/`, `%`
7. `|` (pipe)
