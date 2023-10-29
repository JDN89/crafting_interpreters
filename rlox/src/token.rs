// how the fuck do you structure projects in rust?

// In rust modules are not mapped to the FS like f.e. Java
// you can declare a module with the mod keyword and have multiple mods in the same file
// In rust Sub modules must be declared within the parent module -> so in a lib.rs file?

use crate::token_type::TokenType;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>, // change to appropriate type later
    line: usize,
}

#[allow(unused, dead_code)]
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
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
