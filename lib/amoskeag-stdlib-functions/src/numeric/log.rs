//! log, log10, and ln functions

use crate::{FunctionError, Value};

/// Calculate the base-2 logarithm of a number
/// log(num: Number) -> Number
pub fn log(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => {
            if *n <= 0.0 {
                Err(FunctionError::ValueError {
                    message: format!("Cannot take logarithm of non-positive number: {}", n),
                })
            } else {
                Ok(Value::Number(n.log2()))
            }
        }
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Calculate the base-10 logarithm of a number
/// log10(num: Number) -> Number
pub fn log10(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => {
            if *n <= 0.0 {
                Err(FunctionError::ValueError {
                    message: format!("Cannot take logarithm of non-positive number: {}", n),
                })
            } else {
                Ok(Value::Number(n.log10()))
            }
        }
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Calculate the natural logarithm (base-e) of a number
/// ln(num: Number) -> Number
pub fn ln(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => {
            if *n <= 0.0 {
                Err(FunctionError::ValueError {
                    message: format!("Cannot take logarithm of non-positive number: {}", n),
                })
            } else {
                Ok(Value::Number(n.ln()))
            }
        }
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}
