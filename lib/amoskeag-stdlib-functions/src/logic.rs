//! Logic and conditional functions for Amoskeag

use crate::{FunctionError, Value};

/// Choose an element from an array by 1-based index (Excel-style)
/// choose(index: Number, arr: Array) -> Any
pub fn choose(index: &Value, array: &Value) -> Result<Value, FunctionError> {
    match (index, array) {
        (Value::Number(idx), Value::Array(arr)) => {
            // 1-based indexing (Excel style)
            if *idx < 1.0 {
                return Ok(Value::Nil);
            }
            let i = (*idx as usize) - 1;
            Ok(arr.get(i).cloned().unwrap_or(Value::Nil))
        }
        (Value::Number(_), _) => Err(FunctionError::TypeError {
            expected: "Array".to_string(),
            got: array.type_name().to_string(),
        }),
        _ => Err(FunctionError::TypeError {
            expected: "Number".to_string(),
            got: index.type_name().to_string(),
        }),
    }
}

/// Conditional expression: if condition is true, return true_val, else return false_val
/// if_then_else(condition: Boolean, true_val: Any, false_val: Any) -> Any
pub fn if_then_else(
    condition: &Value,
    true_val: &Value,
    false_val: &Value,
) -> Result<Value, FunctionError> {
    let cond = match condition {
        Value::Boolean(b) => *b,
        Value::Nil => false,
        _ => true, // Truthy values
    };

    if cond {
        Ok(true_val.clone())
    } else {
        Ok(false_val.clone())
    }
}

/// Check if a value is nil
/// is_nil(val: Any) -> Boolean
pub fn is_nil(value: &Value) -> Value {
    Value::Boolean(matches!(value, Value::Nil))
}

/// Check if a value is a number
/// is_number(val: Any) -> Boolean
pub fn is_number(value: &Value) -> Value {
    Value::Boolean(matches!(value, Value::Number(_)))
}

/// Check if a value is a string
/// is_string(val: Any) -> Boolean
pub fn is_string(value: &Value) -> Value {
    Value::Boolean(matches!(value, Value::String(_)))
}

/// Check if a value is a boolean
/// is_boolean(val: Any) -> Boolean
pub fn is_boolean(value: &Value) -> Value {
    Value::Boolean(matches!(value, Value::Boolean(_)))
}

/// Check if a value is an array
/// is_array(val: Any) -> Boolean
pub fn is_array(value: &Value) -> Value {
    Value::Boolean(matches!(value, Value::Array(_)))
}

/// Check if a value is a dictionary
/// is_dictionary(val: Any) -> Boolean
pub fn is_dictionary(value: &Value) -> Value {
    Value::Boolean(matches!(value, Value::Dictionary(_)))
}

/// Check if a value is a symbol
/// is_symbol(val: Any) -> Boolean
pub fn is_symbol(value: &Value) -> Value {
    Value::Boolean(matches!(value, Value::Symbol(_)))
}

/// Coalesce: return the first non-nil value
/// coalesce(val1: Any, val2: Any) -> Any
pub fn coalesce(val1: &Value, val2: &Value) -> Value {
    if matches!(val1, Value::Nil) {
        val2.clone()
    } else {
        val1.clone()
    }
}

/// Default: return value if not nil, else return default
/// default(val: Any, default_val: Any) -> Any
pub fn default(value: &Value, default_val: &Value) -> Value {
    coalesce(value, default_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose() {
        let arr = Value::Array(vec![
            Value::String("first".to_string()),
            Value::String("second".to_string()),
            Value::String("third".to_string()),
        ]);

        // 1-based indexing
        assert_eq!(
            choose(&Value::Number(1.0), &arr).unwrap(),
            Value::String("first".to_string())
        );
        assert_eq!(
            choose(&Value::Number(2.0), &arr).unwrap(),
            Value::String("second".to_string())
        );
        assert_eq!(
            choose(&Value::Number(3.0), &arr).unwrap(),
            Value::String("third".to_string())
        );

        // Out of bounds
        assert_eq!(choose(&Value::Number(10.0), &arr).unwrap(), Value::Nil);

        // Index 0 should return nil (1-based)
        assert_eq!(choose(&Value::Number(0.0), &arr).unwrap(), Value::Nil);
    }

    #[test]
    fn test_if_then_else() {
        assert_eq!(
            if_then_else(
                &Value::Boolean(true),
                &Value::String("yes".to_string()),
                &Value::String("no".to_string())
            )
            .unwrap(),
            Value::String("yes".to_string())
        );

        assert_eq!(
            if_then_else(
                &Value::Boolean(false),
                &Value::String("yes".to_string()),
                &Value::String("no".to_string())
            )
            .unwrap(),
            Value::String("no".to_string())
        );

        // Truthy/falsy behavior
        assert_eq!(
            if_then_else(
                &Value::Nil,
                &Value::String("yes".to_string()),
                &Value::String("no".to_string())
            )
            .unwrap(),
            Value::String("no".to_string())
        );

        assert_eq!(
            if_then_else(
                &Value::Number(42.0),
                &Value::String("yes".to_string()),
                &Value::String("no".to_string())
            )
            .unwrap(),
            Value::String("yes".to_string())
        );
    }

    #[test]
    fn test_is_nil() {
        assert_eq!(is_nil(&Value::Nil), Value::Boolean(true));
        assert_eq!(is_nil(&Value::Number(0.0)), Value::Boolean(false));
    }

    #[test]
    fn test_is_number() {
        assert_eq!(is_number(&Value::Number(42.0)), Value::Boolean(true));
        assert_eq!(
            is_number(&Value::String("42".to_string())),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_is_string() {
        assert_eq!(
            is_string(&Value::String("hello".to_string())),
            Value::Boolean(true)
        );
        assert_eq!(is_string(&Value::Number(42.0)), Value::Boolean(false));
    }

    #[test]
    fn test_is_boolean() {
        assert_eq!(is_boolean(&Value::Boolean(true)), Value::Boolean(true));
        assert_eq!(is_boolean(&Value::Number(1.0)), Value::Boolean(false));
    }

    #[test]
    fn test_is_array() {
        assert_eq!(is_array(&Value::Array(vec![])), Value::Boolean(true));
        assert_eq!(is_array(&Value::String("[]".to_string())), Value::Boolean(false));
    }

    #[test]
    fn test_coalesce() {
        assert_eq!(
            coalesce(&Value::Nil, &Value::Number(42.0)),
            Value::Number(42.0)
        );
        assert_eq!(
            coalesce(&Value::Number(10.0), &Value::Number(42.0)),
            Value::Number(10.0)
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            default(&Value::Nil, &Value::String("default".to_string())),
            Value::String("default".to_string())
        );
        assert_eq!(
            default(
                &Value::String("value".to_string()),
                &Value::String("default".to_string())
            ),
            Value::String("value".to_string())
        );
    }
}
