//! ipmt function

use crate::{FunctionError, Value};
use super::pmt::pmt;

/// Calculate the interest payment for a given period
/// ipmt(rate: Number, per: Number, nper: Number, pv: Number) -> Number
///
/// Returns the interest payment for a given period for an investment based on periodic, constant payments and a constant interest rate
///
/// Example: ipmt(0.1/12, 1, 3*12, 8000) = interest payment for first month of a 3-year loan
pub fn ipmt(rate: &Value, per: &Value, nper: &Value, pv: &Value) -> Result<Value, FunctionError> {
    match (rate, per, nper, pv) {
        (Value::Number(r), Value::Number(p), Value::Number(n), Value::Number(v)) => {
            if *p < 1.0 || *p > *n {
                return Err(FunctionError::ArgumentError {
                    message: format!("per must be between 1 and {}", n),
                });
            }

            if *n <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nper must be greater than 0".to_string(),
                });
            }

            // Calculate total payment using pmt function
            let payment = match pmt(&Value::Number(*r), &Value::Number(*n), &Value::Number(*v))? {
                Value::Number(pmt) => pmt,
                _ => unreachable!(),
            };

            // For period 1, interest is calculated on the full principal
            if *p == 1.0 {
                return Ok(Value::Number(v * r));
            }

            // Calculate remaining balance at start of period
            // Balance = PV * (1+r)^p - PMT * (((1+r)^p - 1) / r)
            let factor = (1.0 + r).powf(p - 1.0);
            let balance = v * factor - payment * ((factor - 1.0) / r);

            // Interest for this period
            let interest = balance * r;

            Ok(Value::Number(interest))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pv.type_name().to_string(),
        }),
        (Value::Number(_), Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: nper.type_name().to_string(),
        }),
        (Value::Number(_), _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: per.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: rate.type_name().to_string(),
        }),
    }
}
