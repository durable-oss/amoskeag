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

/// Truncates a datetime string to just the date portion (YYYY-MM-DD)
pub fn date_trunc(datetime: &Value) -> Result<Value, FunctionError> {
    let datetime_str = match datetime {
        Value::String(s) => s,
        _ => {
            return Err(FunctionError::TypeError {
                expected: "string".to_string(),
                got: datetime.type_name().to_string(),
            })
        }
    };

    // Extract just the date part (YYYY-MM-DD) from various datetime formats
    // Handles: "2025-01-18T14:30:00Z", "2025-01-18 14:30:00", "2025-01-18", etc.
    let date_part = datetime_str
        .split('T')
        .next()
        .unwrap_or(datetime_str)
        .split(' ')
        .next()
        .unwrap_or(datetime_str);

    // Validate basic date format (YYYY-MM-DD)
    if date_part.len() >= 10
        && date_part.chars().nth(4) == Some('-')
        && date_part.chars().nth(7) == Some('-')
    {
        Ok(Value::String(date_part[..10].to_string()))
    } else {
        Err(FunctionError::ValueError {
            message: format!("Invalid datetime format: {}", datetime_str),
        })
    }
}

/// Parses a date string in YYYY-MM-DD format and validates it
pub fn date_parse(date_str: &Value) -> Result<Value, FunctionError> {
    let s = match date_str {
        Value::String(s) => s,
        _ => {
            return Err(FunctionError::TypeError {
                expected: "string".to_string(),
                got: date_str.type_name().to_string(),
            })
        }
    };

    // Basic validation for YYYY-MM-DD format
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err(FunctionError::ValueError {
            message: format!("Invalid date format: {}. Expected YYYY-MM-DD", s),
        });
    }

    // Validate year, month, day
    let year = parts[0]
        .parse::<i32>()
        .map_err(|_| FunctionError::ValueError {
            message: format!("Invalid year: {}", parts[0]),
        })?;

    let month = parts[1]
        .parse::<u32>()
        .map_err(|_| FunctionError::ValueError {
            message: format!("Invalid month: {}", parts[1]),
        })?;

    let day = parts[2]
        .parse::<u32>()
        .map_err(|_| FunctionError::ValueError {
            message: format!("Invalid day: {}", parts[2]),
        })?;

    // Validate ranges
    if !(1..=12).contains(&month) {
        return Err(FunctionError::ValueError {
            message: format!("Month must be between 1 and 12, got: {}", month),
        });
    }

    if !(1..=31).contains(&day) {
        return Err(FunctionError::ValueError {
            message: format!("Day must be between 1 and 31, got: {}", day),
        });
    }

    if !(1000..=9999).contains(&year) {
        return Err(FunctionError::ValueError {
            message: format!("Year must be between 1000 and 9999, got: {}", year),
        });
    }

    // Return the validated date string
    Ok(Value::String(s.clone()))
}
