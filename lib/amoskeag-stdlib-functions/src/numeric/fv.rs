//! fv function

use crate::{FunctionError, Value};

/// Calculate the future value of an investment
/// fv(rate: Number, nper: Number, pmt: Number, pv: Number) -> Number
///
/// Formula: FV = -PV * (1 + r)^n - PMT * (((1 + r)^n - 1) / r)
/// where r = periodic interest rate, n = number of periods, PMT = payment per period, PV = present value
///
/// Example: fv(0.06/12, 10*12, -100, -1000) = future value of $1000 + $100/month for 10 years at 6% annual rate
pub fn fv(rate: &Value, nper: &Value, pmt: &Value, pv: &Value) -> Result<Value, FunctionError> {
    match (rate, nper, pmt, pv) {
        (Value::Number(r), Value::Number(n), Value::Number(p), Value::Number(v)) => {
            if *n <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nper must be greater than 0".to_string(),
                });
            }

            // Handle zero interest rate case
            if *r == 0.0 {
                return Ok(Value::Number(-v - p * n));
            }

            // Formula: FV = -PV * (1 + r)^n - PMT * (((1 + r)^n - 1) / r)
            let factor = (1.0 + r).powf(*n);
            let future_value = -v * factor - p * ((factor - 1.0) / r);

            Ok(Value::Number(future_value))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _) => {
            Err(FunctionError::TypeError {
                expected: "Number".to_string(),
                got: pv.type_name().to_string(),
            })
        }
        (Value::Number(_), Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pmt.type_name().to_string(),
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
