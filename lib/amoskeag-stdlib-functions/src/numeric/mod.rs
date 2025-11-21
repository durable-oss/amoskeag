//! Numeric functions for Amoskeag

pub mod abs;
pub mod ceil;
pub mod divided_by;
pub mod floor;
pub mod max;
pub mod min;
pub mod minus;
pub mod modulo;
pub mod plus;
pub mod round;
pub mod times;

// Financial functions
pub mod cumipmt;
pub mod cumprinc;
pub mod db;
pub mod ddb;
pub mod effect;
pub mod fv;
pub mod ipmt;
pub mod irr;
pub mod mirr;
pub mod nominal;
pub mod nper;
pub mod npv;
pub mod pmt;
pub mod ppmt;
pub mod pv;
pub mod rate;
pub mod sln;

// Re-export all functions
pub use abs::abs;
pub use ceil::ceil;
pub use cumipmt::cumipmt;
pub use cumprinc::cumprinc;
pub use db::db;
pub use ddb::ddb;
pub use divided_by::divided_by;
pub use effect::effect;
pub use floor::floor;
pub use fv::fv;
pub use ipmt::ipmt;
pub use irr::irr;
pub use max::max;
pub use min::min;
pub use minus::minus;
pub use mirr::mirr;
pub use modulo::modulo_fn;
pub use nominal::nominal;
pub use nper::nper;
pub use npv::npv;
pub use plus::plus;
pub use pmt::pmt;
pub use ppmt::ppmt;
pub use pv::pv;
pub use rate::rate;
pub use round::round;
pub use sln::sln;
pub use times::times;

/// Represents a numeric function that can be registered with inventory
pub struct NumericFunction {
    pub name: &'static str,
    pub description: &'static str,
    pub arity: Arity,
}

/// Function arity
pub enum Arity {
    Unary,
    Binary,
    Ternary,
    Quaternary,
    Quinary,
    Senary,
}

inventory::collect!(NumericFunction);

inventory::submit! {
    NumericFunction { name: "abs", description: "Return the absolute value of a number", arity: Arity::Unary }
}

inventory::submit! {
    NumericFunction { name: "ceil", description: "Round a number up to the nearest integer", arity: Arity::Unary }
}

inventory::submit! {
    NumericFunction { name: "floor", description: "Round a number down to the nearest integer", arity: Arity::Unary }
}

inventory::submit! {
    NumericFunction { name: "round", description: "Round a number to a specified number of decimal places", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "plus", description: "Add two numbers", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "minus", description: "Subtract two numbers", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "times", description: "Multiply two numbers", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "divided_by", description: "Divide two numbers", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "modulo", description: "Calculate modulo of two numbers", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "max", description: "Calculate the maximum of two numbers", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "min", description: "Calculate the minimum of two numbers", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "pmt", description: "Calculate loan payment", arity: Arity::Ternary }
}

inventory::submit! {
    NumericFunction { name: "pv", description: "Calculate present value", arity: Arity::Ternary }
}

inventory::submit! {
    NumericFunction { name: "fv", description: "Calculate future value", arity: Arity::Quaternary }
}

inventory::submit! {
    NumericFunction { name: "nper", description: "Calculate number of periods", arity: Arity::Ternary }
}

inventory::submit! {
    NumericFunction { name: "rate", description: "Calculate interest rate", arity: Arity::Ternary }
}

inventory::submit! {
    NumericFunction { name: "npv", description: "Calculate net present value", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "irr", description: "Calculate internal rate of return", arity: Arity::Unary }
}

inventory::submit! {
    NumericFunction { name: "mirr", description: "Calculate modified internal rate of return", arity: Arity::Ternary }
}

inventory::submit! {
    NumericFunction { name: "sln", description: "Calculate straight-line depreciation", arity: Arity::Ternary }
}

inventory::submit! {
    NumericFunction { name: "ddb", description: "Calculate double-declining balance depreciation", arity: Arity::Quaternary }
}

inventory::submit! {
    NumericFunction { name: "db", description: "Calculate declining balance depreciation", arity: Arity::Quinary }
}

inventory::submit! {
    NumericFunction { name: "ipmt", description: "Calculate interest payment for a period", arity: Arity::Quaternary }
}

inventory::submit! {
    NumericFunction { name: "ppmt", description: "Calculate principal payment for a period", arity: Arity::Quaternary }
}

inventory::submit! {
    NumericFunction { name: "cumipmt", description: "Calculate cumulative interest paid", arity: Arity::Senary }
}

inventory::submit! {
    NumericFunction { name: "cumprinc", description: "Calculate cumulative principal paid", arity: Arity::Senary }
}

inventory::submit! {
    NumericFunction { name: "effect", description: "Calculate effective annual interest rate", arity: Arity::Binary }
}

inventory::submit! {
    NumericFunction { name: "nominal", description: "Calculate nominal annual interest rate", arity: Arity::Binary }
}

/// Get all registered numeric functions
pub fn get_all_numeric_functions() -> inventory::iter<NumericFunction> {
    inventory::iter::<NumericFunction>
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FunctionError, Value};
    use std::collections::HashMap;

    #[test]
    fn test_abs() {
        // Basic positive and negative numbers
        assert_eq!(abs(&Value::Number(-5.0)).unwrap(), Value::Number(5.0));
        assert_eq!(abs(&Value::Number(5.0)).unwrap(), Value::Number(5.0));

        // Zero
        assert_eq!(abs(&Value::Number(0.0)).unwrap(), Value::Number(0.0));

        // Decimal numbers
        assert_eq!(abs(&Value::Number(-3.14)).unwrap(), Value::Number(3.14));
        assert_eq!(abs(&Value::Number(2.71)).unwrap(), Value::Number(2.71));

        // Very large numbers
        assert_eq!(abs(&Value::Number(1e308)).unwrap(), Value::Number(1e308));
        assert_eq!(abs(&Value::Number(-1e308)).unwrap(), Value::Number(1e308));

        // Very small numbers
        assert_eq!(abs(&Value::Number(1e-323)).unwrap(), Value::Number(1e-323));
        assert_eq!(abs(&Value::Number(-1e-323)).unwrap(), Value::Number(1e-323));

        // Infinity
        assert_eq!(
            abs(&Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            abs(&Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(f64::INFINITY)
        );

        // NaN
        let nan_result = abs(&Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        // Type errors for non-Number types
        assert!(matches!(
            abs(&Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            abs(&Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            abs(&Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            abs(&Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            abs(&Value::Dictionary(std::collections::HashMap::new())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            abs(&Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_ceil() {
        // Basic positive decimals
        assert_eq!(ceil(&Value::Number(4.3)).unwrap(), Value::Number(5.0));
        assert_eq!(ceil(&Value::Number(1.1)).unwrap(), Value::Number(2.0));
        assert_eq!(ceil(&Value::Number(0.1)).unwrap(), Value::Number(1.0));
        assert_eq!(ceil(&Value::Number(0.9)).unwrap(), Value::Number(1.0));

        // Basic negative decimals
        assert_eq!(ceil(&Value::Number(-4.3)).unwrap(), Value::Number(-4.0));
        assert_eq!(ceil(&Value::Number(-1.1)).unwrap(), Value::Number(-1.0));
        assert_eq!(ceil(&Value::Number(-0.1)).unwrap(), Value::Number(0.0));
        assert_eq!(ceil(&Value::Number(-0.9)).unwrap(), Value::Number(0.0));

        // Integers (should remain unchanged)
        assert_eq!(ceil(&Value::Number(4.0)).unwrap(), Value::Number(4.0));
        assert_eq!(ceil(&Value::Number(0.0)).unwrap(), Value::Number(0.0));
        assert_eq!(ceil(&Value::Number(-5.0)).unwrap(), Value::Number(-5.0));

        // Edge cases around zero
        assert_eq!(ceil(&Value::Number(0.0001)).unwrap(), Value::Number(1.0));
        assert_eq!(ceil(&Value::Number(-0.0001)).unwrap(), Value::Number(0.0));

        // Very large numbers
        assert_eq!(ceil(&Value::Number(1e308)).unwrap(), Value::Number(1e308));
        assert_eq!(ceil(&Value::Number(-1e308)).unwrap(), Value::Number(-1e308));

        // Very small numbers
        assert_eq!(ceil(&Value::Number(1e-323)).unwrap(), Value::Number(1.0));
        assert_eq!(ceil(&Value::Number(-1e-323)).unwrap(), Value::Number(0.0));

        // Infinity
        assert_eq!(
            ceil(&Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            ceil(&Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );

        // NaN
        let nan_result = ceil(&Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        // Type errors for non-Number types
        assert!(matches!(
            ceil(&Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            ceil(&Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            ceil(&Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            ceil(&Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            ceil(&Value::Dictionary(std::collections::HashMap::new())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            ceil(&Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_floor() {
        // Basic positive decimals
        assert_eq!(floor(&Value::Number(4.3)).unwrap(), Value::Number(4.0));
        assert_eq!(floor(&Value::Number(1.1)).unwrap(), Value::Number(1.0));
        assert_eq!(floor(&Value::Number(0.1)).unwrap(), Value::Number(0.0));
        assert_eq!(floor(&Value::Number(0.9)).unwrap(), Value::Number(0.0));

        // Basic negative decimals
        assert_eq!(floor(&Value::Number(-4.3)).unwrap(), Value::Number(-5.0));
        assert_eq!(floor(&Value::Number(-1.1)).unwrap(), Value::Number(-2.0));
        assert_eq!(floor(&Value::Number(-0.1)).unwrap(), Value::Number(-1.0));
        assert_eq!(floor(&Value::Number(-0.9)).unwrap(), Value::Number(-1.0));

        // Integers (should remain unchanged)
        assert_eq!(floor(&Value::Number(4.0)).unwrap(), Value::Number(4.0));
        assert_eq!(floor(&Value::Number(0.0)).unwrap(), Value::Number(0.0));
        assert_eq!(floor(&Value::Number(-5.0)).unwrap(), Value::Number(-5.0));

        // Edge cases around zero
        assert_eq!(floor(&Value::Number(0.0001)).unwrap(), Value::Number(0.0));
        assert_eq!(floor(&Value::Number(-0.0001)).unwrap(), Value::Number(-1.0));

        // Very large numbers
        assert_eq!(floor(&Value::Number(1e308)).unwrap(), Value::Number(1e308));
        assert_eq!(
            floor(&Value::Number(-1e308)).unwrap(),
            Value::Number(-1e308)
        );

        // Very small numbers
        assert_eq!(floor(&Value::Number(1e-323)).unwrap(), Value::Number(0.0));
        assert_eq!(floor(&Value::Number(-1e-323)).unwrap(), Value::Number(-1.0));

        // Infinity
        assert_eq!(
            floor(&Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            floor(&Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );

        // NaN
        let nan_result = floor(&Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        // Type errors for non-Number types
        assert!(matches!(
            floor(&Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            floor(&Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            floor(&Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            floor(&Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            floor(&Value::Dictionary(std::collections::HashMap::new())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            floor(&Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_round() {
        // Basic rounding to decimal places
        assert_eq!(
            round(&Value::Number(1.23456), &Value::Number(2.0)).unwrap(),
            Value::Number(1.23)
        );
        assert_eq!(
            round(&Value::Number(1.235), &Value::Number(2.0)).unwrap(),
            Value::Number(1.24)
        );
        assert_eq!(
            round(&Value::Number(1.2345), &Value::Number(3.0)).unwrap(),
            Value::Number(1.235)
        );

        // Rounding to zero decimal places
        assert_eq!(
            round(&Value::Number(3.5), &Value::Number(0.0)).unwrap(),
            Value::Number(4.0)
        );
        assert_eq!(
            round(&Value::Number(2.4), &Value::Number(0.0)).unwrap(),
            Value::Number(2.0)
        );
        assert_eq!(
            round(&Value::Number(-1.5), &Value::Number(0.0)).unwrap(),
            Value::Number(-2.0)
        );

        // Negative decimal places (round to tens, hundreds, etc.)
        assert_eq!(
            round(&Value::Number(123.456), &Value::Number(-1.0)).unwrap(),
            Value::Number(120.0)
        );
        assert_eq!(
            round(&Value::Number(123.456), &Value::Number(-2.0)).unwrap(),
            Value::Number(100.0)
        );

        // Fractional digits parameter (should be floored)
        assert_eq!(
            round(&Value::Number(1.23456), &Value::Number(2.7)).unwrap(),
            Value::Number(1.23)
        );

        // Negative numbers
        assert_eq!(
            round(&Value::Number(-1.23456), &Value::Number(2.0)).unwrap(),
            Value::Number(-1.23)
        );
        assert_eq!(
            round(&Value::Number(-1.235), &Value::Number(2.0)).unwrap(),
            Value::Number(-1.24)
        );

        // Zero
        assert_eq!(
            round(&Value::Number(0.0), &Value::Number(2.0)).unwrap(),
            Value::Number(0.0)
        );

        // Very large numbers
        assert_eq!(
            round(&Value::Number(1e308), &Value::Number(0.0)).unwrap(),
            Value::Number(1e308)
        );

        // Very small numbers
        assert_eq!(
            round(&Value::Number(1e-323), &Value::Number(2.0)).unwrap(),
            Value::Number(0.0)
        );

        // Infinity
        assert_eq!(
            round(&Value::Number(f64::INFINITY), &Value::Number(2.0)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            round(&Value::Number(f64::NEG_INFINITY), &Value::Number(2.0)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );

        // NaN
        let nan_result = round(&Value::Number(f64::NAN), &Value::Number(2.0)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number"),
        }

        // Type errors
        assert!(matches!(
            round(&Value::String("1.23".to_string()), &Value::Number(2.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            round(&Value::Number(1.23), &Value::String("2".to_string())),
            Err(FunctionError::TypeError { .. })
        ));

        // Clamping digits to -20..20
        assert_eq!(
            round(&Value::Number(1.234567890123456789), &Value::Number(21.0)).unwrap(),
            Value::Number(1.234567890123456789)
        );
        assert_eq!(
            round(
                &Value::Number(60000000000000000000.0),
                &Value::Number(-21.0)
            )
            .unwrap(),
            Value::Number(100000000000000000000.0)
        );

        // Digits at boundaries
        assert_eq!(
            round(&Value::Number(1.234567890123456789), &Value::Number(20.0)).unwrap(),
            Value::Number(1.234567890123456789)
        );
        assert_eq!(
            round(
                &Value::Number(60000000000000000000.0),
                &Value::Number(-20.0)
            )
            .unwrap(),
            Value::Number(100000000000000000000.0)
        );

        // Non-finite digits
        assert!(matches!(
            round(&Value::Number(1.23), &Value::Number(f64::INFINITY)),
            Err(FunctionError::ArgumentError { .. })
        ));
        assert!(matches!(
            round(&Value::Number(1.23), &Value::Number(f64::NEG_INFINITY)),
            Err(FunctionError::ArgumentError { .. })
        ));
        assert!(matches!(
            round(&Value::Number(1.23), &Value::Number(f64::NAN)),
            Err(FunctionError::ArgumentError { .. })
        ));

        // Rounding 0.5 cases (half away from zero)
        assert_eq!(
            round(&Value::Number(1.5), &Value::Number(0.0)).unwrap(),
            Value::Number(2.0)
        );
        assert_eq!(
            round(&Value::Number(2.5), &Value::Number(0.0)).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            round(&Value::Number(-1.5), &Value::Number(0.0)).unwrap(),
            Value::Number(-2.0)
        );
        assert_eq!(
            round(&Value::Number(-2.5), &Value::Number(0.0)).unwrap(),
            Value::Number(-3.0)
        );

        // More fractional digits truncation
        assert_eq!(
            round(&Value::Number(1.23456), &Value::Number(2.9)).unwrap(),
            Value::Number(1.23)
        );
        assert_eq!(
            round(&Value::Number(1.23456), &Value::Number(-0.1)).unwrap(),
            Value::Number(1.0)
        );

        // Edge case: rounding to very high precision
        assert_eq!(
            round(&Value::Number(0.0), &Value::Number(20.0)).unwrap(),
            Value::Number(0.0)
        );

        // Negative zero
        assert_eq!(
            round(&Value::Number(-0.0), &Value::Number(2.0)).unwrap(),
            Value::Number(0.0)
        );
    }

    #[test]
    fn test_plus() {
        // Basic positive number addition
        assert_eq!(
            plus(&Value::Number(3.0), &Value::Number(4.0)).unwrap(),
            Value::Number(7.0)
        );
        assert_eq!(
            plus(&Value::Number(10.0), &Value::Number(5.0)).unwrap(),
            Value::Number(15.0)
        );
        assert_eq!(
            plus(&Value::Number(100.0), &Value::Number(50.0)).unwrap(),
            Value::Number(150.0)
        );

        // Negative number addition
        assert_eq!(
            plus(&Value::Number(-5.0), &Value::Number(3.0)).unwrap(),
            Value::Number(-2.0)
        );
        assert_eq!(
            plus(&Value::Number(5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(2.0)
        );
        assert_eq!(
            plus(&Value::Number(-5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(-8.0)
        );

        // Zero cases
        assert_eq!(
            plus(&Value::Number(0.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            plus(&Value::Number(5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            plus(&Value::Number(0.0), &Value::Number(5.0)).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            plus(&Value::Number(-5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(-5.0)
        );

        // Decimal numbers
        let result = plus(&Value::Number(3.14), &Value::Number(1.59)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 4.73).abs() < 1e-10);
        } else {
            panic!("Expected Number");
        }

        let result = plus(&Value::Number(10.5), &Value::Number(2.25)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 12.75).abs() < 1e-10);
        } else {
            panic!("Expected Number");
        }

        let result = plus(&Value::Number(1.1), &Value::Number(0.1)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 1.2).abs() < 1e-10);
        } else {
            panic!("Expected Number");
        }

        // Very large numbers
        assert_eq!(
            plus(&Value::Number(1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(1.1e308)
        );
        assert_eq!(
            plus(&Value::Number(-1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(-9e307)
        );

        // Very small numbers
        assert_eq!(
            plus(&Value::Number(1e-323), &Value::Number(5e-324)).unwrap(),
            Value::Number(1.5e-323)
        );
        assert_eq!(
            plus(&Value::Number(-1e-323), &Value::Number(-5e-324)).unwrap(),
            Value::Number(-1.5e-323)
        );

        // Infinity cases
        assert_eq!(
            plus(&Value::Number(f64::INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            plus(&Value::Number(1.0), &Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            plus(&Value::Number(f64::NEG_INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            plus(&Value::Number(1.0), &Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        let inf_minus_inf = plus(
            &Value::Number(f64::INFINITY),
            &Value::Number(f64::NEG_INFINITY),
        )
        .unwrap();
        match inf_minus_inf {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", inf_minus_inf),
        }

        // NaN cases
        let nan_result = plus(&Value::Number(f64::NAN), &Value::Number(1.0)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = plus(&Value::Number(1.0), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = plus(&Value::Number(f64::NAN), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        // Type errors for non-Number types
        assert!(matches!(
            plus(&Value::String("test".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Number(1.0), &Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Boolean(true), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Number(1.0), &Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Nil, &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Number(1.0), &Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Array(vec![]), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Number(1.0), &Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(
                &Value::Dictionary(std::collections::HashMap::new()),
                &Value::Number(1.0)
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(
                &Value::Number(1.0),
                &Value::Dictionary(std::collections::HashMap::new())
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Symbol("sym".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            plus(&Value::Number(1.0), &Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_minus() {
        // Basic positive number subtraction
        assert_eq!(
            minus(&Value::Number(10.0), &Value::Number(4.0)).unwrap(),
            Value::Number(6.0)
        );
        assert_eq!(
            minus(&Value::Number(5.0), &Value::Number(3.0)).unwrap(),
            Value::Number(2.0)
        );
        assert_eq!(
            minus(&Value::Number(100.0), &Value::Number(50.0)).unwrap(),
            Value::Number(50.0)
        );

        // Negative number subtraction
        assert_eq!(
            minus(&Value::Number(-5.0), &Value::Number(3.0)).unwrap(),
            Value::Number(-8.0)
        );
        assert_eq!(
            minus(&Value::Number(5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(8.0)
        );
        assert_eq!(
            minus(&Value::Number(-5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(-2.0)
        );

        // Zero cases
        assert_eq!(
            minus(&Value::Number(0.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            minus(&Value::Number(5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            minus(&Value::Number(0.0), &Value::Number(5.0)).unwrap(),
            Value::Number(-5.0)
        );

        // Decimal numbers
        assert_eq!(
            minus(&Value::Number(3.14), &Value::Number(1.59)).unwrap(),
            Value::Number(1.55)
        );
        assert_eq!(
            minus(&Value::Number(10.5), &Value::Number(2.25)).unwrap(),
            Value::Number(8.25)
        );
        assert_eq!(
            minus(&Value::Number(1.1), &Value::Number(0.1)).unwrap(),
            Value::Number(1.0)
        );

        // Very large numbers
        assert_eq!(
            minus(&Value::Number(1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(9e307)
        );
        assert_eq!(
            minus(&Value::Number(-1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(-1.1e308)
        );

        // Very small numbers
        assert_eq!(
            minus(&Value::Number(1e-323), &Value::Number(5e-324)).unwrap(),
            Value::Number(5e-324)
        );
        assert_eq!(
            minus(&Value::Number(-1e-323), &Value::Number(-5e-324)).unwrap(),
            Value::Number(-5e-324)
        );

        // Infinity cases
        assert_eq!(
            minus(&Value::Number(f64::INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            minus(&Value::Number(1.0), &Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            minus(&Value::Number(f64::NEG_INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            minus(&Value::Number(1.0), &Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        // Infinity minus infinity = NaN
        let inf_minus_inf =
            minus(&Value::Number(f64::INFINITY), &Value::Number(f64::INFINITY)).unwrap();
        match inf_minus_inf {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", inf_minus_inf),
        }

        // NaN cases
        let nan_result = minus(&Value::Number(f64::NAN), &Value::Number(1.0)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = minus(&Value::Number(1.0), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = minus(&Value::Number(f64::NAN), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        // Type errors for non-Number types
        assert!(matches!(
            minus(&Value::String("test".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Number(1.0), &Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Boolean(true), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Number(1.0), &Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Nil, &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Number(1.0), &Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Array(vec![]), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Number(1.0), &Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(
                &Value::Dictionary(std::collections::HashMap::new()),
                &Value::Number(1.0)
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(
                &Value::Number(1.0),
                &Value::Dictionary(std::collections::HashMap::new())
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Symbol("sym".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            minus(&Value::Number(1.0), &Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_times() {
        // Basic positive number multiplication
        assert_eq!(
            times(&Value::Number(3.0), &Value::Number(4.0)).unwrap(),
            Value::Number(12.0)
        );
        assert_eq!(
            times(&Value::Number(10.0), &Value::Number(5.0)).unwrap(),
            Value::Number(50.0)
        );
        assert_eq!(
            times(&Value::Number(100.0), &Value::Number(50.0)).unwrap(),
            Value::Number(5000.0)
        );

        // Negative number multiplication
        assert_eq!(
            times(&Value::Number(-5.0), &Value::Number(3.0)).unwrap(),
            Value::Number(-15.0)
        );
        assert_eq!(
            times(&Value::Number(5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(-15.0)
        );
        assert_eq!(
            times(&Value::Number(-5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(15.0)
        );
        assert_eq!(
            times(&Value::Number(-10.0), &Value::Number(2.0)).unwrap(),
            Value::Number(-20.0)
        );

        // Zero cases
        assert_eq!(
            times(&Value::Number(0.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            times(&Value::Number(5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            times(&Value::Number(0.0), &Value::Number(5.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            times(&Value::Number(-5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );

        // Decimal numbers
        let result = times(&Value::Number(3.14), &Value::Number(1.59)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 4.9926).abs() < 1e-10);
        } else {
            panic!("Expected Number");
        }

        let result = times(&Value::Number(10.5), &Value::Number(2.25)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 23.625).abs() < 1e-10);
        } else {
            panic!("Expected Number");
        }

        let result = times(&Value::Number(1.1), &Value::Number(0.1)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 0.11).abs() < 1e-10);
        } else {
            panic!("Expected Number");
        }

        // Very large numbers (overflow to infinity)
        assert_eq!(
            times(&Value::Number(1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            times(&Value::Number(-1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );

        // Very small numbers (underflow to zero)
        assert_eq!(
            times(&Value::Number(1e-323), &Value::Number(5e-324)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            times(&Value::Number(-1e-323), &Value::Number(-5e-324)).unwrap(),
            Value::Number(0.0)
        );

        // Infinity cases
        assert_eq!(
            times(&Value::Number(f64::INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            times(&Value::Number(1.0), &Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            times(&Value::Number(f64::NEG_INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            times(&Value::Number(1.0), &Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            times(
                &Value::Number(f64::INFINITY),
                &Value::Number(f64::NEG_INFINITY)
            )
            .unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            times(
                &Value::Number(f64::NEG_INFINITY),
                &Value::Number(f64::NEG_INFINITY)
            )
            .unwrap(),
            Value::Number(f64::INFINITY)
        );

        // NaN cases
        let nan_result = times(&Value::Number(f64::NAN), &Value::Number(1.0)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = times(&Value::Number(1.0), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = times(&Value::Number(f64::NAN), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        // Type errors for non-Number types
        assert!(matches!(
            times(&Value::String("test".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Number(1.0), &Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Boolean(true), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Number(1.0), &Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Nil, &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Number(1.0), &Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Array(vec![]), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Number(1.0), &Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(
                &Value::Dictionary(std::collections::HashMap::new()),
                &Value::Number(1.0)
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(
                &Value::Number(1.0),
                &Value::Dictionary(std::collections::HashMap::new())
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Symbol("sym".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            times(&Value::Number(1.0), &Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_divided_by_basic() {
        // Basic positive number division
        assert_eq!(
            divided_by(&Value::Number(12.0), &Value::Number(4.0)).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            divided_by(&Value::Number(10.0), &Value::Number(2.0)).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            divided_by(&Value::Number(100.0), &Value::Number(25.0)).unwrap(),
            Value::Number(4.0)
        );
    }

    #[test]
    fn test_divided_by_negative_numbers() {
        // Negative number division
        assert_eq!(
            divided_by(&Value::Number(-12.0), &Value::Number(4.0)).unwrap(),
            Value::Number(-3.0)
        );
        assert_eq!(
            divided_by(&Value::Number(12.0), &Value::Number(-4.0)).unwrap(),
            Value::Number(-3.0)
        );
        assert_eq!(
            divided_by(&Value::Number(-12.0), &Value::Number(-4.0)).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            divided_by(&Value::Number(-10.0), &Value::Number(2.0)).unwrap(),
            Value::Number(-5.0)
        );
    }

    #[test]
    fn test_divided_by_zero() {
        // Division by zero
        let result = divided_by(&Value::Number(12.0), &Value::Number(0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));

        // Negative divided by zero
        let result = divided_by(&Value::Number(-5.0), &Value::Number(0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));

        // Zero divided by zero
        let result = divided_by(&Value::Number(0.0), &Value::Number(0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));
    }

    #[test]
    fn test_divided_by_decimal_numbers() {
        // Decimal number division
        assert_eq!(
            divided_by(&Value::Number(7.5), &Value::Number(2.5)).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            divided_by(&Value::Number(10.0), &Value::Number(3.0)).unwrap(),
            Value::Number(3.3333333333333335)
        );
        assert_eq!(
            divided_by(&Value::Number(1.0), &Value::Number(3.0)).unwrap(),
            Value::Number(0.3333333333333333)
        );
        assert_eq!(
            divided_by(&Value::Number(0.5), &Value::Number(2.0)).unwrap(),
            Value::Number(0.25)
        );
    }

    #[test]
    fn test_divided_by_zero_division() {
        // Zero divided by non-zero
        assert_eq!(
            divided_by(&Value::Number(0.0), &Value::Number(5.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            divided_by(&Value::Number(0.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(0.0)
        );
    }

    #[test]
    fn test_divided_by_large_numbers() {
        // Very large numbers
        assert_eq!(
            divided_by(&Value::Number(1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(10.0)
        );
        assert_eq!(
            divided_by(&Value::Number(-1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(-10.0)
        );
    }

    #[test]
    fn test_divided_by_small_numbers() {
        // Very small numbers
        assert_eq!(
            divided_by(&Value::Number(1e-323), &Value::Number(1e-322)).unwrap(),
            Value::Number(0.1)
        );
        assert_eq!(
            divided_by(&Value::Number(-1e-323), &Value::Number(1e-322)).unwrap(),
            Value::Number(-0.1)
        );
    }

    #[test]
    fn test_divided_by_infinity() {
        // Infinity cases
        assert_eq!(
            divided_by(&Value::Number(f64::INFINITY), &Value::Number(2.0)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            divided_by(&Value::Number(2.0), &Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            divided_by(&Value::Number(f64::NEG_INFINITY), &Value::Number(2.0)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            divided_by(&Value::Number(2.0), &Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(0.0)
        );

        // Infinity divided by infinity = NaN
        let inf_div_inf =
            divided_by(&Value::Number(f64::INFINITY), &Value::Number(f64::INFINITY)).unwrap();
        match inf_div_inf {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", inf_div_inf),
        }

        // Negative infinity divided by infinity = NaN
        let neg_inf_div_inf = divided_by(
            &Value::Number(f64::NEG_INFINITY),
            &Value::Number(f64::INFINITY),
        )
        .unwrap();
        match neg_inf_div_inf {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", neg_inf_div_inf),
        }
    }

    #[test]
    fn test_divided_by_nan() {
        // NaN cases
        let nan_result = divided_by(&Value::Number(f64::NAN), &Value::Number(2.0)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = divided_by(&Value::Number(2.0), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = divided_by(&Value::Number(f64::NAN), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }
    }

    #[test]
    fn test_divided_by_type_errors() {
        // Type errors for first argument
        assert!(matches!(
            divided_by(&Value::String("test".to_string()), &Value::Number(2.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(&Value::Boolean(true), &Value::Number(2.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(&Value::Nil, &Value::Number(2.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(&Value::Array(vec![]), &Value::Number(2.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(
                &Value::Dictionary(std::collections::HashMap::new()),
                &Value::Number(2.0)
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(&Value::Symbol("sym".to_string()), &Value::Number(2.0)),
            Err(FunctionError::TypeError { .. })
        ));

        // Type errors for second argument
        assert!(matches!(
            divided_by(&Value::Number(10.0), &Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(&Value::Number(10.0), &Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(&Value::Number(10.0), &Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(&Value::Number(10.0), &Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(
                &Value::Number(10.0),
                &Value::Dictionary(std::collections::HashMap::new())
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            divided_by(&Value::Number(10.0), &Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_divided_by_negative_zero() {
        // Test with negative zero (though in f64, -0.0 == 0.0)
        let result = divided_by(&Value::Number(10.0), &Value::Number(-0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));

        let result = divided_by(&Value::Number(-0.0), &Value::Number(5.0));
        assert_eq!(result.unwrap(), Value::Number(0.0));
    }

    #[test]
    fn test_divided_by_precision() {
        // Test floating point precision
        let result = divided_by(&Value::Number(1.0), &Value::Number(3.0)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 0.3333333333333333).abs() < 1e-15);
        } else {
            panic!("Expected Number");
        }

        // Test with very precise decimals
        let result = divided_by(&Value::Number(1.23456789012345), &Value::Number(2.0)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 0.617283945061725).abs() < 1e-15);
        } else {
            panic!("Expected Number");
        }
    }

    #[test]
    fn test_modulo_basic() {
        // Basic positive modulo
        assert_eq!(
            modulo_fn(&Value::Number(10.0), &Value::Number(3.0)).unwrap(),
            Value::Number(1.0)
        );
        assert_eq!(
            modulo_fn(&Value::Number(15.0), &Value::Number(4.0)).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            modulo_fn(&Value::Number(7.0), &Value::Number(7.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            modulo_fn(&Value::Number(1.0), &Value::Number(2.0)).unwrap(),
            Value::Number(1.0)
        );
    }

    #[test]
    fn test_modulo_negative_numbers() {
        // Negative dividend
        assert_eq!(
            modulo_fn(&Value::Number(-10.0), &Value::Number(3.0)).unwrap(),
            Value::Number(-1.0)
        );
        assert_eq!(
            modulo_fn(&Value::Number(-15.0), &Value::Number(4.0)).unwrap(),
            Value::Number(-3.0)
        );

        // Negative divisor
        assert_eq!(
            modulo_fn(&Value::Number(10.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(1.0)
        );
        assert_eq!(
            modulo_fn(&Value::Number(15.0), &Value::Number(-4.0)).unwrap(),
            Value::Number(3.0)
        );

        // Both negative
        assert_eq!(
            modulo_fn(&Value::Number(-10.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(-1.0)
        );
        assert_eq!(
            modulo_fn(&Value::Number(-15.0), &Value::Number(-4.0)).unwrap(),
            Value::Number(-3.0)
        );
    }

    #[test]
    fn test_modulo_zero() {
        // Zero dividend
        assert_eq!(
            modulo_fn(&Value::Number(0.0), &Value::Number(5.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            modulo_fn(&Value::Number(0.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(0.0)
        );
    }

    #[test]
    fn test_modulo_fractions() {
        // Fractional numbers
        assert_eq!(
            modulo_fn(&Value::Number(5.5), &Value::Number(2.0)).unwrap(),
            Value::Number(1.5)
        );
        let result = modulo_fn(&Value::Number(10.7), &Value::Number(3.0)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 1.7).abs() < 1e-10);
        }
        assert_eq!(
            modulo_fn(&Value::Number(-5.5), &Value::Number(2.0)).unwrap(),
            Value::Number(-1.5)
        );
        assert_eq!(
            modulo_fn(&Value::Number(5.5), &Value::Number(-2.0)).unwrap(),
            Value::Number(1.5)
        );
        assert_eq!(
            modulo_fn(&Value::Number(-5.5), &Value::Number(-2.0)).unwrap(),
            Value::Number(-1.5)
        );
    }

    #[test]
    fn test_modulo_edge_cases() {
        // Very large numbers
        assert_eq!(
            modulo_fn(&Value::Number(1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(0.0)
        );

        // Very small numbers
        assert_eq!(
            modulo_fn(&Value::Number(1e-323), &Value::Number(1e-322)).unwrap(),
            Value::Number(1e-323)
        );

        // Infinity cases - modulo with infinity should be NaN, but since we use f64 % which gives NaN, but wait, the function doesn't handle it specially.
        // Actually, in Rust, inf % anything is NaN, but since we don't check, it will return NaN.
        let inf_mod = modulo_fn(&Value::Number(f64::INFINITY), &Value::Number(3.0)).unwrap();
        match inf_mod {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number"),
        }

        let mod_inf = modulo_fn(&Value::Number(10.0), &Value::Number(f64::INFINITY)).unwrap();
        match mod_inf {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number"),
        }

        // NaN cases
        let nan_mod = modulo_fn(&Value::Number(f64::NAN), &Value::Number(3.0)).unwrap();
        match nan_mod {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number"),
        }

        let mod_nan = modulo_fn(&Value::Number(10.0), &Value::Number(f64::NAN)).unwrap();
        match mod_nan {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_modulo_by_zero() {
        // Modulo by zero should error
        let result = modulo_fn(&Value::Number(10.0), &Value::Number(0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));

        let result = modulo_fn(&Value::Number(-5.0), &Value::Number(0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));

        let result = modulo_fn(&Value::Number(0.0), &Value::Number(0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));

        // Negative zero
        let result = modulo_fn(&Value::Number(10.0), &Value::Number(-0.0));
        assert!(matches!(
            result,
            Err(FunctionError::InvalidOperation { .. })
        ));
    }

    #[test]
    fn test_modulo_type_errors() {
        // First argument not Number
        assert!(matches!(
            modulo_fn(&Value::String("10".to_string()), &Value::Number(3.0)),
            Err(FunctionError::TypeError {
                expected: _,
                got: _
            })
        ));
        assert!(matches!(
            modulo_fn(&Value::Boolean(true), &Value::Number(3.0)),
            Err(FunctionError::TypeError { .. })
        ));

        // Second argument not Number
        assert!(matches!(
            modulo_fn(&Value::Number(10.0), &Value::String("3".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            modulo_fn(&Value::Number(10.0), &Value::Boolean(false)),
            Err(FunctionError::TypeError { .. })
        ));

        // Both not Number
        assert!(matches!(
            modulo_fn(&Value::Nil, &Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_max() {
        // Basic positive number max
        assert_eq!(
            max(&Value::Number(3.0), &Value::Number(7.0)).unwrap(),
            Value::Number(7.0)
        );
        assert_eq!(
            max(&Value::Number(10.0), &Value::Number(5.0)).unwrap(),
            Value::Number(10.0)
        );
        assert_eq!(
            max(&Value::Number(100.0), &Value::Number(50.0)).unwrap(),
            Value::Number(100.0)
        );

        // Negative number max
        assert_eq!(
            max(&Value::Number(-5.0), &Value::Number(3.0)).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            max(&Value::Number(5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            max(&Value::Number(-5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(-3.0)
        );

        // Zero cases
        assert_eq!(
            max(&Value::Number(0.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            max(&Value::Number(5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            max(&Value::Number(0.0), &Value::Number(5.0)).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            max(&Value::Number(-5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );

        // Decimal numbers
        assert_eq!(
            max(&Value::Number(3.14), &Value::Number(2.71)).unwrap(),
            Value::Number(3.14)
        );
        assert_eq!(
            max(&Value::Number(10.5), &Value::Number(10.25)).unwrap(),
            Value::Number(10.5)
        );
        assert_eq!(
            max(&Value::Number(1.1), &Value::Number(1.01)).unwrap(),
            Value::Number(1.1)
        );

        // Very large numbers
        assert_eq!(
            max(&Value::Number(1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(1e308)
        );
        assert_eq!(
            max(&Value::Number(-1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(1e307)
        );

        // Very small numbers
        assert_eq!(
            max(&Value::Number(1e-323), &Value::Number(5e-324)).unwrap(),
            Value::Number(1e-323)
        );
        assert_eq!(
            max(&Value::Number(-1e-323), &Value::Number(-5e-324)).unwrap(),
            Value::Number(-5e-324)
        );

        // Infinity cases
        assert_eq!(
            max(&Value::Number(f64::INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            max(&Value::Number(1.0), &Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(f64::INFINITY)
        );
        assert_eq!(
            max(&Value::Number(f64::NEG_INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(1.0)
        );
        assert_eq!(
            max(&Value::Number(1.0), &Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(1.0)
        );
        assert_eq!(
            max(
                &Value::Number(f64::INFINITY),
                &Value::Number(f64::NEG_INFINITY)
            )
            .unwrap(),
            Value::Number(f64::INFINITY)
        );

        // NaN cases - max returns the non-NaN value, or NaN if both are NaN
        let nan_result = max(&Value::Number(f64::NAN), &Value::Number(1.0)).unwrap();
        match nan_result {
            Value::Number(n) => assert_eq!(n, 1.0),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = max(&Value::Number(1.0), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert_eq!(n, 1.0),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = max(&Value::Number(f64::NAN), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        // Type errors for non-Number types
        assert!(matches!(
            max(&Value::String("test".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Number(1.0), &Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Boolean(true), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Number(1.0), &Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Nil, &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Number(1.0), &Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Array(vec![]), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Number(1.0), &Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(
                &Value::Dictionary(std::collections::HashMap::new()),
                &Value::Number(1.0)
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(
                &Value::Number(1.0),
                &Value::Dictionary(std::collections::HashMap::new())
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Symbol("sym".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            max(&Value::Number(1.0), &Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_min() {
        // Basic positive number min
        assert_eq!(
            min(&Value::Number(3.0), &Value::Number(7.0)).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            min(&Value::Number(10.0), &Value::Number(5.0)).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            min(&Value::Number(100.0), &Value::Number(50.0)).unwrap(),
            Value::Number(50.0)
        );

        // Negative number min
        assert_eq!(
            min(&Value::Number(-5.0), &Value::Number(3.0)).unwrap(),
            Value::Number(-5.0)
        );
        assert_eq!(
            min(&Value::Number(5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(-3.0)
        );
        assert_eq!(
            min(&Value::Number(-5.0), &Value::Number(-3.0)).unwrap(),
            Value::Number(-5.0)
        );

        // Zero cases
        assert_eq!(
            min(&Value::Number(0.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            min(&Value::Number(5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            min(&Value::Number(0.0), &Value::Number(5.0)).unwrap(),
            Value::Number(0.0)
        );
        assert_eq!(
            min(&Value::Number(-5.0), &Value::Number(0.0)).unwrap(),
            Value::Number(-5.0)
        );

        // Decimal numbers
        assert_eq!(
            min(&Value::Number(3.14), &Value::Number(2.71)).unwrap(),
            Value::Number(2.71)
        );
        assert_eq!(
            min(&Value::Number(10.5), &Value::Number(10.25)).unwrap(),
            Value::Number(10.25)
        );
        assert_eq!(
            min(&Value::Number(1.1), &Value::Number(1.01)).unwrap(),
            Value::Number(1.01)
        );

        // Very large numbers
        assert_eq!(
            min(&Value::Number(1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(1e307)
        );
        assert_eq!(
            min(&Value::Number(-1e308), &Value::Number(1e307)).unwrap(),
            Value::Number(-1e308)
        );

        // Very small numbers
        assert_eq!(
            min(&Value::Number(1e-323), &Value::Number(5e-324)).unwrap(),
            Value::Number(5e-324)
        );
        assert_eq!(
            min(&Value::Number(-1e-323), &Value::Number(-5e-324)).unwrap(),
            Value::Number(-1e-323)
        );

        // Infinity cases
        assert_eq!(
            min(&Value::Number(f64::INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(1.0)
        );
        assert_eq!(
            min(&Value::Number(1.0), &Value::Number(f64::INFINITY)).unwrap(),
            Value::Number(1.0)
        );
        assert_eq!(
            min(&Value::Number(f64::NEG_INFINITY), &Value::Number(1.0)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            min(&Value::Number(1.0), &Value::Number(f64::NEG_INFINITY)).unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );
        assert_eq!(
            min(
                &Value::Number(f64::INFINITY),
                &Value::Number(f64::NEG_INFINITY)
            )
            .unwrap(),
            Value::Number(f64::NEG_INFINITY)
        );

        // NaN cases - min returns the non-NaN value, or NaN if both are NaN
        let nan_result = min(&Value::Number(f64::NAN), &Value::Number(1.0)).unwrap();
        match nan_result {
            Value::Number(n) => assert_eq!(n, 1.0),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = min(&Value::Number(1.0), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert_eq!(n, 1.0),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        let nan_result = min(&Value::Number(f64::NAN), &Value::Number(f64::NAN)).unwrap();
        match nan_result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number, got {:?}", nan_result),
        }

        // Type errors for non-Number types
        assert!(matches!(
            min(&Value::String("test".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Number(1.0), &Value::String("test".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Boolean(true), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Number(1.0), &Value::Boolean(true)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Nil, &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Number(1.0), &Value::Nil),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Array(vec![]), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Number(1.0), &Value::Array(vec![])),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(
                &Value::Dictionary(std::collections::HashMap::new()),
                &Value::Number(1.0)
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(
                &Value::Number(1.0),
                &Value::Dictionary(std::collections::HashMap::new())
            ),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Symbol("sym".to_string()), &Value::Number(1.0)),
            Err(FunctionError::TypeError { .. })
        ));
        assert!(matches!(
            min(&Value::Number(1.0), &Value::Symbol("sym".to_string())),
            Err(FunctionError::TypeError { .. })
        ));
    }

    #[test]
    fn test_pmt_basic() {
        let result = pmt(
            &Value::Number(0.00375),
            &Value::Number(360.0),
            &Value::Number(250000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(payment) = result {
            assert!(
                (payment + 1266.71).abs() < 0.01,
                "Expected ~-1266.71, got {}",
                payment
            );
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_pmt_zero_interest() {
        let result = pmt(
            &Value::Number(0.0),
            &Value::Number(12.0),
            &Value::Number(12000.0),
            &Value::Number(0.0),
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
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_pmt_type_1() {
        // Test type 1 (payment at beginning of period)
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Number(1.0),
        )
        .unwrap();
        if let Value::Number(payment) = result {
            // For type 1, payment should be adjusted from type 0
            let type_0_result = pmt(
                &Value::Number(0.05),
                &Value::Number(12.0),
                &Value::Number(10000.0),
                &Value::Number(0.0),
            )
            .unwrap();
            if let Value::Number(type_0_payment) = type_0_result {
                // Type 1 payment should be type 0 payment / (1 + rate)
                let expected = type_0_payment / 1.05;
                assert!(
                    (payment - expected).abs() < 0.01,
                    "Expected ~{}, got {}",
                    expected,
                    payment
                );
                // Since both are negative, type 1 should be greater than type 0
                assert!(
                    payment > type_0_payment,
                    "Type 1 payment {} should be greater than type 0 payment {} (less negative)",
                    payment,
                    type_0_payment
                );
            }
        }
    }

    #[test]
    fn test_pmt_invalid_type() {
        // Test invalid type values
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Number(2.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));

        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Number(-1.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));

        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Number(0.5),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_pmt_negative_nper() {
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(-12.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_pmt_fractional_nper() {
        // Test with fractional number of periods
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.5),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        assert!(matches!(result, Value::Number(_)));
    }

    #[test]
    fn test_pmt_negative_rate() {
        // Test with negative interest rate
        let result = pmt(
            &Value::Number(-0.05),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        assert!(matches!(result, Value::Number(_)));
        if let Value::Number(payment) = result {
            // With positive PV and negative rate, payment should be negative
            assert!(payment < 0.0, "Payment should be negative: {}", payment);
        }
    }

    #[test]
    fn test_pmt_negative_pv() {
        // Test with negative present value (loan amount)
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.0),
            &Value::Number(-10000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(payment) = result {
            // Negative PV should give positive payment
            assert!(payment > 0.0, "Payment should be positive: {}", payment);
        }
    }

    #[test]
    fn test_pmt_zero_pv() {
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.0),
            &Value::Number(0.0),
            &Value::Number(0.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_pmt_edge_cases() {
        // Test with very small rate
        let result = pmt(
            &Value::Number(1e-10),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        assert!(matches!(result, Value::Number(_)));

        // Test with very large rate
        let result = pmt(
            &Value::Number(1e10),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        assert!(matches!(result, Value::Number(_)));

        // Test with very large nper
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(1e6),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        assert!(matches!(result, Value::Number(_)));
    }

    #[test]
    fn test_pmt_type_errors() {
        // Test type errors for rate
        let result = pmt(
            &Value::String("0.05".to_string()),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // Test type errors for nper
        let result = pmt(
            &Value::Number(0.05),
            &Value::Boolean(true),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // Test type errors for pv
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.0),
            &Value::Array(vec![]),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // Test type errors for type_
        let result = pmt(
            &Value::Number(0.05),
            &Value::Number(12.0),
            &Value::Number(10000.0),
            &Value::Nil,
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_pmt_zero_interest_type_1() {
        // Test zero interest with type 1
        let result = pmt(
            &Value::Number(0.0),
            &Value::Number(12.0),
            &Value::Number(12000.0),
            &Value::Number(1.0),
        )
        .unwrap();
        // Should be the same as type 0 for zero interest
        assert_eq!(result, Value::Number(-1000.0));
    }

    #[test]
    fn test_pv_basic() {
        let result = pv(
            &Value::Number(0.0067),
            &Value::Number(240.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        if let Value::Number(pv) = result {
            assert!(
                pv > 119000.0 && pv < 120000.0,
                "PV out of expected range: {}",
                pv
            );
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
    fn test_pv_invalid_nper() {
        let result = pv(
            &Value::Number(0.05),
            &Value::Number(0.0),
            &Value::Number(-1000.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));

        let result = pv(
            &Value::Number(0.05),
            &Value::Number(-5.0),
            &Value::Number(-1000.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_pv_type_errors() {
        // rate not Number
        let result = pv(
            &Value::String("0.05".to_string()),
            &Value::Number(10.0),
            &Value::Number(-1000.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // nper not Number
        let result = pv(
            &Value::Number(0.05),
            &Value::Boolean(true),
            &Value::Number(-1000.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // pmt not Number
        let result = pv(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Array(vec![]),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_pv_positive_pmt() {
        // Test with positive pmt (inflows)
        let result = pv(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
        )
        .unwrap();
        if let Value::Number(pv) = result {
            assert!(
                pv < -7700.0 && pv > -7750.0,
                "PV out of expected range: {}",
                pv
            );
        }

        // Zero rate with positive pmt
        let result = pv(
            &Value::Number(0.0),
            &Value::Number(12.0),
            &Value::Number(1000.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(-12000.0));
    }

    #[test]
    fn test_pv_negative_rate() {
        // Test with negative rate
        let result = pv(
            &Value::Number(-0.05),
            &Value::Number(10.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        if let Value::Number(pv) = result {
            assert!(
                pv > 13300.0 && pv < 13500.0,
                "PV out of expected range: {}",
                pv
            );
        }
    }

    #[test]
    fn test_pv_small_rate() {
        // Test with very small positive rate
        let result = pv(
            &Value::Number(1e-6),
            &Value::Number(1000.0),
            &Value::Number(-100.0),
        )
        .unwrap();
        if let Value::Number(pv) = result {
            assert!(
                pv > 99500.0 && pv < 100500.0,
                "PV out of expected range: {}",
                pv
            );
        }
    }

    #[test]
    fn test_pv_large_nper() {
        // Test with large nper
        let result = pv(
            &Value::Number(0.01),
            &Value::Number(10000.0),
            &Value::Number(-100.0),
        )
        .unwrap();
        if let Value::Number(pv) = result {
            assert!(
                pv > 9900.0 && pv < 10100.0,
                "PV out of expected range: {}",
                pv
            );
        }
    }

    #[test]
    fn test_fv_basic() {
        let result = fv(
            &Value::Number(0.005),
            &Value::Number(120.0),
            &Value::Number(-100.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        if let Value::Number(fv) = result {
            assert!(
                fv > 18000.0 && fv < 18500.0,
                "FV out of expected range: {}",
                fv
            );
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
        assert_eq!(result, Value::Number(2200.0));
    }

    #[test]
    fn test_nper_basic() {
        let result = nper(
            &Value::Number(0.075 / 12.0),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(periods) = result {
            assert!(
                periods > 35.0 && periods < 36.0,
                "NPER out of expected range: {}",
                periods
            );
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
    fn test_nper_pv_zero() {
        let result = nper(
            &Value::Number(0.05),
            &Value::Number(-100.0),
            &Value::Number(0.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_nper_pmt_zero() {
        let result = nper(
            &Value::Number(0.05),
            &Value::Number(0.0),
            &Value::Number(1000.0),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_nper_type_error_rate() {
        let result = nper(
            &Value::String("0.05".to_string()),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        );
        assert!(result.is_err());
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "String");
        }
    }

    #[test]
    fn test_nper_type_error_pmt() {
        let result = nper(
            &Value::Number(0.05),
            &Value::String("-200".to_string()),
            &Value::Number(8000.0),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_nper_type_error_pv() {
        let result = nper(
            &Value::Number(0.05),
            &Value::Number(-200.0),
            &Value::String("8000".to_string()),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_nper_negative_rate() {
        let result = nper(
            &Value::Number(-0.01),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        );
        // Depending on if negative rate is allowed, but for now, it may compute or error.
        // Since the check may trigger, let's see.
        // For r negative, v*r negative, denominator = p - negative = p + positive, etc.
        // Perhaps it works.
        // For now, just check it doesn't panic.
        let _ = result;
    }

    #[test]
    fn test_nper_small_rate() {
        let result = nper(
            &Value::Number(1e-10),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(periods) = result {
            assert!(periods > 0.0);
        }
    }

    #[test]
    fn test_rate_basic() {
        let result = rate(
            &Value::Number(48.0),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(r) = result {
            assert!(r > 0.007 && r < 0.008, "Rate out of expected range: {}", r);
        }
    }

    #[test]
    fn test_rate_zero_interest() {
        // Test case with low interest rate
        // Large loan, payments over long period
        let result = rate(
            &Value::Number(120.0),
            &Value::Number(-100.0),
            &Value::Number(10000.0),
        )
        .unwrap();
        if let Value::Number(r) = result {
            assert!(r > 0.0 && r < 0.01, "Rate should be low positive: {}", r);
        }
    }

    #[test]
    fn test_rate_negative_pv() {
        // Test with negative PV (investment scenario)
        let result = rate(
            &Value::Number(5.0),
            &Value::Number(1900.0),   // positive cash flows
            &Value::Number(-10000.0), // negative PV (investment)
        )
        .unwrap();
        if let Value::Number(r) = result {
            // Should give positive rate
            assert!(r > 0.0, "Rate should be positive for investment: {}", r);
        }
    }

    #[test]
    fn test_rate_small_nper() {
        // Test with small number of periods
        let result = rate(
            &Value::Number(1.0),
            &Value::Number(-5000.0),
            &Value::Number(5000.0),
        )
        .unwrap();
        if let Value::Number(r) = result {
            assert!(r >= 0.0, "Rate should be non-negative: {}", r);
        }
    }

    #[test]
    fn test_rate_large_nper() {
        // Test with large number of periods
        let result = rate(
            &Value::Number(360.0), // 30 years monthly
            &Value::Number(-500.0),
            &Value::Number(100000.0),
        )
        .unwrap();
        if let Value::Number(r) = result {
            assert!(
                r > 0.0 && r < 0.01,
                "Rate out of expected range for mortgage: {}",
                r
            );
        }
    }

    #[test]
    fn test_rate_invalid_nper_zero() {
        let result = rate(
            &Value::Number(0.0),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_rate_invalid_nper_negative() {
        let result = rate(
            &Value::Number(-5.0),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_rate_type_error_nper() {
        let result = rate(
            &Value::String("48".to_string()),
            &Value::Number(-200.0),
            &Value::Number(8000.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_rate_type_error_pmt() {
        let result = rate(
            &Value::Number(48.0),
            &Value::Boolean(true),
            &Value::Number(8000.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_rate_type_error_pv() {
        let result = rate(
            &Value::Number(48.0),
            &Value::Number(-200.0),
            &Value::Array(vec![]),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_rate_convergence_failure() {
        // Test case that might not converge easily
        // Very high rate scenario
        let result = rate(
            &Value::Number(12.0),
            &Value::Number(-100.0),
            &Value::Number(1000.0),
        );
        // This should either converge or error gracefully
        match result {
            Ok(Value::Number(r)) => assert!(r > 0.0, "Rate should be positive: {}", r),
            Err(FunctionError::ArgumentError { message }) => {
                assert!(
                    message.contains("converge") || message.contains("iterations"),
                    "Should be convergence or iteration error: {}",
                    message
                );
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_rate_fractional_nper() {
        // Test with fractional periods
        let result = rate(
            &Value::Number(12.5),
            &Value::Number(-200.0),
            &Value::Number(2000.0),
        )
        .unwrap();
        if let Value::Number(r) = result {
            assert!(r > 0.0, "Rate should be positive: {}", r);
        }
    }

    #[test]
    fn test_rate_very_small_values() {
        // Test with very small values
        let result = rate(
            &Value::Number(10.0),
            &Value::Number(-0.01),
            &Value::Number(1.0),
        )
        .unwrap();
        if let Value::Number(r) = result {
            assert!(r > 0.0, "Rate should be positive: {}", r);
        }
    }

    #[test]
    fn test_rate_very_large_values() {
        // Test with very large values
        let result = rate(
            &Value::Number(100.0),
            &Value::Number(-1000000.0),
            &Value::Number(100000000.0),
        )
        .unwrap();
        if let Value::Number(r) = result {
            assert!(r > 0.0, "Rate should be positive: {}", r);
        }
    }

    #[test]
    fn test_npv_basic() {
        let cash_flows = Value::Array(vec![
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = npv(&Value::Number(0.1), &cash_flows).unwrap();
        if let Value::Number(npv) = result {
            assert!(
                (npv - 11310.22).abs() < 5.0,
                "NPV not close to expected: {} vs 11310.22",
                npv
            );
        }
    }

    #[test]
    fn test_npv_zero_rate() {
        let cash_flows = Value::Array(vec![
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = npv(&Value::Number(0.0), &cash_flows).unwrap();
        if let Value::Number(npv) = result {
            assert!(
                (npv - 14000.0).abs() < 0.01,
                "NPV with zero rate should be sum: {} vs 14000.0",
                npv
            );
        }
    }

    #[test]
    fn test_npv_empty_array() {
        let cash_flows = Value::Array(vec![]);
        let result = npv(&Value::Number(0.1), &cash_flows);
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_npv_single_value() {
        let cash_flows = Value::Array(vec![Value::Number(5000.0)]);
        let result = npv(&Value::Number(0.1), &cash_flows).unwrap();
        if let Value::Number(npv) = result {
            assert!(
                (npv - 4545.45).abs() < 0.01,
                "NPV for single value: {} vs 4545.45",
                npv
            );
        }
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
    fn test_npv_negative_cash_flows() {
        let cash_flows = Value::Array(vec![Value::Number(-2000.0), Value::Number(-3000.0)]);
        let result = npv(&Value::Number(0.1), &cash_flows).unwrap();
        if let Value::Number(npv) = result {
            assert!(
                (npv - (-4297.52)).abs() < 1.0,
                "NPV for negative flows: {} vs -4297.52",
                npv
            );
        }
    }

    #[test]
    fn test_npv_mixed_cash_flows() {
        let cash_flows = Value::Array(vec![
            Value::Number(-5000.0),
            Value::Number(3000.0),
            Value::Number(4000.0),
        ]);
        let result = npv(&Value::Number(0.1), &cash_flows).unwrap();
        if let Value::Number(npv) = result {
            // -5000/1.1 + 3000/1.21 + 4000/1.331 ≈ -4545.45 + 2479.34 + 3005.99 ≈ 939.88
            assert!(
                (npv - 939.88).abs() < 1.0,
                "NPV for mixed flows: {} vs 939.88",
                npv
            );
        }
    }

    #[test]
    fn test_npv_rate_not_number() {
        let cash_flows = Value::Array(vec![Value::Number(1000.0)]);
        let result = npv(&Value::String("0.1".to_string()), &cash_flows);
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_npv_values_not_array() {
        let result = npv(&Value::Number(0.1), &Value::Number(1000.0));
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_irr_basic() {
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
            assert!(
                irr_rate > 0.16 && irr_rate < 0.17,
                "IRR out of expected range: {}",
                irr_rate
            );
        }
    }

    #[test]
    fn test_irr_no_positive_values() {
        let cash_flows = Value::Array(vec![Value::Number(-10000.0), Value::Number(-3000.0)]);
        let result = irr(&cash_flows);
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_irr_no_negative_values() {
        let cash_flows = Value::Array(vec![Value::Number(10000.0), Value::Number(3000.0)]);
        let result = irr(&cash_flows);
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_irr_empty_array() {
        let cash_flows = Value::Array(vec![]);
        let result = irr(&cash_flows);
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_irr_single_value() {
        let cash_flows = Value::Array(vec![Value::Number(-10000.0)]);
        let result = irr(&cash_flows);
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_irr_two_values_negative_positive() {
        let cash_flows = Value::Array(vec![Value::Number(-1000.0), Value::Number(1100.0)]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
            assert!(
                irr_rate > 0.09 && irr_rate < 0.11,
                "IRR out of expected range: {}",
                irr_rate
            );
        }
    }

    #[test]
    fn test_irr_with_zeros() {
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::Number(0.0),
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
            assert!(
                irr_rate > 0.10 && irr_rate < 0.12,
                "IRR out of expected range: {}",
                irr_rate
            );
        }
    }

    #[test]
    fn test_irr_negative_irr() {
        // Cash flows that should give negative IRR
        let cash_flows = Value::Array(vec![Value::Number(1000.0), Value::Number(-800.0)]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
            assert!(
                (irr_rate + 0.2).abs() < 1e-6,
                "IRR should be approximately -0.2: {}",
                irr_rate
            );
        }
    }

    #[test]
    fn test_irr_type_error_not_array() {
        let result = irr(&Value::Number(1000.0));
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_irr_type_error_array_with_non_number() {
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::String("3000".to_string()),
        ]);
        let result = irr(&cash_flows);
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_irr_precision() {
        // Test with known precise IRR
        let cash_flows = Value::Array(vec![Value::Number(-1000.0), Value::Number(1100.0)]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
            // IRR should be approximately 0.1 (10%)
            assert!(
                (irr_rate - 0.1).abs() < 1e-6,
                "IRR should be close to 0.1: {}",
                irr_rate
            );
        }
    }

    #[test]
    fn test_irr_large_numbers() {
        let cash_flows = Value::Array(vec![
            Value::Number(-1e10),
            Value::Number(3e9),
            Value::Number(4.2e9),
            Value::Number(6.8e9),
        ]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
            assert!(
                irr_rate > 0.16 && irr_rate < 0.17,
                "IRR out of expected range for large numbers: {}",
                irr_rate
            );
        }
    }

    #[test]
    fn test_irr_small_numbers() {
        let cash_flows = Value::Array(vec![
            Value::Number(-1e-6),
            Value::Number(3e-7),
            Value::Number(4.2e-7),
            Value::Number(6.8e-7),
        ]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
            assert!(
                irr_rate > 0.16 && irr_rate < 0.17,
                "IRR out of expected range for small numbers: {}",
                irr_rate
            );
        }
    }

    #[test]
    fn test_irr_with_nan() {
        let cash_flows = Value::Array(vec![Value::Number(f64::NAN), Value::Number(3000.0)]);
        let result = irr(&cash_flows);
        // Should either error or return NaN, but since we convert to f64, it will proceed
        // Actually, since we collect into Vec<f64>, NaN will be there, and NPV will be NaN
        // But the algorithm might not converge properly
        let _ = result; // For now, just check it doesn't panic
    }

    #[test]
    fn test_irr_non_convergence() {
        // Create a case that might not converge easily
        // This is hard to construct, but let's try cash flows that oscillate
        let cash_flows = Value::Array(vec![
            Value::Number(-1000.0),
            Value::Number(2000.0),
            Value::Number(-1500.0),
            Value::Number(1200.0),
        ]);
        let result = irr(&cash_flows);
        // May or may not converge, but should not panic
        let _ = result;
    }

    #[test]
    fn test_mirr_basic() {
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate > 0.15 && mirr_rate < 0.16,
                "MIRR out of expected range: {}",
                mirr_rate
            );
        }
    }

    #[test]
    fn test_mirr_different_rates() {
        // Test with different finance and reinvestment rates
        let cash_flows = Value::Array(vec![
            Value::Number(-5000.0),
            Value::Number(1500.0),
            Value::Number(2000.0),
            Value::Number(2500.0),
        ]);

        // Same rates
        let result = mirr(&cash_flows, &Value::Number(0.08), &Value::Number(0.08)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate > 0.08 && mirr_rate < 0.09,
                "MIRR with same rates out of expected range: {}",
                mirr_rate
            );
        }

        // Higher reinvestment rate
        let result = mirr(&cash_flows, &Value::Number(0.05), &Value::Number(0.15)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate > 0.10 && mirr_rate < 0.12,
                "MIRR with higher reinvestment rate out of expected range: {}",
                mirr_rate
            );
        }
    }

    #[test]
    fn test_mirr_zero_rates() {
        let cash_flows = Value::Array(vec![
            Value::Number(-1000.0),
            Value::Number(300.0),
            Value::Number(400.0),
            Value::Number(500.0),
        ]);

        // Zero finance rate
        let result = mirr(&cash_flows, &Value::Number(0.0), &Value::Number(0.1)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate > 0.09 && mirr_rate < 0.10,
                "MIRR with zero finance rate out of expected range: {}",
                mirr_rate
            );
        }

        // Zero reinvestment rate
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.0)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate > 0.06 && mirr_rate < 0.07,
                "MIRR with zero reinvestment rate out of expected range: {}",
                mirr_rate
            );
        }
    }

    #[test]
    fn test_mirr_negative_rates() {
        let cash_flows = Value::Array(vec![
            Value::Number(-2000.0),
            Value::Number(800.0),
            Value::Number(1000.0),
        ]);

        // Negative finance rate
        let result = mirr(&cash_flows, &Value::Number(-0.05), &Value::Number(0.1)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate > -0.04 && mirr_rate < -0.02,
                "MIRR with negative finance rate out of expected range: {}",
                mirr_rate
            );
        }

        // Negative reinvestment rate
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(-0.05)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate > -0.07 && mirr_rate < -0.05,
                "MIRR with negative reinvestment rate out of expected range: {}",
                mirr_rate
            );
        }
    }

    #[test]
    fn test_mirr_all_positive_cash_flows() {
        // Should error - no negative cash flows
        let cash_flows = Value::Array(vec![
            Value::Number(1000.0),
            Value::Number(2000.0),
            Value::Number(3000.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
        if let Err(FunctionError::ArgumentError { message }) = result {
            assert_eq!(
                message,
                "cash flows must contain both positive and negative values"
            );
        }
    }

    #[test]
    fn test_mirr_all_negative_cash_flows() {
        // Should error - no positive cash flows
        let cash_flows = Value::Array(vec![
            Value::Number(-1000.0),
            Value::Number(-2000.0),
            Value::Number(-3000.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
        if let Err(FunctionError::ArgumentError { message }) = result {
            assert_eq!(
                message,
                "cash flows must contain both positive and negative values"
            );
        }
    }

    #[test]
    fn test_mirr_with_zeros() {
        let cash_flows = Value::Array(vec![
            Value::Number(-1000.0),
            Value::Number(0.0),
            Value::Number(500.0),
            Value::Number(0.0),
            Value::Number(600.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate > 0.05 && mirr_rate < 0.06,
                "MIRR with zeros out of expected range: {}",
                mirr_rate
            );
        }
    }

    #[test]
    fn test_mirr_single_negative_single_positive() {
        let cash_flows = Value::Array(vec![Value::Number(-1000.0), Value::Number(1200.0)]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            // MIRR = (1200 / 1000)^(1/1) - 1 = 1.2 - 1 = 0.2
            assert!(
                (mirr_rate - 0.2).abs() < 0.001,
                "MIRR for simple case should be 0.2, got: {}",
                mirr_rate
            );
        }
    }

    #[test]
    fn test_mirr_empty_array() {
        let cash_flows = Value::Array(vec![]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
        if let Err(FunctionError::ArgumentError { message }) = result {
            assert_eq!(message, "values array cannot be empty");
        }
    }

    #[test]
    fn test_mirr_invalid_array_element() {
        let cash_flows = Value::Array(vec![
            Value::Number(-1000.0),
            Value::String("invalid".to_string()),
            Value::Number(500.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12));
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Array of Numbers");
            assert_eq!(got, "Array containing String");
        }
    }

    #[test]
    fn test_mirr_finance_rate_not_number() {
        let cash_flows = Value::Array(vec![Value::Number(-1000.0), Value::Number(1200.0)]);
        let result = mirr(
            &cash_flows,
            &Value::String("0.1".to_string()),
            &Value::Number(0.12),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "String");
        }
    }

    #[test]
    fn test_mirr_reinvest_rate_not_number() {
        let cash_flows = Value::Array(vec![Value::Number(-1000.0), Value::Number(1200.0)]);
        let result = mirr(
            &cash_flows,
            &Value::Number(0.1),
            &Value::String("0.12".to_string()),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "String");
        }
    }

    #[test]
    fn test_mirr_values_not_array() {
        let result = mirr(
            &Value::Number(1000.0),
            &Value::Number(0.1),
            &Value::Number(0.12),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Array");
            assert_eq!(got, "Number");
        }
    }

    #[test]
    fn test_mirr_extreme_values() {
        // Very large numbers
        let cash_flows = Value::Array(vec![
            Value::Number(-1e10),
            Value::Number(3e9),
            Value::Number(4e9),
            Value::Number(5e9),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate.is_finite(),
                "MIRR should be finite for large numbers"
            );
        }

        // Very small numbers
        let cash_flows = Value::Array(vec![
            Value::Number(-1e-10),
            Value::Number(3e-10),
            Value::Number(4e-10),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate.is_finite(),
                "MIRR should be finite for small numbers"
            );
        }
    }

    #[test]
    fn test_mirr_infinity_and_nan() {
        // Test with infinity in cash flows - negative infinity should work
        let cash_flows = Value::Array(vec![
            Value::Number(f64::NEG_INFINITY),
            Value::Number(1000.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            // With negative infinity as initial investment, MIRR should be defined
            assert!(
                mirr_rate.is_finite(),
                "MIRR with negative infinity should be finite"
            );
        }

        // Test with NaN in reinvest rate (should propagate NaN)
        let cash_flows = Value::Array(vec![
            Value::Number(-1000.0),
            Value::Number(300.0),
            Value::Number(400.0),
            Value::Number(500.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(f64::NAN)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate.is_nan(),
                "MIRR with NaN reinvest rate should be NaN"
            );
        }

        // Test with NaN in finance rate for non-zero periods
        let cash_flows = Value::Array(vec![
            Value::Number(-1000.0),
            Value::Number(-500.0),
            Value::Number(2000.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(f64::NAN), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(
                mirr_rate.is_nan(),
                "MIRR with NaN finance rate and multiple periods should be NaN"
            );
        }
    }

    #[test]
    fn test_mirr_precision() {
        // Test precision with known values
        let cash_flows = Value::Array(vec![
            Value::Number(-1000.0),
            Value::Number(600.0),
            Value::Number(600.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.1)).unwrap();
        if let Value::Number(mirr_rate) = result {
            // PV_negative = -1000
            // FV_positive = 600 * 1.1^(2-1) + 600 * 1.1^(2-2) = 600*1.1 + 600*1 = 660 + 600 = 1260
            // MIRR = (1260 / 1000)^(1/2) - 1 = 1.26^0.5 - 1 ≈ 1.1225 - 1 = 0.1225
            assert!(
                (mirr_rate - 0.1225).abs() < 0.001,
                "MIRR precision test failed: expected ~0.1225, got {}",
                mirr_rate
            );
        }
    }

    #[test]
    fn test_sln_basic() {
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
    fn test_sln_negative_life() {
        let result = sln(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(-5.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_sln_cost_equals_salvage() {
        let result = sln(
            &Value::Number(10000.0),
            &Value::Number(10000.0),
            &Value::Number(5.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_sln_cost_less_than_salvage() {
        let result = sln(
            &Value::Number(5000.0),
            &Value::Number(10000.0),
            &Value::Number(5.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(-1000.0));
    }

    #[test]
    fn test_sln_fractional_life() {
        let result = sln(
            &Value::Number(12000.0),
            &Value::Number(2000.0),
            &Value::Number(3.5),
        )
        .unwrap();
        assert_eq!(result, Value::Number(2857.1428571428573));
    }

    #[test]
    fn test_sln_large_numbers() {
        let result = sln(
            &Value::Number(1_000_000.0),
            &Value::Number(100_000.0),
            &Value::Number(10.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(90_000.0));
    }

    #[test]
    fn test_sln_small_numbers() {
        let result = sln(
            &Value::Number(100.0),
            &Value::Number(10.0),
            &Value::Number(2.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(45.0));
    }

    #[test]
    fn test_sln_very_small_life() {
        let result = sln(
            &Value::Number(1000.0),
            &Value::Number(100.0),
            &Value::Number(0.1),
        )
        .unwrap();
        assert_eq!(result, Value::Number(9000.0));
    }

    #[test]
    fn test_sln_type_error_cost() {
        let result = sln(
            &Value::String("30000".to_string()),
            &Value::Number(7500.0),
            &Value::Number(10.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "String");
        }
    }

    #[test]
    fn test_sln_type_error_salvage() {
        let result = sln(
            &Value::Number(30000.0),
            &Value::Boolean(true),
            &Value::Number(10.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "Boolean");
        }
    }

    #[test]
    fn test_sln_type_error_life() {
        let result = sln(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Array(vec![]),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "Array");
        }
    }

    #[test]
    fn test_sln_all_type_errors() {
        // Test various non-Number types for each parameter
        let string_val = Value::String("test".to_string());
        let bool_val = Value::Boolean(true);
        let nil_val = Value::Nil;
        let array_val = Value::Array(vec![]);
        let dict_val = Value::Dictionary(HashMap::new());
        let symbol_val = Value::Symbol("test".to_string());
        let num_val = Value::Number(30000.0);
        let salvage_val = Value::Number(7500.0);
        let life_val = Value::Number(10.0);

        let test_cases = vec![
            (&string_val, &salvage_val, &life_val),
            (&bool_val, &salvage_val, &life_val),
            (&nil_val, &salvage_val, &life_val),
            (&array_val, &salvage_val, &life_val),
            (&dict_val, &salvage_val, &life_val),
            (&symbol_val, &salvage_val, &life_val),
            (&num_val, &string_val, &life_val),
            (&num_val, &bool_val, &life_val),
            (&num_val, &nil_val, &life_val),
            (&num_val, &array_val, &life_val),
            (&num_val, &dict_val, &life_val),
            (&num_val, &symbol_val, &life_val),
            (&num_val, &salvage_val, &string_val),
            (&num_val, &salvage_val, &bool_val),
            (&num_val, &salvage_val, &nil_val),
            (&num_val, &salvage_val, &array_val),
            (&num_val, &salvage_val, &dict_val),
            (&num_val, &salvage_val, &symbol_val),
        ];

        for (cost, salvage, life) in test_cases {
            let result = sln(cost, salvage, life);
            assert!(
                matches!(result, Err(FunctionError::TypeError { .. })),
                "Expected TypeError for cost={:?}, salvage={:?}, life={:?}",
                cost,
                salvage,
                life
            );
        }
    }

    #[test]
    fn test_sln_precision() {
        // Test precision with floating point arithmetic
        let result = sln(
            &Value::Number(10000.0),
            &Value::Number(2500.0),
            &Value::Number(7.0),
        )
        .unwrap();
        if let Value::Number(depr) = result {
            assert!((depr - 1071.4285714285713).abs() < 1e-10);
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_sln_zero_cost() {
        let result = sln(
            &Value::Number(0.0),
            &Value::Number(0.0),
            &Value::Number(5.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_sln_negative_cost() {
        let result = sln(
            &Value::Number(-10000.0),
            &Value::Number(-2000.0),
            &Value::Number(5.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(-1600.0));
    }

    #[test]
    fn test_ddb_basic() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(1.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(6000.0));
    }

    #[test]
    fn test_ddb_period_10() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(10.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0)); // Capped to not go below salvage
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
    fn test_ddb_non_integer_period() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(1.5),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
        if let Err(FunctionError::ArgumentError { message }) = result {
            assert_eq!(message, "period must be an integer");
        }
    }

    #[test]
    fn test_ddb_period_3() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(3.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(3840.0)); // 24000 * 0.2 * 0.8? Wait, let's calculate properly
                                                   // Year 1: 30000 * 0.2 = 6000, book = 24000
                                                   // Year 2: 24000 * 0.2 = 4800, book = 19200
                                                   // Year 3: 19200 * 0.2 = 3840, book = 15360
                                                   // Yes
    }

    #[test]
    fn test_ddb_period_4() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(4.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(3072.0));
        // Year 4: 15360 * 0.2 = 3072, book = 12288
    }

    #[test]
    fn test_ddb_period_5() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(5.0),
        )
        .unwrap();
        if let Value::Number(n) = result {
            assert!((n - 2457.6).abs() < 1e-10);
        }
        // Year 5: 12288 * 0.2 = 2457.6, book = 9830.4
    }

    #[test]
    fn test_ddb_period_6() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(6.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(1966.08));
        // Year 6: 9830.4 * 0.2 = 1966.08, book = 7864.32
    }

    #[test]
    fn test_ddb_period_7() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(7.0),
        )
        .unwrap();
        if let Value::Number(n) = result {
            assert!((n - 364.32).abs() < 1e-10);
        }
        // Year 7: capped to 7864.32 - 7500 = 364.32
    }

    #[test]
    fn test_ddb_period_8() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(8.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));
        // Year 8: capped to 0
    }

    #[test]
    fn test_ddb_period_9() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(9.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0)); // Capped to not go below salvage
    }

    #[test]
    fn test_ddb_zero_life() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(0.0),
            &Value::Number(1.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ddb_negative_life() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(-5.0),
            &Value::Number(1.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ddb_period_zero() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ddb_cost_less_than_salvage() {
        let result = ddb(
            &Value::Number(5000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(1.0),
        )
        .unwrap();
        // Since cost < salvage, depreciation should be 0
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_ddb_type_error_cost() {
        let result = ddb(
            &Value::String("30000".to_string()),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(1.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ddb_type_error_salvage() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Boolean(true),
            &Value::Number(10.0),
            &Value::Number(1.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ddb_type_error_life() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Array(vec![]),
            &Value::Number(1.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ddb_type_error_period() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Nil,
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ddb_large_numbers() {
        let result = ddb(
            &Value::Number(1e10),
            &Value::Number(1e9),
            &Value::Number(10.0),
            &Value::Number(1.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(2e9));
    }

    #[test]
    fn test_ddb_small_numbers() {
        let result = ddb(
            &Value::Number(100.0),
            &Value::Number(10.0),
            &Value::Number(10.0),
            &Value::Number(1.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    #[test]
    fn test_ddb_zero_salvage() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(0.0),
            &Value::Number(10.0),
            &Value::Number(1.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(6000.0));
    }

    #[test]
    fn test_db_basic() {
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        )
        .unwrap();
        if let Value::Number(depr) = result {
            // DB(1000000, 100000, 6, 1, 7) = 1000000 * 0.319 * 7/12 = 186,083.33
            assert!(
                depr > 185000.0 && depr < 187000.0,
                "DB depreciation out of range: {}",
                depr
            );
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
    fn test_db_invalid_period_not_integer() {
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(1.5),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_db_period_2() {
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(2.0),
            &Value::Number(7.0),
        )
        .unwrap();
        if let Value::Number(depr) = result {
            // Period 2: (1000000 - 186083.33) * 0.319 ≈ 259,639.42
            assert!(
                depr > 258000.0 && depr < 261000.0,
                "DB period 2 depreciation out of range: {}",
                depr
            );
        }
    }

    #[test]
    fn test_db_period_6() {
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(6.0),
            &Value::Number(7.0),
        )
        .unwrap();
        if let Value::Number(depr) = result {
            // Period 6: accumulated depreciation through period 5, then apply rate
            assert!(
                depr > 54000.0 && depr < 57000.0,
                "DB period 6 depreciation out of range: {}",
                depr
            );
        }
    }

    #[test]
    fn test_db_period_7_last() {
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(7.0),
            &Value::Number(7.0),
        )
        .unwrap();
        if let Value::Number(depr) = result {
            // Last period partial: remaining * rate * (12-7)/12
            assert!(
                depr > 14000.0 && depr < 17000.0,
                "DB period 7 depreciation out of range: {}",
                depr
            );
        }
    }

    #[test]
    fn test_db_salvage_greater_equal_cost() {
        let result = db(
            &Value::Number(100000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));

        let result = db(
            &Value::Number(100000.0),
            &Value::Number(150000.0),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_db_life_zero_or_negative() {
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(0.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));

        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(-1.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_db_period_out_of_range() {
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(0.0),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));

        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(8.0),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_db_type_errors() {
        // cost not Number
        let result = db(
            &Value::String("1000000".to_string()),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // salvage not Number
        let result = db(
            &Value::Number(1000000.0),
            &Value::Boolean(true),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // life not Number
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Nil,
            &Value::Number(1.0),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // period not Number
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Array(vec![]),
            &Value::Number(7.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // month not Number
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Dictionary(std::collections::HashMap::new()),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_db_small_numbers() {
        let result = db(
            &Value::Number(1000.0),
            &Value::Number(100.0),
            &Value::Number(5.0),
            &Value::Number(1.0),
            &Value::Number(6.0),
        )
        .unwrap();
        if let Value::Number(depr) = result {
            // rate = 1 - (0.1)^(1/5) ≈ 1 - 0.630957 = 0.369043, rounded 0.369
            // depr = 1000 * 0.369 * 6/12 = 1000 * 0.369 * 0.5 ≈ 184.5
            assert!(
                depr > 180.0 && depr < 190.0,
                "DB small numbers depreciation out of range: {}",
                depr
            );
        }
    }

    #[test]
    fn test_ipmt_first_period() {
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            assert!(
                (interest + 66.67).abs() < 0.1,
                "Expected ~-66.67, got {}",
                interest
            );
        }
    }

    #[test]
    fn test_ppmt_first_period() {
        let result = ppmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(principal) = result {
            // Calculate expected values
            let rate = 0.1 / 12.0;
            let pmt_result = pmt(
                &Value::Number(rate),
                &Value::Number(36.0),
                &Value::Number(8000.0),
                &Value::Number(0.0),
            )
            .unwrap();
            let ipmt_result = ipmt(
                &Value::Number(rate),
                &Value::Number(1.0),
                &Value::Number(36.0),
                &Value::Number(8000.0),
                &Value::Number(0.0),
            )
            .unwrap();
            if let (Value::Number(pmt_val), Value::Number(ipmt_val)) = (pmt_result, ipmt_result) {
                let expected = pmt_val - ipmt_val;
                assert!(
                    (principal - expected).abs() < 0.01,
                    "PPMT calculation incorrect: expected {}, got {}",
                    expected,
                    principal
                );
            }
        }
    }

    #[test]
    fn test_ppmt_last_period() {
        let result = ppmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(36.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(principal) = result {
            // For the last period, principal payment should be the remaining balance
            assert!(
                principal < 0.0,
                "Last period principal should be negative: {}",
                principal
            );
        }
    }

    #[test]
    fn test_ppmt_zero_interest() {
        let result = ppmt(
            &Value::Number(0.0),
            &Value::Number(5.0),
            &Value::Number(10.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        // With zero interest, PPMT should equal PMT (which is -1000)
        assert_eq!(result, Value::Number(-1000.0));
    }

    #[test]
    fn test_ppmt_invalid_period_too_low() {
        let result = ppmt(
            &Value::Number(0.05),
            &Value::Number(0.0),
            &Value::Number(10.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ppmt_invalid_period_too_high() {
        let result = ppmt(
            &Value::Number(0.05),
            &Value::Number(11.0),
            &Value::Number(10.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ppmt_type_error_rate() {
        let result = ppmt(
            &Value::String("0.05".to_string()),
            &Value::Number(1.0),
            &Value::Number(10.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ppmt_type_error_per() {
        let result = ppmt(
            &Value::Number(0.05),
            &Value::Boolean(true),
            &Value::Number(10.0),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ppmt_type_error_nper() {
        let result = ppmt(
            &Value::Number(0.05),
            &Value::Number(1.0),
            &Value::Array(vec![]),
            &Value::Number(10000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ppmt_type_error_pv() {
        let result = ppmt(
            &Value::Number(0.05),
            &Value::Number(1.0),
            &Value::Number(10.0),
            &Value::Dictionary(HashMap::new()),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ppmt_consistency_with_pmt_ipmt() {
        // Test that PPMT = PMT - IPMT for various periods
        let rate = 0.06 / 12.0;
        let nper = 60.0;
        let pv = 20000.0;

        for period in 1..=10 {
            let pmt_result = pmt(
                &Value::Number(rate),
                &Value::Number(nper),
                &Value::Number(pv),
                &Value::Number(0.0),
            )
            .unwrap();
            let ipmt_result = ipmt(
                &Value::Number(rate),
                &Value::Number(period as f64),
                &Value::Number(nper),
                &Value::Number(pv),
                &Value::Number(0.0),
            )
            .unwrap();
            let ppmt_result = ppmt(
                &Value::Number(rate),
                &Value::Number(period as f64),
                &Value::Number(nper),
                &Value::Number(pv),
                &Value::Number(0.0),
            )
            .unwrap();

            if let (Value::Number(pmt_val), Value::Number(ipmt_val), Value::Number(ppmt_val)) =
                (pmt_result, ipmt_result, ppmt_result)
            {
                assert!(
                    (ppmt_val - (pmt_val - ipmt_val)).abs() < 0.01,
                    "PPMT inconsistency at period {}: PPMT={}, PMT-IPMT={}",
                    period,
                    ppmt_val,
                    pmt_val - ipmt_val
                );
            }
        }
    }

    #[test]
    fn test_ppmt_negative_pv() {
        // Test with negative PV (investment scenario)
        let result = ppmt(
            &Value::Number(0.05),
            &Value::Number(1.0),
            &Value::Number(10.0),
            &Value::Number(-10000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(principal) = result {
            // Should be positive (inflow)
            assert!(
                principal > 0.0,
                "PPMT with negative PV should be positive: {}",
                principal
            );
        }
    }

    #[test]
    fn test_ppmt_high_rate() {
        let result = ppmt(
            &Value::Number(0.50), // 50% interest
            &Value::Number(1.0),
            &Value::Number(5.0),
            &Value::Number(1000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(principal) = result {
            assert!(
                principal < 0.0,
                "High rate PPMT should be negative: {}",
                principal
            );
        }
    }

    #[test]
    fn test_ppmt_single_period() {
        let result = ppmt(
            &Value::Number(0.10),
            &Value::Number(1.0),
            &Value::Number(1.0),
            &Value::Number(1000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(principal) = result {
            // For single period, PPMT should equal -PV (full principal payment)
            assert!(
                (principal + 1000.0).abs() < 0.01,
                "Expected ~-1000, got {}",
                principal
            );
        }
    }

    #[test]
    fn test_cumipmt_first_year() {
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
            assert!(
                total_interest < -11000.0 && total_interest > -12300.0,
                "Cumulative interest out of range: {}",
                total_interest
            );
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
            assert!(
                total_principal < 0.0,
                "Cumulative principal should be negative: {}",
                total_principal
            );
        }
    }

    #[test]
    fn test_cumprinc_type_1() {
        let result = cumprinc(
            &Value::Number(0.09 / 12.0),
            &Value::Number(360.0),
            &Value::Number(125000.0),
            &Value::Number(1.0),
            &Value::Number(12.0),
            &Value::Number(1.0),
        )
        .unwrap();
        if let Value::Number(total_principal) = result {
            assert!(
                total_principal < 0.0,
                "Cumulative principal should be negative: {}",
                total_principal
            );
            // For type 1, should be different from type 0
            let type_0_result = cumprinc(
                &Value::Number(0.09 / 12.0),
                &Value::Number(360.0),
                &Value::Number(125000.0),
                &Value::Number(1.0),
                &Value::Number(12.0),
                &Value::Number(0.0),
            )
            .unwrap();
            if let Value::Number(type_0_principal) = type_0_result {
                assert_ne!(
                    total_principal, type_0_principal,
                    "Type 1 should give different result than type 0"
                );
            }
        }
    }

    #[test]
    fn test_cumprinc_single_period() {
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(5.0),
            &Value::Number(5.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(total_principal) = result {
            // Should equal ppmt for that period
            let ppmt_result = ppmt(
                &Value::Number(0.05),
                &Value::Number(5.0),
                &Value::Number(10.0),
                &Value::Number(1000.0),
                &Value::Number(0.0),
            )
            .unwrap();
            if let Value::Number(ppmt_val) = ppmt_result {
                assert!(
                    (total_principal - ppmt_val).abs() < 1e-10,
                    "Single period cumprinc should equal ppmt: {} vs {}",
                    total_principal,
                    ppmt_val
                );
            }
        }
    }

    #[test]
    fn test_cumprinc_invalid_start_period() {
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(0.0), // Invalid: must be >=1
            &Value::Number(5.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_cumprinc_invalid_end_period() {
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(1.0),
            &Value::Number(15.0), // Invalid: > nper
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_cumprinc_start_greater_than_end() {
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(7.0),
            &Value::Number(5.0), // start > end
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_cumprinc_invalid_type() {
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(1.0),
            &Value::Number(5.0),
            &Value::Number(2.0), // Invalid type
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_cumprinc_type_errors() {
        // Non-number rate
        let result = cumprinc(
            &Value::String("0.05".to_string()),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(1.0),
            &Value::Number(5.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // Non-number nper
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Boolean(true),
            &Value::Number(1000.0),
            &Value::Number(1.0),
            &Value::Number(5.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // Non-number pv
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Array(vec![]),
            &Value::Number(1.0),
            &Value::Number(5.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // Non-number start_period
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Nil,
            &Value::Number(5.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // Non-number end_period
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(1.0),
            &Value::Dictionary(std::collections::HashMap::new()),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));

        // Non-number type
        let result = cumprinc(
            &Value::Number(0.05),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(1.0),
            &Value::Number(5.0),
            &Value::Symbol("type".to_string()),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_effect_quarterly_compounding() {
        let result = effect(&Value::Number(0.0525), &Value::Number(4.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(
                (eff - 0.05354).abs() < 0.0001,
                "Expected ~0.05354, got {}",
                eff
            );
        }
    }

    #[test]
    fn test_effect_annual_compounding() {
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
        let result = nominal(&Value::Number(0.05354), &Value::Number(4.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!(
                (nom - 0.0525).abs() < 0.001,
                "Expected ~0.0525, got {}",
                nom
            );
        }
    }

    #[test]
    fn test_nominal_annual_compounding() {
        let result = nominal(&Value::Number(0.05), &Value::Number(1.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!((nom - 0.05).abs() < 0.0001);
        }
    }

    #[test]
    fn test_effect_nominal_roundtrip() {
        let nominal_rate = 0.0625;
        let npery = 12.0;

        let eff_result = effect(&Value::Number(nominal_rate), &Value::Number(npery)).unwrap();
        let back_to_nominal = nominal(&eff_result, &Value::Number(npery)).unwrap();

        if let Value::Number(nom) = back_to_nominal {
            assert!(
                (nom - nominal_rate).abs() < 0.0001,
                "Roundtrip failed: {} != {}",
                nom,
                nominal_rate
            );
        }
    }

    #[test]
    fn test_effect_monthly_compounding() {
        let result = effect(&Value::Number(0.06), &Value::Number(12.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(
                (eff - 0.06167781186449828).abs() < 0.0001,
                "Expected ~0.06167781186449828, got {}",
                eff
            );
        }
    }

    #[test]
    fn test_effect_daily_compounding() {
        let result = effect(&Value::Number(0.06), &Value::Number(365.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(
                (eff - 0.0618365).abs() < 0.0001,
                "Expected ~0.0618365, got {}",
                eff
            );
        }
    }

    #[test]
    fn test_effect_zero_nominal_rate() {
        let result = effect(&Value::Number(0.0), &Value::Number(4.0)).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_effect_negative_nominal_rate() {
        let result = effect(&Value::Number(-0.02), &Value::Number(4.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(
                eff < 0.0,
                "Effective rate should be negative for negative nominal rate: {}",
                eff
            );
        }
    }

    #[test]
    fn test_effect_large_npery() {
        let result = effect(&Value::Number(0.05), &Value::Number(1000.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(
                eff > 0.05,
                "Effective rate should be higher for more frequent compounding: {}",
                eff
            );
        }
    }

    #[test]
    fn test_effect_fractional_npery() {
        let result = effect(&Value::Number(0.05), &Value::Number(2.5)).unwrap();
        if let Value::Number(eff) = result {
            assert!(eff > 0.0, "Effective rate should be positive: {}", eff);
        }
    }

    #[test]
    fn test_effect_invalid_npery_negative() {
        let result = effect(&Value::Number(0.05), &Value::Number(-1.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_effect_invalid_npery_fractional_less_than_one() {
        let result = effect(&Value::Number(0.05), &Value::Number(0.5));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_effect_invalid_nominal_rate_minus_one() {
        let result = effect(&Value::Number(-1.0), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_effect_invalid_nominal_rate_less_than_minus_one() {
        let result = effect(&Value::Number(-1.5), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_effect_type_error_nominal_rate() {
        let result = effect(&Value::String("0.05".to_string()), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "String");
        }
    }

    #[test]
    fn test_effect_type_error_npery() {
        let result = effect(&Value::Number(0.05), &Value::Boolean(true));
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "Boolean");
        }
    }

    #[test]
    fn test_effect_all_type_errors() {
        let string_val = Value::String("test".to_string());
        let bool_val = Value::Boolean(true);
        let nil_val = Value::Nil;
        let array_val = Value::Array(vec![]);
        let dict_val = Value::Dictionary(std::collections::HashMap::new());
        let symbol_val = Value::Symbol("test".to_string());
        let num_val = Value::Number(0.05);
        let npery_val = Value::Number(4.0);

        let test_cases = vec![
            (&string_val, &npery_val),
            (&bool_val, &npery_val),
            (&nil_val, &npery_val),
            (&array_val, &npery_val),
            (&dict_val, &npery_val),
            (&symbol_val, &npery_val),
            (&num_val, &string_val),
            (&num_val, &bool_val),
            (&num_val, &nil_val),
            (&num_val, &array_val),
            (&num_val, &dict_val),
            (&num_val, &symbol_val),
        ];

        for (nominal_rate, npery) in test_cases {
            let result = effect(nominal_rate, npery);
            assert!(
                matches!(result, Err(FunctionError::TypeError { .. })),
                "Expected TypeError for nominal_rate={:?}, npery={:?}",
                nominal_rate,
                npery
            );
        }
    }

    #[test]
    fn test_effect_infinity_nominal_rate() {
        let result = effect(&Value::Number(f64::INFINITY), &Value::Number(4.0)).unwrap();
        assert_eq!(result, Value::Number(f64::INFINITY));
    }

    #[test]
    fn test_effect_negative_infinity_nominal_rate() {
        let result = effect(&Value::Number(f64::NEG_INFINITY), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_effect_nan_nominal_rate() {
        let result = effect(&Value::Number(f64::NAN), &Value::Number(4.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(eff.is_nan());
        }
    }

    #[test]
    fn test_effect_infinity_npery() {
        let result = effect(&Value::Number(0.05), &Value::Number(f64::INFINITY)).unwrap();
        if let Value::Number(eff) = result {
            // With infinity npery: (1 + r/inf)^inf = 1^inf = 1, so 1 - 1 = 0
            assert_eq!(eff, 0.0);
        }
    }

    #[test]
    fn test_effect_very_small_nominal_rate() {
        let result = effect(&Value::Number(1e-10), &Value::Number(12.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(eff > 0.0 && eff < 1e-9);
        }
    }

    #[test]
    fn test_effect_very_large_nominal_rate() {
        let result = effect(&Value::Number(1e10), &Value::Number(4.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(
                eff.is_infinite() || eff > 1e30,
                "Expected very large or infinite result, got {}",
                eff
            );
        }
    }

    #[test]
    fn test_effect_boundary_nominal_rate_minus_one() {
        let result = effect(&Value::Number(-0.999999), &Value::Number(4.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!(eff < 0.0);
        }
    }

    #[test]
    fn test_effect_precision() {
        // Test with known precise values
        let result = effect(&Value::Number(0.1), &Value::Number(2.0)).unwrap();
        if let Value::Number(eff) = result {
            // (1 + 0.1/2)^2 - 1 = (1.05)^2 - 1 = 1.1025 - 1 = 0.1025
            assert!((eff - 0.1025).abs() < 1e-10);
        }
    }

    #[test]
    fn test_effect_compounding_frequency_comparison() {
        let nominal_rate = 0.06;
        let effective_annual = effect(&Value::Number(nominal_rate), &Value::Number(1.0)).unwrap();
        let effective_monthly = effect(&Value::Number(nominal_rate), &Value::Number(12.0)).unwrap();
        let effective_daily = effect(&Value::Number(nominal_rate), &Value::Number(365.0)).unwrap();

        if let (Value::Number(ann), Value::Number(mon), Value::Number(dai)) =
            (effective_annual, effective_monthly, effective_daily)
        {
            assert!(
                ann < mon,
                "Monthly compounding should give higher effective rate than annual: {} vs {}",
                ann,
                mon
            );
            assert!(
                mon < dai,
                "Daily compounding should give higher effective rate than monthly: {} vs {}",
                mon,
                dai
            );
        }
    }

    #[test]
    fn test_nominal_monthly_compounding() {
        let result = nominal(&Value::Number(0.06168), &Value::Number(12.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!((nom - 0.06).abs() < 0.001, "Expected ~0.06, got {}", nom);
        }
    }

    #[test]
    fn test_nominal_daily_compounding() {
        let result = nominal(&Value::Number(0.0618365), &Value::Number(365.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!((nom - 0.06).abs() < 0.001, "Expected ~0.06, got {}", nom);
        }
    }

    #[test]
    fn test_nominal_zero_effective_rate() {
        let result = nominal(&Value::Number(0.0), &Value::Number(4.0)).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_nominal_negative_effective_rate() {
        let result = nominal(&Value::Number(-0.02), &Value::Number(4.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!(
                nom < 0.0,
                "Nominal rate should be negative for negative effective rate: {}",
                nom
            );
        }
    }

    #[test]
    fn test_nominal_large_npery() {
        let result = nominal(&Value::Number(0.05), &Value::Number(1000.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!(
                nom < 0.05,
                "Nominal rate should be lower for more frequent compounding: {}",
                nom
            );
        }
    }

    #[test]
    fn test_nominal_fractional_npery() {
        let result = nominal(&Value::Number(0.05), &Value::Number(2.5)).unwrap();
        if let Value::Number(nom) = result {
            assert!(nom > 0.0, "Nominal rate should be positive: {}", nom);
        }
    }

    #[test]
    fn test_nominal_invalid_npery_zero() {
        let result = nominal(&Value::Number(0.05), &Value::Number(0.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_nominal_invalid_npery_negative() {
        let result = nominal(&Value::Number(0.05), &Value::Number(-1.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_nominal_invalid_npery_fractional_less_than_one() {
        let result = nominal(&Value::Number(0.05), &Value::Number(0.5));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_nominal_invalid_effective_rate() {
        let result = nominal(&Value::Number(-1.0), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_nominal_invalid_effective_rate_less_than_minus_one() {
        let result = nominal(&Value::Number(-1.5), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_nominal_type_error_effect_rate() {
        let result = nominal(&Value::String("0.05".to_string()), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "String");
        }
    }

    #[test]
    fn test_nominal_type_error_npery() {
        let result = nominal(&Value::Number(0.05), &Value::Boolean(true));
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
        if let Err(FunctionError::TypeError { expected, got }) = result {
            assert_eq!(expected, "Number");
            assert_eq!(got, "Boolean");
        }
    }

    #[test]
    fn test_nominal_all_type_errors() {
        let string_val = Value::String("test".to_string());
        let bool_val = Value::Boolean(true);
        let nil_val = Value::Nil;
        let array_val = Value::Array(vec![]);
        let dict_val = Value::Dictionary(std::collections::HashMap::new());
        let symbol_val = Value::Symbol("test".to_string());
        let num_val = Value::Number(0.05);
        let npery_val = Value::Number(4.0);

        let test_cases = vec![
            (&string_val, &npery_val),
            (&bool_val, &npery_val),
            (&nil_val, &npery_val),
            (&array_val, &npery_val),
            (&dict_val, &npery_val),
            (&symbol_val, &npery_val),
            (&num_val, &string_val),
            (&num_val, &bool_val),
            (&num_val, &nil_val),
            (&num_val, &array_val),
            (&num_val, &dict_val),
            (&num_val, &symbol_val),
        ];

        for (effect_rate, npery) in test_cases {
            let result = nominal(effect_rate, npery);
            assert!(
                matches!(result, Err(FunctionError::TypeError { .. })),
                "Expected TypeError for effect_rate={:?}, npery={:?}",
                effect_rate,
                npery
            );
        }
    }

    #[test]
    fn test_nominal_infinity_effective_rate() {
        let result = nominal(&Value::Number(f64::INFINITY), &Value::Number(4.0)).unwrap();
        assert_eq!(result, Value::Number(f64::INFINITY));
    }

    #[test]
    fn test_nominal_negative_infinity_effective_rate() {
        let result = nominal(&Value::Number(f64::NEG_INFINITY), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_nominal_nan_effective_rate() {
        let result = nominal(&Value::Number(f64::NAN), &Value::Number(4.0)).unwrap();
        match result {
            Value::Number(n) => assert!(n.is_nan()),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_nominal_nan_npery() {
        let result = nominal(&Value::Number(0.05), &Value::Number(f64::NAN));
        // This should either return NaN or error, depending on validation order
        match result {
            Ok(Value::Number(n)) => assert!(n.is_nan()),
            Err(_) => (), // Also acceptable
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_nominal_infinity_npery() {
        let result = nominal(&Value::Number(0.05), &Value::Number(f64::INFINITY));
        // This should either return NaN or error, depending on validation order
        match result {
            Ok(Value::Number(n)) => assert!(n.is_nan()),
            Err(_) => (), // Also acceptable
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_nominal_precision() {
        // Test with high precision values
        let result = nominal(&Value::Number(0.053542821), &Value::Number(4.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!((nom - 0.0525).abs() < 1e-3);
        }
    }

    #[test]
    fn test_nominal_very_small_effective_rate() {
        let result = nominal(&Value::Number(1e-10), &Value::Number(12.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!(
                nom > 0.0 && nom < 1e-9,
                "Nominal rate should be very small positive: {}",
                nom
            );
        }
    }

    #[test]
    fn test_nominal_very_large_effective_rate() {
        let result = nominal(&Value::Number(10.0), &Value::Number(4.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!(nom.is_finite(), "Nominal rate should be finite: {}", nom);
        }
    }

    #[test]
    fn test_nominal_boundary_effective_rate_minus_one() {
        // Test exactly -1.0, which should be invalid
        let result = nominal(&Value::Number(-1.0), &Value::Number(4.0));
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_nominal_boundary_npery_one() {
        // Test npery = 1, which should work
        let result = nominal(&Value::Number(0.05), &Value::Number(1.0)).unwrap();
        if let Value::Number(nom) = result {
            assert!((nom - 0.05).abs() < 1e-14, "Expected ~0.05, got {}", nom);
        }
    }

    #[test]
    fn test_nominal_compounding_frequency_relationship() {
        let effective_rate = 0.05;
        let nominal_quarterly =
            nominal(&Value::Number(effective_rate), &Value::Number(4.0)).unwrap();
        let nominal_monthly =
            nominal(&Value::Number(effective_rate), &Value::Number(12.0)).unwrap();

        if let (Value::Number(q), Value::Number(m)) = (nominal_quarterly, nominal_monthly) {
            assert!(
                m < q,
                "Monthly nominal should be lower than quarterly for same effective rate: {} vs {}",
                m,
                q
            );
        }
    }

    #[test]
    fn test_nominal_roundtrip_precision() {
        // Test roundtrip with different precisions
        let test_rates = vec![0.01, 0.05, 0.10, 0.15];
        let test_nperies = vec![1.0, 2.0, 4.0, 12.0, 365.0];

        for &rate in &test_rates {
            for &npery in &test_nperies {
                let eff_result = effect(&Value::Number(rate), &Value::Number(npery)).unwrap();
                let back_to_nominal = nominal(&eff_result, &Value::Number(npery)).unwrap();

                if let Value::Number(nom) = back_to_nominal {
                    assert!(
                        (nom - rate).abs() < 1e-10,
                        "Roundtrip failed for rate={}, npery={}: {} != {}",
                        rate,
                        npery,
                        nom,
                        rate
                    );
                }
            }
        }
    }

    #[test]
    fn test_pmt_ppmt_ipmt_consistency() {
        let rate = &Value::Number(0.06 / 12.0);
        let nper = &Value::Number(60.0);
        let pv = &Value::Number(20000.0);
        let period = &Value::Number(15.0);

        let total_pmt = pmt(rate, nper, pv, &Value::Number(0.0)).unwrap();
        let principal = ppmt(rate, period, nper, pv, &Value::Number(0.0)).unwrap();
        let interest = ipmt(rate, period, nper, pv, &Value::Number(0.0)).unwrap();

        if let (Value::Number(t), Value::Number(p), Value::Number(i)) =
            (total_pmt, principal, interest)
        {
            assert!(
                (t - (p + i)).abs() < 0.01,
                "PMT != PPMT + IPMT: {} != {} + {}",
                t,
                p,
                i
            );
        }
    }

    #[test]
    fn test_inventory_registration() {
        let functions: Vec<_> = get_all_numeric_functions().into_iter().collect();
        assert!(
            !functions.is_empty(),
            "Should have registered numeric functions"
        );

        let names: Vec<_> = functions.iter().map(|f| f.name).collect();
        assert!(names.contains(&"abs"), "Should contain abs function");
        assert!(names.contains(&"pmt"), "Should contain pmt function");
    }

    #[test]
    fn test_ipmt_middle_period() {
        // Test ipmt for a middle period (period 12 out of 36)
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(12.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            // For period 12, interest should be less than first period due to principal reduction
            assert!(interest < 0.0, "Interest should be negative: {}", interest);
            assert!(
                interest > -60.0,
                "Interest should be greater than -60: {}",
                interest
            ); // Rough check
        }
    }

    #[test]
    fn test_ipmt_last_period() {
        // Test ipmt for the last period
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(36.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            assert!(interest < 0.0, "Interest should be negative: {}", interest);
            // Last period interest should be small due to most principal being paid off
            assert!(
                interest > -10.0,
                "Last period interest should be small: {}",
                interest
            );
        }
    }

    #[test]
    fn test_ipmt_zero_interest_rate() {
        // Test ipmt with zero interest rate - should return 0
        let result = ipmt(
            &Value::Number(0.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_ipmt_invalid_period_too_low() {
        // Test ipmt with period < 1
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(0.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ipmt_invalid_period_too_high() {
        // Test ipmt with period > nper
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(37.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ipmt_invalid_nper_zero() {
        // Test ipmt with nper <= 0
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(0.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ipmt_invalid_nper_negative() {
        // Test ipmt with negative nper
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(-1.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::ArgumentError { .. })));
    }

    #[test]
    fn test_ipmt_type_error_rate() {
        // Test ipmt with non-number rate
        let result = ipmt(
            &Value::String("0.1".to_string()),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ipmt_type_error_per() {
        // Test ipmt with non-number per
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::String("1".to_string()),
            &Value::Number(36.0),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ipmt_type_error_nper() {
        // Test ipmt with non-number nper
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::String("36".to_string()),
            &Value::Number(8000.0),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ipmt_type_error_pv() {
        // Test ipmt with non-number pv
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::String("8000".to_string()),
            &Value::Number(0.0),
        );
        assert!(matches!(result, Err(FunctionError::TypeError { .. })));
    }

    #[test]
    fn test_ipmt_single_period_loan() {
        // Test ipmt for a single period loan
        let result = ipmt(
            &Value::Number(0.05),
            &Value::Number(1.0),
            &Value::Number(1.0),
            &Value::Number(1000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            // For single period, interest should be -50 (5% of 1000)
            assert!(
                (interest + 50.0).abs() < 0.01,
                "Expected ~-50, got {}",
                interest
            );
        }
    }

    #[test]
    fn test_ipmt_high_interest_rate() {
        // Test ipmt with high interest rate
        let result = ipmt(
            &Value::Number(0.5), // 50% interest
            &Value::Number(1.0),
            &Value::Number(2.0),
            &Value::Number(1000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            // Interest should be -500 (50% of 1000)
            assert!(
                (interest + 500.0).abs() < 0.01,
                "Expected ~-500, got {}",
                interest
            );
        }
    }

    #[test]
    fn test_ipmt_negative_pv() {
        // Test ipmt with negative PV (loan amount)
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(-8000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            // Interest should be positive (since PV is negative, representing asset)
            assert!(
                interest > 0.0,
                "Interest should be positive for negative PV: {}",
                interest
            );
            assert!(
                (interest - 66.67).abs() < 0.1,
                "Expected ~66.67, got {}",
                interest
            );
        }
    }

    #[test]
    fn test_ipmt_negative_rate() {
        // Test ipmt with negative interest rate
        let result = ipmt(
            &Value::Number(-0.05),
            &Value::Number(1.0),
            &Value::Number(10.0),
            &Value::Number(1000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            // With negative rate, interest should be positive (you earn interest)
            assert!(
                interest > 0.0,
                "Interest should be positive for negative rate: {}",
                interest
            );
        }
    }

    #[test]
    fn test_ipmt_fractional_period() {
        // Test ipmt with fractional period
        let result = ipmt(
            &Value::Number(0.1),
            &Value::Number(1.5),
            &Value::Number(3.0),
            &Value::Number(1000.0),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            // Should calculate for period 1.5
            assert!(interest < 0.0, "Interest should be negative: {}", interest);
        }
    }

    #[test]
    fn test_ipmt_very_small_numbers() {
        // Test ipmt with very small numbers
        let result = ipmt(
            &Value::Number(1e-10),
            &Value::Number(1.0),
            &Value::Number(2.0),
            &Value::Number(1e-5),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            assert!(interest < 0.0, "Interest should be negative: {}", interest);
            assert!(
                interest.abs() < 1e-10,
                "Interest should be very small: {}",
                interest
            );
        }
    }

    #[test]
    fn test_ipmt_very_large_numbers() {
        // Test ipmt with very large numbers
        let result = ipmt(
            &Value::Number(0.01),
            &Value::Number(1.0),
            &Value::Number(2.0),
            &Value::Number(1e15),
            &Value::Number(0.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            assert!(interest < 0.0, "Interest should be negative: {}", interest);
            assert!(
                interest.abs() > 1e10,
                "Interest should be large: {}",
                interest
            );
        }
    }
}
