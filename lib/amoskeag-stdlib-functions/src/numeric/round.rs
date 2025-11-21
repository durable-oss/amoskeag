//! round function

use crate::{FunctionError, Value};

/// Round a number to a specified number of decimal places
/// round(num: Number, digits: Number) -> Number
pub fn round(value: &Value, digits: &Value) -> Result<Value, FunctionError> {
    match (value, digits) {
        (Value::Number(n), Value::Number(d)) => {
            let decimal_places = if d.is_finite() {
                (*d as i32).clamp(-20, 20)
            } else {
                return Err(FunctionError::ArgumentError {
                    message: "digits must be finite".to_string(),
                });
            };
            let multiplier = 10_f64.powi(decimal_places.abs());
            let rounded = if decimal_places >= 0 {
                (n * multiplier).round() / multiplier
            } else {
                // For negative decimal places, round to tens, hundreds, etc.
                (n / multiplier).round() * multiplier
            };
            Ok(Value::Number(rounded))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: digits.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}
