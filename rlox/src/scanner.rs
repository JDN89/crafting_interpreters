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
            self.scan_token();
        }

        // add at the end of source code an EOF.  not needed but cleaner
        self.tokens
            .push(Token::new(Eof, "".to_string(), None, self.line));
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // CHECK: throw error at some point?
    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(LeftParen),
            _ => (), // TODO catch all should be?
        }
        self.advance();
    }

    fn advance(&self) -> char {
        // CHECH: not sure that this will return the corrcect char -> keep an eye out
        let index_next_character = self.current + 1;
        self.source.chars().nth(index_next_character).unwrap() // TODO: add error handling
    }

    fn add_token(&mut self, ttype: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(ttype, lexeme.to_string(), None, self.line);
        self.tokens.push(token);
    }
}
