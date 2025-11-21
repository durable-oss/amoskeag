//! Amoskeag SAST (Static Application Security Testing)
//!
//! This crate provides static analysis tools for detecting programming errors
//! and using algebraic analysis to find input parameters that could cause errors.
//!
//! ## Features
//!
//! - **Error Detection**: Identifies potential programming errors such as division by zero,
//!   type mismatches, unreachable code, and more.
//! - **Algebraic Analysis**: Uses constraint solving and symbolic execution to find
//!   input parameters that could trigger errors.
//! - **Range Analysis**: Tracks possible value ranges for expressions.
//! - **Data Flow Analysis**: Analyzes how data flows through the program.

mod algebraic_analysis;
mod constraint_solver;
mod data_flow;
mod error_detection;
mod range_analysis;

pub use algebraic_analysis::{AlgebraicAnalyzer, InputConstraint, VulnerableInput};
pub use constraint_solver::{Constraint, ConstraintSolver, Solution};
pub use data_flow::{DataFlowAnalyzer, DataFlowNode};
pub use error_detection::{ErrorDetector, ErrorSeverity, ProgrammingError};
pub use range_analysis::{RangeAnalyzer, ValueRange};

use amoskeag_parser::Expr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main SAST analyzer that combines all analysis techniques
pub struct SastAnalyzer {
    error_detector: ErrorDetector,
    algebraic_analyzer: AlgebraicAnalyzer,
    range_analyzer: RangeAnalyzer,
    data_flow_analyzer: DataFlowAnalyzer,
}

/// Complete SAST analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Detected programming errors
    pub errors: Vec<ProgrammingError>,
    /// Vulnerable inputs that could cause errors
    pub vulnerable_inputs: Vec<VulnerableInput>,
    /// Statistics about the analysis
    pub statistics: AnalysisStatistics,
}

/// Analysis statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStatistics {
    /// Total number of expressions analyzed
    pub total_expressions: usize,
    /// Number of critical errors found
    pub critical_errors: usize,
    /// Number of warnings found
    pub warnings: usize,
    /// Number of info-level issues found
    pub info_issues: usize,
    /// Number of vulnerable input patterns found
    pub vulnerable_patterns: usize,
}

impl SastAnalyzer {
    /// Create a new SAST analyzer
    pub fn new() -> Self {
        Self {
            error_detector: ErrorDetector::new(),
            algebraic_analyzer: AlgebraicAnalyzer::new(),
            range_analyzer: RangeAnalyzer::new(),
            data_flow_analyzer: DataFlowAnalyzer::new(),
        }
    }

    /// Analyze an expression for potential errors and vulnerabilities
    pub fn analyze(&mut self, expr: &Expr, symbols: &[&str]) -> AnalysisResult {
        // Perform data flow analysis
        let _data_flow = self.data_flow_analyzer.analyze(expr);

        // Perform range analysis
        let ranges = self.range_analyzer.analyze(expr, &HashMap::new());

        // Detect programming errors
        let errors = self.error_detector.detect(expr, symbols, &ranges);

        // Perform algebraic analysis to find vulnerable inputs
        let vulnerable_inputs = self.algebraic_analyzer.analyze(expr, symbols, &ranges);

        // Calculate statistics
        let statistics = self.calculate_statistics(&errors, &vulnerable_inputs);

        AnalysisResult {
            errors,
            vulnerable_inputs,
            statistics,
        }
    }

    /// Analyze and return a JSON report
    pub fn analyze_json(
        &mut self,
        expr: &Expr,
        symbols: &[&str],
    ) -> Result<String, serde_json::Error> {
        let result = self.analyze(expr, symbols);
        serde_json::to_string_pretty(&result)
    }

    fn calculate_statistics(
        &self,
        errors: &[ProgrammingError],
        vulnerable_inputs: &[VulnerableInput],
    ) -> AnalysisStatistics {
        let critical_errors = errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Critical))
            .count();
        let warnings = errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Warning))
            .count();
        let info_issues = errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Info))
            .count();

        AnalysisStatistics {
            total_expressions: 0, // Will be calculated during traversal
            critical_errors,
            warnings,
            info_issues,
            vulnerable_patterns: vulnerable_inputs.len(),
        }
    }
}

impl Default for SastAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to analyze an expression
pub fn analyze(expr: &Expr, symbols: &[&str]) -> AnalysisResult {
    let mut analyzer = SastAnalyzer::new();
    analyzer.analyze(expr, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;
    use amoskeag_parser::parse;

    #[test]
    fn test_analyze_simple_expression() {
        let expr = parse("1 + 2").unwrap();
        let result = analyze(&expr, &[]);
        assert_eq!(result.errors.len(), 0);
    }

    #[test]
    fn test_analyze_division_by_zero() {
        let expr = parse("10 / 0").unwrap();
        let result = analyze(&expr, &[]);
        assert!(!result.errors.is_empty());
        // Check for division by zero (case insensitive)
        let has_div_zero = result
            .errors
            .iter()
            .any(|e| e.message.to_lowercase().contains("division"));
        if !has_div_zero {
            eprintln!("Errors found:");
            for e in &result.errors {
                eprintln!("  - {}", e.message);
            }
        }
        assert!(has_div_zero, "Expected division by zero error");
    }

    #[test]
    fn test_analyze_potential_division_by_zero() {
        let expr = parse("10 / x").unwrap();
        let result = analyze(&expr, &[]);
        // Should find a vulnerable input where x = 0
        assert!(!result.vulnerable_inputs.is_empty() || !result.errors.is_empty());
    }

    #[test]
    fn test_json_output() {
        let expr = parse("1 + 2").unwrap();
        let mut analyzer = SastAnalyzer::new();
        let json = analyzer.analyze_json(&expr, &[]).unwrap();
        assert!(json.contains("errors"));
        assert!(json.contains("vulnerable_inputs"));
    }
}
