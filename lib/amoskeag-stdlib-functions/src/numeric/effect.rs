//! effect function

use crate::{FunctionError, Value};

/// Calculate the effective annual interest rate
/// effect(nominal_rate: Number, npery: Number) -> Number
///
/// Formula: effective_rate = (1 + nominal_rate/npery)^npery - 1
/// where npery is the number of compounding periods per year
///
/// Example: effect(0.0525, 4) = effective rate of 5.25% nominal compounded quarterly
pub fn effect(nominal_rate: &Value, npery: &Value) -> Result<Value, FunctionError> {
    match (nominal_rate, npery) {
        (Value::Number(r), Value::Number(n)) => {
            if *n < 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "npery must be at least 1".to_string(),
                });
            }

            if *r <= -1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nominal_rate must be greater than -1".to_string(),
                });
            }

            let effective = (1.0 + r / n).powf(*n) - 1.0;

            Ok(Value::Number(effective))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: npery.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: nominal_rate.type_name().to_string(),
        }),
    }
}
