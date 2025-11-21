//! ceil function

use crate::{FunctionError, Value};

/// Round a number up to the nearest integer
/// ceil(num: Number) -> Number
pub fn ceil(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}
