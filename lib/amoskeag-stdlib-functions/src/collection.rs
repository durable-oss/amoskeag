//! Collection manipulation functions for Amoskeag

use crate::{FunctionError, Value};

/// Get the size/length of a collection
/// size(val: String | Array | Dictionary) -> Number
pub fn size(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
        Value::Dictionary(dict) => Ok(Value::Number(dict.len() as f64)),
        _ => Err(FunctionError::TypeError {
            expected: "String, Array, or Dictionary".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Get the first element of an array
/// first(arr: Array) -> Any
pub fn first(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => Ok(arr.first().cloned().unwrap_or(Value::Nil)),
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Get the last element of an array
/// last(arr: Array) -> Any
pub fn last(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => Ok(arr.last().cloned().unwrap_or(Value::Nil)),
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Check if an array contains a value
/// contains(arr: Array, val: Any) -> Boolean
pub fn contains(array: &Value, value: &Value) -> Result<Value, FunctionError> {
    match array {
        Value::Array(arr) => Ok(Value::Boolean(arr.contains(value))),
        Value::String(s) => {
            // Also support checking if a string contains a substring
            match value {
                Value::String(substr) => Ok(Value::Boolean(s.contains(substr.as_str()))),
                _ => Err(FunctionError::TypeError {
                    expected: "String".to_string(),
                    got: value.type_name().to_string(),
                }),
            }
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array or String".to_string(),
            got: array.type_name().to_string(),
        }),
    }
}

/// Sum an array of numbers
/// sum(arr: Array) -> Number
pub fn sum(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => {
            let mut total = 0.0;
            for item in arr {
                match item {
                    Value::Number(n) => total += n,
                    _ => {
                        return Err(FunctionError::TypeError {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {}", item.type_name()),
                        })
                    }
                }
            }
            Ok(Value::Number(total))
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Calculate the average of an array of numbers
/// avg(arr: Array) -> Number
pub fn avg(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(Value::Nil);
            }

            let total = sum(value)?;
            match total {
                Value::Number(n) => Ok(Value::Number(n / arr.len() as f64)),
                _ => unreachable!(),
            }
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Sort an array (ascending order)
/// sort(arr: Array) -> Array
pub fn sort(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => {
            let mut sorted = arr.clone();

            // Check if all elements are numbers or all are strings
            let all_numbers = arr.iter().all(|v| matches!(v, Value::Number(_)));
            let all_strings = arr.iter().all(|v| matches!(v, Value::String(_)));

            if all_numbers {
                sorted.sort_by(|a, b| {
                    if let (Value::Number(x), Value::Number(y)) = (a, b) {
                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                Ok(Value::Array(sorted))
            } else if all_strings {
                sorted.sort_by(|a, b| {
                    if let (Value::String(x), Value::String(y)) = (a, b) {
                        x.cmp(y)
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                Ok(Value::Array(sorted))
            } else {
                Err(FunctionError::InvalidOperation {
                    message: "Array must contain all Numbers or all Strings to sort".to_string(),
                })
            }
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Get the keys of a dictionary
/// keys(dict: Dictionary) -> Array
pub fn keys(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Dictionary(dict) => {
            let mut key_list: Vec<String> = dict.keys().cloned().collect();
            key_list.sort();
            let keys: Vec<Value> = key_list.into_iter().map(Value::String).collect();
            Ok(Value::Array(keys))
        }
        _ => Err(FunctionError::TypeError {
            expected: "Dictionary".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Get the values of a dictionary
/// values(dict: Dictionary) -> Array
pub fn values(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Dictionary(dict) => {
            let values: Vec<Value> = dict.values().cloned().collect();
            Ok(Value::Array(values))
        }
        _ => Err(FunctionError::TypeError {
            expected: "Dictionary".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Reverse an array
/// reverse(arr: Array) -> Array
pub fn reverse(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => {
            let mut reversed = arr.clone();
            reversed.reverse();
            Ok(Value::Array(reversed))
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Get element at index (0-based)
/// at(arr: Array, index: Number) -> Any
pub fn at(array: &Value, index: &Value) -> Result<Value, FunctionError> {
    match (array, index) {
        (Value::Array(arr), Value::Number(idx)) => {
            let i = *idx as i64;
            let actual_index = if i < 0 {
                // Support negative indexing
                (arr.len() as i64 + i) as usize
            } else {
                i as usize
            };

            Ok(arr.get(actual_index).cloned().unwrap_or(Value::Nil))
        }
        (Value::Array(_), _) => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: index.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: array.type_name().to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_size() {
        assert_eq!(
            size(&Value::String("hello".to_string())).unwrap(),
            Value::Number(5.0)
        );
        assert_eq!(
            size(&Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ]))
            .unwrap(),
            Value::Number(3.0)
        );
    }

    #[test]
    fn test_first() {
        let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        assert_eq!(first(&arr).unwrap(), Value::Number(1.0));

        let empty = Value::Array(vec![]);
        assert_eq!(first(&empty).unwrap(), Value::Nil);
    }

    #[test]
    fn test_last() {
        let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        assert_eq!(last(&arr).unwrap(), Value::Number(2.0));

        let empty = Value::Array(vec![]);
        assert_eq!(last(&empty).unwrap(), Value::Nil);
    }

    #[test]
    fn test_contains() {
        let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        assert_eq!(
            contains(&arr, &Value::Number(2.0)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            contains(&arr, &Value::Number(3.0)).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_contains_string() {
        let s = Value::String("hello world".to_string());
        assert_eq!(
            contains(&s, &Value::String("world".to_string())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            contains(&s, &Value::String("foo".to_string())).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_sum() {
        let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]);
        assert_eq!(sum(&arr).unwrap(), Value::Number(6.0));
    }

    #[test]
    fn test_avg() {
        let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]);
        assert_eq!(avg(&arr).unwrap(), Value::Number(2.0));

        let empty = Value::Array(vec![]);
        assert_eq!(avg(&empty).unwrap(), Value::Nil);
    }

    #[test]
    fn test_sort_numbers() {
        let arr = Value::Array(vec![Value::Number(3.0), Value::Number(1.0), Value::Number(2.0)]);
        let sorted = sort(&arr).unwrap();
        assert_eq!(
            sorted,
            Value::Array(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)])
        );
    }

    #[test]
    fn test_sort_strings() {
        let arr = Value::Array(vec![
            Value::String("c".to_string()),
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);
        let sorted = sort(&arr).unwrap();
        assert_eq!(
            sorted,
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
            ])
        );
    }

    #[test]
    fn test_keys() {
        let mut dict = HashMap::new();
        dict.insert("a".to_string(), Value::Number(1.0));
        dict.insert("b".to_string(), Value::Number(2.0));
        let d = Value::Dictionary(dict);

        let result = keys(&d).unwrap();
        // Keys should be sorted
        assert_eq!(
            result,
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
            ])
        );
    }

    #[test]
    fn test_values() {
        let mut dict = HashMap::new();
        dict.insert("a".to_string(), Value::Number(1.0));
        let d = Value::Dictionary(dict);

        let result = values(&d).unwrap();
        assert!(matches!(result, Value::Array(_)));
    }

    #[test]
    fn test_reverse() {
        let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]);
        let reversed = reverse(&arr).unwrap();
        assert_eq!(
            reversed,
            Value::Array(vec![Value::Number(3.0), Value::Number(2.0), Value::Number(1.0)])
        );
    }

    #[test]
    fn test_at() {
        let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]);

        assert_eq!(at(&arr, &Value::Number(0.0)).unwrap(), Value::Number(1.0));
        assert_eq!(at(&arr, &Value::Number(2.0)).unwrap(), Value::Number(3.0));
        assert_eq!(at(&arr, &Value::Number(10.0)).unwrap(), Value::Nil);

        // Test negative indexing
        assert_eq!(
            at(&arr, &Value::Number(-1.0)).unwrap(),
            Value::Number(3.0)
        );
    }
}
