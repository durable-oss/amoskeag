# Discount Calculator

E-commerce pricing engine with tiered discounts and loyalty rewards.

## Demonstrates
- Multi-tier discount logic
- Conditional pricing
- Customer segmentation
- Complex business rules
- Real-world e-commerce logic

## Expected Output
```json
{
  "pricing": {
    "subtotal": 154.97,
    "member_discount": 15.50,
    "volume_discount": 4.18,
    "loyalty_discount": 10.00,
    "total_discount": 29.68,
    "final_price": 125.29
  },
  "savings": {
    "amount": 29.68,
    "percent": 19.2
  }
}
```

## Discount Tiers

### 1. Member Discount (10%)
- Applied first to subtotal
- Only for loyalty members

### 2. Volume Discount
- Order > $150: 5% off
- Order > $100: 3% off
- Applied to member-discounted subtotal

### 3. Loyalty Points
- 500+ points: $10 off
- 200+ points: $5 off
- Fixed amount, not percentage

## Calculation Flow
```
Subtotal: $154.97
- Member discount (10%): -$15.50
= After member: $139.47
- Volume discount (3% of $139.47): -$4.18
- Loyalty bonus: -$10.00
= Final: $125.29

Total savings: $29.68 (19.2%)
```

## Use Cases
- E-commerce checkout
- Quote generation
- Promotional campaigns
- Dynamic pricing
- A/B testing pricing rules

## Advantages
- Business rules separated from application code
- Easy to test different discount scenarios
- Can compile-check all pricing logic
- Thread-safe for high-volume calculations
- Deterministic (no surprises!)
