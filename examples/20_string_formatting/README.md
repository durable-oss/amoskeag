# String Formatting

Advanced string manipulation and formatting.

## Demonstrates
- String transformation chains
- `capitalize`, `upcase`, `downcase`
- `strip`, `truncate`, `replace`
- `join` for array-to-string
- Combining multiple string operations

## Expected Output
```json
{
  "display_name": "John Doe",
  "email": "john.doe@company.com",
  "bio_clean": "Software engineer with 10 years experience.",
  "bio_short": "Software engineer with 10...",
  "tags_joined": "rust, typescript, python",
  "tag_count": 3
}
```

## String Functions Reference

### Case Conversion
- `upcase(str)` - "hello" → "HELLO"
- `downcase(str)` - "HELLO" → "hello"
- `capitalize(str)` - "hello world" → "Hello world"

### Whitespace
- `strip(str)` - Remove leading/trailing whitespace
- `lstrip(str)` - Remove leading whitespace
- `rstrip(str)` - Remove trailing whitespace

### Modification
- `truncate(str, len)` - Cut to max length, add "..."
- `replace(str, find, rep)` - Replace all occurrences
- `split(str, sep)` - Split into array

### Array ↔ String
- `join(array, sep)` - Join array elements with separator
- `split(str, sep)` - Split string into array

## Pattern: Email Generation
```
name | downcase | replace(" ", ".") + "@domain.com"
```

This is safe because it's pure - no actual email is sent!
