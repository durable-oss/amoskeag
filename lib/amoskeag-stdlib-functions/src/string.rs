//! String manipulation functions for Amoskeag

use crate::{FunctionError, Value};

/// Convert a string to uppercase
/// upcase(str: String) -> String
pub fn upcase(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Convert a string to lowercase
/// downcase(str: String) -> String
pub fn downcase(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Capitalize the first character of a string
/// capitalize(str: String) -> String
pub fn capitalize(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::String(s) => {
            let mut chars = s.chars();
            match chars.next() {
                None => Ok(Value::String(String::new())),
                Some(first) => {
                    let capitalized = first.to_uppercase().collect::<String>()
                        + chars.as_str().to_lowercase().as_str();
                    Ok(Value::String(capitalized))
                }
            }
        }
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Remove leading and trailing whitespace
/// strip(str: String) -> String
pub fn strip(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Remove leading whitespace
/// lstrip(str: String) -> String
pub fn lstrip(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::String(s) => Ok(Value::String(s.trim_start().to_string())),
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Remove trailing whitespace
/// rstrip(str: String) -> String
pub fn rstrip(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::String(s) => Ok(Value::String(s.trim_end().to_string())),
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Split a string into an array by a separator
/// split(str: String, sep: String) -> Array
pub fn split(value: &Value, separator: &Value) -> Result<Value, FunctionError> {
    match (value, separator) {
        (Value::String(s), Value::String(sep)) => {
            let parts: Vec<Value> = s
                .split(sep.as_str())
                .map(|part| Value::String(part.to_string()))
                .collect();
            Ok(Value::Array(parts))
        }
        (Value::String(_), _) => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: separator.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Join an array of strings with a separator
/// join(arr: Array, sep: String) -> String
pub fn join(array: &Value, separator: &Value) -> Result<Value, FunctionError> {
    match (array, separator) {
        (Value::Array(arr), Value::String(sep)) => {
            let strings: Result<Vec<String>, FunctionError> = arr
                .iter()
                .map(|v| match v {
                    Value::String(s) => Ok(s.clone()),
                    _ => Err(FunctionError::TypeError {
                        expected: "Array of Strings".to_string(),
                        got: format!("Array containing {}", v.type_name()),
                    }),
                })
                .collect();

            match strings {
                Ok(strs) => Ok(Value::String(strs.join(sep))),
                Err(e) => Err(e),
            }
        }
        (Value::Array(_), _) => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: separator.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: array.type_name().to_string(),
        }),
    }
}

/// Truncate a string to a maximum length
/// truncate(str: String, len: Number) -> String
pub fn truncate(value: &Value, length: &Value) -> Result<Value, FunctionError> {
    match (value, length) {
        (Value::String(s), Value::Number(len)) => {
            let max_len = (*len).max(0.0) as usize;
            if s.len() <= max_len {
                Ok(Value::String(s.clone()))
            } else {
                let truncated: String = s.chars().take(max_len).collect();
                Ok(Value::String(truncated))
            }
        }
        (Value::String(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: length.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Replace all occurrences of a substring
/// replace(str: String, find: String, replacement: String) -> String
pub fn replace(value: &Value, find: &Value, replacement: &Value) -> Result<Value, FunctionError> {
    match (value, find, replacement) {
        (Value::String(s), Value::String(f), Value::String(r)) => {
            Ok(Value::String(s.replace(f, r)))
        }
        (Value::String(_), Value::String(_), _) => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: replacement.type_name().to_string(),
        }),
        (Value::String(_), _, _) => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: find.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upcase() {
        let result = upcase(&Value::String("hello".to_string())).unwrap();
        assert_eq!(result, Value::String("HELLO".to_string()));
    }

    #[test]
    fn test_downcase() {
        let result = downcase(&Value::String("HELLO".to_string())).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_capitalize() {
        let result = capitalize(&Value::String("hello world".to_string())).unwrap();
        assert_eq!(result, Value::String("Hello world".to_string()));
    }

    #[test]
    fn test_strip() {
        let result = strip(&Value::String("  hello  ".to_string())).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_lstrip() {
        let result = lstrip(&Value::String("  hello  ".to_string())).unwrap();
        assert_eq!(result, Value::String("hello  ".to_string()));
    }

    #[test]
    fn test_rstrip() {
        let result = rstrip(&Value::String("  hello  ".to_string())).unwrap();
        assert_eq!(result, Value::String("  hello".to_string()));
    }

    #[test]
    fn test_split() {
        let result = split(
            &Value::String("a,b,c".to_string()),
            &Value::String(",".to_string()),
        )
        .unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
            ])
        );
    }

    #[test]
    fn test_join() {
        let arr = Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("c".to_string()),
        ]);
        let result = join(&arr, &Value::String(",".to_string())).unwrap();
        assert_eq!(result, Value::String("a,b,c".to_string()));
    }

    #[test]
    fn test_truncate() {
        let result = truncate(
            &Value::String("hello world".to_string()),
            &Value::Number(5.0),
        )
        .unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_replace() {
        let result = replace(
            &Value::String("hello world".to_string()),
            &Value::String("world".to_string()),
            &Value::String("rust".to_string()),
        )
        .unwrap();
        assert_eq!(result, Value::String("hello rust".to_string()));
    }
}
