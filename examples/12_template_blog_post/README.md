# Template: Blog Post

Rendering blog post metadata with formatting and badges.

## Demonstrates
- Complex template logic
- String pipe operations
- Nested data access
- Multiple computed display values
- Status badges

## Expected Data Context
```json
{
  "post": {
    "title": "Introduction to Amoskeag Programming",
    "status": "published",
    "author": {
      "name": "Jane Developer",
      "is_verified": true
    }
  }
}
```

## Expected Output
```
"[LIVE] INTRODUCTION TO AMOSKEAG PROGRAMMING by Jane Developer ✓"
```

## Pattern: Computed Display Variables
Instead of inline complex expressions:
```
# Bad (hard to read)
(if p.status == "published" then "[LIVE]" else "[DRAFT]" end) + " " + (p.title | upcase)

# Good (readable)
let status_badge = if p.status == "published" then "[LIVE]" else "[DRAFT]" end
in let title_display = p.title | upcase
in status_badge + " " + title_display
```
