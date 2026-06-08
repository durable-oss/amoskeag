//! array_min function

use crate::{FunctionError, Value};

/// Return the minimum value in an array of numbers
/// array_min(arr: Array) -> Number
pub fn array_min(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(FunctionError::ArgumentError {
                    message: "array_min requires a non-empty array".to_string(),
                });
            }
            let mut min = match &arr[0] {
                Value::Number(n) => *n,
                other => {
                    return Err(FunctionError::TypeError {
                        expected: "Number".to_string(),
                        got: other.type_name().to_string(),
                    })
                }
            };
            for item in &arr[1..] {
                match item {
                    Value::Number(n) => {
                        if *n < min {
                            min = *n;
                        }
                    }
                    other => {
                        return Err(FunctionError::TypeError {
                            expected: "Number".to_string(),
                            got: other.type_name().to_string(),
                        })
                    }
                }
            }
            Ok(Value::Number(min))
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}
