//! Amoskeag to Ruby Transpiler
//!
//! This crate provides functionality to transpile Amoskeag AST expressions
//! into equivalent Ruby code.

use amoskeag_parser::{BinaryOp, Expr, UnaryOp};
use thiserror::Error;

/// Transpiler errors
#[derive(Error, Debug)]
pub enum TranspileError {
    #[error("Unsupported expression type: {0}")]
    UnsupportedExpression(String),

    #[error("Invalid pipe expression")]
    InvalidPipe,
}

/// Transpiler for converting Amoskeag expressions to Ruby code
pub struct RubyTranspiler;

impl RubyTranspiler {
    /// Create a new Ruby transpiler
    pub fn new() -> Self {
        Self
    }

    /// Transpile an Amoskeag expression to Ruby code
    pub fn transpile(&mut self, expr: &Expr) -> Result<String, TranspileError> {
        self.transpile_expr(expr)
    }

    fn transpile_expr(&mut self, expr: &Expr) -> Result<String, TranspileError> {
        match expr {
            Expr::Number(n) => Ok(self.transpile_number(*n)),
            Expr::String(s) => Ok(self.transpile_string(s)),
            Expr::Boolean(b) => Ok(self.transpile_boolean(*b)),
            Expr::Nil => Ok("nil".to_string()),
            Expr::Symbol(s) => Ok(self.transpile_symbol(s)),
            Expr::Array(elements) => self.transpile_array(elements),
            Expr::Dictionary(pairs) => self.transpile_dictionary(pairs),
            Expr::Variable(parts) => Ok(self.transpile_variable(parts)),
            Expr::FunctionCall { name, args } => self.transpile_function_call(name, args),
            Expr::Let { name, value, body } => self.transpile_let(name, value, body),
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => self.transpile_if(condition, then_branch, else_branch),
            Expr::Binary { op, left, right } => self.transpile_binary(*op, left, right),
            Expr::Unary { op, operand } => self.transpile_unary(*op, operand),
            Expr::Pipe { .. } => Err(TranspileError::InvalidPipe),
        }
    }

    fn transpile_number(&self, n: f64) -> String {
        // Handle integers cleanly
        if n.fract() == 0.0 && n.is_finite() {
            format!("{}", n as i64)
        } else {
            format!("{}", n)
        }
    }

    fn transpile_string(&self, s: &str) -> String {
        // Escape special characters for Ruby strings
        let escaped = s
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");
        format!("\"{}\"", escaped)
    }

    fn transpile_boolean(&self, b: bool) -> String {
        if b {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }

    fn transpile_symbol(&self, s: &str) -> String {
        // Ruby symbols start with :
        format!(":{}", s)
    }

    fn transpile_array(&mut self, elements: &[Expr]) -> Result<String, TranspileError> {
        let mut parts = Vec::new();
        for elem in elements {
            parts.push(self.transpile_expr(elem)?);
        }
        Ok(format!("[{}]", parts.join(", ")))
    }

    fn transpile_dictionary(&mut self, pairs: &[(String, Expr)]) -> Result<String, TranspileError> {
        let mut parts = Vec::new();
        for (key, value) in pairs {
            let value_str = self.transpile_expr(value)?;
            // Use symbol key syntax if the key is a valid identifier
            if key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                parts.push(format!("{}: {}", key, value_str));
            } else {
                parts.push(format!("\"{}\": {}", key, value_str));
            }
        }
        Ok(format!("{{{}}}", parts.join(", ")))
    }

    fn transpile_variable(&self, parts: &[String]) -> String {
        // In Ruby, we use method calls for property access
        // driver.age in Amoskeag becomes driver.age in Ruby
        parts.join(".")
    }

    fn transpile_function_call(
        &mut self,
        name: &str,
        args: &[Expr],
    ) -> Result<String, TranspileError> {
        let mut arg_strs = Vec::new();
        for arg in args {
            arg_strs.push(self.transpile_expr(arg)?);
        }

        // Map Amoskeag function names to Ruby equivalents
        let ruby_name = match name {
            "upcase" => "upcase",
            "downcase" => "downcase",
            "truncate" => "truncate",
            "length" => "length",
            "size" => "size",
            "concat" => "concat",
            "join" => "join",
            "split" => "split",
            "map" => "map",
            "filter" => "select",
            "reduce" => "reduce",
            "sum" => "sum",
            "min" => "min",
            "max" => "max",
            "abs" => "abs",
            "round" => "round",
            "floor" => "floor",
            "ceil" => "ceil",
            _ => name, // Keep the original name for unknown functions
        };

        // Ruby method call syntax
        if args.is_empty() {
            Ok(format!("{}()", ruby_name))
        } else {
            Ok(format!("{}({})", ruby_name, arg_strs.join(", ")))
        }
    }

    fn transpile_let(
        &mut self,
        name: &str,
        value: &Expr,
        body: &Expr,
    ) -> Result<String, TranspileError> {
        // In Ruby, we use a lambda with an immediately invoked call
        // or we can use a begin...end block with local variables
        let value_str = self.transpile_expr(value)?;
        let body_str = self.transpile_expr(body)?;

        // Using a lambda for scope isolation
        Ok(format!(
            "lambda {{ |{}| {} }}.call({})",
            name, body_str, value_str
        ))
    }

    fn transpile_if(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<String, TranspileError> {
        let condition_str = self.transpile_expr(condition)?;
        let then_str = self.transpile_expr(then_branch)?;
        let else_str = self.transpile_expr(else_branch)?;

        // Check if branches are simple expressions (no newlines)
        let is_simple = !then_str.contains('\n') && !else_str.contains('\n');

        if is_simple {
            // Use ternary operator for simple expressions
            Ok(format!("{} ? {} : {}", condition_str, then_str, else_str))
        } else {
            // Use if-else-end for complex expressions
            Ok(format!(
                "if {}\n  {}\nelse\n  {}\nend",
                condition_str, then_str, else_str
            ))
        }
    }

    fn transpile_binary(
        &mut self,
        op: BinaryOp,
        left: &Expr,
        right: &Expr,
    ) -> Result<String, TranspileError> {
        let left_str = self.transpile_expr(left)?;
        let right_str = self.transpile_expr(right)?;

        let op_str = match op {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Modulo => "%",
            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "!=",
            BinaryOp::Less => "<",
            BinaryOp::Greater => ">",
            BinaryOp::LessEqual => "<=",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
        };

        // Add parentheses for clarity
        let needs_parens = matches!(
            left,
            Expr::Binary { .. } | Expr::Unary { .. } | Expr::Let { .. } | Expr::If { .. }
        ) || matches!(
            right,
            Expr::Binary { .. } | Expr::Unary { .. } | Expr::Let { .. } | Expr::If { .. }
        );

        if needs_parens {
            Ok(format!("({}) {} ({})", left_str, op_str, right_str))
        } else {
            Ok(format!("{} {} {}", left_str, op_str, right_str))
        }
    }

    fn transpile_unary(&mut self, op: UnaryOp, operand: &Expr) -> Result<String, TranspileError> {
        let operand_str = self.transpile_expr(operand)?;

        let op_str = match op {
            UnaryOp::Not => "!",
            UnaryOp::Negate => "-",
        };

        // Add parentheses if operand is complex
        let needs_parens = !matches!(
            operand,
            Expr::Number(_)
                | Expr::String(_)
                | Expr::Boolean(_)
                | Expr::Nil
                | Expr::Symbol(_)
                | Expr::Variable(_)
        );

        if needs_parens {
            Ok(format!("{}({})", op_str, operand_str))
        } else {
            Ok(format!("{}{}", op_str, operand_str))
        }
    }
}

impl Default for RubyTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to transpile Amoskeag source code to Ruby
pub fn transpile_to_ruby(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let expr = amoskeag_parser::parse(source)?;
    let mut transpiler = RubyTranspiler::new();
    Ok(transpiler.transpile(&expr)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use amoskeag_parser::parse;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_transpile_number() {
        let expr = parse("42").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "42");
    }

    #[test]
    fn test_transpile_float() {
        let expr = parse("3.14").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "3.14");
    }

    #[test]
    fn test_transpile_string() {
        let expr = parse(r#""hello world""#).unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), r#""hello world""#);
    }

    #[test]
    fn test_transpile_boolean() {
        let expr = parse("true").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "true");

        let expr = parse("false").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "false");
    }

    #[test]
    fn test_transpile_nil() {
        let expr = parse("nil").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "nil");
    }

    #[test]
    fn test_transpile_symbol() {
        let expr = parse(":approve").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), ":approve");
    }

    #[test]
    fn test_transpile_array() {
        let expr = parse("[1, 2, 3]").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "[1, 2, 3]");
    }

    #[test]
    fn test_transpile_dictionary() {
        let expr = parse(r#"{ name: "Alice", age: 30 }"#).unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            r#"{name: "Alice", age: 30}"#
        );
    }

    #[test]
    fn test_transpile_variable() {
        let expr = parse("driver").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "driver");
    }

    #[test]
    fn test_transpile_variable_access() {
        let expr = parse("driver.age").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "driver.age");
    }

    #[test]
    fn test_transpile_function_call() {
        let expr = parse("upcase(name)").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "upcase(name)");
    }

    #[test]
    fn test_transpile_binary_op() {
        let expr = parse("1 + 2").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "1 + 2");
    }

    #[test]
    fn test_transpile_comparison() {
        let expr = parse("age > 18").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "age > 18");
    }

    #[test]
    fn test_transpile_logical_and() {
        let expr = parse("age > 18 and age < 65").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            "(age > 18) && (age < 65)"
        );
    }

    #[test]
    fn test_transpile_logical_or() {
        let expr = parse("age < 18 or age > 65").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            "(age < 18) || (age > 65)"
        );
    }

    #[test]
    fn test_transpile_unary_not() {
        let expr = parse("not active").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "!active");
    }

    #[test]
    fn test_transpile_unary_negate() {
        let expr = parse("-5").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "-5");
    }

    #[test]
    fn test_transpile_if_simple() {
        let expr = parse("if age > 18 :adult else :minor end").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            "age > 18 ? :adult : :minor"
        );
    }

    #[test]
    fn test_transpile_let() {
        let expr = parse("let x = 5 in x + 1").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            "lambda { |x| x + 1 }.call(5)"
        );
    }

    #[test]
    fn test_transpile_pipe() {
        let expr = parse("name | upcase").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "upcase(name)");
    }

    #[test]
    fn test_transpile_pipe_with_args() {
        let expr = parse("name | truncate(10)").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(transpiler.transpile(&expr).unwrap(), "truncate(name, 10)");
    }

    #[test]
    fn test_transpile_pipe_chain() {
        let expr = parse("name | downcase | truncate(10)").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            "truncate(downcase(name), 10)"
        );
    }

    #[test]
    fn test_transpile_complex_expression() {
        let source = r#"
            if driver.age > 16
              :continue
            else
              :deny
            end
        "#;
        let expr = parse(source).unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            "driver.age > 16 ? :continue : :deny"
        );
    }

    #[test]
    fn test_transpile_nested_let() {
        let expr = parse("let x = 5 in let y = x + 1 in y * 2").unwrap();
        let mut transpiler = RubyTranspiler::new();
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            "lambda { |x| lambda { |y| y * 2 }.call(x + 1) }.call(5)"
        );
    }

    #[test]
    fn test_transpile_filter_function() {
        let expr = parse("filter(items, is_active)").unwrap();
        let mut transpiler = RubyTranspiler::new();
        // filter maps to select in Ruby
        assert_eq!(
            transpiler.transpile(&expr).unwrap(),
            "select(items, is_active)"
        );
    }
}
