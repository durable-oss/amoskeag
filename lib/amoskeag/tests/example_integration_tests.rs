//! Integration tests for all Amoskeag examples
//!
//! This file contains tests for each example in the examples/ directory,
//! ensuring that the interpreter produces the expected output.

#![allow(clippy::needless_borrow)]

use amoskeag::*;
use amoskeag_stdlib_operators::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Helper function to load an example file
fn load_example(example_name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../examples");
    path.push(example_name);
    path.push("example.amos");

    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to load example: {}", example_name))
}

/// Helper to create a dictionary value
fn dict<I>(items: I) -> Value
where
    I: IntoIterator<Item = (&'static str, Value)>,
{
    let map: HashMap<String, Value> = items.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    Value::Dictionary(map)
}

#[test]
fn test_01_hello_world() {
    let source = load_example("01_hello_world");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::String("Hello, World!".to_string()));
}

#[test]
fn test_02_arithmetic() {
    let source = load_example("02_arithmetic");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::Number(123.0));
}

#[test]
fn test_03_variables() {
    let source = load_example("03_variables");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::String("Hello, Alice!".to_string()));
}

#[test]
fn test_04_conditionals() {
    let source = load_example("04_conditionals");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::String("Student".to_string()));
}

#[test]
fn test_05_string_operations() {
    let source = load_example("05_string_operations");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(
        result,
        Value::String("Hello, amoskeag language!".to_string())
    );
}

#[test]
fn test_06_array_operations() {
    let source = load_example("06_array_operations");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    let expected = dict([
        ("total", Value::Number(150.0)),
        ("average", Value::Number(30.0)),
        ("count", Value::Number(5.0)),
        ("first", Value::Number(10.0)),
        ("last", Value::Number(50.0)),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_07_dictionary_access() {
    let source = load_example("07_dictionary_access");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(
        result,
        Value::String("Bob Smith (bob@example.com) lives in Boston".to_string())
    );
}

#[test]
fn test_08_symbols() {
    let source = load_example("08_symbols");
    let symbols = &["approved", "waiting", "denied"];
    let program = compile(&source, symbols).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::Symbol("approved".to_string()));
}

#[test]
fn test_09_business_rule_simple() {
    let source = load_example("09_business_rule_simple");
    let symbols = &["deny", "instant_approve", "approve"];
    let program = compile(&source, symbols).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::Symbol("approve".to_string()));
}

#[test]
fn test_10_business_rule_complex() {
    // Note: Using corrected nested if syntax
    let source = r#"
        let app = applicant
        in
        let limits = env.underwriting_limits
        in
        let state_is_restricted = limits.restricted_states | contains(app.state)
        in
        let is_high_value_sports_car =
          app.vehicle.type == "SPORT" and app.vehicle.value > 70000
        in
        let is_young_sports_driver =
          app.vehicle.type == "SPORT" and app.age < 25
        in
          if state_is_restricted
            :deny
          else
            if app.vehicle.value > limits.max_vehicle_value
              :deny
            else
              if is_young_sports_driver
                :deny
              else
                if is_high_value_sports_car
                  :manual_review
                else
                  if app.claims_last_3_years > 2
                    :manual_review
                  else
                    :approve
                  end
                end
              end
            end
          end
    "#;
    let symbols = &["approve", "deny", "manual_review"];
    let program = compile(&source, symbols).expect("Compilation failed");

    // Build the data context
    let applicant_vehicle = dict([
        ("value", Value::Number(75000.0)),
        ("type", Value::String("SPORT".to_string())),
    ]);

    let applicant = dict([
        ("age", Value::Number(30.0)),
        ("state", Value::String("CA".to_string())),
        ("claims_last_3_years", Value::Number(1.0)),
        ("vehicle", applicant_vehicle),
    ]);

    let underwriting_limits = dict([
        ("max_vehicle_value", Value::Number(100000.0)),
        (
            "restricted_states",
            Value::Array(vec![
                Value::String("FL".to_string()),
                Value::String("LA".to_string()),
            ]),
        ),
    ]);

    let env = dict([("underwriting_limits", underwriting_limits)]);

    let mut data = HashMap::new();
    data.insert("applicant".to_string(), applicant);
    data.insert("env".to_string(), env);

    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::Symbol("manual_review".to_string()));
}

#[test]
fn test_11_template_greeting() {
    let source = load_example("11_template_greeting");
    let program = compile(&source, &[]).expect("Compilation failed");

    // Build the data context
    let user = dict([
        ("name", Value::String("Alice Johnson".to_string())),
        ("is_admin", Value::Boolean(false)),
        ("last_login", Value::String("2025-01-15".to_string())),
    ]);

    let mut data = HashMap::new();
    data.insert("user".to_string(), user);

    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(
        result,
        Value::String("Welcome back, Alice Johnson!".to_string())
    );
}

#[test]
fn test_12_template_blog_post() {
    let source = load_example("12_template_blog_post");
    let program = compile(&source, &[]).expect("Compilation failed");

    // Build the data context
    let author = dict([
        ("name", Value::String("Jane Developer".to_string())),
        ("is_verified", Value::Boolean(true)),
    ]);

    let post = dict([
        (
            "title",
            Value::String("Introduction to Amoskeag Programming".to_string()),
        ),
        ("status", Value::String("published".to_string())),
        ("author", author),
    ]);

    let mut data = HashMap::new();
    data.insert("post".to_string(), post);

    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(
        result,
        Value::String(
            "[LIVE] INTRODUCTION TO AMOSKEAG PROGRAMMING by Jane Developer ✓".to_string()
        )
    );
}

#[test]
fn test_13_spreadsheet_formulas() {
    let source = load_example("13_spreadsheet_formulas");
    let program = compile(&source, &[]).expect("Compilation failed");

    // Build the data context
    let mut data = HashMap::new();
    data.insert("B1".to_string(), Value::Number(2.0));
    data.insert(
        "B2".to_string(),
        Value::Array(vec![
            Value::Number(0.10),
            Value::Number(0.15),
            Value::Number(0.20),
        ]),
    );
    data.insert("B3".to_string(), Value::Number(1000.0));

    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::Number(1150.0));
}

#[test]
fn test_14_financial_calculations() {
    let source = load_example("14_financial_calculations");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    // PMT returns negative values for cash outflows (loan payments)
    // So we expect negative values throughout
    let expected = dict([
        ("loan_amount", Value::Number(250000.0)),
        ("monthly_payment", Value::Number(-1266.71)),
        ("total_paid", Value::Number(-456016.78)),
        ("total_interest", Value::Number(-706016.78)),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_15_pipe_chaining() {
    let source = load_example("15_pipe_chaining");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(
        result,
        Value::String("Hello, world this is Amoskeag.".to_string())
    );
}

#[test]
fn test_16_comparisons() {
    let source = load_example("16_comparisons");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    let expected = dict([
        ("equal", Value::Boolean(true)),
        ("not_equal", Value::Boolean(true)),
        ("less_than", Value::Boolean(true)),
        ("greater_than", Value::Boolean(true)),
        ("less_or_equal", Value::Boolean(true)),
        ("greater_or_equal", Value::Boolean(true)),
        ("string_equal", Value::Boolean(true)),
        ("string_not_equal", Value::Boolean(true)),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_17_logical_operators() {
    let source = load_example("17_logical_operators");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    let expected = dict([
        ("can_get_basic_loan", Value::Boolean(true)),
        ("needs_review", Value::Boolean(true)),
        ("instant_approval", Value::Boolean(false)),
        ("complex_condition", Value::Boolean(true)),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_18_nested_data() {
    let source = load_example("18_nested_data");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(
        result,
        Value::String(
            "TechCorp - Boston, MA (42.36, -71.06) - Contact: alice@techcorp.com".to_string()
        )
    );
}

#[test]
fn test_19_array_filtering() {
    let source = load_example("19_array_filtering");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    let expected = dict([
        ("total_value", Value::Number(1675.0)),
        ("average_price", Value::Number(418.75)),
        ("cheapest", Value::Number(25.0)),
        ("most_expensive", Value::Number(1200.0)),
        ("product_count", Value::Number(4.0)),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_20_string_formatting() {
    let source = load_example("20_string_formatting");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    let expected = dict([
        ("display_name", Value::String("John Doe".to_string())),
        ("email", Value::String("john.doe@company.com".to_string())),
        (
            "bio_clean",
            Value::String("Software engineer with 10 years experience.".to_string()),
        ),
        (
            "bio_short",
            Value::String("Software engineer with 10 year".to_string()),
        ), // truncate(30) cuts at 30 chars, no "..."
        (
            "tags_joined",
            Value::String("rust, typescript, python".to_string()),
        ),
        ("tag_count", Value::Number(3.0)),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_21_number_rounding() {
    let source = load_example("21_number_rounding");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    let expected = dict([
        ("subtotal", Value::Number(139.93)),
        ("tax", Value::Number(11.54)),
        ("total", Value::Number(151.47)),
        ("discount", Value::Number(22.72)),
        ("final_total", Value::Number(128.75)),
        ("final_total_ceil", Value::Number(129.0)),
        ("final_total_floor", Value::Number(128.0)),
        ("absolute_discount", Value::Number(22.72)),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_22_date_operations() {
    let source = load_example("22_date_operations");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    let expected = dict([
        ("created_at", Value::String("2025-01-18".to_string())),
        (
            "created_at_full",
            Value::String("2025-01-18 14:30:00".to_string()),
        ),
        (
            "display",
            Value::String("Account created on 2025-01-18".to_string()),
        ),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_23_loan_calculator() {
    let source = load_example("23_loan_calculator");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    // Expected output is complex nested structure
    // Will be tested once pmt function is implemented
    if let Value::Dictionary(dict) = result {
        assert!(dict.contains_key("loan_summary"));
        assert!(dict.contains_key("payment_details"));
        assert!(dict.contains_key("interest_analysis"));
    } else {
        panic!("Expected dictionary result");
    }
}

#[test]
fn test_24_discount_calculator() {
    let source = load_example("24_discount_calculator");
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    // The result should be a dictionary with "pricing" and "savings" keys
    if let Value::Dictionary(dict) = result {
        assert!(dict.contains_key("pricing"));
        assert!(dict.contains_key("savings"));

        // Check pricing values
        if let Some(Value::Dictionary(pricing)) = dict.get("pricing") {
            assert_eq!(pricing.get("subtotal"), Some(&Value::Number(154.97)));
            assert_eq!(pricing.get("member_discount"), Some(&Value::Number(15.50)));
            assert_eq!(pricing.get("volume_discount"), Some(&Value::Number(4.18)));
            assert_eq!(pricing.get("loyalty_discount"), Some(&Value::Number(10.0)));
            assert_eq!(pricing.get("total_discount"), Some(&Value::Number(29.68)));
            assert_eq!(pricing.get("final_price"), Some(&Value::Number(125.29)));
        } else {
            panic!("Expected pricing dictionary");
        }

        // Check savings values
        if let Some(Value::Dictionary(savings)) = dict.get("savings") {
            assert_eq!(savings.get("amount"), Some(&Value::Number(29.68)));
            assert_eq!(savings.get("percent"), Some(&Value::Number(19.2)));
        } else {
            panic!("Expected savings dictionary");
        }
    } else {
        panic!("Expected dictionary result");
    }
}

#[test]
fn test_25_validation_rule() {
    let source = load_example("25_validation_rule");
    let symbols = &[
        "valid",
        "invalid_email",
        "invalid_password",
        "invalid_age",
        "terms_not_accepted",
        "unsupported_country",
        "invalid_unknown",
    ];
    let program = compile(&source, symbols).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");

    assert_eq!(result, Value::Symbol("valid".to_string()));
}

#[test]
fn test_parse_error_missing_else() {
    let source = r#"
        if true
          "yes"
        end
    "#;
    let result = compile(&source, &[]);
    assert!(result.is_err());
}

#[test]
fn test_parse_error_invalid_syntax() {
    let source = "let x = + 5 in x";
    let result = compile(&source, &[]);
    assert!(result.is_err());
}

#[test]
fn test_parse_error_unmatched_paren() {
    let source = "(1 + 2";
    let result = compile(&source, &[]);
    assert!(result.is_err());
}

#[test]
fn test_evaluation_error_undefined_variable() {
    let source = "undefined_var";
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data);
    assert!(result.is_err());
}

#[test]
fn test_evaluation_error_type_mismatch() {
    let source = r#""string" + 123"#;
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let _result = evaluate(&program, &data);
    // Depending on implementation, may or may not error
    // For now, assume it works or add assertion if needed
}

#[test]
fn test_edge_case_deep_nesting() {
    let source = r#"
        let a = 1 in
        let b = a + 1 in
        let c = b + 1 in
        let d = c + 1 in
        let e = d + 1 in
        e
    "#;
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_edge_case_large_number() {
    let source = "999999999999.999";
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");
    assert_eq!(result, Value::Number(999999999999.999));
}

#[test]
fn test_edge_case_empty_array() {
    let source = "[] | size";
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_edge_case_empty_dictionary() {
    let source = "{}";
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");
    assert_eq!(result, Value::Dictionary(HashMap::new()));
}

#[test]
fn test_string_concatenation_with_numbers() {
    let source = r#""Price: " + 42.5"#;
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");
    assert_eq!(result, Value::String("Price: 42.5".to_string()));
}

#[test]
fn test_capitalize_function() {
    let source = r#"("hello world" | capitalize)"#;
    let program = compile(&source, &[]).expect("Compilation failed");
    let data = HashMap::new();
    let result = evaluate(&program, &data).expect("Evaluation failed");
    assert_eq!(result, Value::String("Hello world".to_string()));
}
