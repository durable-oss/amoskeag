//! ppmt function

use crate::{FunctionError, Value};
use super::pmt::pmt;
use super::ipmt::ipmt;

/// Calculate the principal payment for a given period
/// ppmt(rate: Number, per: Number, nper: Number, pv: Number) -> Number
///
/// Returns the principal payment for a given period for an investment based on periodic, constant payments and a constant interest rate
///
/// Example: ppmt(0.1/12, 1, 3*12, 8000) = principal payment for first month of a 3-year loan
pub fn ppmt(rate: &Value, per: &Value, nper: &Value, pv: &Value) -> Result<Value, FunctionError> {
    match (rate, per, nper, pv) {
        (Value::Number(_), Value::Number(p), Value::Number(n), Value::Number(_)) => {
            if *p < 1.0 || *p > *n {
                return Err(FunctionError::ArgumentError {
                    message: format!("per must be between 1 and {}", n),
                });
            }

            // Calculate total payment
            let payment = match pmt(rate, nper, pv)? {
                Value::Number(pmt) => pmt,
                _ => unreachable!(),
            };

            // Calculate interest payment
            let interest = match ipmt(rate, per, nper, pv)? {
                Value::Number(ipmt) => ipmt,
                _ => unreachable!(),
            };

            // Principal = Total Payment - Interest
            let principal = payment - interest;

            Ok(Value::Number(principal))
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
