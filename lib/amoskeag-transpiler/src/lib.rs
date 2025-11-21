//! Amoskeag to Rust Transpiler
//!
//! This crate provides functionality to transpile Amoskeag AST into Rust source code.
//! The generated Rust code uses the amoskeag-stdlib-operators and amoskeag-stdlib-functions
//! to maintain semantic equivalence with the interpreted version.

use amoskeag_parser::{BinaryOp, Expr, UnaryOp};
use std::fmt::Write;
use thiserror::Error;

/// Errors that can occur during transpilation
#[derive(Error, Debug)]
pub enum TranspileError {
    #[error("Formatting error: {0}")]
    FormatError(#[from] std::fmt::Error),

    #[error("Unsupported expression: {0}")]
    UnsupportedExpression(String),
}

/// Configuration for the transpiler
#[derive(Debug, Clone)]
pub struct TranspilerConfig {
    /// Generate code with runtime type checking
    pub type_checking: bool,
    /// Add comments to the generated code
    pub add_comments: bool,
    /// Indentation string (default: 4 spaces)
    pub indent: String,
}

impl Default for TranspilerConfig {
    fn default() -> Self {
        Self {
            type_checking: true,
            add_comments: true,
            indent: "    ".to_string(),
        }
    }
}

/// The Amoskeag to Rust transpiler
pub struct Transpiler {
    config: TranspilerConfig,
    indent_level: usize,
}

impl Transpiler {
    /// Create a new transpiler with the default configuration
    pub fn new() -> Self {
        Self {
            config: TranspilerConfig::default(),
            indent_level: 0,
        }
    }

    /// Create a new transpiler with a custom configuration
    pub fn with_config(config: TranspilerConfig) -> Self {
        Self {
            config,
            indent_level: 0,
        }
    }

    /// Transpile an Amoskeag AST to Rust code
    ///
    /// # Arguments
    ///
    /// * `expr` - The root expression to transpile
    ///
    /// # Returns
    ///
    /// A string containing the generated Rust code
    pub fn transpile(&mut self, expr: &Expr) -> Result<String, TranspileError> {
        let mut output = String::new();

        // Add necessary imports
        writeln!(output, "use amoskeag_stdlib_operators::{{Value, *}};")?;
        writeln!(output, "use amoskeag_stdlib_functions::*;")?;
        writeln!(output, "use std::collections::HashMap;")?;
        writeln!(output)?;

        // Generate the main evaluation function
        writeln!(output, "pub fn evaluate(context: &HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {{")?;
        self.indent_level += 1;

        // Generate the expression
        let expr_code = self.transpile_expr(expr)?;
        writeln!(output, "{}Ok({})", self.indent(), expr_code)?;

        self.indent_level -= 1;
        writeln!(output, "}}")?;

        Ok(output)
    }

    /// Get the current indentation string
    fn indent(&self) -> String {
        self.config.indent.repeat(self.indent_level)
    }

    /// Transpile a single expression
    fn transpile_expr(&mut self, expr: &Expr) -> Result<String, TranspileError> {
        match expr {
            Expr::Number(n) => Ok(format!("Value::Number({})", n)),
            Expr::String(s) => Ok(format!("Value::String({:?}.to_string())", s)),
            Expr::Boolean(b) => Ok(format!("Value::Boolean({})", b)),
            Expr::Nil => Ok("Value::Nil".to_string()),
            Expr::Symbol(s) => Ok(format!("Value::Symbol({:?}.to_string())", s)),

            Expr::Array(exprs) => self.transpile_array(exprs),
            Expr::Dictionary(pairs) => self.transpile_dictionary(pairs),
            Expr::Variable(path) => self.transpile_variable(path),
            Expr::FunctionCall { name, args } => self.transpile_function_call(name, args),
            Expr::Let { name, value, body } => self.transpile_let(name, value, body),
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => self.transpile_if(condition, then_branch, else_branch),
            Expr::Binary { op, left, right } => self.transpile_binary(*op, left, right),
            Expr::Unary { op, operand } => self.transpile_unary(*op, operand),
            Expr::Pipe { left, right } => self.transpile_pipe(left, right),
        }
    }

    /// Transpile an array literal
    fn transpile_array(&mut self, exprs: &[Expr]) -> Result<String, TranspileError> {
        let mut items = Vec::new();
        for expr in exprs {
            items.push(self.transpile_expr(expr)?);
        }
        Ok(format!("Value::Array(vec![{}])", items.join(", ")))
    }

    /// Transpile a dictionary literal
    fn transpile_dictionary(&mut self, pairs: &[(String, Expr)]) -> Result<String, TranspileError> {
        let mut output = String::new();
        write!(output, "{{")?;
        write!(output, "let mut map = HashMap::new();")?;

        for (key, value) in pairs {
            let value_code = self.transpile_expr(value)?;
            write!(
                output,
                " map.insert({:?}.to_string(), {});",
                key, value_code
            )?;
        }

        write!(output, " Value::Dictionary(map)")?;
        write!(output, "}}")?;

        Ok(output)
    }

    /// Transpile a variable access (with dot navigation)
    fn transpile_variable(&mut self, path: &[String]) -> Result<String, TranspileError> {
        if path.is_empty() {
            return Ok("Value::Nil".to_string());
        }

        let mut output = String::new();
        write!(output, "{{")?;

        // Look up the root variable
        write!(
            output,
            "let mut current = context.get({:?}).cloned().unwrap_or(Value::Nil);",
            path[0]
        )?;

        // Navigate the path
        for key in &path[1..] {
            write!(
                output,
                " current = match current {{ Value::Dictionary(ref map) => map.get({:?}).cloned().unwrap_or(Value::Nil), _ => Value::Nil }};",
                key
            )?;
        }

        write!(output, " current")?;
        write!(output, "}}")?;

        Ok(output)
    }

    /// Transpile a function call
    fn transpile_function_call(
        &mut self,
        name: &str,
        args: &[Expr],
    ) -> Result<String, TranspileError> {
        let mut arg_codes = Vec::new();
        for arg in args {
            arg_codes.push(self.transpile_expr(arg)?);
        }

        let args_str = arg_codes.join(", ");

        // Map to the corresponding stdlib function
        let result = match name {
            // Most functions match directly
            "upcase" | "downcase" | "capitalize" | "strip" | "split" | "join" | "truncate"
            | "replace" | "abs" | "ceil" | "floor" | "round" | "plus" | "minus" | "times"
            | "divided_by" | "max" | "min" | "size" | "first" | "last" | "contains" | "sum"
            | "avg" | "sort" | "keys" | "values" | "reverse" | "at" | "choose" | "if_then_else"
            | "is_number" | "is_string" | "is_boolean" | "is_nil" | "is_array"
            | "is_dictionary" | "coalesce" | "default" => {
                format!("{}(&{})?", name, args_str)
            }

            // Special case for modulo (function name vs operator)
            "modulo" => {
                format!("modulo_fn(&{})?", args_str)
            }

            _ => {
                return Err(TranspileError::UnsupportedExpression(format!(
                    "Unknown function: {}",
                    name
                )));
            }
        };

        Ok(result)
    }

    /// Transpile a let binding
    fn transpile_let(
        &mut self,
        name: &str,
        value: &Expr,
        body: &Expr,
    ) -> Result<String, TranspileError> {
        let value_code = self.transpile_expr(value)?;

        let mut output = String::new();
        write!(output, "{{")?;
        write!(output, " let mut new_context = context.clone();")?;
        write!(
            output,
            " new_context.insert({:?}.to_string(), {});",
            name, value_code
        )?;

        // Temporarily use new_context for the body
        let old_body = self.transpile_expr(body)?;
        // Replace context references in body with new_context
        let body_code = old_body.replace("context", "&new_context");

        write!(output, " {}", body_code)?;
        write!(output, " }}")?;

        Ok(output)
    }

    /// Transpile an if expression
    fn transpile_if(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<String, TranspileError> {
        let cond_code = self.transpile_expr(condition)?;
        let then_code = self.transpile_expr(then_branch)?;
        let else_code = self.transpile_expr(else_branch)?;

        let mut output = String::new();
        write!(output, "{{")?;
        write!(output, " let cond_value = {};", cond_code)?;
        write!(output, " let is_truthy = match cond_value {{ Value::Boolean(b) => b, Value::Nil => false, _ => true }};")?;
        write!(
            output,
            " if is_truthy {{ {} }} else {{ {} }}",
            then_code, else_code
        )?;
        write!(output, " }}")?;

        Ok(output)
    }

    /// Transpile a binary operation
    fn transpile_binary(
        &mut self,
        op: BinaryOp,
        left: &Expr,
        right: &Expr,
    ) -> Result<String, TranspileError> {
        let left_code = self.transpile_expr(left)?;
        let right_code = self.transpile_expr(right)?;

        let op_fn = match op {
            BinaryOp::Add => "add",
            BinaryOp::Subtract => "subtract",
            BinaryOp::Multiply => "multiply",
            BinaryOp::Divide => "divide",
            BinaryOp::Modulo => "modulo",
            BinaryOp::Equal => "equal",
            BinaryOp::NotEqual => "not_equal",
            BinaryOp::Less => "less_than",
            BinaryOp::Greater => "greater_than",
            BinaryOp::LessEqual => "less_than_or_equal",
            BinaryOp::GreaterEqual => "greater_than_or_equal",
            BinaryOp::And => "logical_and",
            BinaryOp::Or => "logical_or",
        };

        // Equal and not_equal don't return Result
        if matches!(op, BinaryOp::Equal | BinaryOp::NotEqual) {
            Ok(format!("{}(&{}, &{})", op_fn, left_code, right_code))
        } else {
            Ok(format!("{}(&{}, &{})?", op_fn, left_code, right_code))
        }
    }

    /// Transpile a unary operation
    fn transpile_unary(&mut self, op: UnaryOp, operand: &Expr) -> Result<String, TranspileError> {
        let operand_code = self.transpile_expr(operand)?;

        match op {
            UnaryOp::Not => Ok(format!("logical_not(&{})", operand_code)),
            UnaryOp::Negate => {
                let mut output = String::new();
                write!(output, "{{")?;
                write!(output, " let val = {};", operand_code)?;
                write!(output, " match val {{ Value::Number(n) => Value::Number(-n), _ => return Err(\"Type error: expected Number\".into()) }}")?;
                write!(output, " }}")?;
                Ok(output)
            }
        }
    }

    /// Transpile a pipe expression
    fn transpile_pipe(&mut self, left: &Expr, right: &Expr) -> Result<String, TranspileError> {
        let left_code = self.transpile_expr(left)?;

        match right {
            Expr::FunctionCall { name, args } => {
                // Prepend the left value as the first argument
                let mut all_args = vec![left_code];
                for arg in args {
                    all_args.push(self.transpile_expr(arg)?);
                }

                let args_str = all_args.join(", ");

                // Map to the corresponding stdlib function
                let result = match name.as_str() {
                    "upcase" | "downcase" | "capitalize" | "strip" | "split" | "join"
                    | "truncate" | "replace" | "abs" | "ceil" | "floor" | "round" | "plus"
                    | "minus" | "times" | "divided_by" | "max" | "min" | "size" | "first"
                    | "last" | "contains" | "sum" | "avg" | "sort" | "keys" | "values"
                    | "reverse" | "at" | "choose" | "if_then_else" | "is_number" | "is_string"
                    | "is_boolean" | "is_nil" | "is_array" | "is_dictionary" | "coalesce"
                    | "default" => {
                        format!("{}(&{})?", name, args_str)
                    }
                    "modulo" => {
                        format!("modulo_fn(&{})?", args_str)
                    }
                    _ => {
                        return Err(TranspileError::UnsupportedExpression(format!(
                            "Unknown function in pipe: {}",
                            name
                        )));
                    }
                };

                Ok(result)
            }
            Expr::Variable(path) if path.len() == 1 => {
                // Simple function name without args
                let name = &path[0];
                let result = match name.as_str() {
                    "upcase" | "downcase" | "capitalize" | "strip" | "size" | "first" | "last"
                    | "sum" | "avg" | "sort" | "keys" | "values" | "reverse" | "is_number"
                    | "is_string" | "is_boolean" | "is_nil" | "is_array" | "is_dictionary" => {
                        format!("{}(&{})?", name, left_code)
                    }
                    _ => {
                        return Err(TranspileError::UnsupportedExpression(format!(
                            "Unknown function in pipe: {}",
                            name
                        )));
                    }
                };
                Ok(result)
            }
            _ => Err(TranspileError::UnsupportedExpression(
                "Pipe target must be a function call or function name".to_string(),
            )),
        }
    }
}

impl Default for Transpiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpile_number() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::Number(42.0);
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("Value::Number(42)"));
    }

    #[test]
    fn test_transpile_string() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::String("hello".to_string());
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("Value::String"));
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_transpile_boolean() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::Boolean(true);
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("Value::Boolean(true)"));
    }

    #[test]
    fn test_transpile_nil() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::Nil;
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("Value::Nil"));
    }

    #[test]
    fn test_transpile_symbol() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::Symbol("approve".to_string());
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("Value::Symbol"));
        assert!(result.contains("approve"));
    }

    #[test]
    fn test_transpile_array() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::Array(vec![
            Expr::Number(1.0),
            Expr::Number(2.0),
            Expr::Number(3.0),
        ]);
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("Value::Array"));
        assert!(result.contains("vec!"));
    }

    #[test]
    fn test_transpile_variable() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::Variable(vec!["driver".to_string(), "age".to_string()]);
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("driver"));
        assert!(result.contains("age"));
    }

    #[test]
    fn test_transpile_function_call() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::FunctionCall {
            name: "upcase".to_string(),
            args: vec![Expr::String("hello".to_string())],
        };
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("upcase"));
    }

    #[test]
    fn test_transpile_binary_op() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::Number(3.0)),
        };
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("add"));
    }

    #[test]
    fn test_transpile_if() {
        let mut transpiler = Transpiler::new();
        let expr = Expr::If {
            condition: Box::new(Expr::Boolean(true)),
            then_branch: Box::new(Expr::Number(1.0)),
            else_branch: Box::new(Expr::Number(2.0)),
        };
        let result = transpiler.transpile(&expr).unwrap();
        assert!(result.contains("is_truthy"));
    }
}
