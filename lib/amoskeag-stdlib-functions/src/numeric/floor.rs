//! floor function

use crate::{FunctionError, Value};

/// Round a number down to the nearest integer
/// floor(num: Number) -> Number
pub fn floor(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}
