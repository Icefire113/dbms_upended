use std::{iter::Peekable, str::Chars};

use strum::IntoEnumIterator;

use crate::ql::{
    tokenizer::errors::SQLTokenizeError,
    tokenizer::token::{Keyword, LiteralToken, Operator, Token, TokenType},
};

/// The tokenizer itself
#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    num_chars: usize,
    /// An iterator over the characters of the input
    chars: Peekable<Chars<'a>>,
    /// The current position
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    /// Construct a new tokenizer for a given input
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            num_chars: input.chars().count(),
            chars: input.chars().peekable(),
            pos: 0,
        }
    }

    /// Parse the input to that was given to us into a list of raw tokens
    pub fn tokenize(&mut self) -> anyhow::Result<Vec<Token>, SQLTokenizeError> {
        let mut tokens = vec![];

        while let Some(tok) = self.get_next_token() {
            match tok.token_type {
                TokenType::Illegal(pos) => {
                    let (line, col) = self.pos_to_line_col(pos).unwrap();
                    return Err(SQLTokenizeError::IllegalToken(tok.value, line, col));
                }
                TokenType::Unknown(pos) => {
                    let (line, col) = self.pos_to_line_col(pos).unwrap();
                    return Err(SQLTokenizeError::UnknownToken(tok.value, line, col));
                }
                _ => {}
            }
            tokens.push(tok);
        }

        Ok(tokens)
    }

    /// Gets the next token if one exists
    pub fn get_next_token(&mut self) -> Option<Token> {
        if self.pos >= self.num_chars {
            return None;
        }

        match self.chars.peek() {
            Some(&c) => match c {
                // whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                    self.get_next_token()
                }
                // numeric literal
                '0'..='9' => {
                    let mut value = String::new();
                    let mut has_dot = false;

                    while let Some(&c) = self.chars.peek() {
                        if c.is_ascii_digit() {
                            value.push(c);
                            self.advance();
                        } else if c == '.' && !has_dot {
                            has_dot = true;

                            value.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    if let Ok(n) = value.parse::<i32>() {
                        Some(Token::new(TokenType::Literal(LiteralToken::Int(n)), value))
                    } else if let Ok(n) = value.parse::<i64>() {
                        Some(Token::new(
                            TokenType::Literal(LiteralToken::BigInt(n)),
                            value,
                        ))
                    } else if let Ok(n) = value.parse::<f32>() {
                        Some(Token::new(
                            TokenType::Literal(LiteralToken::Float(n)),
                            value,
                        ))
                    } else if let Ok(n) = value.parse::<f64>() {
                        Some(Token::new(
                            TokenType::Literal(LiteralToken::BigFloat(n)),
                            value,
                        ))
                    } else {
                        Some(Token::new(TokenType::Illegal(self.pos), value))
                    }
                }
                // a keyword, or identifier
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut value = String::new();
                    while let Some(&c) = self.chars.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '.' => {
                                value.push(c);
                                self.advance();
                            }
                            _ => break,
                        }
                    }

                    // check if it's a keyword
                    let tok_type = Keyword::iter()
                        .find(|k| k.to_string().to_uppercase() == value.to_uppercase())
                        .map(TokenType::Keyword)
                        .unwrap_or(TokenType::Identifier(value.clone()));

                    Some(Token::new(tok_type, value))
                }
                // a string literal
                '\'' => {
                    self.advance();
                    let mut value = String::new();
                    let mut found_end = false;
                    while let Some(&c) = self.chars.peek() {
                        self.advance();
                        if c == '\'' {
                            if self.chars.peek() == Some(&'\'') {
                                value.push(c);
                                self.advance();
                            } else {
                                found_end = true;
                                break;
                            }
                        } else {
                            value.push(c);
                        }
                    }
                    if found_end {
                        Some(Token::new(
                            TokenType::Literal(LiteralToken::String(value.clone())),
                            value,
                        ))
                    } else {
                        Some(Token::new(TokenType::Illegal(self.pos), value))
                    }
                }
                // a quoted identifier
                '"' => {
                    self.advance();
                    let mut value = String::new();
                    let mut found_end = false;
                    while let Some(&c) = self.chars.peek() {
                        self.advance();
                        if c == '"' {
                            if self.chars.peek() == Some(&'"') {
                                value.push(c);
                                self.advance();
                            } else {
                                found_end = true;
                                break;
                            }
                        } else {
                            value.push(c);
                        }
                    }
                    if found_end {
                        Some(Token::new(
                            TokenType::QuotedIdentifier(value.clone()),
                            value,
                        ))
                    } else {
                        Some(Token::new(TokenType::Illegal(self.pos), value))
                    }
                }
                // an operator that starts with a < sign
                '<' => {
                    self.advance();
                    match self.chars.peek() {
                        Some(&'=') => {
                            self.advance();
                            Some(Token::new(TokenType::Operator(Operator::Lte), "<="))
                        }
                        Some(&'>') => {
                            self.advance();
                            Some(Token::new(TokenType::Operator(Operator::NotEq), "<>"))
                        }
                        Some(&' ') | None => {
                            Some(Token::new(TokenType::Operator(Operator::Lt), "<"))
                        }
                        _ => {
                            self.advance();
                            Some(Token::new(TokenType::Illegal(self.pos), "<"))
                        }
                    }
                }
                // an operator that starts with a > sign
                '>' => {
                    self.advance();
                    match self.chars.peek() {
                        Some(&'=') => {
                            self.advance();
                            Some(Token::new(TokenType::Operator(Operator::Gte), ">="))
                        }
                        Some(&' ') | None => {
                            Some(Token::new(TokenType::Operator(Operator::Gt), ">"))
                        }
                        _ => {
                            self.advance();
                            Some(Token::new(TokenType::Illegal(self.pos), ">"))
                        }
                    }
                }
                // the = operator
                '=' => {
                    self.advance();
                    Some(Token::new(TokenType::Operator(Operator::Equals), "="))
                }
                // an operator that starts with a !
                '!' => {
                    self.advance();
                    match self.chars.peek() {
                        Some(&'=') => {
                            self.advance();
                            Some(Token::new(TokenType::Operator(Operator::NotEq), "!="))
                        }
                        _ => Some(Token::new(TokenType::Illegal(self.pos), "!")),
                    }
                }
                // an operator that starts with a + sign
                '+' => {
                    self.advance();
                    Some(Token::new(TokenType::Operator(Operator::Plus), "+"))
                }
                // an operator that starts with a - sign
                '-' => {
                    self.advance();
                    match self.chars.peek() {
                        Some(&'-') => {
                            self.advance();
                            while let Some(&c) = self.chars.peek() {
                                if c == '\n' || c == '\r' {
                                    break;
                                }
                                self.advance();
                            }
                            self.get_next_token()
                        }
                        _ => Some(Token::new(TokenType::Operator(Operator::Minus), "-")),
                    }
                }
                // a * operator
                '*' => {
                    self.advance();
                    Some(Token::new(TokenType::Operator(Operator::Star), "*"))
                }
                // an operator that starts with a / sign, or some kind of comment
                '/' => {
                    self.advance();
                    match self.chars.peek() {
                        // skip block comments
                        Some(&'*') => {
                            self.advance();
                            loop {
                                match self.chars.peek() {
                                    Some(&'*') => {
                                        self.advance();
                                        if self.chars.peek() == Some(&'/') {
                                            self.advance();
                                            break;
                                        }
                                    }
                                    None => {
                                        break;
                                    }
                                    _ => {
                                        self.advance();
                                    }
                                }
                            }
                            self.get_next_token()
                        }
                        _ => Some(Token::new(TokenType::Operator(Operator::Divide), "/")),
                    }
                }
                // Modulus
                '%' => {
                    self.advance();
                    Some(Token::new(TokenType::Operator(Operator::Modulus), "%"))
                }
                // semicolon
                ';' => {
                    self.advance();
                    Some(Token::new(TokenType::SemiColon, ";"))
                }
                // left parenthesis
                '(' => {
                    self.advance();
                    Some(Token::new(TokenType::LParen, "("))
                }
                // right parenthesis
                ')' => {
                    self.advance();
                    Some(Token::new(TokenType::RParen, ")"))
                }
                // comma
                ',' => {
                    self.advance();
                    Some(Token::new(TokenType::Comma, ","))
                }
                // anything else
                _ => {
                    self.advance();
                    Some(Token::new(TokenType::Unknown(self.pos), c))
                }
            },
            None => None,
        }
    }

    /// Turns a token position into a line and column number of the underlying input string
    ///
    /// If the position is out of bounds, returns None, otherwise returns the line and column number
    fn pos_to_line_col(&self, pos: usize) -> Option<(usize, usize)> {
        if pos > self.input.len() {
            return None;
        }

        let newlines: Vec<usize> = self
            .input
            .bytes()
            .enumerate()
            .filter_map(|(i, b)| (b == b'\n').then_some(i))
            .collect();

        let line = newlines.partition_point(|&i| i < pos);

        let line_start = if line == 0 { 0 } else { newlines[line - 1] + 1 };

        Some((line + 1, pos - line_start))
    }

    /// Advance the tokenizer to the next character and increase our position
    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }
}
