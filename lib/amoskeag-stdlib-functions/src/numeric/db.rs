//! db function

use crate::{FunctionError, Value};

/// Calculate declining balance depreciation
/// db(cost: Number, salvage: Number, life: Number, period: Number, month: Number) -> Number
///
/// Returns the depreciation of an asset for a specified period using the fixed-declining balance method
///
/// Example: db(1000000, 100000, 6, 1, 7) = depreciation for first year with 7 months in first year
pub fn db(cost: &Value, salvage: &Value, life: &Value, period: &Value, month: &Value) -> Result<Value, FunctionError> {
    match (cost, salvage, life, period, month) {
        (Value::Number(c), Value::Number(s), Value::Number(l), Value::Number(p), Value::Number(m)) => {
            if *l <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "life must be greater than 0".to_string(),
                });
            }

            if *p < 1.0 || *p > *l + 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: format!("period must be between 1 and {}", l + 1.0),
                });
            }

            if *m < 1.0 || *m > 12.0 {
                return Err(FunctionError::ArgumentError {
                    message: "month must be between 1 and 12".to_string(),
                });
            }

            if *s >= *c {
                return Ok(Value::Number(0.0));
            }

            // Calculate the fixed rate
            let rate = 1.0 - (s / c).powf(1.0 / l);
            let rate = (rate * 1000.0).round() / 1000.0; // Round to 3 decimal places

            let depreciation;

            // First period (partial year if month != 12)
            if *p == 1.0 {
                depreciation = c * rate * m / 12.0;
            } else {
                // Calculate depreciation for previous periods
                let mut temp_total = 0.0;

                // First period
                let first_depr = c * rate * m / 12.0;
                temp_total += first_depr;

                // Full years
                for _i in 2..(*p as i32) {
                    let depr = (c - temp_total) * rate;
                    temp_total += depr;
                }

                // Current period
                if *p < *l + 1.0 {
                    depreciation = (c - temp_total) * rate;
                } else {
                    // Last period (partial year)
                    depreciation = (c - temp_total) * rate * (12.0 - m) / 12.0;
                }
            }

            Ok(Value::Number(depreciation))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), Value::Number(_), _) => {
            Err(FunctionError::TypeError {
                expected: "Number".to_string(),
                got: month.type_name().to_string(),
            })
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: period.type_name().to_string(),
        }),
        (Value::Number(_), Value::Number(_), _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: life.type_name().to_string(),
        }),
        (Value::Number(_), _, _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: salvage.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: cost.type_name().to_string(),
        }),
    }
}
