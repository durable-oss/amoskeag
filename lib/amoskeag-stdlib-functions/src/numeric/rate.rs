//! rate function

use crate::{FunctionError, Value};

/// Calculate the interest rate per period
/// rate(nper: Number, pmt: Number, pv: Number) -> Number
///
/// Uses Newton-Raphson method to solve for rate
/// This is an iterative numerical method as there's no closed-form solution
///
/// Example: rate(48, -200, 8000) = monthly interest rate for a 48-month $8000 loan with $200 payments
pub fn rate(nper: &Value, pmt: &Value, pv: &Value) -> Result<Value, FunctionError> {
    match (nper, pmt, pv) {
        (Value::Number(n), Value::Number(p), Value::Number(v)) => {
            if *n <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nper must be greater than 0".to_string(),
                });
            }

            // Special case for nper = 1
            if (*n - 1.0).abs() < 1e-10 {
                // For nper=1: PV + PMT * (1 - (1+r)^-1) / r = 0
                // PV + PMT * (r / (1+r)) / r = 0
                // PV + PMT / (1+r) = 0
                // 1+r = -PMT / PV
                // r = -PMT / PV - 1
                let r = -p / v - 1.0;
                return Ok(Value::Number(r));
            }

            // Newton-Raphson method to solve for rate
            // We're solving: PV + PMT * ((1 - (1+r)^-n) / r) = 0

            let mut rate = if *v < 0.0 { 0.2 } else { 0.05 }; // Higher initial guess for investments
            let tolerance = 1e-7;
            let max_iterations = 2000;

            for _ in 0..max_iterations {
                let r = rate;
                let one_plus_r: f64 = 1.0 + r;
                let factor = one_plus_r.powf(*n);
                let p_inv = 1.0 / factor;

                // f(r) = PV + PMT * ((1 - (1+r)^-n) / r)
                let f = v + p * ((1.0 - p_inv) / r);

                // f'(r) = PMT * [(-r*n/(1+r)^(n+1) - 1 + (1+r)^-n) / r^2]
                let df = p * ((-r * n * p_inv / one_plus_r - 1.0 + p_inv) / (r * r));

                if df.abs() < tolerance {
                    return Err(FunctionError::ArgumentError {
                        message: "rate calculation did not converge".to_string(),
                    });
                }

                let new_rate = rate - f / df;

                // Constrain rate to non-negative for financial calculations
                let new_rate = new_rate.max(0.0001);

                if (new_rate - rate).abs() < tolerance {
                    return Ok(Value::Number(new_rate));
                }

                rate = new_rate;

                // Ensure rate stays in reasonable bounds
                if !(0.0001..=50.0).contains(&rate) {
                    return Err(FunctionError::ArgumentError {
                        message: "rate calculation did not converge to a reasonable value"
                            .to_string(),
                    });
                }
            }

            Err(FunctionError::ArgumentError {
                message: "rate calculation exceeded maximum iterations".to_string(),
            })
        }
        (Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pv.type_name().to_string(),
        }),
        (Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pmt.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: nper.type_name().to_string(),
        }),
    }
}
