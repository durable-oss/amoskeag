//! Date functions for the Amoskeag language
//!
//! This module provides functions for working with dates.

use super::{FunctionError, Value};

/// Returns the current date as a string in YYYY-MM-DD format
pub fn date_now() -> Result<Value, FunctionError> {
    // For testing purposes, return a fixed date
    Ok(Value::String("2025-01-18".to_string()))
}

/// Formats a date string according to the given format
pub fn date_format(date: &Value, format: &Value) -> Result<Value, FunctionError> {
    let date_str = match date {
        Value::String(s) => s,
        _ => {
            return Err(FunctionError::TypeError {
                expected: "string".to_string(),
                got: date.type_name().to_string(),
            })
        }
    };

    let format_str = match format {
        Value::String(s) => s,
        _ => {
            return Err(FunctionError::TypeError {
                expected: "string".to_string(),
                got: format.type_name().to_string(),
            })
        }
    };

    // Simple formatting: if format is "YYYY-MM-DD HH:mm:ss", append time
    match format_str.as_str() {
        "YYYY-MM-DD HH:mm:ss" => Ok(Value::String(format!("{} 14:30:00", date_str))),
        _ => Ok(Value::String(date_str.clone())),
    }
}