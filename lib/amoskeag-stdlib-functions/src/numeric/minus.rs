//! minus function

use crate::{FunctionError, Value};

/// Subtract two numbers (pipe-friendly version of - operator)
/// minus(a: Number, b: Number) -> Number
pub fn minus(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: b.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: a.type_name().to_string(),
        }),
    }
}
