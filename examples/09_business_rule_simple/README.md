# Simple Business Rule

A credit card approval decision engine.

## Demonstrates
- Business rule logic with symbols
- Multiple conditional branches
- Data-driven decisions
- Symbol outputs for state machines

## Expected Output
```
:approve
```

## Valid Symbols
```
[":deny", ":instant_approve", ":approve"]
```

## Business Logic
1. Age < 18 → Deny (too young)
2. Income < $30,000 → Deny (insufficient income)
3. Credit score < 650 → Deny (poor credit)
4. Credit score ≥ 750 → Instant approve (excellent credit)
5. Otherwise → Approve (standard approval)

## Use Case
This pattern is ideal for:
- Loan approval systems
- Insurance underwriting
- Risk assessment
- Workflow state transitions
- Any logic requiring compile-time validation of outcomes
