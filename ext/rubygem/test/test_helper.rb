# frozen_string_literal: true

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)

require "minitest/autorun"
require "amoskeag-rb"

# Test helper module
module TestHelper
  # Helper to assert compilation error with message pattern
  def assert_compile_error(source, symbols = nil, message_pattern = nil)
    error = assert_raises(Amoskeag::CompileError) do
      Amoskeag.compile(source, symbols)
    end

    if message_pattern
      assert_match(message_pattern, error.message,
                   "Expected error message to match #{message_pattern.inspect}")
    end

    error
  end

  # Helper to assert evaluation error with message pattern
  def assert_eval_error(program, data, message_pattern = nil)
    error = assert_raises(Amoskeag::EvalError) do
      Amoskeag.evaluate(program, data)
    end

    if message_pattern
      assert_match(message_pattern, error.message,
                   "Expected error message to match #{message_pattern.inspect}")
    end

    error
  end

  # Helper to assert argument error with message pattern
  def assert_argument_error(message_pattern = nil, &block)
    error = assert_raises(ArgumentError, &block)

    if message_pattern
      assert_match(message_pattern, error.message,
                   "Expected error message to match #{message_pattern.inspect}")
    end

    error
  end
end
