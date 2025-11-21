//! pv function

use crate::{FunctionError, Value};

/// Calculate the present value of an investment
/// pv(rate: Number, nper: Number, pmt: Number) -> Number
///
/// Formula: PV = PMT * ((1 - (1 + r)^-n) / r)
/// where r = periodic interest rate, n = number of periods, PMT = payment per period
///
/// Example: pv(0.08/12, 20*12, -1000) = present value of $1000/month for 20 years at 8% annual rate
pub fn pv(rate: &Value, nper: &Value, pmt: &Value) -> Result<Value, FunctionError> {
    match (rate, nper, pmt) {
        (Value::Number(r), Value::Number(n), Value::Number(p)) => {
            if *n <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nper must be greater than 0".to_string(),
                });
            }

            // Handle zero interest rate case
            if *r == 0.0 {
                return Ok(Value::Number(-p * n));
            }

            // Formula: PV = PMT * ((1 - (1 + r)^-n) / r)
            let factor = (1.0 + r).powf(-n);
            let present_value = -p * ((1.0 - factor) / r);

            Ok(Value::Number(present_value))
        }
        (Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pmt.type_name().to_string(),
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
