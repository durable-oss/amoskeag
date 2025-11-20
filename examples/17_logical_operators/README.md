# Logical Operators

Working with boolean logic: `and`, `or`, `not`.

## Demonstrates
- Logical operators: `and`, `or`, `not`
- Combining multiple conditions
- Grouping with parentheses
- Complex boolean expressions

## Expected Output
```json
{
  "can_get_basic_loan": true,
  "needs_review": true,
  "instant_approval": false,
  "complex_condition": true
}
```

## Logical Operators
| Operator | Meaning | Example |
|----------|---------|---------|
| `and` | Logical AND | `a and b` - true if both are true |
| `or` | Logical OR | `a or b` - true if either is true |
| `not` | Logical NOT | `not a` - inverts boolean value |

## Operator Precedence
1. `not` (highest)
2. `and`
3. `or` (lowest)

Example:
```
not a or b and c
≡ (not a) or (b and c)
```

## Short-Circuit Evaluation
- `and`: If left side is false, right side is not evaluated
- `or`: If left side is true, right side is not evaluated

## Grouping
Use parentheses for clarity:
```
(age >= 25 or income > 60000) and (not has_debt or credit_score >= 700)
```
