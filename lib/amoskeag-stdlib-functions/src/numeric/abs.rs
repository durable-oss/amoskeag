//! abs function

use crate::{FunctionError, Value};

/// Return the absolute value of a number
/// abs(num: Number) -> Number
pub fn abs(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}
