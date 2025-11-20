# Spreadsheet Formulas

Excel-like formula evaluation engine.

## Demonstrates
- Spreadsheet cell references
- CHOOSE function (Excel-compatible)
- Formula composition
- Number rounding

## Expected Data Context
```json
{
  "B1": 2,
  "B2": [0.10, 0.15, 0.20],
  "B3": 1000
}
```

## Expected Output
```
1150.00
```

## Calculation Breakdown
1. `B1 = 2` (scenario index: 1=worst, 2=base, 3=best)
2. `B2 = [0.10, 0.15, 0.20]` (growth rate scenarios)
3. `choose(2, [0.10, 0.15, 0.20])` → `0.15` (selects 2nd item, 1-based)
4. `1000 * (1 + 0.15)` → `1150`
5. `round(1150, 2)` → `1150.00`

## Excel Function Equivalents
| Amoskeag | Excel |
|----------|-------|
| `choose(index, array)` | `CHOOSE(index, val1, val2, ...)` |
| `sum(array)` | `SUM(range)` |
| `avg(array)` | `AVERAGE(range)` |
| `round(num, digits)` | `ROUND(num, digits)` |

## Advantages over Traditional Spreadsheets
- Formal execution model (no regex-based parsing)
- Pure functions (deterministic, cacheable)
- Composable (functions can be combined)
- Type-safe evaluation
