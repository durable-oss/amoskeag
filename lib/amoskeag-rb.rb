require 'json'
require 'amoskeag-rb/version'

# Try to load the native extension
begin
  require 'amoskeag_native'
rescue LoadError => e
  raise LoadError, "Could not load the Amoskeag native extension. " \
                   "Please ensure the gem was installed correctly. " \
                   "Original error: #{e.message}"
end

module Amoskeag
  # Amoskeag is a purely functional DSL for business rules, templates, and spreadsheet formulas.
  #
  # @example Basic arithmetic
  #   result = Amoskeag.eval("2 + 2", {})
  #   # => 4.0
  #
  # @example Using variables
  #   result = Amoskeag.eval("user.age * 2", { "user" => { "age" => 25 } })
  #   # => 50.0
  #
  # @example Business rules with symbols
  #   code = "if user.age >= 18 :adult else :minor end"
  #   result = Amoskeag.eval(code, { "user" => { "age" => 25 } }, [:adult, :minor])
  #   # => :adult
  #
  # @example Compile once, evaluate many times
  #   program = Amoskeag.compile("user.age >= 18", [])
  #   Amoskeag.evaluate(program, { "user" => { "age" => 25 } })  # => true
  #   Amoskeag.evaluate(program, { "user" => { "age" => 15 } })  # => false

  class << self
    # Compile an Amoskeag program for later evaluation
    #
    # This is the preferred method when you need to evaluate the same program
    # multiple times with different data. Compilation performs static validation
    # of symbols and functions.
    #
    # @param source [String] The Amoskeag source code
    # @param symbols [Array<Symbol, String>, nil] Optional array of valid symbol names
    #   that can be returned by the program. If provided, the compiler will validate
    #   that only these symbols are used as literals.
    # @return [Amoskeag::Program] A compiled program that can be evaluated
    # @raise [Amoskeag::CompileError] If the source code contains syntax errors,
    #   undefined symbols, or undefined functions
    #
    # @example
    #   program = Amoskeag.compile("if score >= 90 :A else :B end", [:A, :B, :C])
    #   Amoskeag.evaluate(program, { "score" => 95 })  # => :A
    #   Amoskeag.evaluate(program, { "score" => 85 })  # => :B
    def compile(source, symbols = nil)
      symbols = normalize_symbols(symbols) if symbols
      super(source, symbols)
    end

    # Evaluate a compiled program with data
    #
    # @param program [Amoskeag::Program] A program compiled with {compile}
    # @param data [Hash] The data context for evaluation. Keys should be strings,
    #   and values can be any JSON-compatible type (numbers, strings, booleans,
    #   nil, arrays, hashes) or Ruby symbols.
    # @return [Object] The result of evaluation. Can be any Ruby type:
    #   Float, String, true, false, nil, Array, Hash, or Symbol
    # @raise [Amoskeag::EvalError] If evaluation fails (e.g., variable not found,
    #   type error, division by zero)
    #
    # @example
    #   program = Amoskeag.compile("total * tax_rate", [])
    #   Amoskeag.evaluate(program, { "total" => 100, "tax_rate" => 0.08 })
    #   # => 8.0
    def evaluate(program, data)
      data = normalize_data(data)
      super(program, data)
    end

    # Compile and evaluate in one step (convenience method)
    #
    # This is convenient for one-off evaluations, but if you need to evaluate
    # the same source code multiple times, use {compile} and {evaluate} separately
    # for better performance.
    #
    # @param source [String] The Amoskeag source code
    # @param data [Hash] The data context for evaluation
    # @param symbols [Array<Symbol, String>, nil] Optional array of valid symbol names
    # @return [Object] The result of evaluation
    # @raise [Amoskeag::CompileError] If compilation fails
    # @raise [Amoskeag::EvalError] If evaluation fails
    #
    # @example String operations
    #   result = Amoskeag.eval('"hello" | upcase', {})
    #   # => "HELLO"
    #
    # @example Array operations
    #   result = Amoskeag.eval('[1, 2, 3] | sum', {})
    #   # => 6.0
    #
    # @example Financial calculations
    #   # Monthly payment for a $250,000 loan at 4.5% APR for 30 years
    #   result = Amoskeag.eval('pmt(0.045 / 12, 360, 250000) | round(2)', {})
    #   # => -1266.71
    def eval(source, data, symbols = nil)
      data = normalize_data(data)
      symbols = normalize_symbols(symbols) if symbols
      super(source, data, symbols)
    end

    private

    # Convert Ruby symbols to strings for FFI
    def normalize_symbols(symbols)
      return nil if symbols.nil?
      symbols.map { |s| s.is_a?(Symbol) ? s.to_s : s }
    end

    # Ensure data hash has string keys and normalized values
    def normalize_data(data)
      case data
      when Hash
        data.transform_keys(&:to_s).transform_values { |v| normalize_data(v) }
      when Array
        data.map { |v| normalize_data(v) }
      else
        data
      end
    end
  end

  # Compiled Amoskeag program
  #
  # This class cannot be instantiated directly. Use {Amoskeag.compile} to create instances.
  # Programs are immutable and thread-safe, so a single compiled program can be
  # evaluated concurrently from multiple threads.
  class Program
    # Programs cannot be instantiated from Ruby
    # Use Amoskeag.compile instead
  end

  # Raised when compilation fails
  #
  # Common causes:
  # - Syntax errors in the source code
  # - Undefined symbols (symbol literals not in the allowed list)
  # - Undefined functions
  # - Function arity mismatches (wrong number of arguments)
  class CompileError < StandardError; end

  # Raised when evaluation fails
  #
  # Common causes:
  # - Variable not found in data context
  # - Type errors (e.g., trying to add a string and a number)
  # - Division by zero
  # - Invalid operations on nil values
  class EvalError < StandardError; end
end
