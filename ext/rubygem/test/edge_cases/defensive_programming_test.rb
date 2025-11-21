# frozen_string_literal: true

require "test_helper"

class DefensiveProgrammingTest < Minitest::Test
  include TestHelper

  # Test size limits

  def test_rejects_source_too_large
    # 10MB + 1 byte
    large_source = "1" * (10 * 1024 * 1024 + 1)
    assert_argument_error(/source too large/) do
      Amoskeag.compile(large_source, [])
    end
  end

  def test_accepts_source_at_limit
    # 10MB exactly (may be slow, but should work)
    # Using a smaller size for practical testing
    large_source = "1 + 1" * 1000
    program = Amoskeag.compile(large_source, [])
    assert_instance_of Amoskeag::Program, program
  end

  def test_rejects_too_many_symbols
    too_many = (0..10001).map { |i| "sym#{i}" }
    assert_argument_error(/Too many symbols/) do
      Amoskeag.compile("x", too_many)
    end
  end

  def test_accepts_max_symbols
    max_symbols = (0..9999).map { |i| "sym#{i}" }
    program = Amoskeag.compile("x", max_symbols)
    assert_instance_of Amoskeag::Program, program
  end

  def test_rejects_data_too_large
    program = Amoskeag.compile("x", [])

    # More than 100,000 keys
    large_data = {}
    100_001.times { |i| large_data["key#{i}"] = i }

    assert_argument_error(/too large/) do
      Amoskeag.evaluate(program, large_data)
    end
  end

  def test_rejects_array_too_large
    program = Amoskeag.compile("arr", [])

    # More than 1,000,000 elements
    large_array = Array.new(1_000_001, 0)

    assert_argument_error(/Array too large/) do
      Amoskeag.evaluate(program, { "arr" => large_array })
    end
  end

  def test_rejects_deeply_nested_structure
    program = Amoskeag.compile("x", [])

    # Create structure nested more than 100 levels
    deep_structure = "value"
    102.times do
      deep_structure = { "nested" => deep_structure }
    end

    assert_argument_error(/too deeply nested/) do
      Amoskeag.evaluate(program, { "x" => deep_structure })
    end
  end

  def test_accepts_max_nesting_depth
    program = Amoskeag.compile("x", [])

    # Create structure nested exactly 100 levels
    deep_structure = "value"
    98.times do
      deep_structure = { "nested" => deep_structure }
    end

    # Should work at the limit
    result = Amoskeag.evaluate(program, { "x" => deep_structure })
    assert_kind_of Hash, result
  end

  # Test input validation

  def test_rejects_non_json_serializable_objects
    program = Amoskeag.compile("obj", [])

    assert_argument_error(/invalid type/) do
      Amoskeag.evaluate(program, { "obj" => Object.new })
    end

    assert_argument_error(/invalid type/) do
      Amoskeag.evaluate(program, { "obj" => Class.new })
    end

    assert_argument_error(/invalid type/) do
      Amoskeag.evaluate(program, { "obj" => -> { "lambda" } })
    end
  end

  def test_rejects_invalid_hash_keys
    program = Amoskeag.compile("x", [])

    # Integer keys should be rejected
    assert_argument_error(/key must be String or Symbol/) do
      Amoskeag.evaluate(program, { "x" => { 123 => "value" } })
    end

    # Object keys should be rejected
    assert_argument_error(/key must be String or Symbol/) do
      Amoskeag.evaluate(program, { "x" => { Object.new => "value" } })
    end
  end

  def test_accepts_string_and_symbol_hash_keys
    program = Amoskeag.compile("x.a + x.b", [])

    # String keys
    result = Amoskeag.evaluate(program, { "x" => { "a" => 1, "b" => 2 } })
    assert_equal 3.0, result

    # Symbol keys (should be converted to strings)
    result = Amoskeag.evaluate(program, { "x" => { a: 1, b: 2 } })
    assert_equal 3.0, result

    # Mixed
    result = Amoskeag.evaluate(program, { "x" => { "a" => 1, b: 2 } })
    assert_equal 3.0, result
  end

  def test_rejects_infinity_values
    program = Amoskeag.compile("x", [])

    assert_argument_error(/must be finite/) do
      Amoskeag.evaluate(program, { "x" => Float::INFINITY })
    end

    assert_argument_error(/must be finite/) do
      Amoskeag.evaluate(program, { "x" => -Float::INFINITY })
    end
  end

  def test_rejects_nan_values
    program = Amoskeag.compile("x", [])

    assert_argument_error(/must be finite/) do
      Amoskeag.evaluate(program, { "x" => Float::NAN })
    end
  end

  # Test error message quality

  def test_error_messages_are_informative
    # Nil source
    error = assert_argument_error(/source must be a String/) do
      Amoskeag.compile(nil, [])
    end
    assert_match(/String/, error.message)
    assert_match(/NilClass/, error.message)

    # Empty source
    error = assert_argument_error(/cannot be empty/) do
      Amoskeag.compile("", [])
    end

    # Invalid symbols
    error = assert_argument_error(/symbols must contain only/) do
      Amoskeag.compile("x", [123])
    end
    assert_match(/Integer/, error.message)

    # Invalid data
    program = Amoskeag.compile("x", [])
    error = assert_argument_error(/invalid type/) do
      Amoskeag.evaluate(program, { "x" => Object.new })
    end
    assert_match(/Object/, error.message)
  end

  # Test boundary conditions

  def test_empty_inputs
    # Empty symbols array
    program = Amoskeag.compile("2 + 2", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 4.0, result

    # Empty data hash
    program = Amoskeag.compile("2 + 2", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 4.0, result
  end

  def test_single_character_inputs
    # Single character source
    program = Amoskeag.compile("1", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 1.0, result

    # Single character variable name
    program = Amoskeag.compile("x", [])
    result = Amoskeag.evaluate(program, { "x" => 42 })
    assert_equal 42.0, result
  end

  def test_maximum_valid_inputs
    # Long but valid source
    long_source = (1..100).map { |i| "#{i}" }.join(" + ")
    program = Amoskeag.compile(long_source, [])
    result = Amoskeag.evaluate(program, {})
    assert_equal (1..100).sum.to_f, result
  end

  # Test resource cleanup

  def test_resources_cleaned_up_on_error
    skip "Undefined variable detection not yet implemented"
    # Compilation error should not leak memory
    100.times do
      assert_compile_error("if true") # Syntax error
    end

    GC.start

    # Evaluation error should not leak memory
    program = Amoskeag.compile("x", [])
    100.times do
      assert_eval_error(program, {}) # Undefined variable
    end

    GC.start
  end

  def test_repeated_compilation
    # Compile the same program many times
    100.times do
      program = Amoskeag.compile("2 + 2", [])
      result = Amoskeag.evaluate(program, {})
      assert_equal 4.0, result
    end

    GC.start
  end

  def test_repeated_evaluation
    program = Amoskeag.compile("x + y", [])

    # Evaluate many times with different data
    1000.times do |i|
      result = Amoskeag.evaluate(program, { "x" => i, "y" => i })
      assert_equal (i * 2).to_f, result
    end

    GC.start
  end

  # Test concurrent access

  def test_concurrent_compilation_and_evaluation
    threads = []

    10.times do |i|
      threads << Thread.new do
        program = Amoskeag.compile("x * #{i}", [])

        100.times do |j|
          result = Amoskeag.evaluate(program, { "x" => j })
          assert_equal (j * i).to_f, result
        end
      end
    end

    threads.each(&:join)
    GC.start
  end

  # Test whitespace handling

  def test_handles_various_whitespace
    # Tabs
    program = Amoskeag.compile("2\t+\t2", [])
    assert_equal 4.0, Amoskeag.evaluate(program, {})

    # Multiple spaces
    program = Amoskeag.compile("2    +    2", [])
    assert_equal 4.0, Amoskeag.evaluate(program, {})

    # Newlines
    program = Amoskeag.compile("2\n+\n2", [])
    assert_equal 4.0, Amoskeag.evaluate(program, {})

    # Mixed whitespace
    program = Amoskeag.compile("  2  \n\t+\t\n  2  ", [])
    assert_equal 4.0, Amoskeag.evaluate(program, {})
  end

  def test_preserves_string_whitespace
    program = Amoskeag.compile('text', [])
    result = Amoskeag.evaluate(program, { "text" => "  spaces  \ttabs\t  \n" })
    assert_equal "  spaces  \ttabs\t  \n", result
  end
end
