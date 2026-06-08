//! array_max function

use crate::{FunctionError, Value};

/// Return the maximum value in an array of numbers
/// array_max(arr: Array) -> Number
pub fn array_max(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(FunctionError::ArgumentError {
                    message: "array_max requires a non-empty array".to_string(),
                });
            }
            let mut max = match &arr[0] {
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
                        if *n > max {
                            max = *n;
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
            Ok(Value::Number(max))
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}
