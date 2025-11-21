//! mirr function

use crate::{FunctionError, Value};

/// Calculate modified internal rate of return
/// mirr(values: Array, finance_rate: Number, reinvest_rate: Number) -> Number
///
/// MIRR improves on IRR by assuming positive cash flows are reinvested at the reinvestment rate
/// and negative cash flows are financed at the financing rate
///
/// Example: mirr([-10000, 3000, 4200, 6800], 0.1, 0.12) = modified IRR
pub fn mirr(
    values: &Value,
    finance_rate: &Value,
    reinvest_rate: &Value,
) -> Result<Value, FunctionError> {
    match (values, finance_rate, reinvest_rate) {
        (Value::Array(arr), Value::Number(fr), Value::Number(rr)) => {
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

            let n = cash_flows.len() as f64;

            // Calculate present value of negative cash flows (financed at finance_rate)
            let mut pv_negative = 0.0;
            for (i, &cf) in cash_flows.iter().enumerate() {
                if cf < 0.0 {
                    let period = i as f64;
                    pv_negative += cf / (1.0 + fr).powf(period);
                }
            }

            // Calculate future value of positive cash flows (reinvested at reinvest_rate)
            let mut fv_positive = 0.0;
            for (i, &cf) in cash_flows.iter().enumerate() {
                if cf > 0.0 {
                    let period = n - 1.0 - i as f64;
                    fv_positive += cf * (1.0 + rr).powf(period);
                }
            }

            if pv_negative == 0.0 || fv_positive == 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "cash flows must contain both positive and negative values"
                        .to_string(),
                });
            }

            // MIRR = (FV_positive / -PV_negative)^(1/(n-1)) - 1
            let mirr = (fv_positive / -pv_negative).powf(1.0 / (n - 1.0)) - 1.0;

            Ok(Value::Number(mirr))
        }
        (Value::Array(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: reinvest_rate.type_name().to_string(),
        }),
        (Value::Array(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: finance_rate.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: values.type_name().to_string(),
        }),
    }
}
