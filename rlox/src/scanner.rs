use crate::lox_error::LoxError;
use crate::token::Token;
use crate::token_type::TokenType::*;
use std::string::String;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn build_scanner(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !Self::is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            Self::scan_token();
        }
        self.tokens
            .push(Token::build_token(Eof, "".to_string(), None, self.line));
        Ok(&self.tokens)
    }

    fn is_at_end() -> bool {
        todo!()
    }

    fn scan_token() -> Vec<Token> {
        todo!()
    }
}
