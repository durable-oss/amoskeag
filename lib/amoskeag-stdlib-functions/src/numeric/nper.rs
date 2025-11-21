//! nper function

use crate::{FunctionError, Value};

/// Calculate the number of periods for an investment
/// nper(rate: Number, pmt: Number, pv: Number) -> Number
///
/// Formula: NPER = log(PMT / (PMT + PV * r)) / log(1 + r)
/// where r = periodic interest rate, PMT = payment per period, PV = present value
///
/// Example: nper(0.075/12, -200, 8000) = number of months to pay off $8000 at 7.5% with $200/month payments
pub fn nper(rate: &Value, pmt: &Value, pv: &Value) -> Result<Value, FunctionError> {
    match (rate, pmt, pv) {
        (Value::Number(r), Value::Number(p), Value::Number(v)) => {
            // Handle zero interest rate case
            if *r == 0.0 {
                if *p == 0.0 {
                    return Err(FunctionError::ArgumentError {
                        message: "payment cannot be zero when rate is zero".to_string(),
                    });
                }
                return Ok(Value::Number(-v / p));
            }

            // Check for valid parameters
            let denominator = p + v * r;
            if denominator == 0.0 || p / denominator <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "invalid payment or present value for given rate".to_string(),
                });
            }

            // Formula: NPER = log(PMT / (PMT + PV * r)) / log(1 + r)
            let num_periods = (p / denominator).ln() / (1.0 + r).ln();

            Ok(Value::Number(num_periods))
        }
        (Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pv.type_name().to_string(),
        }),
        (Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pmt.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: rate.type_name().to_string(),
        }),
    }
}
