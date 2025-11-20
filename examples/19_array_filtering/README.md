# Array Filtering and Transformation

Working with collections and aggregate operations.

## Demonstrates
- Arrays of dictionaries
- Aggregate functions: `sum`, `avg`, `sort`
- Collection size with `size`
- Array element access with `first`, `last`

## Expected Output
```json
{
  "total_value": 1675,
  "average_price": 418.75,
  "cheapest": 25,
  "most_expensive": 1200,
  "product_count": 4
}
```

## Collection Functions

### Aggregate
- `sum(array)` - Sum all numbers
- `avg(array)` - Average of numbers
- `size(array)` - Count of elements

### Access
- `first(array)` - First element (or nil)
- `last(array)` - Last element (or nil)

### Transform
- `sort(array)` - Sort ascending
- `contains(array, val)` - Check membership

### Dictionary Operations
- `keys(dict)` - Get all keys as array
- `values(dict)` - Get all values as array

## Advanced Pattern
For filtering (future extension with lambdas):
```
products | filter(p => p.category == "electronics")
```

Currently, host can provide custom filter functions in context:
```
products | filter_electronics
```
