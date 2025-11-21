//! Backend type selection and execution

use amoskeag::{evaluate, AmoskeagValue as Value, CompiledProgram};
use anyhow::{bail, Result};
use std::collections::HashMap;

/// Supported backend types for program execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BackendType {
    #[default]
    Interpreter,
    #[cfg(feature = "jit")]
    Jit,
}

impl BackendType {
    /// Parse a backend type from a string
    ///
    /// # Errors
    /// Returns an error if the backend string is not recognized or if
    /// JIT is requested but the feature is not enabled.
    pub fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            bail!("Backend name cannot be empty");
        }

        match s.to_lowercase().as_str() {
            "interpreter" | "interp" => Ok(BackendType::Interpreter),
            #[cfg(feature = "jit")]
            "jit" => Ok(BackendType::Jit),
            #[cfg(not(feature = "jit"))]
            "jit" => bail!(
                "JIT backend not available. This is an enterprise feature. Contact support for access."
            ),
            _ => bail!(
                "Unknown backend: {}. Available backends: interpreter{}",
                s,
                if cfg!(feature = "jit") { ", jit" } else { "" }
            ),
        }
    }

    /// Get the display name for this backend
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            BackendType::Interpreter => "interpreter",
            #[cfg(feature = "jit")]
            BackendType::Jit => "jit",
        }
    }

    /// Get a list of available backend names
    #[must_use]
    pub fn available_backends() -> &'static str {
        if cfg!(feature = "jit") {
            "interpreter, jit"
        } else {
            "interpreter"
        }
    }
}

/// Evaluate a program using the selected backend
///
/// # Errors
/// Returns an error if evaluation fails or if the JIT backend doesn't
/// support the given expression.
pub fn evaluate_with_backend(
    program: &CompiledProgram,
    data: &HashMap<String, Value>,
    backend_type: &BackendType,
) -> Result<Value> {
    match backend_type {
        BackendType::Interpreter => evaluate(program, data).map_err(|e| anyhow::anyhow!("{}", e)),
        #[cfg(feature = "jit")]
        BackendType::Jit => {
            // JIT backend requires enterprise amoskeag-jit crate (not available in open-source version)
            bail!(
                "JIT backend is not available in the open-source version. \
                 Try using --backend interpreter instead."
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_from_str_interpreter() {
        assert!(matches!(
            BackendType::from_str("interpreter"),
            Ok(BackendType::Interpreter)
        ));
        assert!(matches!(
            BackendType::from_str("interp"),
            Ok(BackendType::Interpreter)
        ));
        assert!(matches!(
            BackendType::from_str("INTERPRETER"),
            Ok(BackendType::Interpreter)
        ));
        assert!(matches!(
            BackendType::from_str("Interp"),
            Ok(BackendType::Interpreter)
        ));
    }

    #[test]
    fn test_backend_from_str_unknown() {
        assert!(BackendType::from_str("unknown").is_err());
        assert!(BackendType::from_str("invalid").is_err());
    }

    #[test]
    fn test_backend_from_str_empty() {
        assert!(BackendType::from_str("").is_err());
    }

    #[test]
    fn test_backend_name() {
        assert_eq!(BackendType::Interpreter.name(), "interpreter");
    }

    #[test]
    fn test_backend_default() {
        assert_eq!(BackendType::default(), BackendType::Interpreter);
    }

    #[test]
    fn test_backend_available_backends() {
        let backends = BackendType::available_backends();
        assert!(backends.contains("interpreter"));
    }

    #[cfg(not(feature = "jit"))]
    #[test]
    fn test_backend_jit_unavailable() {
        let result = BackendType::from_str("jit");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("enterprise feature"));
    }

    #[cfg(feature = "jit")]
    #[test]
    fn test_backend_jit_available() {
        assert!(matches!(BackendType::from_str("jit"), Ok(BackendType::Jit)));
        assert_eq!(BackendType::Jit.name(), "jit");
    }
}
