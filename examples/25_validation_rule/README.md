# Validation Rule

Form validation engine with detailed error reporting.

## Demonstrates
- Data validation patterns
- Multiple validation checks
- Symbol outputs for validation states
- Boolean logic composition
- Array membership checking

## Expected Output
```
:valid
```

## Valid Symbols
```
[
  ":valid",
  ":invalid_email",
  ":invalid_password",
  ":invalid_age",
  ":terms_not_accepted",
  ":unsupported_country",
  ":invalid_unknown"
]
```

## Validation Rules

### Email
- Length > 3 characters
- Contains "@" symbol
- (Real regex validation would be in host)

### Password
- Minimum 8 characters

### Age
- Between 18 and 120

### Terms
- Must be explicitly accepted (true)

### Country
- Must be in supported list: US, CA, UK, AU

## Pattern: Validation First, Then Error
```
# 1. Check each field individually
let email_valid = ...
let password_valid = ...

# 2. Check if all valid
let all_valid = email_valid and password_valid and ...

# 3. Return appropriate symbol
if all_valid
  :valid
else if not email_valid
  :invalid_email
...
```

## Advantages Over String Errors

### With Strings (Fragile)
```
if not email_valid
  "invalid_email"  # Typo: "invalld_email" - runtime bug!
```

### With Symbols (Compile-Safe)
```
if not email_valid
  :invalid_email  # Typo: :invalld_email - compile error!
```

The host knows at compile-time all possible validation outcomes!

## Use Cases
- Form validation
- API input validation
- Business rule validation
- Data quality checks
- Pre-flight checks
