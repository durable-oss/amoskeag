# Loan Calculator

Complete mortgage/loan payment calculator with detailed analysis.

## Demonstrates
- PMT function for loan calculations
- Complex financial logic
- Structured result output
- Multi-level calculations
- Real-world application

## Expected Output
```json
{
  "loan_summary": {
    "principal": 350000,
    "rate": 3.75,
    "term_years": 30
  },
  "payment_details": {
    "monthly_payment": 1621.29,
    "total_payments": 360,
    "total_amount_paid": 583664.40
  },
  "interest_analysis": {
    "total_interest": 233664.40,
    "interest_as_percent_of_principal": 66.8
  }
}
```

## Calculation Details

### Monthly Payment Formula
The PMT function calculates:
```
P = L[c(1 + c)^n]/[(1 + c)^n - 1]

Where:
  P = monthly payment
  L = loan amount
  c = monthly interest rate (annual rate / 12)
  n = number of payments (years * 12)
```

### Example
- Loan: $350,000
- Rate: 3.75% annual (0.3125% monthly)
- Term: 30 years (360 payments)
- Payment: $1,621.29/month
- Total paid: $583,664.40
- Interest: $233,664.40 (66.8% of principal!)

## Use Cases
- Mortgage calculators
- Auto loan tools
- Personal finance planning
- Real estate analysis
- Loan comparison tools

## Thread-Safe Batch Processing
Because Amoskeag is pure, you can calculate thousands of loan scenarios in parallel:
```
loans = [loan1, loan2, ..., loan10000]
results = loans.parallel_map(loan => evaluate_amoskeag(script, loan))
```

No locks, no race conditions, perfect parallelism.
