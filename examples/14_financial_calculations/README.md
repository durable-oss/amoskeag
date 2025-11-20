# Financial Calculations

Loan payment calculations using financial functions.

## Demonstrates
- PMT function (Excel/financial calculator equivalent)
- Multi-step financial calculations
- Intermediate variable naming
- Result formatting

## Expected Output
```json
{
  "loan_amount": 250000,
  "monthly_payment": 1266.71,
  "total_paid": 456015.60,
  "total_interest": 206015.60
}
```

## Financial Functions Available
- `pmt(rate, nper, pv)` - Calculate payment amount
  - `rate`: Interest rate per period
  - `nper`: Number of periods
  - `pv`: Present value (loan amount)
- `npv(rate, values)` - Net present value
- `round(num, digits)` - Round to decimal places

## Use Cases
- Mortgage calculators
- Loan amortization
- Investment projections
- Financial modeling
- Retirement planning

## Advantages
- Purely functional (deterministic, cacheable)
- Thread-safe for batch processing
- No floating-point comparison issues
- Composable with other operations
