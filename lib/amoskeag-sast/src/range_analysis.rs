//! Range analysis module
//!
//! Tracks possible value ranges for expressions to detect potential errors.

use amoskeag_parser::{BinaryOp, Expr, UnaryOp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a range of possible values for a numeric expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueRange {
    /// Exact known value
    Exact(f64),
    /// Range between min and max (inclusive)
    Range { min: f64, max: f64 },
    /// Any possible value
    Any,
    /// Non-numeric value (string, boolean, etc.)
    NonNumeric,
}

impl ValueRange {
    /// Create a range from a minimum and maximum value
    pub fn new(min: f64, max: f64) -> Self {
        if min == max {
            ValueRange::Exact(min)
        } else {
            ValueRange::Range { min, max }
        }
    }

    /// Check if the range contains zero
    pub fn contains_zero(&self) -> bool {
        match self {
            ValueRange::Exact(v) => *v == 0.0,
            ValueRange::Range { min, max } => *min <= 0.0 && *max >= 0.0,
            ValueRange::Any => true,
            ValueRange::NonNumeric => false,
        }
    }

    /// Check if the range contains only positive values
    pub fn is_positive(&self) -> bool {
        match self {
            ValueRange::Exact(v) => *v > 0.0,
            ValueRange::Range { min, .. } => *min > 0.0,
            ValueRange::Any => false,
            ValueRange::NonNumeric => false,
        }
    }

    /// Check if the range contains only negative values
    pub fn is_negative(&self) -> bool {
        match self {
            ValueRange::Exact(v) => *v < 0.0,
            ValueRange::Range { max, .. } => *max < 0.0,
            ValueRange::Any => false,
            ValueRange::NonNumeric => false,
        }
    }

    /// Add two ranges
    pub fn add(&self, other: &ValueRange) -> ValueRange {
        match (self, other) {
            (ValueRange::Exact(a), ValueRange::Exact(b)) => ValueRange::Exact(a + b),
            (ValueRange::Exact(a), ValueRange::Range { min, max }) |
            (ValueRange::Range { min, max }, ValueRange::Exact(a)) => {
                ValueRange::Range {
                    min: min + a,
                    max: max + a,
                }
            }
            (ValueRange::Range { min: min1, max: max1 }, ValueRange::Range { min: min2, max: max2 }) => {
                ValueRange::Range {
                    min: min1 + min2,
                    max: max1 + max2,
                }
            }
            _ => ValueRange::Any,
        }
    }

    /// Subtract two ranges
    pub fn subtract(&self, other: &ValueRange) -> ValueRange {
        match (self, other) {
            (ValueRange::Exact(a), ValueRange::Exact(b)) => ValueRange::Exact(a - b),
            (ValueRange::Exact(a), ValueRange::Range { min, max }) => {
                ValueRange::Range {
                    min: a - max,
                    max: a - min,
                }
            }
            (ValueRange::Range { min, max }, ValueRange::Exact(a)) => {
                ValueRange::Range {
                    min: min - a,
                    max: max - a,
                }
            }
            (ValueRange::Range { min: min1, max: max1 }, ValueRange::Range { min: min2, max: max2 }) => {
                ValueRange::Range {
                    min: min1 - max2,
                    max: max1 - min2,
                }
            }
            _ => ValueRange::Any,
        }
    }

    /// Multiply two ranges
    pub fn multiply(&self, other: &ValueRange) -> ValueRange {
        match (self, other) {
            (ValueRange::Exact(a), ValueRange::Exact(b)) => ValueRange::Exact(a * b),
            (ValueRange::Exact(a), ValueRange::Range { min, max }) |
            (ValueRange::Range { min, max }, ValueRange::Exact(a)) => {
                let corners = [a * min, a * max];
                let min_val = corners.iter().cloned().fold(f64::INFINITY, f64::min);
                let max_val = corners.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                ValueRange::Range { min: min_val, max: max_val }
            }
            (ValueRange::Range { min: min1, max: max1 }, ValueRange::Range { min: min2, max: max2 }) => {
                let corners = [
                    min1 * min2,
                    min1 * max2,
                    max1 * min2,
                    max1 * max2,
                ];
                let min_val = corners.iter().cloned().fold(f64::INFINITY, f64::min);
                let max_val = corners.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                ValueRange::Range { min: min_val, max: max_val }
            }
            _ => ValueRange::Any,
        }
    }

    /// Divide two ranges
    pub fn divide(&self, other: &ValueRange) -> ValueRange {
        // Check for division by zero
        if other.contains_zero() {
            return ValueRange::Any;
        }

        match (self, other) {
            (ValueRange::Exact(a), ValueRange::Exact(b)) if *b != 0.0 => ValueRange::Exact(a / b),
            (ValueRange::Exact(a), ValueRange::Range { min, max }) if *min != 0.0 && *max != 0.0 => {
                let corners = [a / min, a / max];
                let min_val = corners.iter().cloned().fold(f64::INFINITY, f64::min);
                let max_val = corners.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                ValueRange::Range { min: min_val, max: max_val }
            }
            (ValueRange::Range { min, max }, ValueRange::Exact(a)) if *a != 0.0 => {
                let corners = [min / a, max / a];
                let min_val = corners.iter().cloned().fold(f64::INFINITY, f64::min);
                let max_val = corners.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                ValueRange::Range { min: min_val, max: max_val }
            }
            (ValueRange::Range { min: min1, max: max1 }, ValueRange::Range { min: min2, max: max2 })
                if *min2 != 0.0 && *max2 != 0.0 => {
                let corners = [
                    min1 / min2,
                    min1 / max2,
                    max1 / min2,
                    max1 / max2,
                ];
                let min_val = corners.iter().cloned().fold(f64::INFINITY, f64::min);
                let max_val = corners.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                ValueRange::Range { min: min_val, max: max_val }
            }
            _ => ValueRange::Any,
        }
    }

    /// Negate a range
    pub fn negate(&self) -> ValueRange {
        match self {
            ValueRange::Exact(v) => ValueRange::Exact(-v),
            ValueRange::Range { min, max } => ValueRange::Range { min: -max, max: -min },
            ValueRange::Any => ValueRange::Any,
            ValueRange::NonNumeric => ValueRange::NonNumeric,
        }
    }

    /// Union of two ranges (widening)
    pub fn union(&self, other: &ValueRange) -> ValueRange {
        match (self, other) {
            (ValueRange::Exact(a), ValueRange::Exact(b)) if a == b => ValueRange::Exact(*a),
            (ValueRange::Exact(a), ValueRange::Exact(b)) => {
                ValueRange::Range {
                    min: a.min(*b),
                    max: a.max(*b),
                }
            }
            (ValueRange::Exact(a), ValueRange::Range { min, max }) |
            (ValueRange::Range { min, max }, ValueRange::Exact(a)) => {
                ValueRange::Range {
                    min: min.min(*a),
                    max: max.max(*a),
                }
            }
            (ValueRange::Range { min: min1, max: max1 }, ValueRange::Range { min: min2, max: max2 }) => {
                ValueRange::Range {
                    min: min1.min(*min2),
                    max: max1.max(*max2),
                }
            }
            (ValueRange::NonNumeric, ValueRange::NonNumeric) => ValueRange::NonNumeric,
            _ => ValueRange::Any,
        }
    }
}

/// Range analyzer
pub struct RangeAnalyzer {
    ranges: HashMap<String, ValueRange>,
}

impl RangeAnalyzer {
    /// Create a new range analyzer
    pub fn new() -> Self {
        Self {
            ranges: HashMap::new(),
        }
    }

    /// Analyze an expression and compute value ranges
    pub fn analyze(&mut self, expr: &Expr, context: &HashMap<String, ValueRange>) -> HashMap<String, ValueRange> {
        self.ranges = context.clone();
        self.analyze_expr(expr);
        self.ranges.clone()
    }

    fn analyze_expr(&mut self, expr: &Expr) -> ValueRange {
        match expr {
            Expr::Number(n) => ValueRange::Exact(*n),

            Expr::String(_) | Expr::Boolean(_) | Expr::Symbol(_) => ValueRange::NonNumeric,

            Expr::Nil => ValueRange::NonNumeric,

            Expr::Variable(parts) => {
                let var_name = parts.join(".");
                self.ranges.get(&var_name).cloned().unwrap_or(ValueRange::Any)
            }

            Expr::Array(elements) => {
                // Analyze all elements
                for elem in elements {
                    self.analyze_expr(elem);
                }
                ValueRange::NonNumeric
            }

            Expr::Dictionary(pairs) => {
                // Analyze all values
                for (_, value) in pairs {
                    self.analyze_expr(value);
                }
                ValueRange::NonNumeric
            }

            Expr::FunctionCall { args, .. } => {
                // Analyze arguments
                for arg in args {
                    self.analyze_expr(arg);
                }
                // Most functions return any value
                ValueRange::Any
            }

            Expr::Let { name, value, body } => {
                let value_range = self.analyze_expr(value);
                self.ranges.insert(name.clone(), value_range);
                self.analyze_expr(body)
            }

            Expr::If { condition, then_branch, else_branch } => {
                self.analyze_expr(condition);
                let then_range = self.analyze_expr(then_branch);
                let else_range = self.analyze_expr(else_branch);
                then_range.union(&else_range)
            }

            Expr::Binary { op, left, right } => {
                let left_range = self.analyze_expr(left);
                let right_range = self.analyze_expr(right);

                match op {
                    BinaryOp::Add => left_range.add(&right_range),
                    BinaryOp::Subtract => left_range.subtract(&right_range),
                    BinaryOp::Multiply => left_range.multiply(&right_range),
                    BinaryOp::Divide | BinaryOp::Modulo => left_range.divide(&right_range),
                    BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::Less |
                    BinaryOp::Greater | BinaryOp::LessEqual | BinaryOp::GreaterEqual |
                    BinaryOp::And | BinaryOp::Or => ValueRange::NonNumeric,
                }
            }

            Expr::Unary { op, operand } => {
                let operand_range = self.analyze_expr(operand);
                match op {
                    UnaryOp::Negate => operand_range.negate(),
                    UnaryOp::Not => ValueRange::NonNumeric,
                }
            }

            Expr::Pipe { left, right } => {
                self.analyze_expr(left);
                self.analyze_expr(right)
            }
        }
    }
}

impl Default for RangeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amoskeag_parser::parse;

    #[test]
    fn test_exact_value() {
        let mut analyzer = RangeAnalyzer::new();
        let expr = parse("42").unwrap();
        let range = analyzer.analyze_expr(&expr);
        assert_eq!(range, ValueRange::Exact(42.0));
    }

    #[test]
    fn test_addition_range() {
        let mut analyzer = RangeAnalyzer::new();
        let expr = parse("10 + 5").unwrap();
        let range = analyzer.analyze_expr(&expr);
        assert_eq!(range, ValueRange::Exact(15.0));
    }

    #[test]
    fn test_contains_zero() {
        let range1 = ValueRange::Exact(0.0);
        assert!(range1.contains_zero());

        let range2 = ValueRange::Range { min: -5.0, max: 5.0 };
        assert!(range2.contains_zero());

        let range3 = ValueRange::Range { min: 1.0, max: 10.0 };
        assert!(!range3.contains_zero());
    }

    #[test]
    fn test_range_arithmetic() {
        let r1 = ValueRange::Range { min: 1.0, max: 5.0 };
        let r2 = ValueRange::Range { min: 2.0, max: 3.0 };

        let sum = r1.add(&r2);
        if let ValueRange::Range { min, max } = sum {
            assert_eq!(min, 3.0);
            assert_eq!(max, 8.0);
        } else {
            panic!("Expected Range");
        }
    }
}
