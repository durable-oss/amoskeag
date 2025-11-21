use amoskeag_stdlib_functions::numeric::fv;
use amoskeag_stdlib_operators::Value;
use amoskeag_stdlib_functions::FunctionError;

#[test]
fn test_fv_nper_zero() {
    let result = fv(
        &Value::Number(0.05),
        &Value::Number(0.0),
        &Value::Number(-100.0),
        &Value::Number(-1000.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::ArgumentError { message }) = result {
        assert_eq!(message, "nper must be greater than 0");
    } else {
        panic!("Expected ArgumentError");
    }
}

#[test]
fn test_fv_nper_negative() {
    let result = fv(
        &Value::Number(0.05),
        &Value::Number(-5.0),
        &Value::Number(-100.0),
        &Value::Number(-1000.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::ArgumentError { message }) = result {
        assert_eq!(message, "nper must be greater than 0");
    } else {
        panic!("Expected ArgumentError");
    }
}

#[test]
fn test_fv_wrong_type_rate() {
    let result = fv(
        &Value::String("0.05".to_string()),
        &Value::Number(12.0),
        &Value::Number(-100.0),
        &Value::Number(-1000.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "Number");
        assert_eq!(got, "String");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_fv_wrong_type_nper() {
    let result = fv(
        &Value::Number(0.05),
        &Value::Boolean(true),
        &Value::Number(-100.0),
        &Value::Number(-1000.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "Number");
        assert_eq!(got, "Boolean");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_fv_wrong_type_pmt() {
    let result = fv(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Nil,
        &Value::Number(-1000.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "Number");
        assert_eq!(got, "Nil");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_fv_wrong_type_pv() {
    let result = fv(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(-100.0),
        &Value::Array(vec![]),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "Number");
        assert_eq!(got, "Array");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_fv_no_payments() {
    // Only principal, no payments
    let result = fv(
        &Value::Number(0.05),
        &Value::Number(5.0),
        &Value::Number(0.0),
        &Value::Number(-1000.0),
    )
    .unwrap();
    if let Value::Number(fv) = result {
        // FV = -(-1000) * (1.05)^5 = 1000 * 1.27628 ≈ 1276.28
        assert!((fv - 1276.28).abs() < 0.01);
    }
}

#[test]
fn test_fv_no_principal() {
    // Only payments, no principal
    let result = fv(
        &Value::Number(0.05),
        &Value::Number(5.0),
        &Value::Number(-100.0),
        &Value::Number(0.0),
    )
    .unwrap();
    if let Value::Number(fv) = result {
        // FV = -(-100) * (((1.05)^5 - 1) / 0.05) = 100 * ((1.27628 - 1) / 0.05) ≈ 100 * (0.27628 / 0.05) ≈ 552.56
        assert!((fv - 552.56).abs() < 0.01);
    }
}

#[test]
fn test_fv_annuity_due() {
    // Payments at beginning of period (positive rate and payments)
    let result = fv(
        &Value::Number(0.06),
        &Value::Number(3.0),
        &Value::Number(100.0),
        &Value::Number(0.0),
    )
    .unwrap();
    if let Value::Number(fv) = result {
        // FV = -0 * (1.06)^3 - 100 * (((1.06)^3 - 1) / 0.06) = -100 * ((1.191016 - 1) / 0.06) ≈ -100 * (0.191016 / 0.06) ≈ -318.36
        assert!((fv - (-318.36)).abs() < 0.01);
    }
}

#[test]
fn test_fv_high_rate() {
    let result = fv(
        &Value::Number(0.50), // 50% interest
        &Value::Number(2.0),
        &Value::Number(-100.0),
        &Value::Number(-1000.0),
    )
    .unwrap();
    if let Value::Number(fv) = result {
        // factor = (1.5)^2 = 2.25
        // FV = -(-1000) * 2.25 - (-100) * ((2.25 - 1) / 0.5) = 2250 + 100 * (1.25 / 0.5) = 2250 + 100 * 2.5 = 2250 + 250 = 2500
        assert!((fv - 2500.0).abs() < 0.01);
    }
}

#[test]
fn test_fv_fractional_periods() {
    let result = fv(
        &Value::Number(0.04),
        &Value::Number(2.5),
        &Value::Number(-50.0),
        &Value::Number(-500.0),
    )
    .unwrap();
    if let Value::Number(fv) = result {
        // factor = (1.04)^2.5 ≈ 1.1032
        // FV = -(-500) * 1.1032 - (-50) * ((1.1032 - 1) / 0.04) ≈ 551.6 + 50 * (0.1032 / 0.04) ≈ 551.6 + 50 * 2.58 ≈ 551.6 + 129 ≈ 680.6
        assert!((fv - 680.28).abs() < 0.01);
    }
}

#[test]
fn test_fv_very_small_rate() {
    let result = fv(
        &Value::Number(0.0001),
        &Value::Number(100.0),
        &Value::Number(-10.0),
        &Value::Number(-100.0),
    )
    .unwrap();
    if let Value::Number(fv) = result {
        // factor = (1.0001)^100 ≈ 1.01005
        // FV = -(-100) * 1.01005 - (-10) * ((1.01005 - 1) / 0.0001) ≈ 101.005 + 10 * (0.01005 / 0.0001) ≈ 101.005 + 10 * 100.5 ≈ 101.005 + 1005 ≈ 1106.005
        assert!((fv - 1105.97).abs() < 0.01);
    }
}

#[test]
fn test_fv_large_numbers() {
    let result = fv(
        &Value::Number(0.03),
        &Value::Number(360.0), // 30 years monthly
        &Value::Number(-1000.0),
        &Value::Number(-200000.0),
    )
    .unwrap();
    if let Value::Number(fv) = result {
        // This should be a large positive number
        assert!(fv > 500000.0);
    }
}