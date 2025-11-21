//! sln function

use crate::{FunctionError, Value};

/// Calculate straight-line depreciation
/// sln(cost: Number, salvage: Number, life: Number) -> Number
///
/// Formula: SLN = (cost - salvage) / life
///
/// Example: sln(30000, 7500, 10) = 2250 (annual depreciation for 10 years)
pub fn sln(cost: &Value, salvage: &Value, life: &Value) -> Result<Value, FunctionError> {
    match (cost, salvage, life) {
        (Value::Number(c), Value::Number(s), Value::Number(l)) => {
            if *l <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "life must be greater than 0".to_string(),
                });
            }

            let depreciation = (c - s) / l;
            Ok(Value::Number(depreciation))
        }
        (Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: life.type_name().to_string(),
        }),
        (Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: salvage.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: cost.type_name().to_string(),
        }),
    }
}
