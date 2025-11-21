//! round function

use crate::{FunctionError, Value};

/// Round a number to a specified number of decimal places
/// round(num: Number, digits: Number) -> Number
pub fn round(value: &Value, digits: &Value) -> Result<Value, FunctionError> {
    match (value, digits) {
        (Value::Number(n), Value::Number(d)) => {
            let decimal_places = (*d).max(0.0) as i32;
            let multiplier = 10_f64.powi(decimal_places);
            let rounded = (n * multiplier).round() / multiplier;
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
