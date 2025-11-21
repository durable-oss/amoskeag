//! ppmt function

use super::ipmt::ipmt;
use super::pmt::pmt;
use crate::{FunctionError, Value};

/// Calculate the principal payment for a given period
/// ppmt(rate: Number, per: Number, nper: Number, pv: Number, type_: Number) -> Number
///
/// type_: 0 = payment at end of period, 1 = payment at beginning of period
///
/// Returns the principal payment for a given period for an investment based on periodic, constant payments and a constant interest rate
///
/// Example: ppmt(0.1/12, 1, 3*12, 8000, 0) = principal payment for first month of a 3-year loan
pub fn ppmt(rate: &Value, per: &Value, nper: &Value, pv: &Value, type_: &Value) -> Result<Value, FunctionError> {
    match (rate, per, nper, pv, type_) {
        (Value::Number(_), Value::Number(p), Value::Number(n), Value::Number(_), Value::Number(t)) => {
            if *p < 1.0 || *p > *n {
                return Err(FunctionError::ArgumentError {
                    message: format!("per must be between 1 and {}", *n),
                });
            }

            if *t != 0.0 && *t != 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "type must be 0 or 1".to_string(),
                });
            }

            // Calculate total payment
            let payment = match pmt(rate, nper, pv, type_)? {
                Value::Number(pmt) => pmt,
                _ => unreachable!(),
            };

            // Calculate interest payment
            let interest = match ipmt(rate, per, nper, pv, type_)? {
                Value::Number(ipmt) => ipmt,
                _ => unreachable!(),
            };

            // Principal = Total Payment - Interest
            let principal = payment - interest;

            Ok(Value::Number(principal))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), Value::Number(_), _) => {
            Err(FunctionError::TypeError {
                expected: "Number".to_string(),
                got: type_.type_name().to_string(),
            })
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pv.type_name().to_string(),
        }),
        (Value::Number(_), Value::Number(_), _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: nper.type_name().to_string(),
        }),
        (Value::Number(_), _, _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: per.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: rate.type_name().to_string(),
        }),
    }
}
