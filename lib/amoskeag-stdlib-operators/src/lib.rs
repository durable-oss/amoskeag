//! Amoskeag Standard Library - Operators
//!
//! This crate implements the core operators for the Amoskeag language,
//! including arithmetic, comparison, and logical operators.

use std::collections::HashMap;

/// The core Value type for Amoskeag
/// Represents all possible values in the language
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Array(Vec<Value>),
    Dictionary(HashMap<String, Value>),
    Symbol(String),
}

/// Error types for operator operations
#[derive(Debug, Clone, PartialEq)]
pub enum OperatorError {
    TypeError { expected: String, got: String },
    DivisionByZero,
    InvalidOperation { op: String, left: String, right: String },
}

impl std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorError::TypeError { expected, got } => {
                write!(f, "Type error: expected {}, got {}", expected, got)
            }
            OperatorError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            OperatorError::InvalidOperation { op, left, right } => {
                write!(f, "Invalid operation: {} {} {}", left, op, right)
            }
        }
    }
}

impl std::error::Error for OperatorError {}

impl Value {
    /// Get the type name of a value
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Boolean(_) => "Boolean",
            Value::Nil => "Nil",
            Value::Array(_) => "Array",
            Value::Dictionary(_) => "Dictionary",
            Value::Symbol(_) => "Symbol",
        }
    }
}

// Arithmetic Operators

/// Addition operator (+)
pub fn add(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
        (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
        _ => Err(OperatorError::InvalidOperation {
            op: "+".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

/// Subtraction operator (-)
pub fn subtract(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
        _ => Err(OperatorError::InvalidOperation {
            op: "-".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

/// Multiplication operator (*)
pub fn multiply(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
        _ => Err(OperatorError::InvalidOperation {
            op: "*".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

/// Division operator (/)
pub fn divide(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            if *r == 0.0 {
                Err(OperatorError::DivisionByZero)
            } else {
                Ok(Value::Number(l / r))
            }
        }
        _ => Err(OperatorError::InvalidOperation {
            op: "/".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

/// Modulo operator (%)
pub fn modulo(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            if *r == 0.0 {
                Err(OperatorError::DivisionByZero)
            } else {
                Ok(Value::Number(l % r))
            }
        }
        _ => Err(OperatorError::InvalidOperation {
            op: "%".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

// Comparison Operators

/// Equality operator (==)
pub fn equal(left: &Value, right: &Value) -> Value {
    Value::Boolean(left == right)
}

/// Inequality operator (!=)
pub fn not_equal(left: &Value, right: &Value) -> Value {
    Value::Boolean(left != right)
}

/// Less than operator (<)
pub fn less_than(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l < r)),
        _ => Err(OperatorError::InvalidOperation {
            op: "<".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

/// Greater than operator (>)
pub fn greater_than(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l > r)),
        _ => Err(OperatorError::InvalidOperation {
            op: ">".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

/// Less than or equal operator (<=)
pub fn less_than_or_equal(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l <= r)),
        _ => Err(OperatorError::InvalidOperation {
            op: "<=".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

/// Greater than or equal operator (>=)
pub fn greater_than_or_equal(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l >= r)),
        _ => Err(OperatorError::InvalidOperation {
            op: ">=".to_string(),
            left: left.type_name().to_string(),
            right: right.type_name().to_string(),
        }),
    }
}

// Logical Operators

/// Logical AND operator
pub fn logical_and(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    let left_bool = to_boolean(left);
    let right_bool = to_boolean(right);
    Ok(Value::Boolean(left_bool && right_bool))
}

/// Logical OR operator
pub fn logical_or(left: &Value, right: &Value) -> Result<Value, OperatorError> {
    let left_bool = to_boolean(left);
    let right_bool = to_boolean(right);
    Ok(Value::Boolean(left_bool || right_bool))
}

/// Logical NOT operator
pub fn logical_not(value: &Value) -> Value {
    Value::Boolean(!to_boolean(value))
}

/// Convert a value to boolean for logical operations
/// Following common truthiness rules: nil and false are false, everything else is true
fn to_boolean(value: &Value) -> bool {
    match value {
        Value::Boolean(b) => *b,
        Value::Nil => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Arithmetic operator tests
    #[test]
    fn test_add_numbers() {
        let result = add(&Value::Number(2.0), &Value::Number(3.0)).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_add_strings() {
        let result = add(
            &Value::String("hello".to_string()),
            &Value::String(" world".to_string()),
        )
        .unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_subtract() {
        let result = subtract(&Value::Number(5.0), &Value::Number(3.0)).unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_multiply() {
        let result = multiply(&Value::Number(4.0), &Value::Number(3.0)).unwrap();
        assert_eq!(result, Value::Number(12.0));
    }

    #[test]
    fn test_divide() {
        let result = divide(&Value::Number(10.0), &Value::Number(2.0)).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(&Value::Number(10.0), &Value::Number(0.0));
        assert!(matches!(result, Err(OperatorError::DivisionByZero)));
    }

    #[test]
    fn test_modulo() {
        let result = modulo(&Value::Number(10.0), &Value::Number(3.0)).unwrap();
        assert_eq!(result, Value::Number(1.0));
    }

    // Comparison operator tests
    #[test]
    fn test_equal() {
        assert_eq!(
            equal(&Value::Number(5.0), &Value::Number(5.0)),
            Value::Boolean(true)
        );
        assert_eq!(
            equal(&Value::Number(5.0), &Value::Number(3.0)),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_not_equal() {
        assert_eq!(
            not_equal(&Value::Number(5.0), &Value::Number(3.0)),
            Value::Boolean(true)
        );
        assert_eq!(
            not_equal(&Value::Number(5.0), &Value::Number(5.0)),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_less_than() {
        assert_eq!(
            less_than(&Value::Number(3.0), &Value::Number(5.0)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            less_than(&Value::Number(5.0), &Value::Number(3.0)).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_greater_than() {
        assert_eq!(
            greater_than(&Value::Number(5.0), &Value::Number(3.0)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            greater_than(&Value::Number(3.0), &Value::Number(5.0)).unwrap(),
            Value::Boolean(false)
        );
    }

    // Logical operator tests
    #[test]
    fn test_logical_and() {
        assert_eq!(
            logical_and(&Value::Boolean(true), &Value::Boolean(true)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            logical_and(&Value::Boolean(true), &Value::Boolean(false)).unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            logical_and(&Value::Boolean(false), &Value::Boolean(false)).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_logical_or() {
        assert_eq!(
            logical_or(&Value::Boolean(true), &Value::Boolean(false)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            logical_or(&Value::Boolean(false), &Value::Boolean(false)).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_logical_not() {
        assert_eq!(logical_not(&Value::Boolean(true)), Value::Boolean(false));
        assert_eq!(logical_not(&Value::Boolean(false)), Value::Boolean(true));
        assert_eq!(logical_not(&Value::Nil), Value::Boolean(true));
    }

    #[test]
    fn test_to_boolean() {
        assert_eq!(to_boolean(&Value::Boolean(true)), true);
        assert_eq!(to_boolean(&Value::Boolean(false)), false);
        assert_eq!(to_boolean(&Value::Nil), false);
        assert_eq!(to_boolean(&Value::Number(0.0)), true);
        assert_eq!(to_boolean(&Value::String("".to_string())), true);
    }
}
