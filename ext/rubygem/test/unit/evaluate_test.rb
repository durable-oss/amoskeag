# frozen_string_literal: true

require "test_helper"

class EvaluateTest < Minitest::Test
  include TestHelper

  def test_evaluate_arithmetic
    program = Amoskeag.compile("2 + 2", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 4.0, result
  end

  def test_evaluate_with_variables
    program = Amoskeag.compile("user.age", [])
    result = Amoskeag.evaluate(program, { "user" => { "age" => 25 } })
    assert_equal 25.0, result
  end

  def test_evaluate_with_nested_variables
    program = Amoskeag.compile("user.address.city", [])
    data = {
      "user" => {
        "address" => {
          "city" => "Boston"
        }
      }
    }
    result = Amoskeag.evaluate(program, data)
    assert_equal "Boston", result
  end

  def test_evaluate_string_operations
    program = Amoskeag.compile('"hello" | upcase', [])
    result = Amoskeag.evaluate(program, {})
    assert_equal "HELLO", result
  end

  def test_evaluate_array_sum
    program = Amoskeag.compile("[1, 2, 3, 4, 5] | sum", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 15.0, result
  end

  def test_evaluate_array_avg
    program = Amoskeag.compile("[1, 2, 3, 4, 5] | avg", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 3.0, result
  end

  def test_evaluate_conditional_true
    program = Amoskeag.compile('if score >= 90 "A" else "B" end', [])
    result = Amoskeag.evaluate(program, { "score" => 95 })
    assert_equal "A", result
  end

  def test_evaluate_conditional_false
    program = Amoskeag.compile('if score >= 90 "A" else "B" end', [])
    result = Amoskeag.evaluate(program, { "score" => 85 })
    assert_equal "B", result
  end

  def test_evaluate_with_symbols
    program = Amoskeag.compile("if age >= 18 :adult else :minor end", [:adult, :minor])
    result = Amoskeag.evaluate(program, { "age" => 25 })
    assert_equal :adult, result

    result = Amoskeag.evaluate(program, { "age" => 15 })
    assert_equal :minor, result
  end

  def test_evaluate_let_bindings
    code = <<~CODE
      let base = 100
      in let tax = base * 0.08
      in let total = base + tax
      in total
    CODE

    program = Amoskeag.compile(code, [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 108.0, result
  end

  def test_evaluate_boolean_true
    program = Amoskeag.compile("true", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal true, result
  end

  def test_evaluate_boolean_false
    program = Amoskeag.compile("false", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal false, result
  end

  def test_evaluate_nil
    program = Amoskeag.compile("nil", [])
    result = Amoskeag.evaluate(program, {})
    assert_nil result
  end

  def test_evaluate_number
    program = Amoskeag.compile("42.5", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 42.5, result
  end

  def test_evaluate_string
    program = Amoskeag.compile('"hello world"', [])
    result = Amoskeag.evaluate(program, {})
    assert_equal "hello world", result
  end

  def test_evaluate_array
    program = Amoskeag.compile("[1, 2, 3]", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal [1.0, 2.0, 3.0], result
  end

  def test_evaluate_safe_navigation
    program = Amoskeag.compile("user.address.street", [])
    result = Amoskeag.evaluate(program, { "user" => {} })
    # Should return nil for missing nested keys
    assert_nil result
  end

  # Error cases

  def test_evaluate_rejects_nil_program
    assert_argument_error(/program must be an Amoskeag::Program/) do
      Amoskeag.evaluate(nil, {})
    end
  end

  def test_evaluate_rejects_non_program
    assert_argument_error(/program must be an Amoskeag::Program/) do
      Amoskeag.evaluate("not a program", {})
    end
  end

  def test_evaluate_rejects_nil_data
    program = Amoskeag.compile("2 + 2", [])
    assert_argument_error(/data must be a Hash/) do
      Amoskeag.evaluate(program, nil)
    end
  end

  def test_evaluate_rejects_non_hash_data
    program = Amoskeag.compile("2 + 2", [])
    assert_argument_error(/data must be a Hash/) do
      Amoskeag.evaluate(program, "not a hash")
    end
  end

  def test_evaluate_rejects_invalid_data_types
    program = Amoskeag.compile("x", [])

    # Test with invalid object type
    invalid_data = { "x" => Object.new }
    assert_argument_error(/contains invalid type/) do
      Amoskeag.evaluate(program, invalid_data)
    end
  end

  def test_evaluate_rejects_too_large_data
    program = Amoskeag.compile("x", [])

    # Create data with more than 100,000 keys
    large_data = {}
    100_001.times { |i| large_data["key#{i}"] = i }

    assert_argument_error(/too large/) do
      Amoskeag.evaluate(program, large_data)
    end
  end

  def test_evaluate_accepts_symbol_data_keys
    program = Amoskeag.compile("user.age", [])
    # Ruby hashes can have symbol keys, should be converted to strings
    result = Amoskeag.evaluate(program, { user: { age: 25 } })
    assert_equal 25.0, result
  end

  def test_evaluate_deeply_nested_data
    program = Amoskeag.compile("a.b.c.d.e", [])

    # Create deeply nested structure (within limits)
    data = { "a" => { "b" => { "c" => { "d" => { "e" => 42 } } } } }
    result = Amoskeag.evaluate(program, data)
    assert_equal 42.0, result
  end

  def test_evaluate_rejects_too_deeply_nested_data
    program = Amoskeag.compile("x", [])

    # Create a structure exceeding MAX_DEPTH (100)
    deep_data = { "x" => "value" }
    current = deep_data
    101.times do |i|
      current["nested"] = { "level#{i}" => "value" }
      current = current["nested"]
    end

    assert_argument_error(/too deeply nested/) do
      Amoskeag.evaluate(program, deep_data)
    end
  end
end
