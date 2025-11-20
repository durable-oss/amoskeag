//! Amoskeag Standard Library - Functions
//!
//! This crate implements the standard library functions for the Amoskeag language,
//! organized by category: string, numeric, collection, logic, date, and financial functions.

// Re-export the Value type from operators
pub use amoskeag_stdlib_operators::{OperatorError, Value};

pub mod string;
pub mod numeric;
pub mod collection;
pub mod logic;

/// Error types for function operations
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionError {
    TypeError { expected: String, got: String },
    ArgumentError { message: String },
    IndexOutOfBounds { index: usize, len: usize },
    InvalidOperation { message: String },
}

impl std::fmt::Display for FunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionError::TypeError { expected, got } => {
                write!(f, "Type error: expected {}, got {}", expected, got)
            }
            FunctionError::ArgumentError { message } => {
                write!(f, "Argument error: {}", message)
            }
            FunctionError::IndexOutOfBounds { index, len } => {
                write!(f, "Index out of bounds: index {} out of length {}", index, len)
            }
            FunctionError::InvalidOperation { message } => {
                write!(f, "Invalid operation: {}", message)
            }
        }
    }
}

impl std::error::Error for FunctionError {}

// Re-export all public functions for convenience
pub use string::*;
pub use numeric::*;
pub use collection::*;
pub use logic::*;
