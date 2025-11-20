#!/usr/bin/env ruby
# frozen_string_literal: true

require 'amoskeag-rb'

puts "Amoskeag Ruby Gem - Basic Usage Examples"
puts "=" * 50

# Example 1: Basic Arithmetic
puts "\n1. Basic Arithmetic"
result = Amoskeag.eval_expression("2 + 2", {})
puts "  2 + 2 = #{result}"

result = Amoskeag.eval_expression("(100 + 50) * 1.08", {})
puts "  (100 + 50) * 1.08 = #{result}"

# Example 2: Using Variables
puts "\n2. Using Variables"
data = { "user" => { "age" => 25, "name" => "Alice" } }
result = Amoskeag.eval_expression("user.age * 2", data)
puts "  user.age * 2 = #{result} (where user.age = 25)"

result = Amoskeag.eval_expression('"Hello, " + user.name + "!"', data)
puts "  Result: #{result}"

# Example 3: String Operations with Pipe
puts "\n3. String Operations"
result = Amoskeag.eval_expression('"  hello world  " | strip | upcase', {})
puts '  "  hello world  " | strip | upcase = "' + result + '"'

result = Amoskeag.eval_expression('"hello,world,ruby" | split(",") | size', {})
puts '  "hello,world,ruby" | split(",") | size = ' + result.to_s

# Example 4: Arrays and Collections
puts "\n4. Arrays and Collections"
result = Amoskeag.eval_expression("[1, 2, 3, 4, 5] | sum", {})
puts "  [1, 2, 3, 4, 5] | sum = #{result}"

result = Amoskeag.eval_expression("[1, 2, 3, 4, 5] | avg", {})
puts "  [1, 2, 3, 4, 5] | avg = #{result}"

# Example 5: Conditional Logic
puts "\n5. Conditional Logic"
data = { "score" => 95 }
result = Amoskeag.eval_expression('if score >= 90 "A" else "B" end', data)
puts "  Grade for score 95: #{result}"

# Example 6: Business Rules with Symbols
puts "\n6. Business Rules with Symbols"
rules = 'if user.age >= 18 :adult else :minor end'
symbols = [:adult, :minor]

program = Amoskeag.compile(rules, symbols)

result = Amoskeag.evaluate(program, { "user" => { "age" => 25 } })
puts "  User age 25: #{result}"

result = Amoskeag.evaluate(program, { "user" => { "age" => 15 } })
puts "  User age 15: #{result}"

# Example 7: Financial Calculations
puts "\n7. Financial Calculations"
formula = "pmt(rate / 12, years * 12, -principal) | round(2)"
program = Amoskeag.compile(formula, [])

data = {
  "rate" => 0.045,      # 4.5% annual rate
  "years" => 30,        # 30-year loan
  "principal" => 250000 # $250,000 loan
}

result = Amoskeag.evaluate(program, data)
puts "  Monthly payment for $250k loan at 4.5% for 30 years: $#{result}"

# Example 8: Template Rendering
puts "\n8. Template Rendering"
template = <<~TEMPLATE
  "Welcome, " + user.name + "! " +
  if user.premium
    "You have premium access."
  else
    "Upgrade to premium today!"
  end
TEMPLATE

program = Amoskeag.compile(template, [])

data = { "user" => { "name" => "Alice", "premium" => true } }
result = Amoskeag.evaluate(program, data)
puts "  Premium user: #{result}"

data = { "user" => { "name" => "Bob", "premium" => false } }
result = Amoskeag.evaluate(program, data)
puts "  Regular user: #{result}"

# Example 9: Safe Navigation
puts "\n9. Safe Navigation (nil-safe)"
result = Amoskeag.eval_expression("user.address.street", { "user" => {} })
puts "  user.address.street (missing): #{result.inspect}"

result = Amoskeag.eval_expression("user.address.street", {
  "user" => {
    "address" => {
      "street" => "123 Main St"
    }
  }
})
puts "  user.address.street (present): #{result}"

# Example 10: Let Bindings
puts "\n10. Let Bindings"
code = <<~CODE
  let base = 100
  in let tax = base * 0.08
  in let total = base + tax
  in total
CODE

result = Amoskeag.eval_expression(code, {})
puts "  Base: 100, Tax: 8%, Total: #{result}"

puts "\n" + "=" * 50
puts "All examples completed successfully!"
