//! Constraint solver module
//!
//! Solves constraints to find input values that satisfy certain conditions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A constraint on a variable
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constraint {
    /// Variable equals a specific value
    Equal { variable: String, value: f64 },
    /// Variable is not equal to a value
    NotEqual { variable: String, value: f64 },
    /// Variable is less than a value
    LessThan { variable: String, value: f64 },
    /// Variable is greater than a value
    GreaterThan { variable: String, value: f64 },
    /// Variable is less than or equal to a value
    LessEqual { variable: String, value: f64 },
    /// Variable is greater than or equal to a value
    GreaterEqual { variable: String, value: f64 },
    /// Variable is within a range
    InRange { variable: String, min: f64, max: f64 },
    /// Variable is a string
    IsString { variable: String },
    /// Variable is a number
    IsNumber { variable: String },
    /// Variable is a boolean
    IsBoolean { variable: String },
    /// Variable is nil
    IsNil { variable: String },
    /// Conjunction (AND) of constraints
    And(Vec<Constraint>),
    /// Disjunction (OR) of constraints
    Or(Vec<Constraint>),
}

impl Constraint {
    /// Check if a solution satisfies this constraint
    pub fn is_satisfied(&self, solution: &Solution) -> bool {
        match self {
            Constraint::Equal { variable, value } => {
                solution.values.get(variable).map(|v| (v - value).abs() < f64::EPSILON).unwrap_or(false)
            }
            Constraint::NotEqual { variable, value } => {
                solution.values.get(variable).map(|v| (v - value).abs() >= f64::EPSILON).unwrap_or(true)
            }
            Constraint::LessThan { variable, value } => {
                solution.values.get(variable).map(|v| v < value).unwrap_or(false)
            }
            Constraint::GreaterThan { variable, value } => {
                solution.values.get(variable).map(|v| v > value).unwrap_or(false)
            }
            Constraint::LessEqual { variable, value } => {
                solution.values.get(variable).map(|v| v <= value).unwrap_or(false)
            }
            Constraint::GreaterEqual { variable, value } => {
                solution.values.get(variable).map(|v| v >= value).unwrap_or(false)
            }
            Constraint::InRange { variable, min, max } => {
                solution.values.get(variable).map(|v| v >= min && v <= max).unwrap_or(false)
            }
            Constraint::And(constraints) => {
                constraints.iter().all(|c| c.is_satisfied(solution))
            }
            Constraint::Or(constraints) => {
                constraints.iter().any(|c| c.is_satisfied(solution))
            }
            _ => true, // Type constraints are handled differently
        }
    }

    /// Negate a constraint
    pub fn negate(&self) -> Constraint {
        match self {
            Constraint::Equal { variable, value } => {
                Constraint::NotEqual { variable: variable.clone(), value: *value }
            }
            Constraint::NotEqual { variable, value } => {
                Constraint::Equal { variable: variable.clone(), value: *value }
            }
            Constraint::LessThan { variable, value } => {
                Constraint::GreaterEqual { variable: variable.clone(), value: *value }
            }
            Constraint::GreaterThan { variable, value } => {
                Constraint::LessEqual { variable: variable.clone(), value: *value }
            }
            Constraint::LessEqual { variable, value } => {
                Constraint::GreaterThan { variable: variable.clone(), value: *value }
            }
            Constraint::GreaterEqual { variable, value } => {
                Constraint::LessThan { variable: variable.clone(), value: *value }
            }
            Constraint::And(constraints) => {
                Constraint::Or(constraints.iter().map(|c| c.negate()).collect())
            }
            Constraint::Or(constraints) => {
                Constraint::And(constraints.iter().map(|c| c.negate()).collect())
            }
            other => other.clone(), // Some constraints can't be easily negated
        }
    }
}

/// A solution to a set of constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    /// Variable assignments
    pub values: HashMap<String, f64>,
    /// Whether this is a satisfying solution
    pub satisfies: bool,
}

impl Solution {
    /// Create a new empty solution
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            satisfies: true,
        }
    }

    /// Add a variable assignment to the solution
    pub fn assign(&mut self, variable: String, value: f64) {
        self.values.insert(variable, value);
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}

/// Constraint solver using simple heuristics
pub struct ConstraintSolver {
    constraints: Vec<Constraint>,
}

impl ConstraintSolver {
    /// Create a new constraint solver
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    /// Add a constraint to solve
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Solve the constraints and find a satisfying solution
    pub fn solve(&self) -> Option<Solution> {
        if self.constraints.is_empty() {
            return Some(Solution::new());
        }

        // Extract all variables
        let variables = self.extract_variables();

        // Try to find a solution using simple heuristics
        let mut solution = Solution::new();

        for variable in &variables {
            // Try to find a value that satisfies all constraints for this variable
            if let Some(value) = self.find_value_for_variable(variable, &solution) {
                solution.assign(variable.clone(), value);
            } else {
                // If we can't find a value, try 0
                solution.assign(variable.clone(), 0.0);
            }
        }

        // Check if the solution satisfies all constraints
        let satisfies = self.constraints.iter().all(|c| c.is_satisfied(&solution));
        solution.satisfies = satisfies;

        if satisfies {
            Some(solution)
        } else {
            // Try backtracking with different values
            self.solve_with_backtracking(&variables)
        }
    }

    fn extract_variables(&self) -> Vec<String> {
        let mut variables = std::collections::HashSet::new();
        for constraint in &self.constraints {
            self.extract_variables_from_constraint(constraint, &mut variables);
        }
        variables.into_iter().collect()
    }

    fn extract_variables_from_constraint(&self, constraint: &Constraint, variables: &mut std::collections::HashSet<String>) {
        match constraint {
            Constraint::Equal { variable, .. } |
            Constraint::NotEqual { variable, .. } |
            Constraint::LessThan { variable, .. } |
            Constraint::GreaterThan { variable, .. } |
            Constraint::LessEqual { variable, .. } |
            Constraint::GreaterEqual { variable, .. } |
            Constraint::InRange { variable, .. } |
            Constraint::IsString { variable } |
            Constraint::IsNumber { variable } |
            Constraint::IsBoolean { variable } |
            Constraint::IsNil { variable } => {
                variables.insert(variable.clone());
            }
            Constraint::And(constraints) | Constraint::Or(constraints) => {
                for c in constraints {
                    self.extract_variables_from_constraint(c, variables);
                }
            }
        }
    }

    fn find_value_for_variable(&self, variable: &str, _solution: &Solution) -> Option<f64> {
        // Look for direct constraints on this variable
        for constraint in &self.constraints {
            match constraint {
                Constraint::Equal { variable: v, value } if v == variable => {
                    return Some(*value);
                }
                Constraint::LessThan { variable: v, value } if v == variable => {
                    return Some(value - 1.0);
                }
                Constraint::GreaterThan { variable: v, value } if v == variable => {
                    return Some(value + 1.0);
                }
                Constraint::LessEqual { variable: v, value } if v == variable => {
                    return Some(*value);
                }
                Constraint::GreaterEqual { variable: v, value } if v == variable => {
                    return Some(*value);
                }
                Constraint::InRange { variable: v, min, max } if v == variable => {
                    return Some((min + max) / 2.0);
                }
                _ => {}
            }
        }
        None
    }

    fn solve_with_backtracking(&self, variables: &[String]) -> Option<Solution> {
        // Try a few common values for each variable
        let test_values = vec![-1000.0, -100.0, -10.0, -1.0, 0.0, 1.0, 10.0, 100.0, 1000.0];

        // Try different combinations (limited search space)
        for i in 0..test_values.len().min(3) {
            let mut solution = Solution::new();
            for variable in variables {
                solution.assign(variable.clone(), test_values[i]);
            }

            if self.constraints.iter().all(|c| c.is_satisfied(&solution)) {
                solution.satisfies = true;
                return Some(solution);
            }
        }

        None
    }

    /// Clear all constraints
    pub fn clear(&mut self) {
        self.constraints.clear();
    }
}

impl Default for ConstraintSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_simple_equality() {
        let mut solver = ConstraintSolver::new();
        solver.add_constraint(Constraint::Equal {
            variable: "x".to_string(),
            value: 42.0,
        });

        let solution = solver.solve().unwrap();
        assert!(solution.satisfies);
        assert_eq!(solution.values.get("x"), Some(&42.0));
    }

    #[test]
    fn test_solve_inequality() {
        let mut solver = ConstraintSolver::new();
        solver.add_constraint(Constraint::GreaterThan {
            variable: "x".to_string(),
            value: 0.0,
        });

        let solution = solver.solve().unwrap();
        assert!(solution.satisfies);
        assert!(solution.values.get("x").unwrap() > &0.0);
    }

    #[test]
    fn test_solve_range() {
        let mut solver = ConstraintSolver::new();
        solver.add_constraint(Constraint::InRange {
            variable: "x".to_string(),
            min: 10.0,
            max: 20.0,
        });

        let solution = solver.solve().unwrap();
        assert!(solution.satisfies);
        let x = solution.values.get("x").unwrap();
        assert!(x >= &10.0 && x <= &20.0);
    }

    #[test]
    fn test_constraint_negation() {
        let constraint = Constraint::Equal {
            variable: "x".to_string(),
            value: 5.0,
        };
        let negated = constraint.negate();

        assert!(matches!(negated, Constraint::NotEqual { .. }));
    }
}
