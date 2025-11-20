# Template: Greeting

Dynamic user greeting for a web application.

## Demonstrates
- Template-style data access
- Nil checking
- Conditional string generation
- User context data

## Expected Data Context
```json
{
  "user": {
    "name": "Alice Johnson",
    "is_admin": false,
    "last_login": "2025-01-15"
  }
}
```

## Expected Output
```
"Welcome back, Alice Johnson!"
```

## Use in ERB-style Templates
In an HTML template:
```html
<div class="greeting">
  <%= [amoskeag expression here] %>
</div>
```

## Advantages over Traditional Templates
- **vs. ERB**: No risk of SSTI/RCE attacks
- **vs. Mustache**: Can express complex logic
- **vs. Liquid**: More composable, full functional programming
