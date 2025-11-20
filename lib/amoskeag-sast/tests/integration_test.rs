//! Integration tests for amoskeag-sast

use amoskeag_parser::parse;
use amoskeag_sast::{analyze, SastAnalyzer};

#[test]
fn test_end_to_end_division_by_zero() {
    let expr = parse("10 / 0").unwrap();
    let result = analyze(&expr, &[]);

    assert!(!result.errors.is_empty());
    assert!(result.errors.iter().any(|e| e.message.to_lowercase().contains("division")));
    assert_eq!(result.statistics.critical_errors, 1);
}

#[test]
fn test_end_to_end_potential_division_by_zero() {
    let expr = parse("10 / x").unwrap();
    let result = analyze(&expr, &[]);

    // Should find either an error or a vulnerable input
    assert!(!result.errors.is_empty() || !result.vulnerable_inputs.is_empty());

    if !result.vulnerable_inputs.is_empty() {
        assert!(result.vulnerable_inputs.iter().any(|v| v.error_type == "DivisionByZero"));
    }
}

#[test]
fn test_end_to_end_unreachable_code() {
    let expr = parse("if true :yes else :no end").unwrap();
    let result = analyze(&expr, &[]);

    assert!(!result.errors.is_empty());
    assert!(result.errors.iter().any(|e| e.message.contains("unreachable")));
}

#[test]
fn test_end_to_end_unused_variable() {
    let expr = parse("let x = 5 in 10").unwrap();
    let result = analyze(&expr, &[]);

    assert!(result.errors.iter().any(|e| e.message.contains("never used")));
}

#[test]
fn test_end_to_end_clean_code() {
    let expr = parse("let x = 5 in x + 10").unwrap();
    let result = analyze(&expr, &[]);

    assert_eq!(result.statistics.critical_errors, 0);
}

#[test]
fn test_end_to_end_complex_expression() {
    let source = r#"
        let x = 10 in
        let y = 20 in
        if x > 5
            y / 2
        else
            y / 0
        end
    "#;
    let expr = parse(source).unwrap();
    let result = analyze(&expr, &[]);

    // Should detect division by zero in else branch
    assert!(!result.errors.is_empty());
    assert!(result.errors.iter().any(|e| e.message.to_lowercase().contains("division")));
}

#[test]
fn test_json_output() {
    let expr = parse("10 / 0").unwrap();
    let mut analyzer = SastAnalyzer::new();
    let json = analyzer.analyze_json(&expr, &[]).unwrap();

    assert!(json.contains("errors"));
    assert!(json.contains("vulnerable_inputs"));
    assert!(json.contains("statistics"));
    assert!(json.contains("DivisionByZero"));
}

#[test]
fn test_array_out_of_bounds() {
    let expr = parse("at([1, 2, 3], 10)").unwrap();
    let result = analyze(&expr, &[]);

    assert!(!result.errors.is_empty());
    assert!(result.errors.iter().any(|e| e.message.contains("out of bounds")));
}

#[test]
fn test_nested_if_expressions() {
    let source = r#"
        if x > 0
            if y > 0
                x / y
            else
                x / 0
            end
        else
            0
        end
    "#;
    let expr = parse(source).unwrap();
    let result = analyze(&expr, &[]);

    // Should detect division by zero in nested else (may be in errors or vulnerable_inputs)
    let has_issue = !result.errors.is_empty() || !result.vulnerable_inputs.is_empty();
    assert!(has_issue);
}

#[test]
fn test_pipe_expression() {
    let expr = parse("x | upcase | truncate(10)").unwrap();
    let result = analyze(&expr, &[]);

    // Pipe expressions should be analyzable
    assert!(result.statistics.critical_errors == 0);
}

#[test]
fn test_multiple_vulnerabilities() {
    let source = r#"
        let a = 10 / 0 in
        let b = 5 in
        let c = 20 / x in
        a + b
    "#;
    let expr = parse(source).unwrap();
    let result = analyze(&expr, &[]);

    // Should detect multiple issues
    assert!(result.errors.len() >= 2 ||
            (result.errors.len() + result.vulnerable_inputs.len()) >= 2);
}

#[test]
fn test_symbol_validation() {
    let expr = parse(":unknown_symbol").unwrap();
    let result = analyze(&expr, &["approved", "denied"]);

    // Should warn about undefined symbol
    assert!(result.errors.iter().any(|e| e.message.contains("unknown_symbol")));
}

#[test]
fn test_function_call_analysis() {
    let expr = parse("divided_by(100, 0)").unwrap();
    let result = analyze(&expr, &[]);

    // Should detect division by zero in function call
    assert!(!result.errors.is_empty());
}

#[test]
fn test_large_array_warning() {
    // Create a large array literal
    let elements: Vec<String> = (0..1500).map(|i| i.to_string()).collect();
    let array_literal = format!("[{}]", elements.join(", "));
    let expr = parse(&array_literal).unwrap();
    let result = analyze(&expr, &[]);

    // Should warn about large array
    assert!(result.errors.iter().any(|e| e.message.contains("Large array")));
}

#[test]
fn test_statistics_accuracy() {
    let expr = parse("10 / 0").unwrap();
    let result = analyze(&expr, &[]);

    assert_eq!(result.statistics.critical_errors, 1);
    // total_expressions is always >= 0 by definition (it's unsigned)
}
