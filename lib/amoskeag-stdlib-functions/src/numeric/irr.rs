//! irr function

use crate::{FunctionError, Value};

/// Calculate internal rate of return for a series of cash flows
/// irr(values: Array) -> Number
///
/// Uses Newton-Raphson method to find the rate where NPV = 0
/// The first value is typically a negative investment, followed by positive returns
///
/// Example: irr([-10000, 3000, 4200, 6800]) = internal rate of return
pub fn irr(values: &Value) -> Result<Value, FunctionError> {
    match values {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(FunctionError::ArgumentError {
                    message: "values array cannot be empty".to_string(),
                });
            }

            // Convert to f64 array and validate
            let cash_flows: Result<Vec<f64>, _> = arr
                .iter()
                .map(|v| match v {
                    Value::Number(n) => Ok(*n),
                    _ => Err(FunctionError::TypeError {
                        expected: "Array of Numbers".to_string(),
                        got: format!("Array containing {}", v.type_name()),
                    }),
                })
                .collect();
            let cash_flows = cash_flows?;

            // Check for at least one positive and one negative value
            let has_positive = cash_flows.iter().any(|&v| v > 0.0);
            let has_negative = cash_flows.iter().any(|&v| v < 0.0);

            if !has_positive || !has_negative {
                return Err(FunctionError::ArgumentError {
                    message: "cash flows must contain both positive and negative values".to_string(),
                });
            }

            // Newton-Raphson method to find rate where NPV = 0
            let mut rate = 0.1; // Initial guess: 10%
            let tolerance = 1e-6;
            let max_iterations = 100;

            for _ in 0..max_iterations {
                let mut npv = 0.0;
                let mut dnpv = 0.0;

                for (i, &cf) in cash_flows.iter().enumerate() {
                    let period = i as f64;
                    let factor = (1.0_f64 + rate).powf(period);
                    npv += cf / factor;
                    dnpv -= period * cf / ((1.0 + rate) * factor);
                }

                if dnpv.abs() < tolerance {
                    return Err(FunctionError::ArgumentError {
                        message: "IRR calculation did not converge".to_string(),
                    });
                }

                let new_rate = rate - npv / dnpv;

                if (new_rate - rate).abs() < tolerance {
                    return Ok(Value::Number(new_rate));
                }

                rate = new_rate;

                // Ensure rate stays in reasonable bounds
                if !(-0.99..=100.0).contains(&rate) {
                    return Err(FunctionError::ArgumentError {
                        message: "IRR calculation did not converge to a reasonable value".to_string(),
                    });
                }
            }

            Err(FunctionError::ArgumentError {
                message: "IRR calculation exceeded maximum iterations".to_string(),
            })
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: values.type_name().to_string(),
        }),
    }
}
