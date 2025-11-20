//! Numeric functions for Amoskeag

use crate::{FunctionError, Value};

/// Return the absolute value of a number
/// abs(num: Number) -> Number
pub fn abs(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Round a number up to the nearest integer
/// ceil(num: Number) -> Number
pub fn ceil(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Round a number down to the nearest integer
/// floor(num: Number) -> Number
pub fn floor(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Round a number to a specified number of decimal places
/// round(num: Number, digits: Number) -> Number
pub fn round(value: &Value, digits: &Value) -> Result<Value, FunctionError> {
    match (value, digits) {
        (Value::Number(n), Value::Number(d)) => {
            let decimal_places = (*d).max(0.0) as i32;
            let multiplier = 10_f64.powi(decimal_places);
            let rounded = (n * multiplier).round() / multiplier;
            Ok(Value::Number(rounded))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: digits.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Add two numbers (pipe-friendly version of + operator)
/// plus(a: Number, b: Number) -> Number
pub fn plus(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: b.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: a.type_name().to_string(),
        }),
    }
}

/// Subtract two numbers (pipe-friendly version of - operator)
/// minus(a: Number, b: Number) -> Number
pub fn minus(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: b.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: a.type_name().to_string(),
        }),
    }
}

/// Multiply two numbers (pipe-friendly version of * operator)
/// times(a: Number, b: Number) -> Number
pub fn times(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: b.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: a.type_name().to_string(),
        }),
    }
}

/// Divide two numbers (pipe-friendly version of / operator)
/// divided_by(a: Number, b: Number) -> Number
pub fn divided_by(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if *y == 0.0 {
                Err(FunctionError::InvalidOperation {
                    message: "Division by zero".to_string(),
                })
            } else {
                Ok(Value::Number(x / y))
            }
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: b.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: a.type_name().to_string(),
        }),
    }
}

/// Calculate modulo (pipe-friendly version of % operator)
/// modulo(a: Number, b: Number) -> Number
pub fn modulo_fn(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if *y == 0.0 {
                Err(FunctionError::InvalidOperation {
                    message: "Modulo by zero".to_string(),
                })
            } else {
                Ok(Value::Number(x % y))
            }
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: b.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: a.type_name().to_string(),
        }),
    }
}

/// Calculate the maximum of two numbers
/// max(a: Number, b: Number) -> Number
pub fn max(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x.max(*y))),
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: b.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: a.type_name().to_string(),
        }),
    }
}

/// Calculate the minimum of two numbers
/// min(a: Number, b: Number) -> Number
pub fn min(a: &Value, b: &Value) -> Result<Value, FunctionError> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x.min(*y))),
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: b.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: a.type_name().to_string(),
        }),
    }
}

// ============================================================================
// FINANCIAL FUNCTIONS
// ============================================================================

/// Calculate the payment for a loan based on constant payments and a constant interest rate
/// pmt(rate: Number, nper: Number, pv: Number) -> Number
///
/// Formula: PMT = PV * (r * (1 + r)^n) / ((1 + r)^n - 1)
/// where r = periodic interest rate, n = number of periods, PV = present value
///
/// Example: pmt(0.00375, 360, 250000) = -1266.71 (monthly payment on $250k loan at 4.5% APR for 30 years)
pub fn pmt(rate: &Value, nper: &Value, pv: &Value) -> Result<Value, FunctionError> {
    match (rate, nper, pv) {
        (Value::Number(r), Value::Number(n), Value::Number(p)) => {
            if *n <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nper must be greater than 0".to_string(),
                });
            }

            // Handle zero interest rate case
            if *r == 0.0 {
                return Ok(Value::Number(-p / n));
            }

            // Standard formula: PMT = PV * (r * (1 + r)^n) / ((1 + r)^n - 1)
            let factor = (1.0 + r).powf(*n);
            let payment = -p * (r * factor) / (factor - 1.0);

            Ok(Value::Number(payment))
        }
        (Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pv.type_name().to_string(),
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
        (Value::Number(_), Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: pv.type_name().to_string(),
        }),
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

/// Calculate the number of periods for an investment
/// nper(rate: Number, pmt: Number, pv: Number) -> Number
///
/// Formula: NPER = log(PMT / (PMT + PV * r)) / log(1 + r)
/// where r = periodic interest rate, PMT = payment per period, PV = present value
///
/// Example: nper(0.075/12, -200, 8000) = number of months to pay off $8000 at 7.5% with $200/month payments
pub fn nper(rate: &Value, pmt: &Value, pv: &Value) -> Result<Value, FunctionError> {
    match (rate, pmt, pv) {
        (Value::Number(r), Value::Number(p), Value::Number(v)) => {
            // Handle zero interest rate case
            if *r == 0.0 {
                if *p == 0.0 {
                    return Err(FunctionError::ArgumentError {
                        message: "payment cannot be zero when rate is zero".to_string(),
                    });
                }
                return Ok(Value::Number(-v / p));
            }

            // Check for valid parameters
            let denominator = p + v * r;
            if denominator == 0.0 || p / denominator <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "invalid payment or present value for given rate".to_string(),
                });
            }

            // Formula: NPER = log(PMT / (PMT + PV * r)) / log(1 + r)
            let num_periods = (p / denominator).ln() / (1.0 + r).ln();

            Ok(Value::Number(num_periods))
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
            got: rate.type_name().to_string(),
        }),
    }
}

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

            // Newton-Raphson method to solve for rate
            // We're solving: PV + PMT * ((1 - (1+r)^-n) / r) = 0

            let mut rate = 0.01; // Initial guess: 1%
            let tolerance = 1e-7;
            let max_iterations = 100;

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

                if (new_rate - rate).abs() < tolerance {
                    return Ok(Value::Number(new_rate));
                }

                rate = new_rate;

                // Ensure rate stays in reasonable bounds
                if rate < -0.99 || rate > 10.0 {
                    return Err(FunctionError::ArgumentError {
                        message: "rate calculation did not converge to a reasonable value".to_string(),
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

/// Calculate net present value of a series of cash flows
/// npv(rate: Number, values: Array) -> Number
///
/// Formula: NPV = sum(values[i] / (1 + rate)^(i+1))
/// where i goes from 0 to n-1
///
/// Note: The first value is at period 1, not period 0. Add initial investment separately.
///
/// Example: npv(0.1, [-10000, 3000, 4200, 6800]) = net present value at 10% discount rate
pub fn npv(rate: &Value, values: &Value) -> Result<Value, FunctionError> {
    match (rate, values) {
        (Value::Number(r), Value::Array(arr)) => {
            if arr.is_empty() {
                return Err(FunctionError::ArgumentError {
                    message: "values array cannot be empty".to_string(),
                });
            }

            let mut npv = 0.0;
            for (i, value) in arr.iter().enumerate() {
                match value {
                    Value::Number(v) => {
                        let period = (i + 1) as f64;
                        npv += v / (1.0 + r).powf(period);
                    }
                    _ => {
                        return Err(FunctionError::TypeError {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {}", value.type_name()),
                        })
                    }
                }
            }

            Ok(Value::Number(npv))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: values.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: rate.type_name().to_string(),
        }),
    }
}

/// Calculate internal rate of return for a series of cash flows
/// irr(values: Array) -> Number
///
/// Uses Newton-Raphson method to find the rate where NPV = 0
/// The first value is typically a negative investment, followed by positive returns
///
/// Example: irr([-10000, 3000, 4200, 6800]) = internal rate of return
pub fn irr(values: &Value) -> Result<Value, FunctionError> {
    match values {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(FunctionError::ArgumentError {
                    message: "values array cannot be empty".to_string(),
                });
            }

            // Convert to f64 array and validate
            let cash_flows: Result<Vec<f64>, _> = arr
                .iter()
                .map(|v| match v {
                    Value::Number(n) => Ok(*n),
                    _ => Err(FunctionError::TypeError {
                        expected: "Array of Numbers".to_string(),
                        got: format!("Array containing {}", v.type_name()),
                    }),
                })
                .collect();
            let cash_flows = cash_flows?;

            // Check for at least one positive and one negative value
            let has_positive = cash_flows.iter().any(|&v| v > 0.0);
            let has_negative = cash_flows.iter().any(|&v| v < 0.0);

            if !has_positive || !has_negative {
                return Err(FunctionError::ArgumentError {
                    message: "cash flows must contain both positive and negative values".to_string(),
                });
            }

            // Newton-Raphson method to find rate where NPV = 0
            let mut rate = 0.1; // Initial guess: 10%
            let tolerance = 1e-6;
            let max_iterations = 100;

            for _ in 0..max_iterations {
                let mut npv = 0.0;
                let mut dnpv = 0.0;

                for (i, &cf) in cash_flows.iter().enumerate() {
                    let period = i as f64;
                    let factor = (1.0_f64 + rate).powf(period);
                    npv += cf / factor;
                    dnpv -= period * cf / ((1.0 + rate) * factor);
                }

                if dnpv.abs() < tolerance {
                    return Err(FunctionError::ArgumentError {
                        message: "IRR calculation did not converge".to_string(),
                    });
                }

                let new_rate = rate - npv / dnpv;

                if (new_rate - rate).abs() < tolerance {
                    return Ok(Value::Number(new_rate));
                }

                rate = new_rate;

                // Ensure rate stays in reasonable bounds
                if rate < -0.99 || rate > 100.0 {
                    return Err(FunctionError::ArgumentError {
                        message: "IRR calculation did not converge to a reasonable value".to_string(),
                    });
                }
            }

            Err(FunctionError::ArgumentError {
                message: "IRR calculation exceeded maximum iterations".to_string(),
            })
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: values.type_name().to_string(),
        }),
    }
}

/// Calculate modified internal rate of return
/// mirr(values: Array, finance_rate: Number, reinvest_rate: Number) -> Number
///
/// MIRR improves on IRR by assuming positive cash flows are reinvested at the reinvestment rate
/// and negative cash flows are financed at the financing rate
///
/// Example: mirr([-10000, 3000, 4200, 6800], 0.1, 0.12) = modified IRR
pub fn mirr(values: &Value, finance_rate: &Value, reinvest_rate: &Value) -> Result<Value, FunctionError> {
    match (values, finance_rate, reinvest_rate) {
        (Value::Array(arr), Value::Number(fr), Value::Number(rr)) => {
            if arr.is_empty() {
                return Err(FunctionError::ArgumentError {
                    message: "values array cannot be empty".to_string(),
                });
            }

            // Convert to f64 array and validate
            let cash_flows: Result<Vec<f64>, _> = arr
                .iter()
                .map(|v| match v {
                    Value::Number(n) => Ok(*n),
                    _ => Err(FunctionError::TypeError {
                        expected: "Array of Numbers".to_string(),
                        got: format!("Array containing {}", v.type_name()),
                    }),
                })
                .collect();
            let cash_flows = cash_flows?;

            let n = cash_flows.len() as f64;

            // Calculate present value of negative cash flows (financed at finance_rate)
            let mut pv_negative = 0.0;
            for (i, &cf) in cash_flows.iter().enumerate() {
                if cf < 0.0 {
                    let period = i as f64;
                    pv_negative += cf / (1.0 + fr).powf(period);
                }
            }

            // Calculate future value of positive cash flows (reinvested at reinvest_rate)
            let mut fv_positive = 0.0;
            for (i, &cf) in cash_flows.iter().enumerate() {
                if cf > 0.0 {
                    let period = n - 1.0 - i as f64;
                    fv_positive += cf * (1.0 + rr).powf(period);
                }
            }

            if pv_negative == 0.0 || fv_positive == 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "cash flows must contain both positive and negative values".to_string(),
                });
            }

            // MIRR = (FV_positive / -PV_negative)^(1/(n-1)) - 1
            let mirr = (fv_positive / -pv_negative).powf(1.0 / (n - 1.0)) - 1.0;

            Ok(Value::Number(mirr))
        }
        (Value::Array(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: reinvest_rate.type_name().to_string(),
        }),
        (Value::Array(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: finance_rate.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: values.type_name().to_string(),
        }),
    }
}

/// Calculate straight-line depreciation
/// sln(cost: Number, salvage: Number, life: Number) -> Number
///
/// Formula: SLN = (cost - salvage) / life
///
/// Example: sln(30000, 7500, 10) = 2250 (annual depreciation for 10 years)
pub fn sln(cost: &Value, salvage: &Value, life: &Value) -> Result<Value, FunctionError> {
    match (cost, salvage, life) {
        (Value::Number(c), Value::Number(s), Value::Number(l)) => {
            if *l <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "life must be greater than 0".to_string(),
                });
            }

            let depreciation = (c - s) / l;
            Ok(Value::Number(depreciation))
        }
        (Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: life.type_name().to_string(),
        }),
        (Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: salvage.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: cost.type_name().to_string(),
        }),
    }
}

/// Calculate double-declining balance depreciation
/// ddb(cost: Number, salvage: Number, life: Number, period: Number) -> Number
///
/// Uses declining balance method with double the straight-line rate
///
/// Example: ddb(30000, 7500, 10, 1) = depreciation for first year
pub fn ddb(cost: &Value, salvage: &Value, life: &Value, period: &Value) -> Result<Value, FunctionError> {
    match (cost, salvage, life, period) {
        (Value::Number(c), Value::Number(s), Value::Number(l), Value::Number(p)) => {
            if *l <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "life must be greater than 0".to_string(),
                });
            }

            if *p < 1.0 || *p > *l {
                return Err(FunctionError::ArgumentError {
                    message: format!("period must be between 1 and {}", l),
                });
            }

            let rate = 2.0 / l; // Double declining rate
            let mut book_value = *c;
            let mut depreciation = 0.0;

            for i in 1..=(*p as i32) {
                depreciation = book_value * rate;

                // Don't depreciate below salvage value
                if book_value - depreciation < *s {
                    depreciation = book_value - s;
                }

                book_value -= depreciation;

                if i == *p as i32 {
                    break;
                }
            }

            Ok(Value::Number(depreciation.max(0.0)))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: period.type_name().to_string(),
        }),
        (Value::Number(_), Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: life.type_name().to_string(),
        }),
        (Value::Number(_), _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: salvage.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: cost.type_name().to_string(),
        }),
    }
}

/// Calculate declining balance depreciation
/// db(cost: Number, salvage: Number, life: Number, period: Number, month: Number) -> Number
///
/// Returns the depreciation of an asset for a specified period using the fixed-declining balance method
///
/// Example: db(1000000, 100000, 6, 1, 7) = depreciation for first year with 7 months in first year
pub fn db(cost: &Value, salvage: &Value, life: &Value, period: &Value, month: &Value) -> Result<Value, FunctionError> {
    match (cost, salvage, life, period, month) {
        (Value::Number(c), Value::Number(s), Value::Number(l), Value::Number(p), Value::Number(m)) => {
            if *l <= 0.0 {
                return Err(FunctionError::ArgumentError {
                    message: "life must be greater than 0".to_string(),
                });
            }

            if *p < 1.0 || *p > *l + 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: format!("period must be between 1 and {}", l + 1.0),
                });
            }

            if *m < 1.0 || *m > 12.0 {
                return Err(FunctionError::ArgumentError {
                    message: "month must be between 1 and 12".to_string(),
                });
            }

            if *s >= *c {
                return Ok(Value::Number(0.0));
            }

            // Calculate the fixed rate
            let rate = 1.0 - (s / c).powf(1.0 / l);
            let rate = (rate * 1000.0).round() / 1000.0; // Round to 3 decimal places

            let depreciation;

            // First period (partial year if month != 12)
            if *p == 1.0 {
                depreciation = c * rate * m / 12.0;
            } else {
                // Calculate depreciation for previous periods
                let mut temp_total = 0.0;

                // First period
                let first_depr = c * rate * m / 12.0;
                temp_total += first_depr;

                // Full years
                for _i in 2..(*p as i32) {
                    let depr = (c - temp_total) * rate;
                    temp_total += depr;
                }

                // Current period
                if *p < *l + 1.0 {
                    depreciation = (c - temp_total) * rate;
                } else {
                    // Last period (partial year)
                    depreciation = (c - temp_total) * rate * (12.0 - m) / 12.0;
                }
            }

            Ok(Value::Number(depreciation))
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), Value::Number(_), _) => {
            Err(FunctionError::TypeError {
                expected: "Number".to_string(),
                got: month.type_name().to_string(),
            })
        }
        (Value::Number(_), Value::Number(_), Value::Number(_), _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: period.type_name().to_string(),
        }),
        (Value::Number(_), Value::Number(_), _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: life.type_name().to_string(),
        }),
        (Value::Number(_), _, _, _, _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: salvage.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: cost.type_name().to_string(),
        }),
    }
}

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
                )? {
                    Value::Number(i) => i,
                    _ => unreachable!(),
                };

                // Adjust for payment timing
                let adjusted_interest = if *t == 1.0 {
                    interest / (1.0 + r)
                } else {
                    interest
                };

                total_interest += adjusted_interest;
            }

            Ok(Value::Number(total_interest))
        }
        _ => Err(FunctionError::TypeError {
            expected: "all arguments must be Numbers".to_string(),
            got: "mixed types".to_string(),
        }),
    }
}

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
                )? {
                    Value::Number(p) => p,
                    _ => unreachable!(),
                };

                // Adjust for payment timing
                let adjusted_principal = if *t == 1.0 {
                    principal / (1.0 + r)
                } else {
                    principal
                };

                total_principal += adjusted_principal;
            }

            Ok(Value::Number(total_principal))
        }
        _ => Err(FunctionError::TypeError {
            expected: "all arguments must be Numbers".to_string(),
            got: "mixed types".to_string(),
        }),
    }
}

/// Calculate the effective annual interest rate
/// effect(nominal_rate: Number, npery: Number) -> Number
///
/// Formula: effective_rate = (1 + nominal_rate/npery)^npery - 1
/// where npery is the number of compounding periods per year
///
/// Example: effect(0.0525, 4) = effective rate of 5.25% nominal compounded quarterly
pub fn effect(nominal_rate: &Value, npery: &Value) -> Result<Value, FunctionError> {
    match (nominal_rate, npery) {
        (Value::Number(r), Value::Number(n)) => {
            if *n < 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "npery must be at least 1".to_string(),
                });
            }

            if *r <= -1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "nominal_rate must be greater than -1".to_string(),
                });
            }

            let effective = (1.0 + r / n).powf(*n) - 1.0;

            Ok(Value::Number(effective))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: npery.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: nominal_rate.type_name().to_string(),
        }),
    }
}

/// Calculate the nominal annual interest rate
/// nominal(effect_rate: Number, npery: Number) -> Number
///
/// Formula: nominal_rate = ((1 + effect_rate)^(1/npery) - 1) * npery
/// where npery is the number of compounding periods per year
///
/// Example: nominal(0.053543, 4) = nominal rate for 5.3543% effective rate compounded quarterly
pub fn nominal(effect_rate: &Value, npery: &Value) -> Result<Value, FunctionError> {
    match (effect_rate, npery) {
        (Value::Number(r), Value::Number(n)) => {
            if *n < 1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "npery must be at least 1".to_string(),
                });
            }

            if *r <= -1.0 {
                return Err(FunctionError::ArgumentError {
                    message: "effect_rate must be greater than -1".to_string(),
                });
            }

            let nominal = ((1.0 + r).powf(1.0 / n) - 1.0) * n;

            Ok(Value::Number(nominal))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: npery.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: effect_rate.type_name().to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(&Value::Number(-5.0)).unwrap(), Value::Number(5.0));
        assert_eq!(abs(&Value::Number(5.0)).unwrap(), Value::Number(5.0));
    }

    #[test]
    fn test_ceil() {
        assert_eq!(ceil(&Value::Number(4.3)).unwrap(), Value::Number(5.0));
        assert_eq!(ceil(&Value::Number(4.0)).unwrap(), Value::Number(4.0));
    }

    #[test]
    fn test_floor() {
        assert_eq!(floor(&Value::Number(4.7)).unwrap(), Value::Number(4.0));
        assert_eq!(floor(&Value::Number(4.0)).unwrap(), Value::Number(4.0));
    }

    #[test]
    fn test_round() {
        assert_eq!(
            round(&Value::Number(3.14159), &Value::Number(2.0)).unwrap(),
            Value::Number(3.14)
        );
        assert_eq!(
            round(&Value::Number(3.5), &Value::Number(0.0)).unwrap(),
            Value::Number(4.0)
        );
    }

    #[test]
    fn test_plus() {
        assert_eq!(
            plus(&Value::Number(3.0), &Value::Number(4.0)).unwrap(),
            Value::Number(7.0)
        );
    }

    #[test]
    fn test_minus() {
        assert_eq!(
            minus(&Value::Number(10.0), &Value::Number(4.0)).unwrap(),
            Value::Number(6.0)
        );
    }

    #[test]
    fn test_times() {
        assert_eq!(
            times(&Value::Number(3.0), &Value::Number(4.0)).unwrap(),
            Value::Number(12.0)
        );
    }

    #[test]
    fn test_divided_by() {
        assert_eq!(
            divided_by(&Value::Number(12.0), &Value::Number(4.0)).unwrap(),
            Value::Number(3.0)
        );
    }

    #[test]
    fn test_divided_by_zero() {
        let result = divided_by(&Value::Number(12.0), &Value::Number(0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));
    }

    #[test]
    fn test_modulo() {
        assert_eq!(
            modulo_fn(&Value::Number(10.0), &Value::Number(3.0)).unwrap(),
            Value::Number(1.0)
        );
    }

    #[test]
    fn test_max() {
        assert_eq!(
            max(&Value::Number(3.0), &Value::Number(7.0)).unwrap(),
            Value::Number(7.0)
        );
        assert_eq!(
            max(&Value::Number(10.0), &Value::Number(5.0)).unwrap(),
            Value::Number(10.0)
        );
    }

    #[test]
    fn test_min() {
        assert_eq!(
            min(&Value::Number(3.0), &Value::Number(7.0)).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            min(&Value::Number(10.0), &Value::Number(5.0)).unwrap(),
            Value::Number(5.0)
        );
    }

    // ========================================================================
    // FINANCIAL FUNCTION TESTS
    // ========================================================================

    #[test]
    fn test_pmt_basic() {
        // Test case from example 14: $250,000 loan at 0.375% monthly for 360 months
        let result = pmt(
            &Value::Number(0.00375),
            &Value::Number(360.0),
            &Value::Number(250000.0),
        )
        .unwrap();
        if let Value::Number(payment) = result {
            // Expected: -1266.71
            assert!((payment + 1266.71).abs() < 0.01, "Expected ~-1266.71, got {}", payment);
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_pmt_zero_interest() {
        // Zero interest rate should give simple division
        let result = pmt(
            &Value::Number(0.0),
            &Value::Number(12.0),
            &Value::Number(12000.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(-1000.0));
    }

    #[test]
    fn test_pmt_invalid_nper() {
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(0.0),
            &Value::Number(10000.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_pv_basic() {
        // Present value of $1000/month for 240 months at 0.67% monthly
        let result = pv(
            &Value::Number(0.0067),
            &Value::Number(240.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        if let Value::Number(pv) = result {
            // Should be around $119,000-$120,000
            assert!(pv > 119000.0 && pv < 120000.0, "PV out of expected range: {}", pv);
        }
    }

    #[test]
    fn test_pv_zero_interest() {
        let result = pv(
            &Value::Number(0.0),
            &Value::Number(12.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(12000.0));
    }

    #[test]
    fn test_fv_basic() {
        // Future value of $1000 lump sum + $100/month for 120 months at 0.5% monthly
        let result = fv(
            &Value::Number(0.005),
            &Value::Number(120.0),
            &Value::Number(-100.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        if let Value::Number(fv) = result {
            // Should be around $18,000-$18,500
            assert!(fv > 18000.0 && fv < 18500.0, "FV out of expected range: {}", fv);
        }
    }

    #[test]
    fn test_fv_zero_interest() {
        let result = fv(
            &Value::Number(0.0),
            &Value::Number(12.0),
            &Value::Number(-100.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(2200.0)); // 1000 + 12*100
    }

    #[test]
    fn test_nper_basic() {
        // How many months to pay off $8000 at 7.5% annual (0.625% monthly) with $200/month?
        let result = nper(
            &Value::Number(0.075 / 12.0),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(periods) = result {
            // Should be around 45-50 months
            assert!(periods > 45.0 && periods < 50.0, "NPER out of expected range: {}", periods);
        }
    }

    #[test]
    fn test_nper_zero_interest() {
        let result = nper(
            &Value::Number(0.0),
            &Value::Number(-100.0),
            &Value::Number(1000.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_rate_basic() {
        // What rate for a 48-month $8000 loan with $200 monthly payment?
        let result = rate(
            &Value::Number(48.0),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(r) = result {
            // Should be around 0.77% monthly (9.24% annual)
            assert!(r > 0.007 && r < 0.008, "Rate out of expected range: {}", r);
        }
    }

    #[test]
    fn test_npv_basic() {
        // NPV of cash flows at 10% discount rate
        let cash_flows = Value::Array(vec![
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = npv(&Value::Number(0.1), &cash_flows).unwrap();
        if let Value::Number(npv) = result {
            // Should be around $11,200
            assert!(npv > 11000.0 && npv < 11500.0, "NPV out of expected range: {}", npv);
        }
    }

    #[test]
    fn test_npv_empty_array() {
        let cash_flows = Value::Array(vec![]);
        let result = npv(&Value::Number(0.1), &cash_flows);
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_npv_invalid_array() {
        let cash_flows = Value::Array(vec![
            Value::Number(1000.0),
            Value::String("invalid".to_string()),
        ]);
        let result = npv(&Value::Number(0.1), &cash_flows);
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_irr_basic() {
        // IRR for investment with initial -10000 and returns of 3000, 4200, 6800
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
            // Should be around 16-17%
            assert!(irr_rate > 0.16 && irr_rate < 0.17, "IRR out of expected range: {}", irr_rate);
        }
    }

    #[test]
    fn test_irr_no_positive_values() {
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::Number(-3000.0),
        ]);
        let result = irr(&cash_flows);
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_mirr_basic() {
        // MIRR with 10% finance rate and 12% reinvestment rate
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            // MIRR should be lower than IRR, around 15-16%
            assert!(mirr_rate > 0.15 && mirr_rate < 0.16, "MIRR out of expected range: {}", mirr_rate);
        }
    }

    #[test]
    fn test_sln_basic() {
        // Straight-line depreciation: $30,000 asset, $7,500 salvage, 10 years
        let result = sln(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(2250.0));
    }

    #[test]
    fn test_sln_zero_life() {
        let result = sln(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ddb_basic() {
        // Double-declining balance: $30,000 asset, $7,500 salvage, 10 years, year 1
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(1.0),
        )
        .unwrap();
        // Year 1: 30000 * 0.2 = 6000
        assert_eq!(result, Value::Number(6000.0));
    }

    #[test]
    fn test_ddb_year_2() {
        // Year 2 of DDB
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(2.0),
        )
        .unwrap();
        // Year 2: (30000 - 6000) * 0.2 = 4800
        assert_eq!(result, Value::Number(4800.0));
    }

    #[test]
    fn test_ddb_invalid_period() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(11.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_db_basic() {
        // Declining balance: $1,000,000 asset, $100,000 salvage, 6 years, period 1, 7 months
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        )
        .unwrap();
        if let Value::Number(depr) = result {
            // Should be around $186,000-$187,000 for first partial year
            assert!(depr > 180000.0 && depr < 190000.0, "DB depreciation out of range: {}", depr);
        }
    }

    #[test]
    fn test_db_invalid_month() {
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Number(13.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ipmt_first_period() {
        // Interest payment for first month of a loan
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            // First period interest = 8000 * (0.1/12) = 66.67
            assert!((interest - 66.67).abs() < 0.1, "Expected ~66.67, got {}", interest);
        }
    }

    #[test]
    fn test_ppmt_first_period() {
        // Principal payment for first month of a loan
        let result = ppmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(principal) = result {
            // Should be negative (payment) and around -$220-$330
            assert!(principal < -220.0 && principal > -330.0, "Principal out of range: {}", principal);
        }
    }

    #[test]
    fn test_cumipmt_first_year() {
        // Cumulative interest for first 12 months
        let result = cumipmt(
            &Value::Number(0.09 / 12.0),
            &Value::Number(360.0),
            &Value::Number(125000.0),
            &Value::Number(1.0),
            &Value::Number(12.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(total_interest) = result {
            // Should be positive (interest paid) and around $11,000-$12,300
            assert!(total_interest > 11000.0 && total_interest < 12300.0, "Cumulative interest out of range: {}", total_interest);
        }
    }

    #[test]
    fn test_cumipmt_invalid_periods() {
        let result = cumipmt(
            &Value::Number(0.09 / 12.0),
            &Value::Number(360.0),
            &Value::Number(125000.0),
            &Value::Number(13.0),
            &Value::Number(12.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_cumprinc_first_year() {
        // Cumulative principal for first 12 months
        let result = cumprinc(
            &Value::Number(0.09 / 12.0),
            &Value::Number(360.0),
            &Value::Number(125000.0),
            &Value::Number(1.0),
            &Value::Number(12.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(total_principal) = result {
            // Should be negative (paid down)
            assert!(total_principal < 0.0, "Cumulative principal should be negative: {}", total_principal);
        }
    }

    #[test]
    fn test_effect_quarterly_compounding() {
        // Effective rate for 5.25% nominal compounded quarterly
        let result = effect(&Value::Number(0.0525), &Value::Number(4.0)).unwrap();
        if let Value::Number(eff) = result {
            // Effective rate should be around 5.354%
            assert!((eff - 0.05354).abs() < 0.0001, "Expected ~0.05354, got {}", eff);
        }
    }

    #[test]
    fn test_effect_annual_compounding() {
        // Effective rate for annual compounding should equal nominal rate
        let result = effect(&Value::Number(0.05), &Value::Number(1.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!((eff - 0.05).abs() < 0.0001, "Expected ~0.05, got {}", eff);
        }
    }

    #[test]
    fn test_effect_invalid_npery() {
        let result = effect(&Value::Number(0.05), &Value::Number(0.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_nominal_quarterly_compounding() {
        // Nominal rate for 5.354% effective compounded quarterly
        let result = nominal(&Value::Number(0.05354), &Value::Number(4.0)).unwrap();
        if let Value::Number(nom) = result {
            // Should be around 5.25%
            assert!((nom - 0.0525).abs() < 0.001, "Expected ~0.0525, got {}", nom);
        }
    }

    #[test]
    fn test_nominal_annual_compounding() {
        // Nominal rate for annual compounding should equal effective rate
        let result = nominal(&Value::Number(0.05), &Value::Number(1.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!((nom - 0.05).abs() < 0.0001);
        }
    }

    #[test]
    fn test_effect_nominal_roundtrip() {
        // Test that effect and nominal are inverses
        let nominal_rate = 0.0625;
        let npery = 12.0;

        let eff_result = effect(&Value::Number(nominal_rate), &Value::Number(npery)).unwrap();
        let back_to_nominal = nominal(&eff_result, &Value::Number(npery)).unwrap();

        if let Value::Number(nom) = back_to_nominal {
            assert!((nom - nominal_rate).abs() < 0.0001, "Roundtrip failed: {} != {}", nom, nominal_rate);
        }
    }

    #[test]
    fn test_pmt_ppmt_ipmt_consistency() {
        // PMT should equal PPMT + IPMT for any period
        let rate = &Value::Number(0.06 / 12.0);
        let nper = &Value::Number(60.0);
        let pv = &Value::Number(20000.0);
        let period = &Value::Number(15.0);

        let total_pmt = pmt(rate, nper, pv).unwrap();
        let principal = ppmt(rate, period, nper, pv).unwrap();
        let interest = ipmt(rate, period, nper, pv).unwrap();

        if let (Value::Number(t), Value::Number(p), Value::Number(i)) = (total_pmt, principal, interest) {
            assert!((t - (p + i)).abs() < 0.01, "PMT != PPMT + IPMT: {} != {} + {}", t, p, i);
        }
    }
}
