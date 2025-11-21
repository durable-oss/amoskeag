use amoskeag_stdlib_functions::numeric::cumipmt;
use amoskeag_stdlib_functions::FunctionError;
use amoskeag_stdlib_operators::Value;

#[test]
fn test_cumipmt_wrong_type_rate() {
    let result = cumipmt(
        &Value::String("0.05".to_string()),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::Number(12.0),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "all arguments must be Numbers");
        assert_eq!(got, "mixed types");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_cumipmt_wrong_type_nper() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Boolean(true),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::Number(12.0),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "all arguments must be Numbers");
        assert_eq!(got, "mixed types");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_cumipmt_wrong_type_pv() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Nil,
        &Value::Number(1.0),
        &Value::Number(12.0),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "all arguments must be Numbers");
        assert_eq!(got, "mixed types");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_cumipmt_wrong_type_start_period() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Array(vec![]),
        &Value::Number(12.0),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "all arguments must be Numbers");
        assert_eq!(got, "mixed types");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_cumipmt_wrong_type_end_period() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::String("12".to_string()),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "all arguments must be Numbers");
        assert_eq!(got, "mixed types");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_cumipmt_wrong_type_type() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::Number(12.0),
        &Value::Boolean(false),
    );
    assert!(result.is_err());
    if let Err(FunctionError::TypeError { expected, got }) = result {
        assert_eq!(expected, "all arguments must be Numbers");
        assert_eq!(got, "mixed types");
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn test_cumipmt_start_period_too_low() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(0.0),
        &Value::Number(12.0),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::ArgumentError { message }) = result {
        assert_eq!(message, "start_period must be between 1 and 12");
    } else {
        panic!("Expected ArgumentError");
    }
}

#[test]
fn test_cumipmt_start_period_too_high() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(13.0),
        &Value::Number(12.0),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::ArgumentError { message }) = result {
        assert_eq!(message, "start_period must be between 1 and 12");
    } else {
        panic!("Expected ArgumentError");
    }
}

#[test]
fn test_cumipmt_end_period_less_than_start() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(5.0),
        &Value::Number(3.0),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::ArgumentError { message }) = result {
        assert_eq!(message, "end_period must be between 5 and 12");
    } else {
        panic!("Expected ArgumentError");
    }
}

#[test]
fn test_cumipmt_end_period_too_high() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::Number(15.0),
        &Value::Number(0.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::ArgumentError { message }) = result {
        assert_eq!(message, "end_period must be between 1 and 12");
    } else {
        panic!("Expected ArgumentError");
    }
}

#[test]
fn test_cumipmt_invalid_type() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::Number(12.0),
        &Value::Number(2.0),
    );
    assert!(result.is_err());
    if let Err(FunctionError::ArgumentError { message }) = result {
        assert_eq!(message, "type must be 0 or 1");
    } else {
        panic!("Expected ArgumentError");
    }
}

#[test]
fn test_cumipmt_single_period() {
    let result = cumipmt(
        &Value::Number(0.1 / 12.0),
        &Value::Number(3.0 * 12.0),
        &Value::Number(8000.0),
        &Value::Number(1.0),
        &Value::Number(1.0),
        &Value::Number(0.0),
    )
    .unwrap();
    if let Value::Number(cumipmt) = result {
        // Should equal ipmt for period 1
        // ipmt(0.1/12, 1, 36, 8000, 0) ≈ -66.67
        assert!((cumipmt - (-66.67)).abs() < 0.01);
    }
}

#[test]
fn test_cumipmt_first_year() {
    let result = cumipmt(
        &Value::Number(0.09 / 12.0),
        &Value::Number(30.0 * 12.0),
        &Value::Number(125000.0),
        &Value::Number(1.0),
        &Value::Number(12.0),
        &Value::Number(0.0),
    )
    .unwrap();
    if let Value::Number(cumipmt) = result {
        // Total interest paid in first year
        // Approximate value from Excel or calculation
        assert!(cumipmt < 0.0); // Interest is negative in this convention
        assert!(cumipmt > -12000.0); // Reasonable range
    }
}

#[test]
fn test_cumipmt_type_1() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::Number(6.0),
        &Value::Number(1.0),
    )
    .unwrap();
    if let Value::Number(cumipmt) = result {
        // Type 1: payments at beginning
        assert!(cumipmt < 0.0);
    }
}

#[test]
fn test_cumipmt_zero_rate() {
    let result = cumipmt(
        &Value::Number(0.0),
        &Value::Number(12.0),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::Number(12.0),
        &Value::Number(0.0),
    )
    .unwrap();
    if let Value::Number(cumipmt) = result {
        // Zero interest
        assert!((cumipmt - 0.0).abs() < 0.01);
    }
}

#[test]
fn test_cumipmt_full_periods() {
    let result = cumipmt(
        &Value::Number(0.05),
        &Value::Number(10.0),
        &Value::Number(1000.0),
        &Value::Number(1.0),
        &Value::Number(10.0),
        &Value::Number(0.0),
    )
    .unwrap();
    if let Value::Number(cumipmt) = result {
        // Cumulative interest over all periods
        assert!(cumipmt < 0.0);
        assert!(cumipmt > -500.0); // Reasonable range
    }
}
