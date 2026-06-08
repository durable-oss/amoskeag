//! power function

use crate::{FunctionError, Value};

/// Raise a number to a power
/// power(base: Number, exponent: Number) -> Number
pub fn power(base: &Value, exponent: &Value) -> Result<Value, FunctionError> {
    match (base, exponent) {
        (Value::Number(b), Value::Number(e)) => Ok(Value::Number(b.powf(*e))),
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: exponent.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: base.type_name().to_string(),
        }),
    }
}
