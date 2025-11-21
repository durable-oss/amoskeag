//! cumprinc function

use super::ppmt::ppmt;
use crate::{FunctionError, Value};

/// Calculate cumulative principal paid between two periods
/// cumprinc(rate: Number, nper: Number, pv: Number, start_period: Number, end_period: Number, type_: Number) -> Number
///
/// type: 0 = payment at end of period, 1 = payment at beginning of period
///
/// Example: cumprinc(0.09/12, 30*12, 125000, 1, 12, 0) = total principal paid in first year
pub fn cumprinc(
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

            let mut total_principal = 0.0;

            for period in (*sp as i32)..=(*ep as i32) {
                let principal = match ppmt(
                    &Value::Number(*r),
                    &Value::Number(period as f64),
                    &Value::Number(*n),
                    &Value::Number(*v),
                    &Value::Number(*t),
                )? {
                    Value::Number(p) => p,
                    _ => unreachable!(),
                };

                total_principal += principal;
            }

            Ok(Value::Number(total_principal))
        }
        _ => Err(FunctionError::TypeError {
            expected: "all arguments must be Numbers".to_string(),
            got: "mixed types".to_string(),
        }),
    }
}
