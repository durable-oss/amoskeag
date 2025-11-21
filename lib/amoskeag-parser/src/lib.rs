//! Amoskeag Parser
//!
//! The parser for the Amoskeag language.
//! This module is responsible for consuming tokens from the lexer and
//! producing an Abstract Syntax Tree (AST) using a recursive descent parser.

use amoskeag_lexer::{Token, TokenType};
use std::fmt;
use thiserror::Error;

/// AST node representing an expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Symbol(String),
    Array(Vec<Expr>),
    Dictionary(Vec<(String, Expr)>),

    // Variable access (e.g., driver.age)
    Variable(Vec<String>),

    // Function call
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },

    // Let binding
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },

    // If expression
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },

    // Binary operations
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // Unary operations
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },

    // Pipe expression (will be transformed during AST building)
    Pipe {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Logical
    And,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::LessEqual => write!(f, "<="),
            BinaryOp::GreaterEqual => write!(f, ">="),
            BinaryOp::And => write!(f, "and"),
            BinaryOp::Or => write!(f, "or"),
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Negate,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Not => write!(f, "not"),
            UnaryOp::Negate => write!(f, "-"),
        }
    }
}

/// Parser errors
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, found {found} at line {line}, column {column}")]
    UnexpectedToken {
        expected: String,
        found: String,
        line: usize,
        column: usize,
    },

    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("Invalid expression at line {line}, column {column}")]
    InvalidExpression { line: usize, column: usize },
}

/// Parser state
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Create a new parser from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse the token stream into an AST
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    // Recursive descent parser implementation

    fn expression(&mut self) -> Result<Expr, ParseError> {
        // Expression ::= LetExpression | IfExpression | LogicalExpression
        if self.check(&TokenType::Let) {
            self.let_expression()
        } else if self.check(&TokenType::If) {
            self.if_expression()
        } else {
            self.logical_expression()
        }
    }

    fn let_expression(&mut self) -> Result<Expr, ParseError> {
        // LetExpression ::= "let" IDENTIFIER "=" Expression "in" Expression
        self.consume_token(&TokenType::Let, "let")?;

        let name = self.consume_identifier()?;

        self.consume_token(&TokenType::Assign, "=")?;

        let value = Box::new(self.expression()?);

        self.consume_token(&TokenType::In, "in")?;

        let body = Box::new(self.expression()?);

        Ok(Expr::Let { name, value, body })
    }

    fn if_expression(&mut self) -> Result<Expr, ParseError> {
        self.consume_token(&TokenType::If, "if")?;

        let mut conditions = vec![];
        let mut thens = vec![];

        let condition = Box::new(self.expression()?);
        if self.check(&TokenType::Then) {
            self.advance();
        }
        let then_branch = Box::new(self.expression()?);

        conditions.push(condition);
        thens.push(then_branch);

        while self.match_token(&TokenType::Else) {
            if self.check(&TokenType::If) {
                self.advance();
                let cond = Box::new(self.expression()?);
                if self.check(&TokenType::Then) {
                    self.advance();
                }
                let then = Box::new(self.expression()?);
                conditions.push(cond);
                thens.push(then);
            } else {
                let else_branch = Box::new(self.expression()?);
                self.consume_token(&TokenType::End, "end")?;

                // Build the nested if from the inside out
                let mut expr = *else_branch;
                for i in (0..conditions.len()).rev() {
                    expr = Expr::If {
                        condition: conditions[i].clone(),
                        then_branch: thens[i].clone(),
                        else_branch: Box::new(expr),
                    };
                }
                return Ok(expr);
            }
        }

        // If we reach here, no else was found
        Err(ParseError::UnexpectedToken {
            expected: "else".to_string(),
            found: format!("{}", self.peek().token_type),
            line: self.peek().line,
            column: self.peek().column,
        })
    }

    fn logical_expression(&mut self) -> Result<Expr, ParseError> {
        // LogicalExpression ::= ComparisonExpression ( ("or" | "||" | "and" | "&&") ComparisonExpression )*
        self.binary_op(
            Self::comparison_expression,
            &[
                (TokenType::Or, BinaryOp::Or),
                (TokenType::LogicalOr, BinaryOp::Or),
                (TokenType::And, BinaryOp::And),
                (TokenType::LogicalAnd, BinaryOp::And),
            ],
        )
    }

    fn comparison_expression(&mut self) -> Result<Expr, ParseError> {
        // ComparisonExpression ::= AdditiveExpression ( ( "==" | "!=" | "<" | ">" | "<=" | ">=" ) AdditiveExpression )*
        self.binary_op(
            Self::additive_expression,
            &[
                (TokenType::Equal, BinaryOp::Equal),
                (TokenType::NotEqual, BinaryOp::NotEqual),
                (TokenType::Less, BinaryOp::Less),
                (TokenType::Greater, BinaryOp::Greater),
                (TokenType::LessEqual, BinaryOp::LessEqual),
                (TokenType::GreaterEqual, BinaryOp::GreaterEqual),
            ],
        )
    }

    fn additive_expression(&mut self) -> Result<Expr, ParseError> {
        // AdditiveExpression ::= MultiplicativeExpression ( ( "+" | "-" ) MultiplicativeExpression )*
        self.binary_op(
            Self::multiplicative_expression,
            &[
                (TokenType::Plus, BinaryOp::Add),
                (TokenType::Minus, BinaryOp::Subtract),
            ],
        )
    }

    fn multiplicative_expression(&mut self) -> Result<Expr, ParseError> {
        // MultiplicativeExpression ::= PipeExpression ( ( "*" | "/" | "%" ) PipeExpression )*
        self.binary_op(
            Self::pipe_expression,
            &[
                (TokenType::Star, BinaryOp::Multiply),
                (TokenType::Slash, BinaryOp::Divide),
                (TokenType::Percent, BinaryOp::Modulo),
            ],
        )
    }

    fn pipe_expression(&mut self) -> Result<Expr, ParseError> {
        // PipeExpression ::= PrimaryExpression ( "|" FunctionCall )*
        let mut expr = self.primary_expression()?;

        while self.match_token(&TokenType::Pipe) {
            // After pipe, we expect either:
            // 1. An identifier (becomes a function call with expr as first arg)
            // 2. A function call (expr becomes first argument)

            let right = self.primary_expression()?;

            // Transform pipe into function call
            expr = match right {
                Expr::Variable(ref parts) if parts.len() == 1 => {
                    // Simple identifier: x | func => func(x)
                    Expr::FunctionCall {
                        name: parts[0].clone(),
                        args: vec![expr],
                    }
                }
                Expr::FunctionCall { name, mut args } => {
                    // Function call: x | func(a, b) => func(x, a, b)
                    args.insert(0, expr);
                    Expr::FunctionCall { name, args }
                }
                _ => {
                    return Err(ParseError::InvalidExpression {
                        line: self.current_token().line,
                        column: self.current_token().column,
                    })
                }
            };
        }

        Ok(expr)
    }

    fn primary_expression(&mut self) -> Result<Expr, ParseError> {
        // PrimaryExpression ::= Literal | SymbolLiteral | FunctionCall | VariableAccess | "(" Expression ")"

        let token = self.peek();

        match &token.token_type {
            // Literals
            TokenType::Number(n) => {
                let n = *n;
                self.advance();
                Ok(Expr::Number(n))
            }
            TokenType::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::String(s))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Boolean(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Boolean(false))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Nil)
            }
            TokenType::Symbol(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::Symbol(s))
            }

            // Array literal
            TokenType::LeftBracket => self.array_literal(),

            // Dictionary literal
            TokenType::LeftBrace => self.dictionary_literal(),

            // Grouped expression
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume_token(&TokenType::RightParen, ")")?;
                Ok(expr)
            }

            // Unary operators
            TokenType::Not | TokenType::Bang => {
                self.advance();
                let operand = Box::new(self.primary_expression()?);
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    operand,
                })
            }
            TokenType::Minus => {
                self.advance();
                let operand = Box::new(self.primary_expression()?);
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    operand,
                })
            }

            // Identifier (variable access or function call)
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();

                // Check if it's a function call
                if self.check(&TokenType::LeftParen) {
                    self.function_call(name)
                } else {
                    // Variable access with potential dot notation
                    self.variable_access(name)
                }
            }

            _ => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: format!("{}", token.token_type),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn array_literal(&mut self) -> Result<Expr, ParseError> {
        // ArrayLiteral ::= "[" ( Expression ( "," Expression )* )? "]"
        self.consume_token(&TokenType::LeftBracket, "[")?;

        let mut elements = Vec::new();

        if !self.check(&TokenType::RightBracket) {
            loop {
                elements.push(self.expression()?);

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume_token(&TokenType::RightBracket, "]")?;

        Ok(Expr::Array(elements))
    }

    fn dictionary_literal(&mut self) -> Result<Expr, ParseError> {
        // DictionaryLiteral ::= "{" ( ( STRING | IDENTIFIER ) ":" Expression ( "," ... )* )? "}"
        self.consume_token(&TokenType::LeftBrace, "{")?;

        let mut pairs = Vec::new();

        if !self.check(&TokenType::RightBrace) {
            loop {
                // Key can be either string or identifier
                let key = match &self.peek().token_type {
                    TokenType::String(s) => {
                        let s = s.clone();
                        self.advance();
                        s
                    }
                    TokenType::Identifier(s) => {
                        let s = s.clone();
                        self.advance();
                        s
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: "string or identifier".to_string(),
                            found: format!("{}", self.peek().token_type),
                            line: self.peek().line,
                            column: self.peek().column,
                        })
                    }
                };

                self.consume_token(&TokenType::Colon, ":")?;

                let value = self.expression()?;

                pairs.push((key, value));

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume_token(&TokenType::RightBrace, "}")?;

        Ok(Expr::Dictionary(pairs))
    }

    fn function_call(&mut self, name: String) -> Result<Expr, ParseError> {
        // Already consumed the identifier, now parse arguments
        self.consume_token(&TokenType::LeftParen, "(")?;

        let mut args = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                args.push(self.expression()?);

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume_token(&TokenType::RightParen, ")")?;

        Ok(Expr::FunctionCall { name, args })
    }

    fn variable_access(&mut self, first: String) -> Result<Expr, ParseError> {
        // VariableAccess ::= IDENTIFIER ( "." IDENTIFIER )*
        let mut parts = vec![first];

        while self.match_token(&TokenType::Dot) {
            let ident = self.consume_identifier()?;
            parts.push(ident);
        }

        Ok(Expr::Variable(parts))
    }

    // Generic binary operator parser
    fn binary_op(
        &mut self,
        sub_expr: fn(&mut Self) -> Result<Expr, ParseError>,
        operators: &[(TokenType, BinaryOp)],
    ) -> Result<Expr, ParseError> {
        let mut left = sub_expr(self)?;

        loop {
            let mut matched = false;

            for (token_type, op) in operators {
                if self.match_token_ref(token_type) {
                    let right = sub_expr(self)?;
                    left = Expr::Binary {
                        op: *op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    matched = true;
                    break;
                }
            }

            if !matched {
                break;
            }
        }

        Ok(left)
    }

    // Token stream helpers

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_token_ref(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume_token(&mut self, token_type: &TokenType, expected: &str) -> Result<(), ParseError> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            let token = self.peek();
            Err(ParseError::UnexpectedToken {
                expected: expected.to_string(),
                found: format!("{}", token.token_type),
                line: token.line,
                column: token.column,
            })
        }
    }

    fn consume_identifier(&mut self) -> Result<String, ParseError> {
        match &self.peek().token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => {
                let token = self.peek();
                Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{}", token.token_type),
                    line: token.line,
                    column: token.column,
                })
            }
        }
    }
}

/// Convenience function to parse source code
pub fn parse(source: &str) -> Result<Expr, Box<dyn std::error::Error>> {
    let mut lexer = amoskeag_lexer::Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    Ok(parser.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_number() {
        let expr = parse("42").unwrap();
        assert_eq!(expr, Expr::Number(42.0));
    }

    #[test]
    fn test_parse_string() {
        let expr = parse(r#""hello""#).unwrap();
        assert_eq!(expr, Expr::String("hello".to_string()));
    }

    #[test]
    fn test_parse_boolean() {
        let expr = parse("true").unwrap();
        assert_eq!(expr, Expr::Boolean(true));

        let expr = parse("false").unwrap();
        assert_eq!(expr, Expr::Boolean(false));
    }

    #[test]
    fn test_parse_symbol() {
        let expr = parse(":approve").unwrap();
        assert_eq!(expr, Expr::Symbol("approve".to_string()));
    }

    #[test]
    fn test_parse_variable() {
        let expr = parse("driver").unwrap();
        assert_eq!(expr, Expr::Variable(vec!["driver".to_string()]));

        let expr = parse("driver.age").unwrap();
        assert_eq!(
            expr,
            Expr::Variable(vec!["driver".to_string(), "age".to_string()])
        );
    }

    #[test]
    fn test_parse_function_call() {
        let expr = parse("upcase(name)").unwrap();
        assert_eq!(
            expr,
            Expr::FunctionCall {
                name: "upcase".to_string(),
                args: vec![Expr::Variable(vec!["name".to_string()])],
            }
        );
    }

    #[test]
    fn test_parse_binary_op() {
        let expr = parse("1 + 2").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Number(1.0)),
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn test_parse_comparison() {
        let expr = parse("age > 18").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::Greater,
                left: Box::new(Expr::Variable(vec!["age".to_string()])),
                right: Box::new(Expr::Number(18.0)),
            }
        );
    }

    #[test]
    fn test_parse_if_expression() {
        let expr = parse("if age > 18 :adult else :minor end").unwrap();
        assert_eq!(
            expr,
            Expr::If {
                condition: Box::new(Expr::Binary {
                    op: BinaryOp::Greater,
                    left: Box::new(Expr::Variable(vec!["age".to_string()])),
                    right: Box::new(Expr::Number(18.0)),
                }),
                then_branch: Box::new(Expr::Symbol("adult".to_string())),
                else_branch: Box::new(Expr::Symbol("minor".to_string())),
            }
        );
    }

    #[test]
    fn test_parse_let_expression() {
        let expr = parse("let x = 5 in x + 1").unwrap();
        assert_eq!(
            expr,
            Expr::Let {
                name: "x".to_string(),
                value: Box::new(Expr::Number(5.0)),
                body: Box::new(Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::Variable(vec!["x".to_string()])),
                    right: Box::new(Expr::Number(1.0)),
                }),
            }
        );
    }

    #[test]
    fn test_parse_pipe() {
        let expr = parse("name | upcase").unwrap();
        assert_eq!(
            expr,
            Expr::FunctionCall {
                name: "upcase".to_string(),
                args: vec![Expr::Variable(vec!["name".to_string()])],
            }
        );
    }

    #[test]
    fn test_parse_pipe_with_args() {
        let expr = parse("name | truncate(10)").unwrap();
        assert_eq!(
            expr,
            Expr::FunctionCall {
                name: "truncate".to_string(),
                args: vec![
                    Expr::Variable(vec!["name".to_string()]),
                    Expr::Number(10.0),
                ],
            }
        );
    }

    #[test]
    fn test_parse_pipe_chain() {
        let expr = parse("name | downcase | truncate(10)").unwrap();
        assert_eq!(
            expr,
            Expr::FunctionCall {
                name: "truncate".to_string(),
                args: vec![
                    Expr::FunctionCall {
                        name: "downcase".to_string(),
                        args: vec![Expr::Variable(vec!["name".to_string()])],
                    },
                    Expr::Number(10.0),
                ],
            }
        );
    }

    #[test]
    fn test_parse_array() {
        let expr = parse("[1, 2, 3]").unwrap();
        assert_eq!(
            expr,
            Expr::Array(vec![
                Expr::Number(1.0),
                Expr::Number(2.0),
                Expr::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_parse_dictionary() {
        let expr = parse(r#"{ "name": "Alice", age: 30 }"#).unwrap();
        assert_eq!(
            expr,
            Expr::Dictionary(vec![
                ("name".to_string(), Expr::String("Alice".to_string())),
                ("age".to_string(), Expr::Number(30.0)),
            ])
        );
    }

    #[test]
    fn test_parse_complex_expression() {
        let source = r#"
            if driver.age > 16
              :continue
            else
              :deny
            end
        "#;
        let expr = parse(source).unwrap();

        assert_eq!(
            expr,
            Expr::If {
                condition: Box::new(Expr::Binary {
                    op: BinaryOp::Greater,
                    left: Box::new(Expr::Variable(vec![
                        "driver".to_string(),
                        "age".to_string()
                    ])),
                    right: Box::new(Expr::Number(16.0)),
                }),
                then_branch: Box::new(Expr::Symbol("continue".to_string())),
                else_branch: Box::new(Expr::Symbol("deny".to_string())),
            }
        );
    }

    #[test]
    fn test_operator_precedence() {
        let expr = parse("1 + 2 * 3").unwrap();
        // Should parse as 1 + (2 * 3)
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Number(1.0)),
                right: Box::new(Expr::Binary {
                    op: BinaryOp::Multiply,
                    left: Box::new(Expr::Number(2.0)),
                    right: Box::new(Expr::Number(3.0)),
                }),
            }
        );
    }

    #[test]
    fn test_parse_nil() {
        let expr = parse("nil").unwrap();
        assert_eq!(expr, Expr::Nil);
    }

    #[test]
    fn test_parse_empty_array() {
        let expr = parse("[]").unwrap();
        assert_eq!(expr, Expr::Array(vec![]));
    }

    #[test]
    fn test_parse_empty_dictionary() {
        let expr = parse("{}").unwrap();
        assert_eq!(expr, Expr::Dictionary(vec![]));
    }

    #[test]
    fn test_parse_nested_dictionary() {
        let expr = parse(r#"{"outer": {"inner": 42}}"#).unwrap();

        if let Expr::Dictionary(outer_pairs) = expr {
            assert_eq!(outer_pairs.len(), 1);
            assert_eq!(outer_pairs[0].0, "outer");

            if let Expr::Dictionary(inner_pairs) = &outer_pairs[0].1 {
                assert_eq!(inner_pairs.len(), 1);
                assert_eq!(inner_pairs[0].0, "inner");
                assert_eq!(inner_pairs[0].1, Expr::Number(42.0));
            } else {
                panic!("Expected nested dictionary");
            }
        } else {
            panic!("Expected dictionary");
        }
    }

    #[test]
    fn test_parse_unary_not() {
        let expr = parse("not true").unwrap();
        assert_eq!(
            expr,
            Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(Expr::Boolean(true)),
            }
        );
    }

    #[test]
    fn test_parse_unary_negate() {
        let expr = parse("-10").unwrap();
        assert_eq!(
            expr,
            Expr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(Expr::Number(10.0)),
            }
        );
    }

    #[test]
    fn test_parse_logical_and() {
        let expr = parse("true and false").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Boolean(true)),
                right: Box::new(Expr::Boolean(false)),
            }
        );
    }

    #[test]
    fn test_parse_logical_or() {
        let expr = parse("true or false").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(Expr::Boolean(true)),
                right: Box::new(Expr::Boolean(false)),
            }
        );
    }

    #[test]
    fn test_parse_modulo() {
        let expr = parse("10 % 3").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::Modulo,
                left: Box::new(Expr::Number(10.0)),
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_parenthesized() {
        let expr = parse("(1 + 2) * 3").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::Multiply,
                left: Box::new(Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::Number(1.0)),
                    right: Box::new(Expr::Number(2.0)),
                }),
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_nested_let() {
        let expr = parse("let x = 1 in let y = 2 in x + y").unwrap();

        if let Expr::Let { name, value, body } = expr {
            assert_eq!(name, "x");
            assert_eq!(*value, Expr::Number(1.0));

            if let Expr::Let { name, value, body } = *body {
                assert_eq!(name, "y");
                assert_eq!(*value, Expr::Number(2.0));

                assert!(matches!(*body, Expr::Binary { .. }));
            } else {
                panic!("Expected nested let");
            }
        } else {
            panic!("Expected let expression");
        }
    }

    #[test]
    fn test_parse_nested_if() {
        let expr = parse("if true if false 1 else 2 end else 3 end").unwrap();

        if let Expr::If { condition, then_branch, else_branch } = expr {
            assert_eq!(*condition, Expr::Boolean(true));
            assert!(matches!(*then_branch, Expr::If { .. }));
            assert_eq!(*else_branch, Expr::Number(3.0));
        } else {
            panic!("Expected if expression");
        }
    }

    #[test]
    fn test_parse_function_no_args() {
        let expr = parse("func()").unwrap();
        assert_eq!(
            expr,
            Expr::FunctionCall {
                name: "func".to_string(),
                args: vec![],
            }
        );
    }

    #[test]
    fn test_parse_function_multiple_args() {
        let expr = parse("add(1, 2, 3)").unwrap();
        assert_eq!(
            expr,
            Expr::FunctionCall {
                name: "add".to_string(),
                args: vec![
                    Expr::Number(1.0),
                    Expr::Number(2.0),
                    Expr::Number(3.0),
                ],
            }
        );
    }

    #[test]
    fn test_parse_deep_variable_access() {
        let expr = parse("a.b.c.d.e").unwrap();
        assert_eq!(
            expr,
            Expr::Variable(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ])
        );
    }

    #[test]
    fn test_parse_array_with_expressions() {
        let expr = parse("[1 + 2, 3 * 4, x]").unwrap();

        if let Expr::Array(elements) = expr {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[0], Expr::Binary { .. }));
            assert!(matches!(elements[1], Expr::Binary { .. }));
            assert!(matches!(elements[2], Expr::Variable(_)));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_parse_dictionary_identifier_keys() {
        let expr = parse("{name: \"Alice\", age: 30}").unwrap();

        if let Expr::Dictionary(pairs) = expr {
            assert_eq!(pairs.len(), 2);
            assert_eq!(pairs[0].0, "name");
            assert_eq!(pairs[1].0, "age");
        } else {
            panic!("Expected dictionary");
        }
    }

    #[test]
    fn test_parse_all_comparison_operators() {
        let test_cases = vec![
            ("a == b", BinaryOp::Equal),
            ("a != b", BinaryOp::NotEqual),
            ("a < b", BinaryOp::Less),
            ("a > b", BinaryOp::Greater),
            ("a <= b", BinaryOp::LessEqual),
            ("a >= b", BinaryOp::GreaterEqual),
        ];

        for (source, expected_op) in test_cases {
            let expr = parse(source).unwrap();
            if let Expr::Binary { op, .. } = expr {
                assert_eq!(op, expected_op, "Failed for: {}", source);
            } else {
                panic!("Expected binary expression for: {}", source);
            }
        }
    }

    #[test]
    fn test_parse_chained_comparisons() {
        // Note: This should parse as (a < b) < c, not a < b < c
        let expr = parse("a < b < c").unwrap();

        if let Expr::Binary { op, left, right } = expr {
            assert_eq!(op, BinaryOp::Less);
            assert!(matches!(*left, Expr::Binary { .. }));
            assert!(matches!(*right, Expr::Variable(_)));
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_parse_mixed_operators() {
        let expr = parse("a + b * c - d / e").unwrap();
        // Should parse as (a + (b * c)) - (d / e)

        if let Expr::Binary { op, .. } = expr {
            assert_eq!(op, BinaryOp::Subtract);
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_binary_op_display() {
        assert_eq!(format!("{}", BinaryOp::Add), "+");
        assert_eq!(format!("{}", BinaryOp::And), "and");
        assert_eq!(format!("{}", BinaryOp::Equal), "==");
    }

    #[test]
    fn test_unary_op_display() {
        assert_eq!(format!("{}", UnaryOp::Not), "not");
        assert_eq!(format!("{}", UnaryOp::Negate), "-");
    }

    #[test]
    fn test_parse_string_with_special_chars() {
        let expr = parse(r#""hello\nworld""#).unwrap();
        assert_eq!(expr, Expr::String("hello\nworld".to_string()));
    }

    #[test]
    fn test_parse_triple_pipe() {
        let expr = parse("x | f | g | h").unwrap();
        // Should parse as h(g(f(x)))

        if let Expr::FunctionCall { name, args } = expr {
            assert_eq!(name, "h");
            assert_eq!(args.len(), 1);

            if let Expr::FunctionCall { name, args } = &args[0] {
                assert_eq!(name, "g");
                assert_eq!(args.len(), 1);

                if let Expr::FunctionCall { name, args } = &args[0] {
                    assert_eq!(name, "f");
                    assert_eq!(args.len(), 1);
                } else {
                    panic!("Expected third function call");
                }
            } else {
                panic!("Expected second function call");
            }
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_parse_complex_pipe() {
        let expr = parse("data | filter(x > 10) | map(double)").unwrap();

        if let Expr::FunctionCall { name, .. } = expr {
            assert_eq!(name, "map");
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_parse_let_with_complex_value() {
        let expr = parse("let x = if true 1 else 0 end in x + 1").unwrap();

        if let Expr::Let { value, .. } = expr {
            assert!(matches!(*value, Expr::If { .. }));
        } else {
            panic!("Expected let expression");
        }
    }

    #[test]
    fn test_parse_logical_and_symbol() {
        let expr = parse("true && false").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Boolean(true)),
                right: Box::new(Expr::Boolean(false)),
            }
        );
    }

    #[test]
    fn test_parse_logical_or_symbol() {
        let expr = parse("true || false").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(Expr::Boolean(true)),
                right: Box::new(Expr::Boolean(false)),
            }
        );
    }

    #[test]
    fn test_parse_bang_operator() {
        let expr = parse("!true").unwrap();
        assert_eq!(
            expr,
            Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(Expr::Boolean(true)),
            }
        );
    }

    #[test]
    fn test_parse_mixed_logical_operators() {
        let expr = parse("true && false || true").unwrap();
        // Should parse as (true && false) || true
        if let Expr::Binary { op, left, right } = expr {
            assert_eq!(op, BinaryOp::Or);
            assert!(matches!(*left, Expr::Binary { .. }));
            assert_eq!(*right, Expr::Boolean(true));
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_parse_nested_multiline_if() {
        let expr = parse(r#"
            if true
              if false
                1
              else
                2
              end
            else
              3
            end
        "#).unwrap();

        if let Expr::If { condition, then_branch, else_branch } = expr {
            assert_eq!(*condition, Expr::Boolean(true));
            assert!(matches!(*then_branch, Expr::If { .. }));
            assert_eq!(*else_branch, Expr::Number(3.0));
        } else {
            panic!("Expected if expression");
        }
    }

    #[test]
    fn test_parse_deeply_nested_if() {
        let expr = parse(r#"
            if true
              if false
                if true
                  1
                else
                  2
                end
              else
                3
              end
            else
              4
            end
        "#).unwrap();

        // Just verify it parses successfully
        assert!(matches!(expr, Expr::If { .. }));
    }

    #[test]
    fn test_parse_complex_logical_expression() {
        let expr = parse("(true || false) && (false || true)").unwrap();
        // Should parse with proper precedence and parentheses
        assert!(matches!(expr, Expr::Binary { op: BinaryOp::And, .. }));
    }
}
