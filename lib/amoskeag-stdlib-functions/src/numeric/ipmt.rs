//! ipmt function

use super::pmt::pmt;
use crate::{FunctionError, Value};

/// Calculate the interest payment for a given period
/// ipmt(rate: Number, per: Number, nper: Number, pv: Number, type_: Number) -> Number
///
/// type_: 0 = payment at end of period, 1 = payment at beginning of period
///
/// Returns the interest payment for a given period for an investment based on periodic, constant payments and a constant interest rate
///
/// Example: ipmt(0.1/12, 1, 3*12, 8000, 0) = interest payment for first month of a 3-year loan
pub fn ipmt(
    rate: &Value,
    per: &Value,
    nper: &Value,
    pv: &Value,
    type_: &Value,
) -> Result<Value, FunctionError> {
    match (rate, per, nper, pv, type_) {
        (
            Value::Number(r),
            Value::Number(p),
            Value::Number(n),
            Value::Number(v),
            Value::Number(t),
        ) => {
            if *p < 1.0 || *p > *n {
                return Err(FunctionError::ArgumentError {
                    message: format!("per must be between 1 and {}", *n),
                });
            }

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
                return Ok(Value::Number(0.0));
            }

            // Calculate total payment using pmt function
            let payment = match pmt(
                &Value::Number(*r),
                &Value::Number(*n),
                &Value::Number(*v),
                &Value::Number(*t),
            )? {
                Value::Number(pmt) => pmt,
                _ => unreachable!(),
            };

            // For period 1, interest is calculated on the full principal
            if *p == 1.0 {
                let interest = -(*v * *r);
                let adjusted_interest = if *t == 1.0 {
                    interest / (1.0 + *r)
                } else {
                    interest
                };
                return Ok(Value::Number(adjusted_interest));
            }

            // Calculate remaining balance at start of period
            // Balance = PV * (1+r)^(p-1) - PMT * (((1+r)^(p-1) - 1) / r)
            // Since PMT is negative in this codebase, adjust the formula
            let factor = (1.0 + *r).powf(*p - 1.0);
            let balance = *v * factor + payment * ((factor - 1.0) / *r);

            // Interest for this period
            let interest = -(balance * *r);

            // For type 1, adjust interest
            let adjusted_interest = if *t == 1.0 {
                interest / (1.0 + *r)
            } else {
                interest
            };

            Ok(Value::Number(adjusted_interest))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), Value::Number(_), _) => {
            Err(FunctionError::TypeError {
                expected: "Number".to_string(),
                got: type_.type_name().to_string(),
            })
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _, _) => {
            Err(FunctionError::TypeError {
                expected: "Number".to_string(),
                got: pv.type_name().to_string(),
            })
        }
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
