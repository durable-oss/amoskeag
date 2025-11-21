//! Error detection module
//!
//! Detects common programming errors in Amoskeag expressions.

use crate::range_analysis::ValueRange;
use amoskeag_parser::{BinaryOp, Expr, UnaryOp};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Severity level of a detected error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Critical error that will definitely cause a runtime error
    Critical,
    /// Warning about potential issues
    Warning,
    /// Informational message about code quality
    Info,
}

/// A detected programming error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgrammingError {
    /// Severity of the error
    pub severity: ErrorSeverity,
    /// Error message
    pub message: String,
    /// Error category
    pub category: ErrorCategory,
    /// Location in the source (if available)
    pub location: Option<String>,
    /// Suggestion for fixing the error
    pub suggestion: Option<String>,
}

/// Category of programming error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Division by zero
    DivisionByZero,
    /// Type mismatch
    TypeMismatch,
    /// Undefined variable
    UndefinedVariable,
    /// Unreachable code
    UnreachableCode,
    /// Unused variable
    UnusedVariable,
    /// Infinite loop
    InfiniteLoop,
    /// Null pointer dereference
    NullDereference,
    /// Array out of bounds
    ArrayOutOfBounds,
    /// Integer overflow
    IntegerOverflow,
    /// Complexity warning
    Complexity,
    /// Security issue
    Security,
}

/// Error detector
pub struct ErrorDetector {
    errors: Vec<ProgrammingError>,
    defined_symbols: HashSet<String>,
    used_variables: HashSet<String>,
    defined_variables: HashSet<String>,
}

impl ErrorDetector {
    /// Create a new error detector
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            defined_symbols: HashSet::new(),
            used_variables: HashSet::new(),
            defined_variables: HashSet::new(),
        }
    }

    /// Detect errors in an expression
    pub fn detect(
        &mut self,
        expr: &Expr,
        symbols: &[&str],
        ranges: &HashMap<String, ValueRange>,
    ) -> Vec<ProgrammingError> {
        self.errors.clear();
        self.defined_symbols = symbols.iter().map(|s| s.to_string()).collect();
        self.used_variables.clear();
        self.defined_variables.clear();

        self.analyze_expr(expr, ranges);

        // Check for unused variables
        self.check_unused_variables();

        self.errors.clone()
    }

    fn analyze_expr(&mut self, expr: &Expr, ranges: &HashMap<String, ValueRange>) {
        match expr {
            Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | Expr::Nil => {
                // Literals are always safe
            }

            Expr::Symbol(s) => {
                // Check if symbol is defined
                if !self.defined_symbols.contains(s) {
                    self.errors.push(ProgrammingError {
                        severity: ErrorSeverity::Warning,
                        message: format!(
                            "Symbol '{}' may not be defined in the runtime context",
                            s
                        ),
                        category: ErrorCategory::UndefinedVariable,
                        location: Some(format!("symbol: {}", s)),
                        suggestion: Some(format!(
                            "Ensure symbol '{}' is in the allowed symbols list",
                            s
                        )),
                    });
                }
            }

            Expr::Variable(parts) => {
                let var_name = parts.join(".");
                self.used_variables.insert(var_name.clone());

                // Check for potential null dereference on nested access
                if parts.len() > 1 {
                    self.errors.push(ProgrammingError {
                        severity: ErrorSeverity::Info,
                        message: format!(
                            "Nested property access '{}' may fail if intermediate values are nil",
                            var_name
                        ),
                        category: ErrorCategory::NullDereference,
                        location: Some(format!("variable: {}", var_name)),
                        suggestion: Some(
                            "Consider using defensive checks or coalesce function".to_string(),
                        ),
                    });
                }
            }

            Expr::Array(elements) => {
                for elem in elements {
                    self.analyze_expr(elem, ranges);
                }

                // Check for large arrays
                if elements.len() > 1000 {
                    self.errors.push(ProgrammingError {
                        severity: ErrorSeverity::Warning,
                        message: format!(
                            "Large array literal with {} elements may impact performance",
                            elements.len()
                        ),
                        category: ErrorCategory::Complexity,
                        location: Some("array literal".to_string()),
                        suggestion: Some(
                            "Consider generating arrays programmatically or using smaller datasets"
                                .to_string(),
                        ),
                    });
                }
            }

            Expr::Dictionary(pairs) => {
                for (_, value) in pairs {
                    self.analyze_expr(value, ranges);
                }

                // Check for large dictionaries
                if pairs.len() > 100 {
                    self.errors.push(ProgrammingError {
                        severity: ErrorSeverity::Warning,
                        message: format!("Large dictionary literal with {} entries may impact performance", pairs.len()),
                        category: ErrorCategory::Complexity,
                        location: Some("dictionary literal".to_string()),
                        suggestion: Some("Consider using smaller dictionaries or breaking into multiple smaller structures".to_string()),
                    });
                }

                // Check for duplicate keys
                let mut keys = HashSet::new();
                for (key, _) in pairs {
                    if !keys.insert(key) {
                        self.errors.push(ProgrammingError {
                            severity: ErrorSeverity::Warning,
                            message: format!("Duplicate key '{}' in dictionary literal", key),
                            category: ErrorCategory::Security,
                            location: Some(format!("dictionary key: {}", key)),
                            suggestion: Some("Remove duplicate keys".to_string()),
                        });
                    }
                }
            }

            Expr::FunctionCall { name, args } => {
                // Analyze arguments
                for arg in args {
                    self.analyze_expr(arg, ranges);
                }

                // Check for specific risky function patterns
                self.check_function_call(name, args, ranges);
            }

            Expr::Let { name, value, body } => {
                self.defined_variables.insert(name.clone());
                self.analyze_expr(value, ranges);
                self.analyze_expr(body, ranges);
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.analyze_expr(condition, ranges);

                // Check for constant conditions
                if let Some(const_val) = self.is_constant_boolean(condition) {
                    let dead_branch = if const_val { "else" } else { "then" };
                    self.errors.push(ProgrammingError {
                        severity: ErrorSeverity::Warning,
                        message: format!(
                            "Condition is always {}, {} branch is unreachable",
                            const_val, dead_branch
                        ),
                        category: ErrorCategory::UnreachableCode,
                        location: Some("if expression".to_string()),
                        suggestion: Some(format!(
                            "Remove the if expression and keep only the {} branch",
                            if const_val { "then" } else { "else" }
                        )),
                    });
                }

                self.analyze_expr(then_branch, ranges);
                self.analyze_expr(else_branch, ranges);
            }

            Expr::Binary { op, left, right } => {
                self.analyze_expr(left, ranges);
                self.analyze_expr(right, ranges);

                // Check for division by zero
                if matches!(op, BinaryOp::Divide | BinaryOp::Modulo) {
                    if let Expr::Number(n) = **right {
                        if n == 0.0 {
                            self.errors.push(ProgrammingError {
                                severity: ErrorSeverity::Critical,
                                message: "Division by zero".to_string(),
                                category: ErrorCategory::DivisionByZero,
                                location: Some(format!(
                                    "{} by zero",
                                    if matches!(op, BinaryOp::Divide) {
                                        "division"
                                    } else {
                                        "modulo"
                                    }
                                )),
                                suggestion: Some(
                                    "Replace divisor with a non-zero value or add a check"
                                        .to_string(),
                                ),
                            });
                        }
                    } else {
                        // Check if right side could be zero based on range analysis
                        if let Some(var_name) = self.extract_variable_name(right) {
                            if let Some(range) = ranges.get(&var_name) {
                                if range.contains_zero() {
                                    self.errors.push(ProgrammingError {
                                        severity: ErrorSeverity::Warning,
                                        message: format!("Potential division by zero: variable '{}' may be zero", var_name),
                                        category: ErrorCategory::DivisionByZero,
                                        location: Some(format!("division by variable '{}'", var_name)),
                                        suggestion: Some(format!("Add a check to ensure '{}' is not zero before division", var_name)),
                                    });
                                }
                            }
                        }
                    }
                }

                // Check for integer overflow
                if matches!(op, BinaryOp::Multiply | BinaryOp::Add) {
                    if let (Expr::Number(a), Expr::Number(b)) = (&**left, &**right) {
                        if a.is_finite() && b.is_finite() {
                            let result = match op {
                                BinaryOp::Multiply => a * b,
                                BinaryOp::Add => a + b,
                                _ => 0.0,
                            };
                            if !result.is_finite() {
                                self.errors.push(ProgrammingError {
                                    severity: ErrorSeverity::Warning,
                                    message: format!(
                                        "Arithmetic operation results in overflow: {} {} {}",
                                        a, op, b
                                    ),
                                    category: ErrorCategory::IntegerOverflow,
                                    location: Some("arithmetic expression".to_string()),
                                    suggestion: Some(
                                        "Use smaller values or check for overflow".to_string(),
                                    ),
                                });
                            }
                        }
                    }
                }

                // Check for suspicious comparisons
                if matches!(op, BinaryOp::Equal | BinaryOp::NotEqual) {
                    if let (Expr::Number(a), Expr::Number(b)) = (&**left, &**right) {
                        if (a - b).abs() < f64::EPSILON && *a != *b {
                            self.errors.push(ProgrammingError {
                                severity: ErrorSeverity::Info,
                                message: "Floating-point equality comparison may be unreliable".to_string(),
                                category: ErrorCategory::TypeMismatch,
                                location: Some("comparison".to_string()),
                                suggestion: Some("Consider using a threshold-based comparison for floating-point values".to_string()),
                            });
                        }
                    }
                }
            }

            Expr::Unary { op, operand } => {
                self.analyze_expr(operand, ranges);

                // Check for double negation
                if let UnaryOp::Negate = op {
                    if let Expr::Unary {
                        op: UnaryOp::Negate,
                        ..
                    } = **operand
                    {
                        self.errors.push(ProgrammingError {
                            severity: ErrorSeverity::Info,
                            message: "Double negation can be simplified".to_string(),
                            category: ErrorCategory::Complexity,
                            location: Some("unary negation".to_string()),
                            suggestion: Some("Remove double negation".to_string()),
                        });
                    }
                }
            }

            Expr::Pipe { left, right } => {
                self.analyze_expr(left, ranges);
                self.analyze_expr(right, ranges);
            }
        }
    }

    fn check_function_call(
        &mut self,
        name: &str,
        args: &[Expr],
        _ranges: &HashMap<String, ValueRange>,
    ) {
        // Check for common risky patterns
        match name {
            "at" => {
                // Array access function - check for out of bounds
                if args.len() >= 2 {
                    if let (Expr::Array(arr), Expr::Number(idx)) = (&args[0], &args[1]) {
                        let index = *idx as i64;
                        if index < 0 || index >= arr.len() as i64 {
                            self.errors.push(ProgrammingError {
                                severity: ErrorSeverity::Critical,
                                message: format!(
                                    "Array index {} out of bounds (array length: {})",
                                    index,
                                    arr.len()
                                ),
                                category: ErrorCategory::ArrayOutOfBounds,
                                location: Some(format!("at({}, {})", arr.len(), index)),
                                suggestion: Some("Use an index within array bounds".to_string()),
                            });
                        }
                    }
                }
            }
            "divided_by" => {
                // Check for division by zero
                if args.len() >= 2 {
                    if let Expr::Number(n) = &args[1] {
                        if *n == 0.0 {
                            self.errors.push(ProgrammingError {
                                severity: ErrorSeverity::Critical,
                                message: "Division by zero in divided_by function".to_string(),
                                category: ErrorCategory::DivisionByZero,
                                location: Some("divided_by function".to_string()),
                                suggestion: Some("Use a non-zero divisor".to_string()),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn is_constant_boolean(&self, expr: &Expr) -> Option<bool> {
        match expr {
            Expr::Boolean(b) => Some(*b),
            Expr::Binary { op, left, right } => match op {
                BinaryOp::Equal => {
                    if let (Expr::Number(a), Expr::Number(b)) = (&**left, &**right) {
                        Some(a == b)
                    } else {
                        None
                    }
                }
                BinaryOp::NotEqual => {
                    if let (Expr::Number(a), Expr::Number(b)) = (&**left, &**right) {
                        Some(a != b)
                    } else {
                        None
                    }
                }
                BinaryOp::Less => {
                    if let (Expr::Number(a), Expr::Number(b)) = (&**left, &**right) {
                        Some(a < b)
                    } else {
                        None
                    }
                }
                BinaryOp::Greater => {
                    if let (Expr::Number(a), Expr::Number(b)) = (&**left, &**right) {
                        Some(a > b)
                    } else {
                        None
                    }
                }
                BinaryOp::LessEqual => {
                    if let (Expr::Number(a), Expr::Number(b)) = (&**left, &**right) {
                        Some(a <= b)
                    } else {
                        None
                    }
                }
                BinaryOp::GreaterEqual => {
                    if let (Expr::Number(a), Expr::Number(b)) = (&**left, &**right) {
                        Some(a >= b)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn extract_variable_name(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Variable(parts) => Some(parts.join(".")),
            _ => None,
        }
    }

    fn check_unused_variables(&mut self) {
        for defined in &self.defined_variables {
            if !self.used_variables.contains(defined) {
                self.errors.push(ProgrammingError {
                    severity: ErrorSeverity::Info,
                    message: format!("Variable '{}' is defined but never used", defined),
                    category: ErrorCategory::UnusedVariable,
                    location: Some(format!("let binding: {}", defined)),
                    suggestion: Some(format!(
                        "Remove unused variable '{}' or use it in the expression",
                        defined
                    )),
                });
            }
        }
    }
}

impl Default for ErrorDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amoskeag_parser::parse;

    #[test]
    fn test_detect_division_by_zero() {
        let mut detector = ErrorDetector::new();
        let expr = parse("10 / 0").unwrap();
        let errors = detector.detect(&expr, &[], &HashMap::new());
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| matches!(e.category, ErrorCategory::DivisionByZero)));
    }

    #[test]
    fn test_detect_unreachable_code() {
        let mut detector = ErrorDetector::new();
        let expr = parse("if true :yes else :no end").unwrap();
        let errors = detector.detect(&expr, &[], &HashMap::new());
        assert!(errors
            .iter()
            .any(|e| matches!(e.category, ErrorCategory::UnreachableCode)));
    }

    #[test]
    fn test_detect_unused_variable() {
        let mut detector = ErrorDetector::new();
        let expr = parse("let x = 5 in 10").unwrap();
        let errors = detector.detect(&expr, &[], &HashMap::new());
        assert!(errors
            .iter()
            .any(|e| matches!(e.category, ErrorCategory::UnusedVariable)));
    }

    #[test]
    fn test_no_errors_clean_code() {
        let mut detector = ErrorDetector::new();
        let expr = parse("let x = 5 in x + 10").unwrap();
        let errors = detector.detect(&expr, &[], &HashMap::new());
        let critical_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Critical))
            .collect();
        assert_eq!(critical_errors.len(), 0);
    }
}
