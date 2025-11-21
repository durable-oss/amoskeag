//! nominal function

use crate::{FunctionError, Value};

/// Calculate the nominal annual interest rate
/// nominal(effect_rate: Number, npery: Number) -> Number
///
/// Formula: nominal_rate = ((1 + effect_rate)^(1/npery) - 1) * npery
/// where npery is the number of compounding periods per year
///
/// Example: nominal(0.053543, 4) = nominal rate for 5.3543% effective rate compounded quarterly
pub fn nominal(effect_rate: &Value, npery: &Value) -> Result<Value, FunctionError> {
    match (effect_rate, npery) {
        (Value::Number(r), Value::Number(n)) => {
            if *n < 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "npery must be at least 1".to_string(),
                });
            }

            if *r <= -1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "effect_rate must be greater than -1".to_string(),
                });
            }

            let nominal = ((1.0 + r).powf(1.0 / n) - 1.0) * n;

            Ok(Value::Number(nominal))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: npery.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: effect_rate.type_name().to_string(),
        }),
    }
}
