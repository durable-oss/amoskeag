# Number Rounding and Math

Mathematical operations with rounding and precision control.

## Demonstrates
- Arithmetic operations
- `round`, `ceil`, `floor`, `abs`
- Financial calculations
- Precision control

## Expected Output
```json
{
  "subtotal": 139.93,
  "tax": 11.54,
  "total": 151.47,
  "discount": 22.72,
  "final_total": 128.75,
  "final_total_ceil": 129,
  "final_total_floor": 128,
  "absolute_discount": 22.72
}
```

## Math Functions

### Rounding
- `round(num, digits)` - Round to N decimal places
  - `round(3.14159, 2)` → `3.14`
- `ceil(num)` - Round up to nearest integer
  - `ceil(3.1)` → `4`
- `floor(num)` - Round down to nearest integer
  - `floor(3.9)` → `3`
- `abs(num)` - Absolute value
  - `abs(-5)` → `5`

### Arithmetic (operators)
- `+` - Addition
- `-` - Subtraction
- `*` - Multiplication
- `/` - Division
- `%` - Modulo (remainder)

### Arithmetic (functions, for pipe)
- `plus(a, b)` - For piping: `x | plus(5)`
- `minus(a, b)` - For piping: `x | minus(3)`
- `times(a, b)` - For piping: `x | times(2)`
- `divided_by(a, b)` - For piping: `x | divided_by(4)`
- `modulo(a, b)` - For piping: `x | modulo(10)`

## Financial Precision
Always use `round(num, 2)` for money to avoid floating-point issues:
```
let price = 10.00
in let tax = price * 0.0825 | round(2)  # Exactly 0.83, not 0.825000001
```
