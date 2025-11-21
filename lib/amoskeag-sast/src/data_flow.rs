//! Data flow analysis module
//!
//! Analyzes how data flows through an expression.

use amoskeag_parser::Expr;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A node in the data flow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowNode {
    /// Node identifier
    pub id: usize,
    /// Type of node
    pub node_type: DataFlowNodeType,
    /// Variables defined at this node
    pub defines: HashSet<String>,
    /// Variables used at this node
    pub uses: HashSet<String>,
    /// Variables that are live after this node
    pub live_out: HashSet<String>,
}

/// Type of data flow node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataFlowNodeType {
    /// Entry point
    Entry,
    /// Exit point
    Exit,
    /// Assignment/definition
    Definition,
    /// Use of a variable
    Use,
    /// Conditional branch
    Branch,
    /// Function call
    FunctionCall,
    /// Expression evaluation
    Expression,
}

/// Data flow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowGraph {
    /// All nodes in the graph
    pub nodes: Vec<DataFlowNode>,
    /// Edges between nodes (from -> to)
    pub edges: Vec<(usize, usize)>,
    /// Variable definitions (variable name -> defining nodes)
    pub definitions: HashMap<String, Vec<usize>>,
    /// Variable uses (variable name -> using nodes)
    pub uses: HashMap<String, Vec<usize>>,
}

impl DataFlowGraph {
    /// Create a new empty data flow graph
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            definitions: HashMap::new(),
            uses: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node_type: DataFlowNodeType) -> usize {
        let id = self.nodes.len();
        self.nodes.push(DataFlowNode {
            id,
            node_type,
            defines: HashSet::new(),
            uses: HashSet::new(),
            live_out: HashSet::new(),
        });
        id
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }

    /// Record a variable definition at a node
    pub fn add_definition(&mut self, node_id: usize, variable: String) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.defines.insert(variable.clone());
        }
        self.definitions.entry(variable).or_default().push(node_id);
    }

    /// Record a variable use at a node
    pub fn add_use(&mut self, node_id: usize, variable: String) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.uses.insert(variable.clone());
        }
        self.uses.entry(variable).or_default().push(node_id);
    }

    /// Compute liveness analysis
    pub fn compute_liveness(&mut self) {
        // Backward dataflow analysis
        let mut changed = true;
        while changed {
            changed = false;

            for i in (0..self.nodes.len()).rev() {
                let mut new_live_out = HashSet::new();

                // live_out[n] = union of live_in[s] for all successors s
                for &(from, to) in &self.edges {
                    if from == i {
                        // live_in[s] = use[s] union (live_out[s] - def[s])
                        let successor = &self.nodes[to];
                        let mut live_in = successor.uses.clone();
                        for var in &successor.live_out {
                            if !successor.defines.contains(var) {
                                live_in.insert(var.clone());
                            }
                        }
                        new_live_out.extend(live_in);
                    }
                }

                if new_live_out != self.nodes[i].live_out {
                    self.nodes[i].live_out = new_live_out;
                    changed = true;
                }
            }
        }
    }

    /// Find reaching definitions for a variable at a node
    pub fn reaching_definitions(&self, node_id: usize, variable: &str) -> Vec<usize> {
        // Simple forward analysis to find definitions that reach this node
        let mut reaching = Vec::new();

        if let Some(def_nodes) = self.definitions.get(variable) {
            for &def_node in def_nodes {
                if self.can_reach(def_node, node_id) {
                    reaching.push(def_node);
                }
            }
        }

        reaching
    }

    /// Check if there's a path from one node to another
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }

        let mut visited = HashSet::new();
        let mut stack = vec![from];

        while let Some(current) = stack.pop() {
            if current == to {
                return true;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            for &(edge_from, edge_to) in &self.edges {
                if edge_from == current {
                    stack.push(edge_to);
                }
            }
        }

        false
    }
}

impl Default for DataFlowGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Data flow analyzer
pub struct DataFlowAnalyzer {
    graph: DataFlowGraph,
    node_counter: usize,
}

impl DataFlowAnalyzer {
    /// Create a new data flow analyzer
    pub fn new() -> Self {
        Self {
            graph: DataFlowGraph::new(),
            node_counter: 0,
        }
    }

    /// Analyze an expression and build a data flow graph
    pub fn analyze(&mut self, expr: &Expr) -> DataFlowGraph {
        self.graph = DataFlowGraph::new();
        self.node_counter = 0;

        let entry = self.graph.add_node(DataFlowNodeType::Entry);
        let exit = self.graph.add_node(DataFlowNodeType::Exit);

        let expr_nodes = self.analyze_expr(expr);

        // Connect entry to expression
        if let Some(&first) = expr_nodes.first() {
            self.graph.add_edge(entry, first);
        }

        // Connect expression to exit
        if let Some(&last) = expr_nodes.last() {
            self.graph.add_edge(last, exit);
        }

        // Compute liveness
        self.graph.compute_liveness();

        self.graph.clone()
    }

    fn analyze_expr(&mut self, expr: &Expr) -> Vec<usize> {
        match expr {
            Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | Expr::Nil | Expr::Symbol(_) => {
                // Literals create no data flow
                vec![]
            }

            Expr::Variable(parts) => {
                let node_id = self.graph.add_node(DataFlowNodeType::Use);
                let var_name = parts.join(".");
                self.graph.add_use(node_id, var_name);
                vec![node_id]
            }

            Expr::Let { name, value, body } => {
                let value_nodes = self.analyze_expr(value);
                let def_node = self.graph.add_node(DataFlowNodeType::Definition);
                self.graph.add_definition(def_node, name.clone());

                // Connect value to definition
                if let Some(&last_value) = value_nodes.last() {
                    self.graph.add_edge(last_value, def_node);
                }

                let body_nodes = self.analyze_expr(body);

                // Connect definition to body
                if let Some(&first_body) = body_nodes.first() {
                    self.graph.add_edge(def_node, first_body);
                }

                // Return nodes from value to end of body
                let mut all_nodes = value_nodes;
                all_nodes.push(def_node);
                all_nodes.extend(body_nodes);
                all_nodes
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_nodes = self.analyze_expr(condition);
                let branch_node = self.graph.add_node(DataFlowNodeType::Branch);

                // Connect condition to branch
                if let Some(&last_cond) = cond_nodes.last() {
                    self.graph.add_edge(last_cond, branch_node);
                }

                let then_nodes = self.analyze_expr(then_branch);
                let else_nodes = self.analyze_expr(else_branch);

                // Connect branch to both branches
                if let Some(&first_then) = then_nodes.first() {
                    self.graph.add_edge(branch_node, first_then);
                }
                if let Some(&first_else) = else_nodes.first() {
                    self.graph.add_edge(branch_node, first_else);
                }

                // Create a join node
                let join_node = self.graph.add_node(DataFlowNodeType::Expression);

                // Connect both branches to join
                if let Some(&last_then) = then_nodes.last() {
                    self.graph.add_edge(last_then, join_node);
                }
                if let Some(&last_else) = else_nodes.last() {
                    self.graph.add_edge(last_else, join_node);
                }

                vec![branch_node, join_node]
            }

            Expr::Binary { left, right, .. } => {
                let left_nodes = self.analyze_expr(left);
                let right_nodes = self.analyze_expr(right);
                let expr_node = self.graph.add_node(DataFlowNodeType::Expression);

                // Connect left to right
                if let (Some(&last_left), Some(&first_right)) =
                    (left_nodes.last(), right_nodes.first())
                {
                    self.graph.add_edge(last_left, first_right);
                }

                // Connect right to expression
                if let Some(&last_right) = right_nodes.last() {
                    self.graph.add_edge(last_right, expr_node);
                }

                vec![expr_node]
            }

            Expr::Unary { operand, .. } => {
                let operand_nodes = self.analyze_expr(operand);
                let expr_node = self.graph.add_node(DataFlowNodeType::Expression);

                // Connect operand to expression
                if let Some(&last_operand) = operand_nodes.last() {
                    self.graph.add_edge(last_operand, expr_node);
                }

                vec![expr_node]
            }

            Expr::FunctionCall { name: _, args } => {
                let call_node = self.graph.add_node(DataFlowNodeType::FunctionCall);

                // Analyze all arguments
                for arg in args {
                    let arg_nodes = self.analyze_expr(arg);
                    if let Some(&last_arg) = arg_nodes.last() {
                        self.graph.add_edge(last_arg, call_node);
                    }
                }

                vec![call_node]
            }

            Expr::Array(elements) => {
                let expr_node = self.graph.add_node(DataFlowNodeType::Expression);

                for elem in elements {
                    let elem_nodes = self.analyze_expr(elem);
                    if let Some(&last_elem) = elem_nodes.last() {
                        self.graph.add_edge(last_elem, expr_node);
                    }
                }

                vec![expr_node]
            }

            Expr::Dictionary(pairs) => {
                let expr_node = self.graph.add_node(DataFlowNodeType::Expression);

                for (_, value) in pairs {
                    let value_nodes = self.analyze_expr(value);
                    if let Some(&last_value) = value_nodes.last() {
                        self.graph.add_edge(last_value, expr_node);
                    }
                }

                vec![expr_node]
            }

            Expr::Pipe { left, right } => {
                let left_nodes = self.analyze_expr(left);
                let right_nodes = self.analyze_expr(right);

                // Connect left to right
                if let (Some(&last_left), Some(&first_right)) =
                    (left_nodes.last(), right_nodes.first())
                {
                    self.graph.add_edge(last_left, first_right);
                }

                right_nodes
            }
        }
    }
}

impl Default for DataFlowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amoskeag_parser::parse;

    #[test]
    fn test_simple_data_flow() {
        let mut analyzer = DataFlowAnalyzer::new();
        let expr = parse("x + 1").unwrap();
        let graph = analyzer.analyze(&expr);

        assert!(!graph.nodes.is_empty());
    }

    #[test]
    fn test_let_binding_flow() {
        let mut analyzer = DataFlowAnalyzer::new();
        let expr = parse("let x = 5 in x + 1").unwrap();
        let graph = analyzer.analyze(&expr);

        // Should have definition and use nodes
        assert!(graph.definitions.contains_key("x"));
        assert!(graph.uses.contains_key("x"));
    }

    #[test]
    fn test_if_expression_flow() {
        let mut analyzer = DataFlowAnalyzer::new();
        let expr = parse("if x > 0 10 else 20 end").unwrap();
        let graph = analyzer.analyze(&expr);

        // Should have a branch node
        assert!(graph
            .nodes
            .iter()
            .any(|n| matches!(n.node_type, DataFlowNodeType::Branch)));
    }
}
