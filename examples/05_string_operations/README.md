# String Operations

String manipulation using the pipe operator and standard library functions.

## Demonstrates
- Pipe operator (`|`) for function chaining
- String functions: `strip`, `downcase`, `capitalize`
- Left-to-right data flow

## Expected Output
```
"Hello, amoskeag language!"
```

## Pipeline Breakdown
1. `"  Hello, Amoskeag Language!  "` - original string
2. `| strip` - removes leading/trailing whitespace → `"Hello, Amoskeag Language!"`
3. `| downcase` - converts to lowercase → `"hello, amoskeag language!"`
4. `| capitalize` - capitalizes first word → `"Hello, amoskeag language!"`

## Available String Functions
- `upcase` - convert to uppercase
- `downcase` - convert to lowercase
- `capitalize` - capitalize first word
- `strip` - remove leading/trailing whitespace
- `lstrip` - remove leading whitespace
- `rstrip` - remove trailing whitespace
- `truncate(len)` - truncate to length
- `replace(find, rep)` - replace all occurrences
