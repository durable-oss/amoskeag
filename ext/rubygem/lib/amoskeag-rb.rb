# frozen_string_literal: true

require 'json'
require_relative 'amoskeag-rb/version'

# Load the native extension
begin
  require 'amoskeag_native'
rescue LoadError => e
  raise LoadError, "Failed to load amoskeag_native extension. " \
                   "Make sure the gem is properly installed and compiled. " \
                   "Original error: #{e.message}"
end

# Amoskeag - A secure, functional DSL for business rules
#
# This module provides the main interface for compiling and evaluating
# Amoskeag programs from Ruby.
module Amoskeag
  class << self
    # Store the native methods before we override them
    alias_method :compile_native, :compile
    alias_method :evaluate_native, :evaluate
    alias_method :eval_expression_native, :eval_expression

    # Compile an Amoskeag program with optional symbols
    #
    # @param source [String] The Amoskeag source code to compile
    # @param symbols [Array<String, Symbol>, nil] Optional array of valid symbol names
    # @return [Amoskeag::Program] A compiled program that can be evaluated
    # @raise [ArgumentError] If source is not a string or symbols is not an array
    # @raise [Amoskeag::CompileError] If compilation fails
    #
    # @example Basic compilation
    #   program = Amoskeag.compile("2 + 2", [])
    #
    # @example With symbols
    #   program = Amoskeag.compile("if age >= 18 :adult else :minor end", [:adult, :minor])
    def compile(source, symbols = nil)
      # Defensive programming: validate inputs
      raise ArgumentError, "source must be a String, got #{source.class}" unless source.is_a?(String)
      raise ArgumentError, "source cannot be empty" if source.empty?

      if symbols
        raise ArgumentError, "symbols must be an Array, got #{symbols.class}" unless symbols.is_a?(Array)

        # Convert symbols to strings for FFI
        symbols = symbols.map do |s|
          case s
          when String
            s
          when Symbol
            s.to_s
          else
            raise ArgumentError, "symbols must contain only Strings or Symbols, got #{s.class}"
          end
        end
      end

      # Call the native compile method
      compile_native(source, symbols)
    end

    # Evaluate a compiled program with data
    #
    # @param program [Amoskeag::Program] A compiled program
    # @param data [Hash] The data context for evaluation
    # @return [Object] The result of evaluation (Number, String, Boolean, nil, Array, Hash, or Symbol)
    # @raise [ArgumentError] If program is not a Program or data is not a Hash
    # @raise [Amoskeag::EvalError] If evaluation fails
    #
    # @example
    #   program = Amoskeag.compile("user.age * 2", [])
    #   result = Amoskeag.evaluate(program, {"user" => {"age" => 25}})
    #   # => 50.0
    def evaluate(program, data)
      # Defensive programming: validate inputs
      raise ArgumentError, "program must be an Amoskeag::Program, got #{program.class}" unless program.is_a?(Program)
      raise ArgumentError, "data must be a Hash, got #{data.class}" unless data.is_a?(Hash)

      # Validate data contains only JSON-serializable values
      validate_data!(data)

      # Call the native evaluate method
      evaluate_native(program, data)
    end

    # Compile and evaluate in one step (convenience method)
    #
    # @param source [String] The Amoskeag source code to compile and evaluate
    # @param data [Hash] The data context for evaluation
    # @param symbols [Array<String, Symbol>, nil] Optional array of valid symbol names
    # @return [Object] The result of evaluation
    # @raise [ArgumentError] If arguments are invalid
    # @raise [Amoskeag::CompileError] If compilation fails
    # @raise [Amoskeag::EvalError] If evaluation fails
    #
    # @example
    #   result = Amoskeag.eval_expression("2 + 2", {})
    #   # => 4.0
    def eval_expression(source, data, symbols = nil)
      # Defensive programming: validate inputs
      raise ArgumentError, "source must be a String, got #{source.class}" unless source.is_a?(String)
      raise ArgumentError, "source cannot be empty" if source.empty?
      raise ArgumentError, "data must be a Hash, got #{data.class}" unless data.is_a?(Hash)

      if symbols
        raise ArgumentError, "symbols must be an Array, got #{symbols.class}" unless symbols.is_a?(Array)

        # Convert symbols to strings for FFI
        symbols = symbols.map do |s|
          case s
          when String
            s
          when Symbol
            s.to_s
          else
            raise ArgumentError, "symbols must contain only Strings or Symbols, got #{s.class}"
          end
        end
      end

      # Validate data contains only JSON-serializable values
      validate_data!(data)

      # Call the native eval method
      eval_expression_native(source, data, symbols)
    end

    private

    # Validate that data contains only JSON-serializable values
    # @param obj [Object] The object to validate
    # @param path [String] The current path for error reporting
    # @raise [ArgumentError] If data contains invalid types
    def validate_data!(obj, path = "data")
      case obj
      when Hash
        obj.each do |key, value|
          unless key.is_a?(String) || key.is_a?(Symbol)
            raise ArgumentError, "#{path}: Hash key must be String or Symbol, got #{key.class}"
          end
          validate_data!(value, "#{path}[#{key.inspect}]")
        end
      when Array
        obj.each_with_index do |item, idx|
          validate_data!(item, "#{path}[#{idx}]")
        end
      when String, TrueClass, FalseClass, NilClass, Symbol
        # These are valid types
      when Numeric
        # Check for Infinity and NaN
        if obj.respond_to?(:finite?) && !obj.finite?
          raise ArgumentError, "Invalid number in #{path}: must be finite (got #{obj})"
        end
      else
        raise ArgumentError, "#{path} contains invalid type: #{obj.class}. " \
                             "Only Hash, Array, String, Numeric, Boolean, nil, and Symbol are allowed."
      end
    end
  end

  # Base error class for Amoskeag errors
  class Error < StandardError; end

  # The CompileError and EvalError classes are defined in the C extension
  # We just document them here for RDoc
  # class CompileError < Error; end
  # class EvalError < Error; end

  # Represents a compiled Amoskeag program
  #
  # This class cannot be instantiated directly. Use {Amoskeag.compile} instead.
  # Program objects are immutable and thread-safe.
  class Program
    # @!visibility private
    # Prevent direct instantiation
    private_class_method :new
  end
end
