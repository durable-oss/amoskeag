//! Amoskeag Lexer
//!
//! The lexical analyzer (lexer) for the Amoskeag language. This module is responsible
//! for consuming UTF-8 source text and producing a stream of tokens.
//!
//! # Features
//!
//! - Tokenizes keywords, operators, literals, identifiers, and symbols
//! - Handles whitespace and comments
//! - Provides detailed error reporting with line and column information
//! - Supports string literals with escape sequences

use std::fmt;
use thiserror::Error;

/// Token types in the Amoskeag language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    If,
    Then,
    Else,
    End,
    Let,
    In,
    True,
    False,
    Nil,
    And,
    Or,
    Not,

    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    Symbol(String), // The value after the colon, e.g., :approve -> "approve"

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Pipe,
    Dot,
    Bang, // ! (unary not)

    // Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Logical operators (symbols)
    LogicalAnd, // &&
    LogicalOr,  // ||

    // Assignment/Binding
    Assign,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,

    // Special
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::If => write!(f, "if"),
            TokenType::Then => write!(f, "then"),
            TokenType::Else => write!(f, "else"),
            TokenType::End => write!(f, "end"),
            TokenType::Let => write!(f, "let"),
            TokenType::In => write!(f, "in"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::And => write!(f, "and"),
            TokenType::Or => write!(f, "or"),
            TokenType::Not => write!(f, "not"),
            TokenType::Number(n) => write!(f, "{}", n),
            TokenType::String(s) => write!(f, "\"{}\"", s),
            TokenType::Identifier(s) => write!(f, "{}", s),
            TokenType::Symbol(s) => write!(f, ":{}", s),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Percent => write!(f, "%"),
            TokenType::Pipe => write!(f, "|"),
            TokenType::Dot => write!(f, "."),
            TokenType::Bang => write!(f, "!"),
            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::Less => write!(f, "<"),
            TokenType::Greater => write!(f, ">"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::LogicalAnd => write!(f, "&&"),
            TokenType::LogicalOr => write!(f, "||"),
            TokenType::Assign => write!(f, "="),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Colon => write!(f, ":"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

/// A token with location information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}

/// Lexer errors
#[derive(Error, Debug)]
pub enum LexError {
    #[error("Unexpected character '{character}' at line {line}, column {column}")]
    UnexpectedCharacter {
        character: char,
        line: usize,
        column: usize,
    },

    #[error("Unterminated string at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },

    #[error("Invalid number format at line {line}, column {column}")]
    InvalidNumber { line: usize, column: usize },

    #[error("Invalid escape sequence '\\{sequence}' at line {line}, column {column}")]
    InvalidEscape {
        sequence: char,
        line: usize,
        column: usize,
    },
}

/// The Amoskeag lexer
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer from source code
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            if token.token_type == TokenType::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            return Ok(Token::new(
                TokenType::Eof,
                String::new(),
                self.line,
                self.column,
            ));
        }

        let start_line = self.line;
        let start_column = self.column;
        let ch = self.advance();

        match ch {
            // Single-character tokens
            '(' => Ok(self.make_token(TokenType::LeftParen, "(", start_line, start_column)),
            ')' => Ok(self.make_token(TokenType::RightParen, ")", start_line, start_column)),
            '[' => Ok(self.make_token(TokenType::LeftBracket, "[", start_line, start_column)),
            ']' => Ok(self.make_token(TokenType::RightBracket, "]", start_line, start_column)),
            '{' => Ok(self.make_token(TokenType::LeftBrace, "{", start_line, start_column)),
            '}' => Ok(self.make_token(TokenType::RightBrace, "}", start_line, start_column)),
            ',' => Ok(self.make_token(TokenType::Comma, ",", start_line, start_column)),
            '.' => Ok(self.make_token(TokenType::Dot, ".", start_line, start_column)),
            '+' => Ok(self.make_token(TokenType::Plus, "+", start_line, start_column)),
            '-' => Ok(self.make_token(TokenType::Minus, "-", start_line, start_column)),
            '*' => Ok(self.make_token(TokenType::Star, "*", start_line, start_column)),
            '/' => Ok(self.make_token(TokenType::Slash, "/", start_line, start_column)),
            '%' => Ok(self.make_token(TokenType::Percent, "%", start_line, start_column)),

            // Pipe or logical OR
            '|' => {
                if self.match_char('|') {
                    Ok(self.make_token(TokenType::LogicalOr, "||", start_line, start_column))
                } else {
                    Ok(self.make_token(TokenType::Pipe, "|", start_line, start_column))
                }
            }

            // Logical AND
            '&' => {
                if self.match_char('&') {
                    Ok(self.make_token(TokenType::LogicalAnd, "&&", start_line, start_column))
                } else {
                    Err(LexError::UnexpectedCharacter {
                        character: '&',
                        line: start_line,
                        column: start_column,
                    })
                }
            }

            // Two-character tokens or single character
            '=' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenType::Equal, "==", start_line, start_column))
                } else {
                    Ok(self.make_token(TokenType::Assign, "=", start_line, start_column))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenType::NotEqual, "!=", start_line, start_column))
                } else {
                    Ok(self.make_token(TokenType::Bang, "!", start_line, start_column))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenType::LessEqual, "<=", start_line, start_column))
                } else {
                    Ok(self.make_token(TokenType::Less, "<", start_line, start_column))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenType::GreaterEqual, ">=", start_line, start_column))
                } else {
                    Ok(self.make_token(TokenType::Greater, ">", start_line, start_column))
                }
            }

            // Symbol literals
            ':' => self.scan_symbol(start_line, start_column),

            // String literals
            '"' | '\'' => self.scan_string(ch, start_line, start_column),

            // Numbers
            '0'..='9' => self.scan_number(ch, start_line, start_column),

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(ch, start_line, start_column),

            _ => Err(LexError::UnexpectedCharacter {
                character: ch,
                line: start_line,
                column: start_column,
            }),
        }
    }

    // Helper methods

    fn make_token(&self, token_type: TokenType, lexeme: &str, line: usize, column: usize) -> Token {
        Token::new(token_type, lexeme.to_string(), line, column)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn advance(&mut self) -> char {
        debug_assert!(
            self.position < self.input.len(),
            "advance() called at end of input"
        );
        let ch = self.input[self.position];
        self.position += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        ch
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.input[self.position])
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + 1])
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.input[self.position] != expected {
            return false;
        }
        self.advance();
        true
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                    self.advance();
                }
                Some('#') => {
                    // Skip comment until end of line
                    while let Some(ch) = self.peek() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn scan_string(
        &mut self,
        quote: char,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, LexError> {
        let mut value = String::new();

        while let Some(ch) = self.peek() {
            if ch == quote {
                self.advance(); // Consume closing quote
                let lexeme = format!("{}{}{}", quote, value, quote);
                return Ok(Token::new(
                    TokenType::String(value),
                    lexeme,
                    start_line,
                    start_column,
                ));
            }

            if ch == '\\' {
                self.advance(); // Consume backslash
                if let Some(escaped) = self.peek() {
                    self.advance();
                    let unescaped = match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        '"' => '"',
                        _ => {
                            return Err(LexError::InvalidEscape {
                                sequence: escaped,
                                line: self.line,
                                column: self.column - 1,
                            });
                        }
                    };
                    value.push(unescaped);
                } else {
                    return Err(LexError::UnterminatedString {
                        line: start_line,
                        column: start_column,
                    });
                }
            } else {
                value.push(ch);
                self.advance();
            }
        }

        Err(LexError::UnterminatedString {
            line: start_line,
            column: start_column,
        })
    }

    fn scan_number(
        &mut self,
        first: char,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, LexError> {
        debug_assert!(
            first.is_ascii_digit(),
            "scan_number() called with non-digit"
        );
        let mut lexeme = String::new();
        lexeme.push(first);

        // Scan integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                lexeme.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal part
        if let Some('.') = self.peek() {
            if let Some(next) = self.peek_next() {
                if next.is_ascii_digit() {
                    lexeme.push('.');
                    self.advance();

                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_digit() {
                            lexeme.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        match lexeme.parse::<f64>() {
            Ok(num) if num.is_finite() => Ok(Token::new(
                TokenType::Number(num),
                lexeme,
                start_line,
                start_column,
            )),
            Ok(_) => Err(LexError::InvalidNumber {
                line: start_line,
                column: start_column,
            }),
            Err(_) => Err(LexError::InvalidNumber {
                line: start_line,
                column: start_column,
            }),
        }
    }

    fn scan_identifier(
        &mut self,
        first: char,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, LexError> {
        let mut lexeme = String::new();
        lexeme.push(first);

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                lexeme.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = match lexeme.as_str() {
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "else" => TokenType::Else,
            "end" => TokenType::End,
            "let" => TokenType::Let,
            "in" => TokenType::In,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "nil" => TokenType::Nil,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            _ => TokenType::Identifier(lexeme.clone()),
        };

        Ok(Token::new(token_type, lexeme, start_line, start_column))
    }

    fn scan_symbol(&mut self, start_line: usize, start_column: usize) -> Result<Token, LexError> {
        // Symbol is : followed by either an identifier or a string
        match self.peek() {
            Some('"') | Some('\'') => {
                let quote = self.advance();
                let mut value = String::new();

                while let Some(ch) = self.peek() {
                    if ch == quote {
                        self.advance();
                        let lexeme = format!(":{}{}{}", quote, value, quote);
                        return Ok(Token::new(
                            TokenType::Symbol(value),
                            lexeme,
                            start_line,
                            start_column,
                        ));
                    }

                    if ch == '\\' {
                        self.advance();
                        if let Some(escaped) = self.peek() {
                            self.advance();
                            let unescaped = match escaped {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                '\\' => '\\',
                                '\'' => '\'',
                                '"' => '"',
                                _ => {
                                    return Err(LexError::InvalidEscape {
                                        sequence: escaped,
                                        line: self.line,
                                        column: self.column - 1,
                                    });
                                }
                            };
                            value.push(unescaped);
                        } else {
                            return Err(LexError::UnterminatedString {
                                line: start_line,
                                column: start_column,
                            });
                        }
                    } else {
                        value.push(ch);
                        self.advance();
                    }
                }

                Err(LexError::UnterminatedString {
                    line: start_line,
                    column: start_column,
                })
            }
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                let mut value = String::new();
                value.push(self.advance());

                while let Some(ch) = self.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        value.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                let lexeme = format!(":{}", value);
                Ok(Token::new(
                    TokenType::Symbol(value),
                    lexeme,
                    start_line,
                    start_column,
                ))
            }
            _ => {
                // Just a colon (for dictionary literals)
                Ok(Token::new(
                    TokenType::Colon,
                    ":".to_string(),
                    start_line,
                    start_column,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_keywords() {
        let input = "if else end let in true false nil and or not";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::If);
        assert_eq!(tokens[1].token_type, TokenType::Else);
        assert_eq!(tokens[2].token_type, TokenType::End);
        assert_eq!(tokens[3].token_type, TokenType::Let);
        assert_eq!(tokens[4].token_type, TokenType::In);
        assert_eq!(tokens[5].token_type, TokenType::True);
        assert_eq!(tokens[6].token_type, TokenType::False);
        assert_eq!(tokens[7].token_type, TokenType::Nil);
        assert_eq!(tokens[8].token_type, TokenType::And);
        assert_eq!(tokens[9].token_type, TokenType::Or);
        assert_eq!(tokens[10].token_type, TokenType::Not);
    }

    #[test]
    fn test_numbers() {
        let input = "123 45.67 0.5";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Number(123.0));
        assert_eq!(tokens[1].token_type, TokenType::Number(45.67));
        assert_eq!(tokens[2].token_type, TokenType::Number(0.5));
    }

    #[test]
    fn test_strings() {
        let input = r#""hello" 'world' "hello\nworld""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::String("hello".to_string()));
        assert_eq!(tokens[1].token_type, TokenType::String("world".to_string()));
        assert_eq!(
            tokens[2].token_type,
            TokenType::String("hello\nworld".to_string())
        );
    }

    #[test]
    fn test_symbols() {
        let input = r#":approve :deny :"test.something""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens[0].token_type,
            TokenType::Symbol("approve".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::Symbol("deny".to_string()));
        assert_eq!(
            tokens[2].token_type,
            TokenType::Symbol("test.something".to_string())
        );
    }

    #[test]
    fn test_operators() {
        let input = "+ - * / % | . == != < > <= >=";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Star);
        assert_eq!(tokens[3].token_type, TokenType::Slash);
        assert_eq!(tokens[4].token_type, TokenType::Percent);
        assert_eq!(tokens[5].token_type, TokenType::Pipe);
        assert_eq!(tokens[6].token_type, TokenType::Dot);
        assert_eq!(tokens[7].token_type, TokenType::Equal);
        assert_eq!(tokens[8].token_type, TokenType::NotEqual);
        assert_eq!(tokens[9].token_type, TokenType::Less);
        assert_eq!(tokens[10].token_type, TokenType::Greater);
        assert_eq!(tokens[11].token_type, TokenType::LessEqual);
        assert_eq!(tokens[12].token_type, TokenType::GreaterEqual);
    }

    #[test]
    fn test_comments() {
        let input = "let x = 5 # this is a comment\n# another comment\nlet y = 10";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("x".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Assign);
        assert_eq!(tokens[3].token_type, TokenType::Number(5.0));
        assert_eq!(tokens[4].token_type, TokenType::Let);
        assert_eq!(tokens[5].token_type, TokenType::Identifier("y".to_string()));
    }

    #[test]
    fn test_complete_expression() {
        let input = r#"
            if driver.age > 16
              :continue
            else
              :deny
            end
        "#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::If);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("driver".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::Dot);
        assert_eq!(
            tokens[3].token_type,
            TokenType::Identifier("age".to_string())
        );
        assert_eq!(tokens[4].token_type, TokenType::Greater);
        assert_eq!(tokens[5].token_type, TokenType::Number(16.0));
        assert_eq!(
            tokens[6].token_type,
            TokenType::Symbol("continue".to_string())
        );
        assert_eq!(tokens[7].token_type, TokenType::Else);
        assert_eq!(tokens[8].token_type, TokenType::Symbol("deny".to_string()));
        assert_eq!(tokens[9].token_type, TokenType::End);
    }

    #[test]
    fn test_pipe_expression() {
        let input = "salesperson.name | downcase";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("salesperson".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::Dot);
        assert_eq!(
            tokens[2].token_type,
            TokenType::Identifier("name".to_string())
        );
        assert_eq!(tokens[3].token_type, TokenType::Pipe);
        assert_eq!(
            tokens[4].token_type,
            TokenType::Identifier("downcase".to_string())
        );
    }

    #[test]
    fn test_error_unterminated_string() {
        let input = r#""unterminated"#;
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        assert!(result.is_err());

        if let Err(LexError::UnterminatedString { line, column }) = result {
            assert_eq!(line, 1);
            assert_eq!(column, 1);
        } else {
            panic!("Expected UnterminatedString error");
        }
    }

    #[test]
    fn test_error_unexpected_character() {
        let input = "@";
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        assert!(result.is_err());

        if let Err(LexError::UnexpectedCharacter {
            character,
            line,
            column,
        }) = result
        {
            assert_eq!(character, '@');
            assert_eq!(line, 1);
            assert_eq!(column, 1);
        } else {
            panic!("Expected UnexpectedCharacter error");
        }
    }

    #[test]
    fn test_error_invalid_escape() {
        let input = r#""hello\x""#;
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        assert!(result.is_err());

        if let Err(LexError::InvalidEscape { sequence, .. }) = result {
            assert_eq!(sequence, 'x');
        } else {
            panic!("Expected InvalidEscape error");
        }
    }

    #[test]
    fn test_line_and_column_tracking() {
        let input = "let\nx\n=\n5";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].line, 1); // let
        assert_eq!(tokens[1].line, 2); // x
        assert_eq!(tokens[2].line, 3); // =
        assert_eq!(tokens[3].line, 4); // 5
    }

    #[test]
    fn test_multiline_comment() {
        let input = "# comment\nlet x = 5\n# another comment";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // Should have: let, x, =, 5, EOF
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Let);
    }

    #[test]
    fn test_escape_sequences() {
        let input = r#""hello\nworld\t\r\\\"\'""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens[0].token_type,
            TokenType::String("hello\nworld\t\r\\\"'".to_string())
        );
    }

    #[test]
    fn test_number_formats() {
        let input = "0 123 0.5 123.456 .0";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Number(0.0));
        assert_eq!(tokens[1].token_type, TokenType::Number(123.0));
        assert_eq!(tokens[2].token_type, TokenType::Number(0.5));
        assert_eq!(tokens[3].token_type, TokenType::Number(123.456));
        // .0 should be tokenized as . and 0
        assert_eq!(tokens[4].token_type, TokenType::Dot);
        assert_eq!(tokens[5].token_type, TokenType::Number(0.0));
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "  let   x   =   5  ";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // Should have: let, x, =, 5, EOF
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Let);
    }

    #[test]
    fn test_single_and_double_quotes() {
        let input = r#""double" 'single'"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens[0].token_type,
            TokenType::String("double".to_string())
        );
        assert_eq!(
            tokens[1].token_type,
            TokenType::String("single".to_string())
        );
    }

    #[test]
    fn test_symbol_with_quotes() {
        let input = r#":simple :"with spaces" :'single quoted'"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens[0].token_type,
            TokenType::Symbol("simple".to_string())
        );
        assert_eq!(
            tokens[1].token_type,
            TokenType::Symbol("with spaces".to_string())
        );
        assert_eq!(
            tokens[2].token_type,
            TokenType::Symbol("single quoted".to_string())
        );
    }

    #[test]
    fn test_colon_in_dictionary() {
        let input = r#"{"key": "value"}"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::String("key".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::String("value".to_string()));
        assert_eq!(tokens[4].token_type, TokenType::RightBrace);
    }

    #[test]
    fn test_exclamation_mark() {
        let input = "!";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Bang);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_all_operators() {
        let input = "+ - * / % | . == != < > <= >= =";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        let expected = vec![
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Percent,
            TokenType::Pipe,
            TokenType::Dot,
            TokenType::Equal,
            TokenType::NotEqual,
            TokenType::Less,
            TokenType::Greater,
            TokenType::LessEqual,
            TokenType::GreaterEqual,
            TokenType::Assign,
            TokenType::Eof,
        ];

        for (i, expected_type) in expected.iter().enumerate() {
            assert_eq!(tokens[i].token_type, *expected_type);
        }
    }

    #[test]
    fn test_identifiers_with_underscores() {
        let input = "_start middle_ _under_score_";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("_start".to_string())
        );
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("middle_".to_string())
        );
        assert_eq!(
            tokens[2].token_type,
            TokenType::Identifier("_under_score_".to_string())
        );
    }

    #[test]
    fn test_logical_and_operator() {
        let input = "&&";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::LogicalAnd);
    }

    #[test]
    fn test_logical_or_operator() {
        let input = "||";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::LogicalOr);
    }

    #[test]
    fn test_pipe_vs_logical_or() {
        let input = "| ||";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Pipe);
        assert_eq!(tokens[1].token_type, TokenType::LogicalOr);
    }

    #[test]
    fn test_bang_operator() {
        let input = "!true";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Bang);
        assert_eq!(tokens[1].token_type, TokenType::True);
    }

    #[test]
    fn test_bang_vs_not_equal() {
        let input = "! !=";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Bang);
        assert_eq!(tokens[1].token_type, TokenType::NotEqual);
    }

    #[test]
    fn test_all_logical_operators() {
        let input = "and or && || not !";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::And);
        assert_eq!(tokens[1].token_type, TokenType::Or);
        assert_eq!(tokens[2].token_type, TokenType::LogicalAnd);
        assert_eq!(tokens[3].token_type, TokenType::LogicalOr);
        assert_eq!(tokens[4].token_type, TokenType::Not);
        assert_eq!(tokens[5].token_type, TokenType::Bang);
    }

    #[test]
    fn test_mixed_case_keywords() {
        let input = "if IF If";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // Only lowercase "if" is a keyword
        assert_eq!(tokens[0].token_type, TokenType::If);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("IF".to_string())
        );
        assert_eq!(
            tokens[2].token_type,
            TokenType::Identifier("If".to_string())
        );
    }
}
