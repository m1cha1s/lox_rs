use std::collections::LinkedList;

use super::{
    error::ParsingError,
    token::{Literal, Token, TokenType},
};

pub struct Scanner {
    source: String,
    tokens: LinkedList<Token>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: LinkedList::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&LinkedList<Token>, ParsingError> {
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), ParsingError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, Literal::NONE),
            ')' => self.add_token(TokenType::RIGHT_PAREN, Literal::NONE),
            '{' => self.add_token(TokenType::LEFT_BRACE, Literal::NONE),
            '}' => self.add_token(TokenType::RIGHT_BRACE, Literal::NONE),
            ',' => self.add_token(TokenType::COMMA, Literal::NONE),
            '.' => self.add_token(TokenType::DOT, Literal::NONE),
            '-' => self.add_token(TokenType::MINUS, Literal::NONE),
            '+' => self.add_token(TokenType::PLUS, Literal::NONE),
            ';' => self.add_token(TokenType::SEMICOLON, Literal::NONE),
            '*' => self.add_token(TokenType::STAR, Literal::NONE),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BANG_EQUAL, Literal::NONE)
                } else {
                    self.add_token(TokenType::BANG, Literal::NONE)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, Literal::NONE)
                } else {
                    self.add_token(TokenType::EQUAL, Literal::NONE)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LESS_EQUAL, Literal::NONE)
                } else {
                    self.add_token(TokenType::LESS, Literal::NONE)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GREATER_EQUAL, Literal::NONE)
                } else {
                    self.add_token(TokenType::GREATER, Literal::NONE)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, Literal::NONE);
                }
            }

            ' ' => {}
            '\r' => {}
            '\t' => {}

            '\n' => self.line += 1,

            _ => {
                return Err(ParsingError::new(
                    self.line,
                    "Unexpected character.".to_string(),
                ))
            }
        };
        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u64
    }

    fn advance(&mut self) -> char {
        let out = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        out
    }

    fn add_token(&mut self, tok_type: TokenType, literal: Literal) {
        let text = self
            .source
            .as_str()
            .get((self.start as usize)..(self.current as usize))
            .unwrap()
            .to_string();
        self.tokens
            .push_back(Token::new(tok_type, text, literal, self.line));
    }
}
