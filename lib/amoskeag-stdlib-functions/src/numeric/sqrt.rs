//! sqrt function

use crate::{FunctionError, Value};

/// Calculate the square root of a number
/// sqrt(num: Number) -> Number
pub fn sqrt(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => {
            if *n < 0.0 {
                Err(FunctionError::ValueError {
                    message: format!("Cannot take square root of negative number: {}", n),
                })
            } else {
                Ok(Value::Number(n.sqrt()))
            }
        }
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}
