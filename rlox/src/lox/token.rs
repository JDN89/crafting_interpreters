// LEARNED:

// In rust modules are not mapped to the FS like f.e. Java
// you can declare a module with the mod keyword and have multiple mods in the same file
// In rust Sub modules must be declared within the parent module -> in our case inside the main.rs
// file

use crate::token_type::TokenType;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Integer(f64),
}

#[allow(unused, dead_code)]
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
