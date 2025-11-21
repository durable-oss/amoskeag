//! cumipmt function

use super::ipmt::ipmt;
use crate::{FunctionError, Value};

/// Calculate cumulative interest paid between two periods
/// cumipmt(rate: Number, nper: Number, pv: Number, start_period: Number, end_period: Number, type_: Number) -> Number
///
/// type: 0 = payment at end of period, 1 = payment at beginning of period
///
/// Example: cumipmt(0.09/12, 30*12, 125000, 1, 12, 0) = total interest paid in first year
pub fn cumipmt(
    rate: &Value,
    nper: &Value,
    pv: &Value,
    start_period: &Value,
    end_period: &Value,
    type_: &Value,
) -> Result<Value, FunctionError> {
    match (rate, nper, pv, start_period, end_period, type_) {
        (
            Value::Number(r),
            Value::Number(n),
            Value::Number(v),
            Value::Number(sp),
            Value::Number(ep),
            Value::Number(t),
        ) => {
            if *sp < 1.0 || *sp > *n {
                return Err(FunctionError::ArgumentError {
                    message: format!("start_period must be between 1 and {}", n),
                });
            }

            if *ep < *sp || *ep > *n {
                return Err(FunctionError::ArgumentError {
                    message: format!("end_period must be between {} and {}", sp, n),
                });
            }

            if *t != 0.0 && *t != 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "type must be 0 or 1".to_string(),
                });
            }

            let mut total_interest = 0.0;

            for period in (*sp as i32)..=(*ep as i32) {
                 let interest = match ipmt(
                     &Value::Number(*r),
                     &Value::Number(period as f64),
                     &Value::Number(*n),
                     &Value::Number(*v),
                     &Value::Number(*t),
                 )? {
                     Value::Number(i) => i,
                     _ => unreachable!(),
                 };

                 total_interest += interest;
            }

            Ok(Value::Number(total_interest))
        }
        _ => Err(FunctionError::TypeError {
            expected: "all arguments must be Numbers".to_string(),
            got: "mixed types".to_string(),
        }),
    }
}
