//! pmt function

use crate::{FunctionError, Value};

/// Calculate the payment for a loan based on constant payments and a constant interest rate
/// pmt(rate: Number, nper: Number, pv: Number, type_: Number) -> Number
///
/// type_: 0 = payment at end of period, 1 = payment at beginning of period
///
/// Formula: PMT = PV * (r * (1 + r)^n) / ((1 + r)^n - 1)
/// For type 1, PMT = PMT_type0 / (1 + r)
/// where r = periodic interest rate, n = number of periods, PV = present value
///
/// Example: pmt(0.00375, 360, 250000, 0) = -1266.71 (monthly payment on $250k loan at 4.5% APR for 30 years)
pub fn pmt(rate: &Value, nper: &Value, pv: &Value, type_: &Value) -> Result<Value, FunctionError> {
    match (rate, nper, pv, type_) {
        (Value::Number(r), Value::Number(n), Value::Number(p), Value::Number(t)) => {
            if *n <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nper must be greater than 0".to_string(),
                });
            }

            if *t != 0.0 && *t != 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "type must be 0 or 1".to_string(),
                });
            }

            // Handle zero interest rate case
            if *r == 0.0 {
                let payment = -*p / *n;
                return Ok(Value::Number(if *t == 1.0 { payment / (1.0 + *r) } else { payment }));
            }

            // Standard formula: PMT = PV * (r * (1 + r)^n) / ((1 + r)^n - 1)
            let factor = (1.0 + *r).powf(*n);
            let payment = -*p * (*r * factor) / (factor - 1.0);

            // For type 1, adjust payment
            let adjusted_payment = if *t == 1.0 { payment / (1.0 + *r) } else { payment };

            Ok(Value::Number(adjusted_payment))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: type_.type_name().to_string(),
        }),
        (Value::Number(_), Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pv.type_name().to_string(),
        }),
        (Value::Number(_), _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: nper.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: rate.type_name().to_string(),
        }),
    }
}
