use std::fmt;

use crate::frontend::token_type::TokenType;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<super::lox_value::LoxValue>,
    pub line: usize,
}

#[allow(unused, dead_code)]
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<super::lox_value::LoxValue>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token: '{:?}', literal: {:?}", self.lexeme, self.literal)
    }
}
