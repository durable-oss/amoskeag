//! ddb function

use crate::{FunctionError, Value};

/// Calculate double-declining balance depreciation
/// ddb(cost: Number, salvage: Number, life: Number, period: Number) -> Number
///
/// Uses declining balance method with double the straight-line rate
///
/// Example: ddb(30000, 7500, 10, 1) = depreciation for first year
pub fn ddb(
    cost: &Value,
    salvage: &Value,
    life: &Value,
    period: &Value,
) -> Result<Value, FunctionError> {
    match (cost, salvage, life, period) {
        (Value::Number(c), Value::Number(s), Value::Number(l), Value::Number(p)) => {
            if *l <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "life must be greater than 0".to_string(),
                });
            }

            if *p < 1.0 || *p > *l {
                return Err(FunctionError::ArgumentError {
                    message: format!("period must be between 1 and {}", l),
                });
            }

            if *p != (*p as i32 as f64) {
                return Err(FunctionError::ArgumentError {
                    message: "period must be an integer".to_string(),
                });
            }

            let rate = 2.0 / l; // Double declining rate
            let mut book_value = *c;
            let mut depreciation = 0.0;

            for i in 1..=(*p as i32) {
                depreciation = book_value * rate;

                // Don't depreciate below salvage value
                if book_value - depreciation < *s {
                    depreciation = book_value - s;
                }

                book_value -= depreciation;

                if i == *p as i32 {
                    break;
                }
            }

            Ok(Value::Number(depreciation.max(0.0)))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _) => {
            Err(FunctionError::TypeError {
                expected: "Number".to_string(),
                got: period.type_name().to_string(),
            })
        }
        (Value::Number(_), Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: life.type_name().to_string(),
        }),
        (Value::Number(_), _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: salvage.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: cost.type_name().to_string(),
        }),
    }
}
