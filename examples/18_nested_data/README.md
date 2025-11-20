# Nested Data Access

Deep dictionary navigation and safe access patterns.

## Demonstrates
- Multi-level dictionary access
- Nested data structures
- Safe navigation (nil-safe)
- Combining dictionary and array access
- Complex data extraction

## Expected Output
```
"TechCorp - Boston, MA (42.36, -71.06) - Contact: alice@techcorp.com"
```

## Safe Navigation
Amoskeag uses **nil-safe** navigation:

```
company.address.city
```

If `company` is nil → returns nil (doesn't crash)
If `address` is missing → returns nil (doesn't crash)
If `city` is missing → returns nil (doesn't crash)

This is critical for template engines where data might be incomplete.

## Navigation Patterns

### Dictionary Access
```
user.profile.settings.theme  ≡  user["profile"]["settings"]["theme"]
```

### Array + Dictionary
```
users | first → get first element
users | first → then access .name
```

### Combining Operations
```
company.employees          # Get array
  | first                  # Get first element
  | .contact               # Access nested dict
  | .email                 # Get email string
```

## Best Practice
Extract complex nested access into named variables for readability:
```
let main_contact = company.employees | first
in let email = main_contact.contact.email
in ...
```
