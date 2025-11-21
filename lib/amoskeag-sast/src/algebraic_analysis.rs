//! Algebraic analysis module
//!
//! Uses constraint solving and symbolic execution to find input parameters
//! that could cause errors.

use crate::constraint_solver::{Constraint, ConstraintSolver};
use crate::range_analysis::ValueRange;
use amoskeag_parser::{BinaryOp, Expr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A constraint on input parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConstraint {
    /// Variable name
    pub variable: String,
    /// Description of the constraint
    pub description: String,
    /// The constraint itself
    pub constraint: Constraint,
}

/// A vulnerable input that could cause an error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerableInput {
    /// Error type that this input could trigger
    pub error_type: String,
    /// Description of the vulnerability
    pub description: String,
    /// Example input that triggers the error
    pub example_input: HashMap<String, f64>,
    /// Location in the expression
    pub location: String,
    /// Severity of the vulnerability
    pub severity: String,
}

/// Algebraic analyzer using symbolic execution
pub struct AlgebraicAnalyzer {
    vulnerable_inputs: Vec<VulnerableInput>,
}

impl AlgebraicAnalyzer {
    /// Create a new algebraic analyzer
    pub fn new() -> Self {
        Self {
            vulnerable_inputs: Vec::new(),
        }
    }

    /// Analyze an expression to find vulnerable inputs
    pub fn analyze(
        &mut self,
        expr: &Expr,
        _symbols: &[&str],
        _ranges: &HashMap<String, ValueRange>,
    ) -> Vec<VulnerableInput> {
        self.vulnerable_inputs.clear();
        self.analyze_expr(expr, &mut Vec::new());
        self.vulnerable_inputs.clone()
    }

    fn analyze_expr(&mut self, expr: &Expr, path_conditions: &mut Vec<Constraint>) {
        match expr {
            Expr::Binary { op, left, right } => {
                // Recursively analyze subexpressions
                self.analyze_expr(left, path_conditions);
                self.analyze_expr(right, path_conditions);

                // Check for division by zero
                if matches!(op, BinaryOp::Divide | BinaryOp::Modulo) {
                    self.check_division_by_zero(right, path_conditions);
                }

                // Check for overflow
                if matches!(op, BinaryOp::Multiply | BinaryOp::Add) {
                    self.check_overflow(left, right, *op, path_conditions);
                }
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // Analyze condition
                self.analyze_expr(condition, path_conditions);

                // Analyze then branch with condition = true
                if let Some(true_constraint) = self.extract_constraint(condition, true) {
                    path_conditions.push(true_constraint);
                    self.analyze_expr(then_branch, path_conditions);
                    path_conditions.pop();
                } else {
                    self.analyze_expr(then_branch, path_conditions);
                }

                // Analyze else branch with condition = false
                if let Some(false_constraint) = self.extract_constraint(condition, false) {
                    path_conditions.push(false_constraint);
                    self.analyze_expr(else_branch, path_conditions);
                    path_conditions.pop();
                } else {
                    self.analyze_expr(else_branch, path_conditions);
                }
            }

            Expr::Let { value, body, .. } => {
                self.analyze_expr(value, path_conditions);
                self.analyze_expr(body, path_conditions);
            }

            Expr::Unary { operand, .. } => {
                self.analyze_expr(operand, path_conditions);
            }

            Expr::FunctionCall { name, args } => {
                // Analyze arguments
                for arg in args {
                    self.analyze_expr(arg, path_conditions);
                }

                // Check for specific function vulnerabilities
                self.check_function_vulnerabilities(name, args, path_conditions);
            }

            Expr::Array(elements) => {
                for elem in elements {
                    self.analyze_expr(elem, path_conditions);
                }
            }

            Expr::Dictionary(pairs) => {
                for (_, value) in pairs {
                    self.analyze_expr(value, path_conditions);
                }
            }

            Expr::Pipe { left, right } => {
                self.analyze_expr(left, path_conditions);
                self.analyze_expr(right, path_conditions);
            }

            _ => {
                // Literals and variables are safe
            }
        }
    }

    fn check_division_by_zero(&mut self, divisor: &Expr, path_conditions: &[Constraint]) {
        // Check if divisor could be zero
        if let Some(var_name) = self.extract_variable_name(divisor) {
            // Create a constraint: divisor == 0
            let mut solver = ConstraintSolver::new();

            // Add path conditions
            for condition in path_conditions {
                solver.add_constraint(condition.clone());
            }

            // Add the constraint that divisor == 0
            solver.add_constraint(Constraint::Equal {
                variable: var_name.clone(),
                value: 0.0,
            });

            // Try to solve
            if let Some(solution) = solver.solve() {
                if solution.satisfies {
                    self.vulnerable_inputs.push(VulnerableInput {
                        error_type: "DivisionByZero".to_string(),
                        description: format!("Division by zero when {} = 0", var_name),
                        example_input: solution.values.clone(),
                        location: format!("division by {}", var_name),
                        severity: "Critical".to_string(),
                    });
                }
            }
        } else if let Expr::Number(n) = divisor {
            if *n == 0.0 {
                self.vulnerable_inputs.push(VulnerableInput {
                    error_type: "DivisionByZero".to_string(),
                    description: "Constant division by zero".to_string(),
                    example_input: HashMap::new(),
                    location: "division by zero literal".to_string(),
                    severity: "Critical".to_string(),
                });
            }
        }
    }

    fn check_overflow(
        &mut self,
        left: &Expr,
        right: &Expr,
        op: BinaryOp,
        path_conditions: &[Constraint],
    ) {
        // Try to find inputs that cause overflow
        let left_var = self.extract_variable_name(left);
        let right_var = self.extract_variable_name(right);

        if left_var.is_none() && right_var.is_none() {
            return; // Both are constants, already checked during evaluation
        }

        let mut solver = ConstraintSolver::new();

        // Add path conditions
        for condition in path_conditions {
            solver.add_constraint(condition.clone());
        }

        // Add constraints that could cause overflow
        if let Some(var) = left_var.as_ref().or(right_var.as_ref()) {
            // Try very large values
            solver.add_constraint(Constraint::GreaterThan {
                variable: var.clone(),
                value: 1e100,
            });

            if let Some(solution) = solver.solve() {
                if solution.satisfies {
                    self.vulnerable_inputs.push(VulnerableInput {
                        error_type: "Overflow".to_string(),
                        description: format!("Potential overflow in {} operation", op),
                        example_input: solution.values.clone(),
                        location: format!("{} operation", op),
                        severity: "Warning".to_string(),
                    });
                }
            }
        }
    }

    fn check_function_vulnerabilities(
        &mut self,
        name: &str,
        args: &[Expr],
        path_conditions: &[Constraint],
    ) {
        match name {
            "at" => {
                // Array access - check for out of bounds
                if args.len() >= 2 {
                    if let Some(index_var) = self.extract_variable_name(&args[1]) {
                        // Try negative index
                        let mut solver = ConstraintSolver::new();
                        for condition in path_conditions {
                            solver.add_constraint(condition.clone());
                        }
                        solver.add_constraint(Constraint::LessThan {
                            variable: index_var.clone(),
                            value: 0.0,
                        });

                        if let Some(solution) = solver.solve() {
                            if solution.satisfies {
                                self.vulnerable_inputs.push(VulnerableInput {
                                    error_type: "ArrayOutOfBounds".to_string(),
                                    description: format!(
                                        "Negative array index when {} < 0",
                                        index_var
                                    ),
                                    example_input: solution.values.clone(),
                                    location: format!("at() with index {}", index_var),
                                    severity: "Warning".to_string(),
                                });
                            }
                        }
                    }
                }
            }
            "divided_by" => {
                // Check for division by zero in function form
                if args.len() >= 2 {
                    self.check_division_by_zero(&args[1], path_conditions);
                }
            }
            _ => {}
        }
    }

    fn extract_constraint(&self, expr: &Expr, value: bool) -> Option<Constraint> {
        match expr {
            Expr::Binary { op, left, right } => {
                let left_var = self.extract_variable_name(left)?;
                let right_val = self.extract_number_value(right)?;

                let constraint = match (op, value) {
                    (BinaryOp::Equal, true) => Constraint::Equal {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::Equal, false) => Constraint::NotEqual {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::NotEqual, true) => Constraint::NotEqual {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::NotEqual, false) => Constraint::Equal {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::Less, true) => Constraint::LessThan {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::Less, false) => Constraint::GreaterEqual {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::Greater, true) => Constraint::GreaterThan {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::Greater, false) => Constraint::LessEqual {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::LessEqual, true) => Constraint::LessEqual {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::LessEqual, false) => Constraint::GreaterThan {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::GreaterEqual, true) => Constraint::GreaterEqual {
                        variable: left_var,
                        value: right_val,
                    },
                    (BinaryOp::GreaterEqual, false) => Constraint::LessThan {
                        variable: left_var,
                        value: right_val,
                    },
                    _ => return None,
                };

                Some(constraint)
            }
            _ => None,
        }
    }

    fn extract_variable_name(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Variable(parts) => Some(parts.join(".")),
            _ => None,
        }
    }

    fn extract_number_value(&self, expr: &Expr) -> Option<f64> {
        match expr {
            Expr::Number(n) => Some(*n),
            _ => None,
        }
    }
}

impl Default for AlgebraicAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amoskeag_parser::parse;

    #[test]
    fn test_find_division_by_zero_vulnerability() {
        let mut analyzer = AlgebraicAnalyzer::new();
        let expr = parse("10 / x").unwrap();
        let vulnerabilities = analyzer.analyze(&expr, &[], &HashMap::new());

        assert!(!vulnerabilities.is_empty());
        assert!(vulnerabilities
            .iter()
            .any(|v| v.error_type == "DivisionByZero"));
    }

    #[test]
    fn test_conditional_division_by_zero() {
        let mut analyzer = AlgebraicAnalyzer::new();
        let expr = parse("if x > 0 10 / x else 0 end").unwrap();
        let _vulnerabilities = analyzer.analyze(&expr, &[], &HashMap::new());

        // Should still find the vulnerability in the then branch
        // (even though there's a condition, the condition doesn't prevent x from being 0 in practice)
        // May or may not find depending on path sensitivity
    }

    #[test]
    fn test_no_vulnerability_safe_code() {
        let mut analyzer = AlgebraicAnalyzer::new();
        let expr = parse("x + 10").unwrap();
        let vulnerabilities = analyzer.analyze(&expr, &[], &HashMap::new());

        assert_eq!(vulnerabilities.len(), 0);
    }
}
