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

/// Remove duplicate elements from an array
/// uniq(arr: Array) -> Array
pub fn uniq(value: &Value) -> Result<Value, FunctionError> {
    match value {
        Value::Array(arr) => {
            let mut seen = Vec::new();
            let mut unique = Vec::new();

            for item in arr {
                if !seen.contains(item) {
                    seen.push(item.clone());
                    unique.push(item.clone());
                }
            }

            Ok(Value::Array(unique))
        }
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: value.type_name().to_string(),
        }),
    }
}

/// Group array elements by a key
/// group_by(arr: Array, key: String) -> Dictionary
pub fn group_by(array: &Value, key: &Value) -> Result<Value, FunctionError> {
    match (array, key) {
        (Value::Array(arr), Value::String(key_str)) => {
            let mut groups = std::collections::HashMap::new();

            for item in arr {
                match item {
                    Value::Dictionary(dict) => {
                        if let Some(key_value) = dict.get(key_str) {
                            // Convert the key value to a string for grouping
                            let group_key = match key_value {
                                Value::String(s) => s.clone(),
                                Value::Number(n) => n.to_string(),
                                Value::Boolean(b) => b.to_string(),
                                Value::Nil => "nil".to_string(),
                                _ => continue, // Skip complex types
                            };

                            groups
                                .entry(group_key)
                                .or_insert_with(Vec::new)
                                .push(item.clone());
                        }
                    }
                    _ => {
                        return Err(FunctionError::TypeError {
                            expected: "Array of Dictionaries".to_string(),
                            got: format!("Array containing {}", item.type_name()),
                        })
                    }
                }
            }

            // Convert HashMap<String, Vec<Value>> to HashMap<String, Value>
            let result: std::collections::HashMap<String, Value> = groups
                .into_iter()
                .map(|(k, v)| (k, Value::Array(v)))
                .collect();

            Ok(Value::Dictionary(result))
        }
        (Value::Array(_), _) => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: key.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: array.type_name().to_string(),
        }),
    }
}

/// Map a key from an array of dictionaries to an array of values
/// map(arr: Array, key: String) -> Array
pub fn map(array: &Value, key: &Value) -> Result<Value, FunctionError> {
    match (array, key) {
        (Value::Array(arr), Value::String(key_str)) => {
            let mut result = Vec::new();

            for item in arr {
                match item {
                    Value::Dictionary(dict) => {
                        if let Some(value) = dict.get(key_str) {
                            result.push(value.clone());
                        } else {
                            result.push(Value::Nil);
                        }
                    }
                    _ => {
                        return Err(FunctionError::TypeError {
                            expected: "Array of Dictionaries".to_string(),
                            got: format!("Array containing {}", item.type_name()),
                        })
                    }
                }
            }

            Ok(Value::Array(result))
        }
        (Value::Array(_), _) => Err(FunctionError::TypeError {
            expected: "String".to_string(),
            got: key.type_name().to_string(),
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
        let arr = Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        assert_eq!(sum(&arr).unwrap(), Value::Number(6.0));
    }

    #[test]
    fn test_avg() {
        let arr = Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        assert_eq!(avg(&arr).unwrap(), Value::Number(2.0));

        let empty = Value::Array(vec![]);
        assert_eq!(avg(&empty).unwrap(), Value::Nil);
    }

    #[test]
    fn test_sort_numbers() {
        let arr = Value::Array(vec![
            Value::Number(3.0),
            Value::Number(1.0),
            Value::Number(2.0),
        ]);
        let sorted = sort(&arr).unwrap();
        assert_eq!(
            sorted,
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ])
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
        let arr = Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        let reversed = reverse(&arr).unwrap();
        assert_eq!(
            reversed,
            Value::Array(vec![
                Value::Number(3.0),
                Value::Number(2.0),
                Value::Number(1.0)
            ])
        );
    }

    #[test]
    fn test_at() {
        let arr = Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);

        assert_eq!(at(&arr, &Value::Number(0.0)).unwrap(), Value::Number(1.0));
        assert_eq!(at(&arr, &Value::Number(2.0)).unwrap(), Value::Number(3.0));
        assert_eq!(at(&arr, &Value::Number(10.0)).unwrap(), Value::Nil);

        // Test negative indexing
        assert_eq!(at(&arr, &Value::Number(-1.0)).unwrap(), Value::Number(3.0));
    }

    #[test]
    fn test_uniq() {
        let arr = Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(1.0),
            Value::Number(3.0),
            Value::Number(2.0),
        ]);
        let result = uniq(&arr).unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );

        // Test with strings
        let arr_str = Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("a".to_string()),
            Value::String("c".to_string()),
        ]);
        let result_str = uniq(&arr_str).unwrap();
        assert_eq!(
            result_str,
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
            ])
        );

        // Test empty array
        let empty = Value::Array(vec![]);
        assert_eq!(uniq(&empty).unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn test_group_by() {
        // Create test data: array of dictionaries with a "type" field
        let mut dict1 = HashMap::new();
        dict1.insert("type".to_string(), Value::String("fruit".to_string()));
        dict1.insert("name".to_string(), Value::String("apple".to_string()));

        let mut dict2 = HashMap::new();
        dict2.insert("type".to_string(), Value::String("vegetable".to_string()));
        dict2.insert("name".to_string(), Value::String("carrot".to_string()));

        let mut dict3 = HashMap::new();
        dict3.insert("type".to_string(), Value::String("fruit".to_string()));
        dict3.insert("name".to_string(), Value::String("banana".to_string()));

        let arr = Value::Array(vec![
            Value::Dictionary(dict1.clone()),
            Value::Dictionary(dict2.clone()),
            Value::Dictionary(dict3.clone()),
        ]);

        let result = group_by(&arr, &Value::String("type".to_string())).unwrap();

        match result {
            Value::Dictionary(groups) => {
                assert_eq!(groups.len(), 2);

                // Check fruit group
                if let Some(Value::Array(fruits)) = groups.get("fruit") {
                    assert_eq!(fruits.len(), 2);
                }

                // Check vegetable group
                if let Some(Value::Array(vegetables)) = groups.get("vegetable") {
                    assert_eq!(vegetables.len(), 1);
                }
            }
            _ => panic!("Expected Dictionary result"),
        }
    }

    #[test]
    fn test_group_by_with_numbers() {
        let mut dict1 = HashMap::new();
        dict1.insert("score".to_string(), Value::Number(100.0));
        dict1.insert("name".to_string(), Value::String("alice".to_string()));

        let mut dict2 = HashMap::new();
        dict2.insert("score".to_string(), Value::Number(95.0));
        dict2.insert("name".to_string(), Value::String("bob".to_string()));

        let mut dict3 = HashMap::new();
        dict3.insert("score".to_string(), Value::Number(100.0));
        dict3.insert("name".to_string(), Value::String("charlie".to_string()));

        let arr = Value::Array(vec![
            Value::Dictionary(dict1),
            Value::Dictionary(dict2),
            Value::Dictionary(dict3),
        ]);

        let result = group_by(&arr, &Value::String("score".to_string())).unwrap();

        match result {
            Value::Dictionary(groups) => {
                assert_eq!(groups.len(), 2);

                // Check 100 group
                if let Some(Value::Array(perfect_scores)) = groups.get("100") {
                    assert_eq!(perfect_scores.len(), 2);
                }

                // Check 95 group
                if let Some(Value::Array(high_scores)) = groups.get("95") {
                    assert_eq!(high_scores.len(), 1);
                }
            }
            _ => panic!("Expected Dictionary result"),
        }
    }

    #[test]
    fn test_map() {
        let mut dict1 = HashMap::new();
        dict1.insert("name".to_string(), Value::String("alice".to_string()));
        dict1.insert("age".to_string(), Value::Number(30.0));

        let mut dict2 = HashMap::new();
        dict2.insert("name".to_string(), Value::String("bob".to_string()));
        dict2.insert("age".to_string(), Value::Number(25.0));

        let mut dict3 = HashMap::new();
        dict3.insert("name".to_string(), Value::String("charlie".to_string()));
        dict3.insert("age".to_string(), Value::Number(35.0));

        let arr = Value::Array(vec![
            Value::Dictionary(dict1),
            Value::Dictionary(dict2),
            Value::Dictionary(dict3),
        ]);

        // Test mapping "name" key
        let result = map(&arr, &Value::String("name".to_string())).unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::String("alice".to_string()),
                Value::String("bob".to_string()),
                Value::String("charlie".to_string()),
            ])
        );

        // Test mapping "age" key
        let result = map(&arr, &Value::String("age".to_string())).unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::Number(30.0),
                Value::Number(25.0),
                Value::Number(35.0),
            ])
        );
    }

    #[test]
    fn test_map_missing_key() {
        let mut dict1 = HashMap::new();
        dict1.insert("name".to_string(), Value::String("alice".to_string()));

        let mut dict2 = HashMap::new();
        dict2.insert("name".to_string(), Value::String("bob".to_string()));
        dict2.insert("age".to_string(), Value::Number(25.0));

        let arr = Value::Array(vec![Value::Dictionary(dict1), Value::Dictionary(dict2)]);

        // Test mapping "age" key where first dict doesn't have it
        let result = map(&arr, &Value::String("age".to_string())).unwrap();
        assert_eq!(result, Value::Array(vec![Value::Nil, Value::Number(25.0),]));
    }
}
