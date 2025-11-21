//! min function

use crate::{FunctionError, Value};

/// Calculate the minimum of two numbers
/// min(a: Number, b: Number) -> Number
pub fn min(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x.min(*y))),
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
