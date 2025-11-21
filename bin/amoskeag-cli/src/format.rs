//! Value formatting utilities

use amoskeag::AmoskeagValue as Value;

/// Maximum recursion depth for formatting to prevent stack overflow
const MAX_FORMAT_DEPTH: usize = 50;

/// Format a Value for display
#[must_use]
pub fn format_value(value: &Value) -> String {
    format_value_with_depth(value, 0)
}

fn format_value_with_depth(value: &Value, depth: usize) -> String {
    if depth > MAX_FORMAT_DEPTH {
        return "[...]".to_string();
    }

    match value {
        Value::Number(n) => {
            if n.is_nan() {
                "NaN".to_string()
            } else if n.is_infinite() {
                if *n > 0.0 {
                    "Infinity".to_string()
                } else {
                    "-Infinity".to_string()
                }
            } else if n.fract() == 0.0 && n.abs() < 1e15 {
                // Display as integer if it's a whole number
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        Value::String(s) => s.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::Nil => "nil".to_string(),
        Value::Symbol(s) => format!(":{}", s),
        Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| format_value_with_depth(v, depth + 1))
                    .collect();
                format!("[{}]", items.join(", "))
            }
        }
        Value::Dictionary(map) => {
            if map.is_empty() {
                "{}".to_string()
            } else {
                let mut items: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, format_value_with_depth(v, depth + 1)))
                    .collect();
                // Sort for consistent output
                items.sort();
                format!("{{{}}}", items.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_format_number_integer() {
        assert_eq!(format_value(&Value::Number(42.0)), "42");
        assert_eq!(format_value(&Value::Number(-100.0)), "-100");
        assert_eq!(format_value(&Value::Number(0.0)), "0");
    }

    #[test]
    fn test_format_number_float() {
        assert_eq!(format_value(&Value::Number(3.15)), "3.15");
        assert_eq!(format_value(&Value::Number(-2.5)), "-2.5");
    }

    #[test]
    fn test_format_number_special() {
        assert_eq!(format_value(&Value::Number(f64::NAN)), "NaN");
        assert_eq!(format_value(&Value::Number(f64::INFINITY)), "Infinity");
        assert_eq!(format_value(&Value::Number(f64::NEG_INFINITY)), "-Infinity");
    }

    #[test]
    fn test_format_string() {
        assert_eq!(format_value(&Value::String("hello".to_string())), "hello");
        assert_eq!(format_value(&Value::String("".to_string())), "");
        assert_eq!(
            format_value(&Value::String("with spaces".to_string())),
            "with spaces"
        );
    }

    #[test]
    fn test_format_boolean() {
        assert_eq!(format_value(&Value::Boolean(true)), "true");
        assert_eq!(format_value(&Value::Boolean(false)), "false");
    }

    #[test]
    fn test_format_nil() {
        assert_eq!(format_value(&Value::Nil), "nil");
    }

    #[test]
    fn test_format_symbol() {
        assert_eq!(
            format_value(&Value::Symbol("approve".to_string())),
            ":approve"
        );
        assert_eq!(format_value(&Value::Symbol("test".to_string())), ":test");
    }

    #[test]
    fn test_format_empty_array() {
        assert_eq!(format_value(&Value::Array(vec![])), "[]");
    }

    #[test]
    fn test_format_array() {
        let arr = Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        assert_eq!(format_value(&arr), "[1, 2, 3]");
    }

    #[test]
    fn test_format_mixed_array() {
        let arr = Value::Array(vec![
            Value::Number(42.0),
            Value::String("hello".to_string()),
            Value::Boolean(true),
        ]);
        assert_eq!(format_value(&arr), "[42, hello, true]");
    }

    #[test]
    fn test_format_empty_dictionary() {
        assert_eq!(format_value(&Value::Dictionary(HashMap::new())), "{}");
    }

    #[test]
    fn test_format_dictionary() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::String("alice".to_string()));
        let result = format_value(&Value::Dictionary(map));
        assert!(result.contains("name: alice"));
    }

    #[test]
    fn test_format_nested_structures() {
        let inner = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        let mut map = HashMap::new();
        map.insert("items".to_string(), inner);
        let outer = Value::Dictionary(map);
        let result = format_value(&outer);
        assert!(result.contains("items: [1, 2]"));
    }

    #[test]
    fn test_format_large_integer() {
        // Numbers larger than 1e15 should use float format
        assert_eq!(format_value(&Value::Number(1e14)), "100000000000000");
    }
}
