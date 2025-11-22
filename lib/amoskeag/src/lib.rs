//! Amoskeag: A Purely Functional, Statically-Validated Evaluation Language
//!
//! This is the main crate that provides the compile and evaluate API for Amoskeag programs.
//! It combines the lexer, parser, and standard library to provide a complete execution environment.

pub mod backend;

use amoskeag_lexer::Lexer;
use amoskeag_parser::{BinaryOp, Expr, Parser, UnaryOp};
use amoskeag_stdlib_functions::FunctionError;
use amoskeag_stdlib_operators::{OperatorError, Value};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

// Re-export the Value type for convenience
pub use amoskeag_stdlib_operators::Value as AmoskeagValue;

// Re-export backend types
pub use backend::{
    Backend, BackendCapabilities, BackendError, BackendRegistry, BackendResult, PerformanceTier,
};

/// Errors that can occur during compilation
#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Lexer error: {0}")]
    LexerError(String),

    #[error("Parser error: {0}")]
    ParserError(String),

    #[error("Symbol '{symbol}' is not defined in the execution contract")]
    UndefinedSymbol { symbol: String },

    #[error("Function '{function}' is not defined")]
    UndefinedFunction { function: String },

    #[error("Function '{function}' expects {expected} arguments, but {actual} were provided")]
    ArityMismatch {
        function: String,
        expected: String,
        actual: usize,
    },
}

/// Errors that can occur during evaluation
#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Variable '{0}' not found")]
    VariableNotFound(String),

    #[error("Operator error: {0}")]
    OperatorError(#[from] OperatorError),

    #[error("Function error: {0}")]
    FunctionError(#[from] FunctionError),

    #[error("Type error: expected {expected}, got {got}")]
    TypeError { expected: String, got: String },

    #[error("Invalid dictionary key: {0}")]
    InvalidDictionaryKey(String),
}

/// A compiled Amoskeag program, ready for evaluation
pub struct CompiledProgram {
    ast: Expr,
    #[allow(dead_code)]
    symbols: HashSet<String>,
}

impl CompiledProgram {
    /// Get the AST of the compiled program
    ///
    /// This is useful for backends that need direct access to the AST
    pub fn ast(&self) -> &Expr {
        &self.ast
    }
}

/// The execution context for evaluating an Amoskeag program
pub struct Context {
    /// Local variable bindings (from let expressions)
    locals: HashMap<String, Value>,
    /// The data dictionary (implicit context)
    data: HashMap<String, Value>,
}

impl Context {
    /// Create a new context with the given data
    pub fn new(data: HashMap<String, Value>) -> Self {
        Self {
            locals: HashMap::new(),
            data,
        }
    }

    /// Create a child context with a new local binding
    fn with_local(&self, name: String, value: Value) -> Self {
        let mut locals = self.locals.clone();
        locals.insert(name, value);
        Self {
            locals,
            data: self.data.clone(),
        }
    }

    /// Look up a variable in the context
    /// Resolution order: locals -> data -> nil
    ///
    /// # Safety
    /// This method implements safe navigation - it never fails,
    /// returning Nil for undefined variables instead of an error.
    fn lookup(&self, name: &str) -> Value {
        debug_assert!(!name.is_empty(), "lookup() called with empty name");

        // 1. Check local scope
        if let Some(value) = self.locals.get(name) {
            return value.clone();
        }

        // 2. Check data dictionary
        if let Some(value) = self.data.get(name) {
            return value.clone();
        }

        // 3. Return nil if not found (safe navigation)
        Value::Nil
    }

    fn contains(&self, name: &str) -> bool {
        debug_assert!(!name.is_empty(), "contains() called with empty name");
        self.locals.contains_key(name) || self.data.contains_key(name)
    }
}

/// Compile an Amoskeag program with static validation
///
/// # Arguments
///
/// * `source` - The source code to compile
/// * `symbols` - The list of valid symbol literals for this program
///
/// # Returns
///
/// A compiled program or a compilation error
pub fn compile(source: &str, symbols: &[&str]) -> Result<CompiledProgram, CompileError> {
    // Parse the source code
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| CompileError::LexerError(e.to_string()))?;

    let mut parser = Parser::new(tokens);
    let ast = parser
        .parse()
        .map_err(|e| CompileError::ParserError(e.to_string()))?;

    // Build the symbol table
    let symbol_table: HashSet<String> = symbols.iter().map(|s| s.to_string()).collect();

    // Validate symbols and functions in the AST
    validate_ast(&ast, &symbol_table)?;

    Ok(CompiledProgram {
        ast,
        symbols: symbol_table,
    })
}

/// Validate the AST for undefined symbols and functions
fn validate_ast(expr: &Expr, symbols: &HashSet<String>) -> Result<(), CompileError> {
    match expr {
        Expr::Symbol(s) => {
            if !symbols.contains(s) {
                return Err(CompileError::UndefinedSymbol { symbol: s.clone() });
            }
            Ok(())
        }

        Expr::Array(exprs) => {
            for e in exprs {
                validate_ast(e, symbols)?;
            }
            Ok(())
        }

        Expr::Dictionary(pairs) => {
            for (_, e) in pairs {
                validate_ast(e, symbols)?;
            }
            Ok(())
        }

        Expr::FunctionCall { name, args } => {
            // Validate function exists and has correct arity
            validate_function_call(name, args.len())?;

            // Validate arguments
            for arg in args {
                validate_ast(arg, symbols)?;
            }
            Ok(())
        }

        Expr::Let { value, body, .. } => {
            validate_ast(value, symbols)?;
            validate_ast(body, symbols)?;
            Ok(())
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            validate_ast(condition, symbols)?;
            validate_ast(then_branch, symbols)?;
            validate_ast(else_branch, symbols)?;
            Ok(())
        }

        Expr::Binary { left, right, .. } => {
            validate_ast(left, symbols)?;
            validate_ast(right, symbols)?;
            Ok(())
        }

        Expr::Unary { operand, .. } => validate_ast(operand, symbols),

        Expr::Pipe { left, right } => {
            validate_ast(left, symbols)?;
            validate_ast(right, symbols)?;
            Ok(())
        }

        // Literals and variables don't need validation
        Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | Expr::Nil | Expr::Variable(_) => {
            Ok(())
        }
    }
}

/// Validate a function call (existence and arity)
fn validate_function_call(name: &str, arg_count: usize) -> Result<(), CompileError> {
    // Define function signatures (name -> (min_args, max_args))
    // For variadic functions, min and max are different
    let function_signatures: HashMap<&str, (usize, usize)> = [
        // String functions
        ("upcase", (1, 1)),
        ("downcase", (1, 1)),
        ("capitalize", (1, 1)),
        ("strip", (1, 1)),
        ("split", (2, 2)),
        ("join", (2, 2)),
        ("truncate", (2, 2)),
        ("replace", (3, 3)),
        // Numeric functions
        ("abs", (1, 1)),
        ("ceil", (1, 1)),
        ("floor", (1, 1)),
        ("round", (1, 2)),
        ("plus", (2, 2)),
        ("minus", (2, 2)),
        ("times", (2, 2)),
        ("divided_by", (2, 2)),
        ("modulo", (2, 2)),
        ("max", (2, 2)),
        ("min", (2, 2)),
        // Collection functions
        ("size", (1, 1)),
        ("first", (1, 1)),
        ("last", (1, 1)),
        ("contains", (2, 2)),
        ("sum", (1, 1)),
        ("avg", (1, 1)),
        ("sort", (1, 1)),
        ("keys", (1, 1)),
        ("values", (1, 1)),
        ("reverse", (1, 1)),
        ("at", (2, 2)),
        ("uniq", (1, 1)),
        ("group_by", (2, 2)),
        ("map", (2, 2)),
        // Logic functions
        ("choose", (2, 2)),
        ("if_then_else", (3, 3)),
        ("is_number", (1, 1)),
        ("is_string", (1, 1)),
        ("is_boolean", (1, 1)),
        ("is_nil", (1, 1)),
        ("is_array", (1, 1)),
        ("is_dictionary", (1, 1)),
        ("coalesce", (2, 2)),
        ("default", (2, 2)),
        // Financial functions - Time Value of Money
        ("pmt", (4, 4)),
        ("pv", (3, 3)),
        ("fv", (4, 4)),
        ("nper", (3, 3)),
        ("rate", (3, 3)),
        // Financial functions - Investment Analysis
        ("npv", (2, 2)),
        ("irr", (1, 1)),
        ("mirr", (3, 3)),
        // Financial functions - Depreciation
        ("sln", (3, 3)),
        ("ddb", (4, 4)),
        ("db", (5, 5)),
        // Financial functions - Payment Components
        ("ipmt", (5, 5)),
        ("ppmt", (5, 5)),
        ("cumipmt", (6, 6)),
        ("cumprinc", (6, 6)),
        // Financial functions - Interest Rate Conversion
        ("effect", (2, 2)),
        ("nominal", (2, 2)),
        // Date functions
        ("date_now", (0, 0)),
        ("date_format", (2, 2)),
        ("date_trunc", (1, 1)),
        ("date_parse", (1, 1)),
    ]
    .iter()
    .cloned()
    .collect();

    if let Some((min_args, max_args)) = function_signatures.get(name) {
        if arg_count < *min_args || arg_count > *max_args {
            let expected = if min_args == max_args {
                format!("{}", min_args)
            } else {
                format!("{}-{}", min_args, max_args)
            };
            return Err(CompileError::ArityMismatch {
                function: name.to_string(),
                expected,
                actual: arg_count,
            });
        }
        Ok(())
    } else {
        Err(CompileError::UndefinedFunction {
            function: name.to_string(),
        })
    }
}

/// Evaluate a compiled Amoskeag program
///
/// # Arguments
///
/// * `program` - The compiled program to evaluate
/// * `data` - The data dictionary (execution context)
///
/// # Returns
///
/// The result of the evaluation
pub fn evaluate(
    program: &CompiledProgram,
    data: &HashMap<String, Value>,
) -> Result<Value, EvalError> {
    let context = Context::new(data.clone());
    eval_expr(&program.ast, &context)
}

/// Evaluate an expression in a given context
///
/// This function is public to allow backend implementations to use it directly.
pub fn eval_expr(expr: &Expr, context: &Context) -> Result<Value, EvalError> {
    match expr {
        // Literals
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Boolean(b) => Ok(Value::Boolean(*b)),
        Expr::Nil => Ok(Value::Nil),
        Expr::Symbol(s) => Ok(Value::Symbol(s.clone())),

        // Array literal
        Expr::Array(exprs) => {
            let mut values = Vec::new();
            for e in exprs {
                values.push(eval_expr(e, context)?);
            }
            Ok(Value::Array(values))
        }

        // Dictionary literal
        Expr::Dictionary(pairs) => {
            let mut map = HashMap::new();
            for (key, value_expr) in pairs {
                let value = eval_expr(value_expr, context)?;
                map.insert(key.clone(), value);
            }
            Ok(Value::Dictionary(map))
        }

        // Variable access (with dot navigation)
        //
        // # Safe Navigation
        // Implements safe navigation: accessing undefined variables or invalid
        // paths returns Nil instead of an error, preventing null pointer exceptions.
        Expr::Variable(path) => {
            if path.is_empty() {
                return Ok(Value::Nil);
            }

            // If it's a simple variable (no dots), check if it exists
            if path.len() == 1 && !context.contains(&path[0]) {
                return Err(EvalError::VariableNotFound(path[0].clone()));
            }

            // Look up the root variable
            let mut current = context.lookup(&path[0]);

            // Navigate the path with safe navigation semantics
            for key in &path[1..] {
                current = match current {
                    Value::Dictionary(ref map) => map.get(key).cloned().unwrap_or(Value::Nil),
                    _ => Value::Nil, // Safe navigation: return nil if not a dictionary
                };
            }

            Ok(current)
        }

        // Function call
        Expr::FunctionCall { name, args } => {
            let arg_values: Result<Vec<_>, _> =
                args.iter().map(|a| eval_expr(a, context)).collect();
            let arg_values = arg_values?;
            call_function(name, &arg_values)
        }

        // Let binding
        Expr::Let { name, value, body } => {
            let val = eval_expr(value, context)?;
            let new_context = context.with_local(name.clone(), val);
            eval_expr(body, &new_context)
        }

        // If expression
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond_value = eval_expr(condition, context)?;
            let is_truthy = match cond_value {
                Value::Boolean(b) => b,
                Value::Nil => false,
                _ => true, // Everything else is truthy
            };

            if is_truthy {
                eval_expr(then_branch, context)
            } else {
                eval_expr(else_branch, context)
            }
        }

        // Binary operations
        Expr::Binary { op, left, right } => {
            let left_val = eval_expr(left, context)?;
            let right_val = eval_expr(right, context)?;
            eval_binary_op(*op, &left_val, &right_val)
        }

        // Unary operations
        Expr::Unary { op, operand } => {
            let val = eval_expr(operand, context)?;
            eval_unary_op(*op, &val)
        }

        // Pipe expression (this should have been transformed by the parser,
        // but we handle it here for completeness)
        Expr::Pipe { left, right } => {
            let left_val = eval_expr(left, context)?;

            // The right side should be a function call
            match right.as_ref() {
                Expr::FunctionCall { name, args } => {
                    // Prepend the left value as the first argument
                    let mut new_args = vec![left_val];
                    for arg in args {
                        new_args.push(eval_expr(arg, context)?);
                    }
                    call_function(name, &new_args)
                }
                Expr::Variable(path) if path.len() == 1 => {
                    // Simple function name without args
                    call_function(&path[0], &[left_val])
                }
                _ => {
                    // Invalid pipe target
                    Err(EvalError::TypeError {
                        expected: "function call".to_string(),
                        got: "expression".to_string(),
                    })
                }
            }
        }
    }
}

/// Evaluate a binary operation
///
/// # Defensive Programming
/// All operations are validated by the stdlib operators module.
/// Division by zero and other invalid operations return appropriate errors.
fn eval_binary_op(op: BinaryOp, left: &Value, right: &Value) -> Result<Value, EvalError> {
    use amoskeag_stdlib_operators::*;

    match op {
        BinaryOp::Add => add(left, right).map_err(EvalError::from),
        BinaryOp::Subtract => subtract(left, right).map_err(EvalError::from),
        BinaryOp::Multiply => multiply(left, right).map_err(EvalError::from),
        BinaryOp::Divide => divide(left, right).map_err(EvalError::from),
        BinaryOp::Modulo => modulo(left, right).map_err(EvalError::from),
        BinaryOp::Equal => Ok(equal(left, right)),
        BinaryOp::NotEqual => Ok(not_equal(left, right)),
        BinaryOp::Less => less_than(left, right).map_err(EvalError::from),
        BinaryOp::Greater => greater_than(left, right).map_err(EvalError::from),
        BinaryOp::LessEqual => less_than_or_equal(left, right).map_err(EvalError::from),
        BinaryOp::GreaterEqual => greater_than_or_equal(left, right).map_err(EvalError::from),
        BinaryOp::And => logical_and(left, right).map_err(EvalError::from),
        BinaryOp::Or => logical_or(left, right).map_err(EvalError::from),
    }
}

/// Evaluate a unary operation
fn eval_unary_op(op: UnaryOp, operand: &Value) -> Result<Value, EvalError> {
    use amoskeag_stdlib_operators::*;

    match op {
        UnaryOp::Not => Ok(logical_not(operand)),
        UnaryOp::Negate => match operand {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(EvalError::TypeError {
                expected: "Number".to_string(),
                got: operand.type_name().to_string(),
            }),
        },
    }
}

/// Call a standard library function
fn call_function(name: &str, args: &[Value]) -> Result<Value, EvalError> {
    use amoskeag_stdlib_functions::*;

    match name {
        // String functions
        "upcase" => upcase(&args[0]).map_err(EvalError::from),
        "downcase" => downcase(&args[0]).map_err(EvalError::from),
        "capitalize" => capitalize(&args[0]).map_err(EvalError::from),
        "strip" => strip(&args[0]).map_err(EvalError::from),
        "split" => split(&args[0], &args[1]).map_err(EvalError::from),
        "join" => join(&args[0], &args[1]).map_err(EvalError::from),
        "truncate" => truncate(&args[0], &args[1]).map_err(EvalError::from),
        "replace" => replace(&args[0], &args[1], &args[2]).map_err(EvalError::from),

        // Numeric functions
        "abs" => abs(&args[0]).map_err(EvalError::from),
        "ceil" => ceil(&args[0]).map_err(EvalError::from),
        "floor" => floor(&args[0]).map_err(EvalError::from),
        "round" => {
            if args.len() == 2 {
                round(&args[0], &args[1]).map_err(EvalError::from)
            } else {
                round(&args[0], &Value::Number(0.0)).map_err(EvalError::from)
            }
        }
        "plus" => plus(&args[0], &args[1]).map_err(EvalError::from),
        "minus" => minus(&args[0], &args[1]).map_err(EvalError::from),
        "times" => times(&args[0], &args[1]).map_err(EvalError::from),
        "divided_by" => divided_by(&args[0], &args[1]).map_err(EvalError::from),
        "modulo" => modulo_fn(&args[0], &args[1]).map_err(EvalError::from),
        "max" => max(&args[0], &args[1]).map_err(EvalError::from),
        "min" => min(&args[0], &args[1]).map_err(EvalError::from),

        // Collection functions
        "size" => size(&args[0]).map_err(EvalError::from),
        "first" => first(&args[0]).map_err(EvalError::from),
        "last" => last(&args[0]).map_err(EvalError::from),
        "contains" => contains(&args[0], &args[1]).map_err(EvalError::from),
        "sum" => sum(&args[0]).map_err(EvalError::from),
        "avg" => avg(&args[0]).map_err(EvalError::from),
        "sort" => sort(&args[0]).map_err(EvalError::from),
        "keys" => keys(&args[0]).map_err(EvalError::from),
        "values" => values(&args[0]).map_err(EvalError::from),
        "reverse" => reverse(&args[0]).map_err(EvalError::from),
        "at" => at(&args[0], &args[1]).map_err(EvalError::from),
        "uniq" => uniq(&args[0]).map_err(EvalError::from),
        "group_by" => group_by(&args[0], &args[1]).map_err(EvalError::from),
        "map" => map(&args[0], &args[1]).map_err(EvalError::from),

        // Logic functions
        "choose" => choose(&args[0], &args[1]).map_err(EvalError::from),
        "if_then_else" => if_then_else(&args[0], &args[1], &args[2]).map_err(EvalError::from),
        "is_number" => Ok(is_number(&args[0])),
        "is_string" => Ok(is_string(&args[0])),
        "is_boolean" => Ok(is_boolean(&args[0])),
        "is_nil" => Ok(is_nil(&args[0])),
        "is_array" => Ok(is_array(&args[0])),
        "is_dictionary" => Ok(is_dictionary(&args[0])),
        "coalesce" => Ok(coalesce(&args[0], &args[1])),
        "default" => Ok(default(&args[0], &args[1])),

        // Financial functions - Time Value of Money
        "pmt" => pmt(&args[0], &args[1], &args[2], &args[3]).map_err(EvalError::from),
        "pv" => pv(&args[0], &args[1], &args[2]).map_err(EvalError::from),
        "fv" => fv(&args[0], &args[1], &args[2], &args[3]).map_err(EvalError::from),
        "nper" => nper(&args[0], &args[1], &args[2]).map_err(EvalError::from),
        "rate" => rate(&args[0], &args[1], &args[2]).map_err(EvalError::from),

        // Financial functions - Investment Analysis
        "npv" => npv(&args[0], &args[1]).map_err(EvalError::from),
        "irr" => irr(&args[0]).map_err(EvalError::from),
        "mirr" => mirr(&args[0], &args[1], &args[2]).map_err(EvalError::from),

        // Financial functions - Depreciation
        "sln" => sln(&args[0], &args[1], &args[2]).map_err(EvalError::from),
        "ddb" => ddb(&args[0], &args[1], &args[2], &args[3]).map_err(EvalError::from),
        "db" => db(&args[0], &args[1], &args[2], &args[3], &args[4]).map_err(EvalError::from),

        // Financial functions - Payment Components
        "ipmt" => ipmt(&args[0], &args[1], &args[2], &args[3], &args[4]).map_err(EvalError::from),
        "ppmt" => ppmt(&args[0], &args[1], &args[2], &args[3], &args[4]).map_err(EvalError::from),
        "cumipmt" => cumipmt(&args[0], &args[1], &args[2], &args[3], &args[4], &args[5])
            .map_err(EvalError::from),
        "cumprinc" => cumprinc(&args[0], &args[1], &args[2], &args[3], &args[4], &args[5])
            .map_err(EvalError::from),

        // Financial functions - Interest Rate Conversion
        "effect" => effect(&args[0], &args[1]).map_err(EvalError::from),
        "nominal" => nominal(&args[0], &args[1]).map_err(EvalError::from),

        // Date functions
        "date_now" => {
            if args.is_empty() {
                date_now().map_err(EvalError::from)
            } else {
                Err(EvalError::TypeError {
                    expected: "0 arguments".to_string(),
                    got: format!("{} arguments", args.len()),
                })
            }
        }
        "date_format" => date_format(&args[0], &args[1]).map_err(EvalError::from),
        "date_trunc" => date_trunc(&args[0]).map_err(EvalError::from),
        "date_parse" => date_parse(&args[0]).map_err(EvalError::from),

        _ => Err(EvalError::TypeError {
            expected: "known function".to_string(),
            got: name.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_and_evaluate_simple() {
        let source = "42";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_compile_and_evaluate_arithmetic() {
        let source = "2 + 3 * 4";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    #[test]
    fn test_compile_and_evaluate_variable() {
        let source = "driver.age";
        let program = compile(source, &[]).unwrap();

        let mut driver = HashMap::new();
        driver.insert("age".to_string(), Value::Number(25.0));

        let mut data = HashMap::new();
        data.insert("driver".to_string(), Value::Dictionary(driver));

        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(25.0));
    }

    #[test]
    fn test_compile_and_evaluate_if() {
        let source = r#"
            if driver.age > 16
                :continue
            else
                :deny
            end
        "#;
        let program = compile(source, &["continue", "deny"]).unwrap();

        let mut driver = HashMap::new();
        driver.insert("age".to_string(), Value::Number(25.0));

        let mut data = HashMap::new();
        data.insert("driver".to_string(), Value::Dictionary(driver));

        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Symbol("continue".to_string()));
    }

    #[test]
    fn test_compile_symbol_validation() {
        let source = ":approve";

        // Should succeed with valid symbol
        let program = compile(source, &["approve", "deny"]);
        assert!(program.is_ok());

        // Should fail with undefined symbol
        let program = compile(source, &["deny"]);
        assert!(program.is_err());
    }

    #[test]
    fn test_compile_function_validation() {
        let source = "upcase('hello')";

        // Should succeed with valid function
        let program = compile(source, &[]);
        assert!(program.is_ok());

        // Invalid function name
        let source = "unknown_function('hello')";
        let program = compile(source, &[]);
        assert!(program.is_err());

        // Invalid arity
        let source = "upcase('hello', 'world')";
        let program = compile(source, &[]);
        assert!(program.is_err());
    }

    #[test]
    fn test_evaluate_let() {
        let source = r#"
            let price = 100
            in price * 1.2
        "#;
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(120.0));
    }

    #[test]
    fn test_evaluate_function_call() {
        let source = "upcase('hello')";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("HELLO".to_string()));
    }

    #[test]
    fn test_evaluate_pipe() {
        let source = "'hello' | upcase";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("HELLO".to_string()));
    }

    #[test]
    fn test_evaluate_pipe_with_args() {
        let source = "'hello world' | truncate(5)";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_safe_navigation() {
        let source = "missing.key.nested";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_division_by_zero() {
        let source = "10 / 0";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data);
        assert!(result.is_err());
    }

    #[test]
    fn test_undefined_function() {
        let source = "undefined_func(1)";
        let result = compile(source, &[]);
        assert!(result.is_err());
        if let Err(CompileError::UndefinedFunction { function }) = result {
            assert_eq!(function, "undefined_func");
        } else {
            panic!("Expected UndefinedFunction error");
        }
    }

    #[test]
    fn test_function_arity_mismatch() {
        let source = "upcase('hello', 'world')";
        let result = compile(source, &[]);
        assert!(result.is_err());
        if let Err(CompileError::ArityMismatch {
            function,
            expected,
            actual,
        }) = result
        {
            assert_eq!(function, "upcase");
            assert_eq!(expected, "1");
            assert_eq!(actual, 2);
        } else {
            panic!("Expected ArityMismatch error");
        }
    }

    #[test]
    fn test_nested_let_bindings() {
        let source = r#"
            let x = 10
            in let y = 20
            in x + y
        "#;
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(30.0));
    }

    #[test]
    fn test_nested_if_expressions() {
        let source = r#"
            if true
                if false
                    1
                else
                    2
                end
            else
                3
            end
        "#;
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_empty_array() {
        let source = "[]";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_empty_dictionary() {
        let source = "{}";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Dictionary(HashMap::new()));
    }

    #[test]
    fn test_nested_arrays() {
        let source = "[[1, 2], [3, 4]]";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
                Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]),
            ])
        );
    }

    #[test]
    fn test_logical_operators() {
        let source = "true and false";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Boolean(false));

        let source = "true or false";
        let program = compile(source, &[]).unwrap();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Boolean(true));

        let source = "not true";
        let program = compile(source, &[]).unwrap();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_comparison_operators() {
        let test_cases = vec![
            ("5 < 10", Value::Boolean(true)),
            ("10 > 5", Value::Boolean(true)),
            ("5 <= 5", Value::Boolean(true)),
            ("5 >= 5", Value::Boolean(true)),
            ("5 == 5", Value::Boolean(true)),
            ("5 != 10", Value::Boolean(true)),
        ];

        for (source, expected) in test_cases {
            let program = compile(source, &[]).unwrap();
            let data = HashMap::new();
            let result = evaluate(&program, &data).unwrap();
            assert_eq!(result, expected, "Failed for: {}", source);
        }
    }

    #[test]
    fn test_string_concatenation() {
        let source = r#""hello" + " " + "world""#;
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_modulo_operation() {
        let source = "10 % 3";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn test_unary_negation() {
        let source = "-5";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    #[test]
    fn test_complex_expression() {
        let source = "(10 + 5) * 2 - 3";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(27.0));
    }

    #[test]
    fn test_nil_in_conditionals() {
        let source = r#"
            if nil
                "truthy"
            else
                "falsy"
            end
        "#;
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("falsy".to_string()));
    }

    #[test]
    fn test_symbol_validation_nested() {
        let source = r#"
            if true
                :approved
            else
                :denied
            end
        "#;

        // Should succeed with both symbols defined
        let program = compile(source, &["approved", "denied"]);
        assert!(program.is_ok());

        // Should fail with only one symbol defined
        let program = compile(source, &["approved"]);
        assert!(program.is_err());
    }

    #[test]
    fn test_deep_nested_dictionary_access() {
        let source = "a.b.c.d.e";
        let program = compile(source, &[]).unwrap();

        let mut level5 = HashMap::new();
        level5.insert("e".to_string(), Value::Number(42.0));

        let mut level4 = HashMap::new();
        level4.insert("d".to_string(), Value::Dictionary(level5));

        let mut level3 = HashMap::new();
        level3.insert("c".to_string(), Value::Dictionary(level4));

        let mut level2 = HashMap::new();
        level2.insert("b".to_string(), Value::Dictionary(level3));

        let mut data = HashMap::new();
        data.insert("a".to_string(), Value::Dictionary(level2));

        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_pipe_chain_multiple() {
        let source = r#""HELLO" | downcase | capitalize"#;
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("Hello".to_string()));
    }

    #[test]
    fn test_let_shadowing() {
        let source = r#"
            let x = 10
            in let x = 20
            in x
        "#;
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    #[test]
    fn test_array_of_mixed_types() {
        let source = r#"[1, "hello", true, nil, :symbol]"#;
        let program = compile(source, &["symbol"]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();

        if let Value::Array(arr) = result {
            assert_eq!(arr.len(), 5);
            assert_eq!(arr[0], Value::Number(1.0));
            assert_eq!(arr[1], Value::String("hello".to_string()));
            assert_eq!(arr[2], Value::Boolean(true));
            assert_eq!(arr[3], Value::Nil);
            assert_eq!(arr[4], Value::Symbol("symbol".to_string()));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_string_functions() {
        let test_cases = vec![
            ("upcase('hello')", Value::String("HELLO".to_string())),
            ("downcase('HELLO')", Value::String("hello".to_string())),
            (
                "capitalize('hello world')",
                Value::String("Hello world".to_string()),
            ),
            ("strip('  hello  ')", Value::String("hello".to_string())),
            (
                "split('a,b,c', ',')",
                Value::Array(vec![
                    Value::String("a".to_string()),
                    Value::String("b".to_string()),
                    Value::String("c".to_string()),
                ]),
            ),
            (
                "join(['a', 'b', 'c'], ',')",
                Value::String("a,b,c".to_string()),
            ),
            (
                "truncate('hello world', 5)",
                Value::String("hello".to_string()),
            ),
            (
                "replace('hello world', 'world', 'rust')",
                Value::String("hello rust".to_string()),
            ),
        ];

        for (source, expected) in test_cases {
            let program = compile(source, &[]).unwrap();
            let data = HashMap::new();
            let result = evaluate(&program, &data).unwrap();
            assert_eq!(result, expected, "Failed for: {}", source);
        }
    }

    #[test]
    fn test_numeric_functions() {
        let test_cases = vec![
            ("abs(-5)", Value::Number(5.0)),
            ("abs(5)", Value::Number(5.0)),
            ("ceil(3.2)", Value::Number(4.0)),
            ("floor(3.8)", Value::Number(3.0)),
            ("round(3.5)", Value::Number(4.0)),
            ("round(2.71828, 2)", Value::Number(2.72)),
            ("plus(2, 3)", Value::Number(5.0)),
            ("minus(5, 3)", Value::Number(2.0)),
            ("times(4, 5)", Value::Number(20.0)),
            ("divided_by(10, 2)", Value::Number(5.0)),
            ("modulo(10, 3)", Value::Number(1.0)),
            ("max(5, 10)", Value::Number(10.0)),
            ("min(5, 10)", Value::Number(5.0)),
        ];

        for (source, expected) in test_cases {
            let program = compile(source, &[]).unwrap();
            let data = HashMap::new();
            let result = evaluate(&program, &data).unwrap();
            assert_eq!(result, expected, "Failed for: {}", source);
        }
    }

    #[test]
    fn test_collection_functions() {
        let test_cases = vec![
            ("size([1, 2, 3])", Value::Number(3.0)),
            ("size({'a': 1, 'b': 2})", Value::Number(2.0)),
            ("first([1, 2, 3])", Value::Number(1.0)),
            ("last([1, 2, 3])", Value::Number(3.0)),
            ("contains([1, 2, 3], 2)", Value::Boolean(true)),
            ("contains([1, 2, 3], 4)", Value::Boolean(false)),
            ("sum([1, 2, 3, 4])", Value::Number(10.0)),
            ("avg([1, 2, 3, 4])", Value::Number(2.5)),
            ("at([10, 20, 30], 1)", Value::Number(20.0)),
            (
                "keys({'a': 1, 'b': 2})",
                Value::Array(vec![
                    Value::String("a".to_string()),
                    Value::String("b".to_string()),
                ]),
            ),
            (
                "uniq([1, 2, 1, 3, 2])",
                Value::Array(vec![
                    Value::Number(1.0),
                    Value::Number(2.0),
                    Value::Number(3.0),
                ]),
            ),
        ];

        for (source, expected) in test_cases {
            let program = compile(source, &[]).unwrap();
            let data = HashMap::new();
            let result = evaluate(&program, &data).unwrap();
            assert_eq!(result, expected, "Failed for: {}", source);
        }
    }

    #[test]
    fn test_logic_functions() {
        let test_cases = vec![
            ("choose(1, ['yes', 'no'])", Value::String("yes".to_string())),
            ("choose(2, ['yes', 'no'])", Value::String("no".to_string())),
            ("if_then_else(true, 1, 2)", Value::Number(1.0)),
            ("if_then_else(false, 1, 2)", Value::Number(2.0)),
            ("is_number(42)", Value::Boolean(true)),
            ("is_number('42')", Value::Boolean(false)),
            ("is_string('hello')", Value::Boolean(true)),
            ("is_string(42)", Value::Boolean(false)),
            ("is_boolean(true)", Value::Boolean(true)),
            ("is_boolean(42)", Value::Boolean(false)),
            ("is_nil(nil)", Value::Boolean(true)),
            ("is_nil(42)", Value::Boolean(false)),
            ("is_array([1, 2])", Value::Boolean(true)),
            ("is_array(42)", Value::Boolean(false)),
            ("is_dictionary({'a': 1})", Value::Boolean(true)),
            ("is_dictionary(42)", Value::Boolean(false)),
            (
                "coalesce(nil, 'default')",
                Value::String("default".to_string()),
            ),
            (
                "coalesce('value', 'default')",
                Value::String("value".to_string()),
            ),
            (
                "default(nil, 'default')",
                Value::String("default".to_string()),
            ),
            (
                "default('value', 'default')",
                Value::String("value".to_string()),
            ),
        ];

        for (source, expected) in test_cases {
            let program = compile(source, &[]).unwrap();
            let data = HashMap::new();
            let result = evaluate(&program, &data).unwrap();
            assert_eq!(result, expected, "Failed for: {}", source);
        }
    }

    #[test]
    fn test_group_by_function() {
        let source = "group_by([{'type': 'a', 'val': 1}, {'type': 'b', 'val': 2}, {'type': 'a', 'val': 3}], 'type')";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();

        match result {
            Value::Dictionary(groups) => {
                assert_eq!(groups.len(), 2);
                assert!(groups.contains_key("a"));
                assert!(groups.contains_key("b"));

                if let Some(Value::Array(a_group)) = groups.get("a") {
                    assert_eq!(a_group.len(), 2);
                }
                if let Some(Value::Array(b_group)) = groups.get("b") {
                    assert_eq!(b_group.len(), 1);
                }
            }
            _ => panic!("Expected Dictionary result"),
        }
    }

    #[test]
    fn test_map_function() {
        let source = "map([{'name': 'alice', 'age': 30}, {'name': 'bob', 'age': 25}], 'name')";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();

        assert_eq!(
            result,
            Value::Array(vec![
                Value::String("alice".to_string()),
                Value::String("bob".to_string()),
            ])
        );
    }

    #[test]
    fn test_financial_functions() {
        // Test PMT function: payment for loan
        // PMT(rate, nper, pv) - rate=5%, nper=12, pv=1000 should be approximately -85.61
        let source = "pmt(0.05/12, 12, 1000, 0)";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        if let Value::Number(payment) = result {
            // Allow small floating point differences
            assert!((payment + 85.61).abs() < 0.01, "PMT result: {}", payment);
        } else {
            panic!("Expected number for PMT");
        }

        // Test PV function: present value
        let source = "pv(0.05/12, 12, -85.61)";
        let program = compile(source, &[]).unwrap();
        let result = evaluate(&program, &data).unwrap();
        if let Value::Number(pv) = result {
            assert!((pv - 1000.0).abs() < 1.0, "PV result: {}", pv);
        } else {
            panic!("Expected number for PV");
        }
    }

    #[test]
    fn test_date_functions() {
        // Test date_now returns a string
        let source = "date_now()";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert!(matches!(result, Value::String(_)));

        // Test date_format
        let source = r#"date_format("2023-01-15", "formatted: %Y-%m-%d")"#;
        let program = compile(source, &[]).unwrap();
        let result = evaluate(&program, &data).unwrap();
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_error_cases() {
        let data = HashMap::new();

        // Type errors in functions
        let error_cases = vec![
            "abs('not a number')",
            "upcase(123)",
            "first('not array')",
            "last('not array')",
            "at('not array', 0)",
            "contains('not array', 1)",
            "choose('not number', [1, 2])",
            "choose(1, 'not array')",
        ];

        for source in error_cases {
            let program = compile(source, &[]).unwrap();
            let result = evaluate(&program, &data);
            assert!(result.is_err(), "Expected error for: {}", source);
        }
    }

    #[test]
    fn test_edge_cases() {
        // Very large numbers
        let source = "999999999999999.999999999999";
        let program = compile(source, &[]).unwrap();
        let data = HashMap::new();
        let result = evaluate(&program, &data).unwrap();
        assert!(matches!(result, Value::Number(_)));

        // Empty strings
        let source = r#"upcase("")"#;
        let program = compile(source, &[]).unwrap();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("".to_string()));

        // Nested function calls
        let source = "upcase(capitalize('hello world'))";
        let program = compile(source, &[]).unwrap();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));

        // Complex pipe chains
        let source = r#""  hello world  " | strip | capitalize | truncate(5)"#;
        let program = compile(source, &[]).unwrap();
        let result = evaluate(&program, &data).unwrap();
        assert_eq!(result, Value::String("Hello".to_string()));
    }
}
