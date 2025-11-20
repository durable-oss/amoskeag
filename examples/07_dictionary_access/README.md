# Dictionary Access

Using dot notation to navigate dictionaries (key-value maps).

## Demonstrates
- Dictionary literals with `{key: value}` syntax
- Nested dictionaries
- Dot notation for key access (`.`)
- Safe navigation (returns `nil` for missing keys, doesn't crash)

## Expected Output
```
"Bob Smith (bob@example.com) lives in Boston"
```

## Key Concepts
- The `.` operator is for dictionary navigation, not method calls
- `user.name` is equivalent to `user["name"]` in other languages
- Accessing a missing key returns `nil` (safe navigation)
- Can chain dots for nested access: `user.profile.city`
