//! modulo function

use crate::{FunctionError, Value};

/// Calculate modulo (pipe-friendly version of % operator)
/// modulo(a: Number, b: Number) -> Number
pub fn modulo_fn(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if *y == 0.0 {
                Err(FunctionError::InvalidOperation {
                    message: "Modulo by zero".to_string(),
                })
            } else {
                Ok(Value::Number(x % y))
            }
        }
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
