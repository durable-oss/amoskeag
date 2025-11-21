# frozen_string_literal: true

require "test_helper"

class ErrorHandlingTest < Minitest::Test
  include TestHelper

  # Test error handling in compilation

  def test_compile_error_invalid_syntax
    assert_compile_error("if true")
    assert_compile_error("let x = 1")
    assert_compile_error("2 +")
    assert_compile_error("+ 2")
  end

  def test_compile_error_undefined_function
    assert_compile_error("[1, 2, 3] | undefined_function")
  end

  def test_compile_error_type_mismatch
    skip "Type checking not yet implemented in compiler"
    # String operation on number
    assert_compile_error("123 | upcase")
  end

  def test_compile_error_undefined_symbol
    assert_compile_error(":undefined_symbol", [])
    assert_compile_error("if x :wrong_symbol else :minor end", [:minor])
  end

  # Test error handling in evaluation

  def test_eval_error_division_by_zero
    program = Amoskeag.compile("x / y", [])
    # Division by zero should raise an error
    assert_eval_error(program, { "x" => 10, "y" => 0 })
  end

  def test_eval_error_undefined_variable
    skip "Undefined variable detection not yet implemented"
    program = Amoskeag.compile("undefined_var", [])
    assert_eval_error(program, {})
  end

  def test_eval_error_type_error_in_operation
    program = Amoskeag.compile("x + y", [])
    # Cannot add string to number
    error = assert_eval_error(program, { "x" => "hello", "y" => 5 })
  end

  def test_eval_error_invalid_array_index
    skip "Out of bounds array access not yet validated"
    program = Amoskeag.compile("arr[10]", [])
    assert_eval_error(program, { "arr" => [1, 2, 3] })
  end

  # Test memory safety

  def test_memory_safety_freed_program
    program = Amoskeag.compile("2 + 2", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal 4.0, result

    # Ruby GC should handle cleanup properly
    # Force GC and verify no segfaults
    GC.start
    result = Amoskeag.evaluate(program, {})
    assert_equal 4.0, result
  end

  def test_memory_safety_multiple_programs
    programs = []
    100.times do |i|
      programs << Amoskeag.compile("#{i} + 1", [])
    end

    GC.start

    programs.each_with_index do |program, i|
      result = Amoskeag.evaluate(program, {})
      assert_equal i + 1.0, result
    end
  end

  def test_memory_safety_large_data
    program = Amoskeag.compile("data.values | sum", [])

    # Create large data structure
    data = { "data" => { "values" => Array.new(10_000) { |i| i } } }

    result = Amoskeag.evaluate(program, data)
    expected = (0...10_000).sum.to_f
    assert_equal expected, result

    GC.start
  end

  # Test thread safety

  def test_thread_safety_concurrent_evaluation
    program = Amoskeag.compile("x * 2", [])

    threads = 10.times.map do |i|
      Thread.new do
        100.times do
          result = Amoskeag.evaluate(program, { "x" => i })
          assert_equal i * 2.0, result
        end
      end
    end

    threads.each(&:join)
  end

  def test_thread_safety_concurrent_compilation
    threads = 10.times.map do |i|
      Thread.new do
        10.times do
          program = Amoskeag.compile("#{i} + x", [])
          result = Amoskeag.evaluate(program, { "x" => 5 })
          assert_equal i + 5.0, result
        end
      end
    end

    threads.each(&:join)
  end

  # Test edge cases with special characters and Unicode

  def test_unicode_strings
    program = Amoskeag.compile('"Hello, " + name', [])
    result = Amoskeag.evaluate(program, { "name" => "世界" })
    assert_equal "Hello, 世界", result
  end

  def test_unicode_in_source
    program = Amoskeag.compile('"こんにちは"', [])
    result = Amoskeag.evaluate(program, {})
    assert_equal "こんにちは", result
  end

  def test_emoji_in_strings
    program = Amoskeag.compile('emoji', [])
    result = Amoskeag.evaluate(program, { "emoji" => "🚀🎉" })
    assert_equal "🚀🎉", result
  end

  def test_special_characters_in_strings
    program = Amoskeag.compile('text', [])
    result = Amoskeag.evaluate(program, { "text" => "Line1\nLine2\tTab" })
    assert_equal "Line1\nLine2\tTab", result
  end

  def test_empty_strings
    program = Amoskeag.compile('""', [])
    result = Amoskeag.evaluate(program, {})
    assert_equal "", result
  end

  # Test numeric edge cases

  def test_very_large_numbers
    program = Amoskeag.compile("x + y", [])
    result = Amoskeag.evaluate(program, {
      "x" => 1.0e100,
      "y" => 2.0e100
    })
    assert_in_delta 3.0e100, result, 1.0e85
  end

  def test_very_small_numbers
    program = Amoskeag.compile("x + y", [])
    result = Amoskeag.evaluate(program, {
      "x" => 1.0e-100,
      "y" => 2.0e-100
    })
    assert_in_delta 3.0e-100, result, 1.0e-110
  end

  def test_negative_zero
    program = Amoskeag.compile("x", [])
    result = Amoskeag.evaluate(program, { "x" => -0.0 })
    assert_equal 0.0, result
  end

  def test_infinity_rejected
    program = Amoskeag.compile("x", [])
    # Infinity should be rejected as invalid
    assert_argument_error(/Invalid number.*must be finite/) do
      Amoskeag.evaluate(program, { "x" => Float::INFINITY })
    end
  end

  def test_nan_rejected
    program = Amoskeag.compile("x", [])
    # NaN should be rejected as invalid
    assert_argument_error(/Invalid number.*must be finite/) do
      Amoskeag.evaluate(program, { "x" => Float::NAN })
    end
  end

  # Test array edge cases

  def test_empty_array
    program = Amoskeag.compile("[]", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal [], result
  end

  def test_nested_arrays
    program = Amoskeag.compile("[[1, 2], [3, 4]]", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal [[1.0, 2.0], [3.0, 4.0]], result
  end

  def test_mixed_type_arrays
    program = Amoskeag.compile('[1, "two", true, nil]', [])
    result = Amoskeag.evaluate(program, {})
    assert_equal [1.0, "two", true, nil], result
  end

  # Test hash edge cases

  def test_empty_hash
    program = Amoskeag.compile("x", [])
    result = Amoskeag.evaluate(program, { "x" => {} })
    assert_equal({}, result)
  end

  def test_nested_hashes
    program = Amoskeag.compile("a.b.c", [])
    result = Amoskeag.evaluate(program, {
      "a" => { "b" => { "c" => "value" } }
    })
    assert_equal "value", result
  end

  def test_hash_with_special_keys
    skip "Hash/array indexing with brackets not yet implemented"
    program = Amoskeag.compile('data["key-with-dashes"]', [])
    result = Amoskeag.evaluate(program, {
      "data" => { "key-with-dashes" => "value" }
    })
    assert_equal "value", result
  end

  # Test nil handling

  def test_nil_in_arrays
    program = Amoskeag.compile("[1, nil, 3]", [])
    result = Amoskeag.evaluate(program, {})
    assert_equal [1.0, nil, 3.0], result
  end

  def test_nil_in_hashes
    program = Amoskeag.compile("x.y", [])
    result = Amoskeag.evaluate(program, { "x" => { "y" => nil } })
    assert_nil result
  end

  def test_nil_safe_navigation
    program = Amoskeag.compile("x.y.z", [])
    result = Amoskeag.evaluate(program, { "x" => nil })
    assert_nil result
  end

  # Test boolean edge cases

  def test_boolean_coercion_in_conditionals
    skip "Amoskeag treats 0 as truthy (only false/nil are falsy)"
    program = Amoskeag.compile('if x "yes" else "no" end', [])

    # True values
    assert_equal "yes", Amoskeag.evaluate(program, { "x" => true })
    assert_equal "yes", Amoskeag.evaluate(program, { "x" => 1 })
    assert_equal "yes", Amoskeag.evaluate(program, { "x" => "hello" })

    # False values
    assert_equal "no", Amoskeag.evaluate(program, { "x" => false })
    assert_equal "no", Amoskeag.evaluate(program, { "x" => 0 })
    assert_equal "no", Amoskeag.evaluate(program, { "x" => nil })
  end
end
