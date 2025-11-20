# Date Operations

Working with dates provided by the host context.

## Demonstrates
- `date_now()` function
- `date_format()` for formatting
- Host-provided date/time
- Deterministic time for pure functions

## Expected Output
```json
{
  "created_at": "2025-01-18",
  "created_at_full": "2025-01-18 14:30:00",
  "display": "Account created on 2025-01-18"
}
```

## Date Functions

### Core Functions
- `date_now()` - Get current timestamp (from host)
- `date_format(date, format)` - Format a date

### Format Strings
Common format patterns:
- `"YYYY-MM-DD"` → `"2025-01-18"`
- `"YYYY-MM-DD HH:mm:ss"` → `"2025-01-18 14:30:00"`
- `"MMM DD, YYYY"` → `"Jan 18, 2025"`

## Pure Functional Dates

Amoskeag has no I/O, so how does `date_now()` work?

The host provides the "current time" in the execution context:
```
metadata = {
  'execution_time': '2025-01-18T14:30:00Z'
}
```

The `date_now()` function returns this value, making it:
- **Deterministic**: Same input → same output
- **Testable**: Control time in tests
- **Cacheable**: Can memoize results
- **Thread-safe**: No global state

This is the same pattern used by DMN/FEEL and other pure evaluation languages.

## Use Cases
- Timestamp generation
- Date formatting for templates
- Business rules with date conditions
- Audit trails
