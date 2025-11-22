//! JSON data parsing utilities

use amoskeag::AmoskeagValue as Value;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;

/// Maximum JSON depth to prevent stack overflow
const MAX_JSON_DEPTH: usize = 100;

/// Parse JSON string into a HashMap of Values
///
/// # Errors
/// Returns an error if the JSON is invalid or not an object at the top level.
pub fn parse_json_data(json: &str) -> Result<HashMap<String, Value>> {
    if json.trim().is_empty() {
        return Ok(HashMap::new());
    }

    let json_value: serde_json::Value =
        serde_json::from_str(json).with_context(|| "Failed to parse JSON data")?;

    json_to_value_map(&json_value)
}

/// Convert a JSON value to a HashMap of Values
///
/// # Errors
/// Returns an error if the JSON is not an object at the top level.
pub fn json_to_value_map(json: &serde_json::Value) -> Result<HashMap<String, Value>> {
    match json {
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::with_capacity(obj.len());
            for (key, value) in obj {
                map.insert(key.clone(), json_to_value_with_depth(value, 0)?);
            }
            Ok(map)
        }
        serde_json::Value::Null => Ok(HashMap::new()),
        _ => bail!("Data must be a JSON object, got: {}", json_type_name(json)),
    }
}

/// Convert a JSON value to an Amoskeag Value
///
/// # Errors
/// Returns an error if the JSON contains invalid numbers or exceeds max depth.
pub fn json_to_value(json: &serde_json::Value) -> Result<Value> {
    json_to_value_with_depth(json, 0)
}

fn json_to_value_with_depth(json: &serde_json::Value, depth: usize) -> Result<Value> {
    if depth > MAX_JSON_DEPTH {
        bail!("JSON nesting too deep (max {} levels)", MAX_JSON_DEPTH);
    }

    match json {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if f.is_nan() {
                    bail!("JSON number converted to NaN");
                }
                if f.is_infinite() {
                    bail!("JSON number converted to infinity");
                }
                Ok(Value::Number(f))
            } else {
                bail!("Invalid number in JSON: cannot convert to f64")
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let values: Result<Vec<Value>> = arr
                .iter()
                .map(|v| json_to_value_with_depth(v, depth + 1))
                .collect();
            Ok(Value::Array(values?))
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::with_capacity(obj.len());
            for (key, value) in obj {
                map.insert(key.clone(), json_to_value_with_depth(value, depth + 1)?);
            }
            Ok(Value::Dictionary(map))
        }
    }
}

fn json_type_name(json: &serde_json::Value) -> &'static str {
    match json {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_string() {
        let result = parse_json_data("").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_whitespace_only() {
        let result = parse_json_data("   \n\t  ").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_null() {
        let result = parse_json_data("null").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_simple_object() {
        let result = parse_json_data(r#"{"name": "test", "value": 42}"#).unwrap();
        assert_eq!(result.len(), 2);
        assert!(matches!(result.get("name"), Some(Value::String(s)) if s == "test"));
        assert!(matches!(result.get("value"), Some(Value::Number(n)) if *n == 42.0));
    }

    #[test]
    fn test_parse_nested_object() {
        let result = parse_json_data(r#"{"user": {"name": "alice", "age": 30}}"#).unwrap();
        assert_eq!(result.len(), 1);
        if let Some(Value::Dictionary(user)) = result.get("user") {
            assert!(matches!(user.get("name"), Some(Value::String(s)) if s == "alice"));
            assert!(matches!(user.get("age"), Some(Value::Number(n)) if *n == 30.0));
        } else {
            panic!("Expected dictionary");
        }
    }

    #[test]
    fn test_parse_array() {
        let result = parse_json_data(r#"{"items": [1, 2, 3]}"#).unwrap();
        if let Some(Value::Array(arr)) = result.get("items") {
            assert_eq!(arr.len(), 3);
            assert!(matches!(arr[0], Value::Number(n) if n == 1.0));
            assert!(matches!(arr[1], Value::Number(n) if n == 2.0));
            assert!(matches!(arr[2], Value::Number(n) if n == 3.0));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_parse_boolean_values() {
        let result = parse_json_data(r#"{"active": true, "deleted": false}"#).unwrap();
        assert!(matches!(result.get("active"), Some(Value::Boolean(true))));
        assert!(matches!(result.get("deleted"), Some(Value::Boolean(false))));
    }

    #[test]
    fn test_parse_null_value() {
        let result = parse_json_data(r#"{"value": null}"#).unwrap();
        assert!(matches!(result.get("value"), Some(Value::Nil)));
    }

    #[test]
    fn test_parse_invalid_json() {
        assert!(parse_json_data("{invalid}").is_err());
        assert!(parse_json_data("not json").is_err());
    }

    #[test]
    fn test_parse_non_object_top_level() {
        assert!(parse_json_data("[1, 2, 3]").is_err());
        assert!(parse_json_data("\"string\"").is_err());
        assert!(parse_json_data("42").is_err());
        assert!(parse_json_data("true").is_err());
    }

    #[test]
    fn test_json_to_value_all_types() {
        assert!(matches!(
            json_to_value(&serde_json::Value::Null),
            Ok(Value::Nil)
        ));
        assert!(matches!(
            json_to_value(&serde_json::Value::Bool(true)),
            Ok(Value::Boolean(true))
        ));
        assert!(matches!(
            json_to_value(&serde_json::json!(42)),
            Ok(Value::Number(n)) if n == 42.0
        ));
        assert!(matches!(
            json_to_value(&serde_json::json!("hello")),
            Ok(Value::String(s)) if s == "hello"
        ));
    }

    #[test]
    fn test_json_type_name() {
        assert_eq!(json_type_name(&serde_json::Value::Null), "null");
        assert_eq!(json_type_name(&serde_json::Value::Bool(true)), "boolean");
        assert_eq!(json_type_name(&serde_json::json!(42)), "number");
        assert_eq!(json_type_name(&serde_json::json!("hello")), "string");
        assert_eq!(json_type_name(&serde_json::json!([1, 2, 3])), "array");
        assert_eq!(json_type_name(&serde_json::json!({"a": 1})), "object");
    }

    #[test]
    fn test_float_numbers() {
        let result = parse_json_data(r#"{"value": 1.23456, "neg": -2.5}"#).unwrap();
        assert!(
            matches!(result.get("value"), Some(Value::Number(n)) if (*n - 1.23456).abs() < 0.0001)
        );
        assert!(matches!(result.get("neg"), Some(Value::Number(n)) if *n == -2.5));
    }

    #[test]
    fn test_empty_object() {
        let result = parse_json_data("{}").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_empty_array_in_object() {
        let result = parse_json_data(r#"{"items": []}"#).unwrap();
        if let Some(Value::Array(arr)) = result.get("items") {
            assert!(arr.is_empty());
        } else {
            panic!("Expected array");
        }
    }
}
