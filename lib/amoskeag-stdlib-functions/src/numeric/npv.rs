//! npv function

use crate::{FunctionError, Value};

/// Calculate net present value of a series of cash flows
/// npv(rate: Number, values: Array) -> Number
///
/// Formula: NPV = sum(values[i] / (1 + rate)^(i+1))
/// where i goes from 0 to n-1
///
/// Note: The first value is at period 1, not period 0. Add initial investment separately.
///
/// Example: npv(0.1, [-10000, 3000, 4200, 6800]) = net present value at 10% discount rate
pub fn npv(rate: &Value, values: &Value) -> Result<Value, FunctionError> {
    match (rate, values) {
        (Value::Number(r), Value::Array(arr)) => {
            if arr.is_empty() {
                return Err(FunctionError::ArgumentError {
                    message: "values array cannot be empty".to_string(),
                });
            }

            let mut npv = 0.0;
            for (i, value) in arr.iter().enumerate() {
                match value {
                    Value::Number(v) => {
                        let period = (i + 1) as f64;
                        npv += v / (1.0 + r).powf(period);
                    }
                    _ => {
                        return Err(FunctionError::TypeError {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {}", value.type_name()),
                        })
                    }
                }
            }

            Ok(Value::Number(npv))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: values.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: rate.type_name().to_string(),
        }),
    }
}
