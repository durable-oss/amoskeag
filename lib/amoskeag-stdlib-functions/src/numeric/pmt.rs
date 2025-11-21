//! pmt function

use crate::{FunctionError, Value};

/// Calculate the payment for a loan based on constant payments and a constant interest rate
/// pmt(rate: Number, nper: Number, pv: Number) -> Number
///
/// Formula: PMT = PV * (r * (1 + r)^n) / ((1 + r)^n - 1)
/// where r = periodic interest rate, n = number of periods, PV = present value
///
/// Example: pmt(0.00375, 360, 250000) = -1266.71 (monthly payment on $250k loan at 4.5% APR for 30 years)
pub fn pmt(rate: &Value, nper: &Value, pv: &Value) -> Result<Value, FunctionError> {
    match (rate, nper, pv) {
        (Value::Number(r), Value::Number(n), Value::Number(p)) => {
            if *n <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nper must be greater than 0".to_string(),
                });
            }

            // Handle zero interest rate case
            if *r == 0.0 {
                return Ok(Value::Number(-p / n));
            }

            // Standard formula: PMT = PV * (r * (1 + r)^n) / ((1 + r)^n - 1)
            let factor = (1.0 + r).powf(*n);
            let payment = -p * (r * factor) / (factor - 1.0);

            Ok(Value::Number(payment))
        }
        (Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pv.type_name().to_string(),
        }),
        (Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: nper.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: rate.type_name().to_string(),
        }),
    }
}
