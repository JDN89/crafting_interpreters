// LEARNED:

use crate::lox_error::LoxError;
use crate::token::Token;
use crate::token_type::TokenType::{self, *};
use std::string::String;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

// self is instance of Scanner, you call instance methods on self.
// Self is the type Scanner
impl Scanner {
    pub fn build_scanner(source: &String) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    // CHECK could we encounter an error during the scan_tokens process? if create and pass it up
    // through the chain
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token()?;
        }

        // add at the end of source code an EOF. Not needed but cleaner
        self.tokens
            .push(Token::new(Eof, "".to_string(), None, self.line));
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        match self.advance() {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                if self.is_match('=') {
                    self.add_token(BangEqual)
                } else {
                    self.add_token(Bang)
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(EqualEqual)
                } else {
                    self.add_token(Equal)
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(LessEqual)
                } else {
                    self.add_token(Less)
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(GreaterEqual)
                } else {
                    self.add_token(Greater)
                }
            }
            // _ => (),
            _ => {
                return Err(LoxError::new(
                    self.line,                  // Specify the line
                    self.current,               // Specify the location
                    "Character not recognized", // Specify the message
                ));
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let current_character = self.source.chars().nth(self.current).unwrap(); // TODO: add error handling
        self.current += 1; // current not used after this funciton call? because +1
        current_character
    }

    // we're going ot handle literals here later
    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None);
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<String>) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(ttype, lexeme.to_string(), literal, self.line);
        self.tokens.push(token);
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }
}
