# Complex Business Rule

Auto insurance underwriting with environmental data and complex conditions.

## Demonstrates
- Multi-source data evaluation (`applicant` and `env`)
- Computed intermediate variables for clarity
- Logical operators (`and`, `or`)
- Array contains operation
- Complex business logic

## Expected Data Context
```json
{
  "applicant": {
    "age": 30,
    "state": "CA",
    "claims_last_3_years": 1,
    "vehicle": {
      "value": 75000,
      "type": "SPORT"
    }
  },
  "env": {
    "underwriting_limits": {
      "max_vehicle_value": 100000,
      "restricted_states": ["FL", "LA"]
    }
  }
}
```

## Expected Output
```
:manual_review
```
(High-value sports car requires manual review)

## Valid Symbols
```
[":approve", ":deny", ":manual_review"]
```

## Business Rules
1. Restricted state → Deny
2. Vehicle value exceeds limit → Deny
3. Young driver (<25) with sports car → Deny
4. High-value sports car (>$70k) → Manual review
5. More than 2 claims in 3 years → Manual review
6. Otherwise → Approve

## Key Pattern
Breaking complex conditions into named intermediate variables (`is_young_sports_driver`) makes the logic more readable and maintainable.
