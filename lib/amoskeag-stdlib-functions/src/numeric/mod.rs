//! Numeric functions for Amoskeag

pub mod abs;
pub mod ceil;
pub mod floor;
pub mod round;
pub mod plus;
pub mod minus;
pub mod times;
pub mod divided_by;
pub mod modulo;
pub mod max;
pub mod min;

// Financial functions
pub mod pmt;
pub mod pv;
pub mod fv;
pub mod nper;
pub mod rate;
pub mod npv;
pub mod irr;
pub mod mirr;
pub mod sln;
pub mod ddb;
pub mod db;
pub mod ipmt;
pub mod ppmt;
pub mod cumipmt;
pub mod cumprinc;
pub mod effect;
pub mod nominal;

// Re-export all functions
pub use abs::abs;
pub use ceil::ceil;
pub use floor::floor;
pub use round::round;
pub use plus::plus;
pub use minus::minus;
pub use times::times;
pub use divided_by::divided_by;
pub use modulo::modulo_fn;
pub use max::max;
pub use min::min;
pub use pmt::pmt;
pub use pv::pv;
pub use fv::fv;
pub use nper::nper;
pub use rate::rate;
pub use npv::npv;
pub use irr::irr;
pub use mirr::mirr;
pub use sln::sln;
pub use ddb::ddb;
pub use db::db;
pub use ipmt::ipmt;
pub use ppmt::ppmt;
pub use cumipmt::cumipmt;
pub use cumprinc::cumprinc;
pub use effect::effect;
pub use nominal::nominal;

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
        let result = round(&Value::Number(1.23456), &Value::Number(2.0)).unwrap();
        if let Value::Number(n) = result {
            assert!((n - 1.23).abs() < 0.001);
        } else {
            panic!("Expected Number");
        }
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

    #[test]
    fn test_pmt_basic() {
        let result = pmt(
            &Value::Number(0.00375),
            &Value::Number(360.0),
            &Value::Number(250000.0),
        )
        .unwrap();
        if let Value::Number(payment) = result {
            assert!((payment + 1266.71).abs() < 0.01, "Expected ~-1266.71, got {}", payment);
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
        let result = pv(
            &Value::Number(0.0067),
            &Value::Number(240.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        if let Value::Number(pv) = result {
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
        let result = fv(
            &Value::Number(0.005),
            &Value::Number(120.0),
            &Value::Number(-100.0),
            &Value::Number(-1000.0),
        )
        .unwrap();
        if let Value::Number(fv) = result {
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
    fn test_npv_basic() {
        let cash_flows = Value::Array(vec![
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = npv(&Value::Number(0.1), &cash_flows).unwrap();
        if let Value::Number(npv) = result {
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
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = irr(&cash_flows).unwrap();
        if let Value::Number(irr_rate) = result {
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
        let cash_flows = Value::Array(vec![
            Value::Number(-10000.0),
            Value::Number(3000.0),
            Value::Number(4200.0),
            Value::Number(6800.0),
        ]);
        let result = mirr(&cash_flows, &Value::Number(0.1), &Value::Number(0.12)).unwrap();
        if let Value::Number(mirr_rate) = result {
            assert!(mirr_rate > 0.15 && mirr_rate < 0.16, "MIRR out of expected range: {}", mirr_rate);
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
    fn test_ddb_year_2() {
        let result = ddb(
            &Value::Number(30000.0),
            &Value::Number(7500.0),
            &Value::Number(10.0),
            &Value::Number(2.0),
        )
        .unwrap();
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
        let result = db(
            &Value::Number(1000000.0),
            &Value::Number(100000.0),
            &Value::Number(6.0),
            &Value::Number(1.0),
            &Value::Number(7.0),
        )
        .unwrap();
        if let Value::Number(depr) = result {
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
        let result = ipmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(interest) = result {
            assert!((interest - 66.67).abs() < 0.1, "Expected ~66.67, got {}", interest);
        }
    }

    #[test]
    fn test_ppmt_first_period() {
        let result = ppmt(
            &Value::Number(0.1 / 12.0),
            &Value::Number(1.0),
            &Value::Number(36.0),
            &Value::Number(8000.0),
        )
        .unwrap();
        if let Value::Number(principal) = result {
            assert!(principal < -220.0 && principal > -330.0, "Principal out of range: {}", principal);
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
            assert!(total_principal < 0.0, "Cumulative principal should be negative: {}", total_principal);
        }
    }

    #[test]
    fn test_effect_quarterly_compounding() {
        let result = effect(&Value::Number(0.0525), &Value::Number(4.0)).unwrap();
        if let Value::Number(eff) = result {
            assert!((eff - 0.05354).abs() < 0.0001, "Expected ~0.05354, got {}", eff);
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
            assert!((nom - 0.0525).abs() < 0.001, "Expected ~0.0525, got {}", nom);
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
            assert!((nom - nominal_rate).abs() < 0.0001, "Roundtrip failed: {} != {}", nom, nominal_rate);
        }
    }

    #[test]
    fn test_pmt_ppmt_ipmt_consistency() {
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

    #[test]
    fn test_inventory_registration() {
        let functions: Vec<_> = get_all_numeric_functions().into_iter().collect();
        assert!(!functions.is_empty(), "Should have registered numeric functions");

        let names: Vec<_> = functions.iter().map(|f| f.name).collect();
        assert!(names.contains(&"abs"), "Should contain abs function");
        assert!(names.contains(&"pmt"), "Should contain pmt function");
    }
}
