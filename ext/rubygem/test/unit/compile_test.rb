# frozen_string_literal: true

require "test_helper"

class CompileTest < Minitest::Test
  include TestHelper

  def test_compile_simple_expression
    program = Amoskeag.compile("2 + 2", [])
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_with_variables
    program = Amoskeag.compile("user.age", [])
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_with_symbols_as_strings
    program = Amoskeag.compile("if x :success else :failure end", ["success", "failure"])
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_with_symbols_as_symbols
    program = Amoskeag.compile("if x :success else :failure end", [:success, :failure])
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_with_mixed_symbols
    program = Amoskeag.compile("if x :success else :failure end", ["success", :failure])
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_complex_expression
    source = <<~CODE
      let base = 100
      in let tax = base * 0.08
      in base + tax
    CODE

    program = Amoskeag.compile(source, [])
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_with_conditionals
    program = Amoskeag.compile('if age >= 18 "adult" else "minor" end', [])
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_with_string_operations
    program = Amoskeag.compile('"hello" | upcase', [])
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_with_array_operations
    program = Amoskeag.compile("[1, 2, 3] | sum", [])
    assert_instance_of Amoskeag::Program, program
  end

  # Error cases

  def test_compile_rejects_nil_source
    assert_argument_error(/source must be a String/) do
      Amoskeag.compile(nil, [])
    end
  end

  def test_compile_rejects_empty_source
    assert_argument_error(/source cannot be empty/) do
      Amoskeag.compile("", [])
    end
  end

  def test_compile_rejects_non_string_source
    assert_argument_error(/source must be a String/) do
      Amoskeag.compile(123, [])
    end
  end

  def test_compile_rejects_non_array_symbols
    assert_argument_error(/symbols must be an Array/) do
      Amoskeag.compile("2 + 2", "not an array")
    end
  end

  def test_compile_rejects_invalid_symbols
    assert_argument_error(/symbols must contain only Strings or Symbols/) do
      Amoskeag.compile(":symbol", [123])
    end
  end

  def test_compile_rejects_syntax_error
    assert_compile_error("if true") # Missing end
  end

  def test_compile_rejects_undefined_symbol
    # Symbol not in allowed list should fail compilation
    assert_compile_error("if x :unknown else :failure end", [:failure])
  end

  def test_compile_rejects_too_large_source
    # Create a string that exceeds the 10MB limit
    large_source = "x" * (10 * 1024 * 1024 + 1)
    assert_argument_error(/source too large/) do
      Amoskeag.compile(large_source, [])
    end
  end

  def test_compile_rejects_too_many_symbols
    # Create more than 10,000 symbols
    too_many_symbols = (0..10001).map { |i| "sym#{i}" }
    assert_argument_error(/Too many symbols/) do
      Amoskeag.compile(":sym0", too_many_symbols)
    end
  end

  def test_compile_immutable_program
    program = Amoskeag.compile("2 + 2", [])

    # Should not be able to create new Program instances
    assert_raises(NoMethodError) do
      Amoskeag::Program.new
    end
  end

  def test_compile_accepts_nil_symbols
    program = Amoskeag.compile("2 + 2", nil)
    assert_instance_of Amoskeag::Program, program
  end

  def test_compile_accepts_empty_symbols
    program = Amoskeag.compile("2 + 2", [])
    assert_instance_of Amoskeag::Program, program
  end
end
