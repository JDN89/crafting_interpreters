use std::fmt::{self, Display};
use crate::frontend::token_type::TokenType;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Integer(f64),
    Boolean(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => s.fmt(f),
            Literal::Integer(num) => num.fmt(f),
            Literal::Boolean(b) => b.fmt(f),
            Literal::Nil => write!(f, "`nil`"),
        }
    }
}

impl Literal {
    pub fn as_str(&self) -> String {
        match self {
            Literal::String(s) => s.clone(),
            Literal::Integer(num) => num.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => String::from("nil"),
        }
    }
}


// Implement custom equality impl because equality for lox is laxer than equality for rust and we
// can have nill types
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Nil, Self::Nil) => true,
            (Self::Nil, _) => false,
            (_, Self::Nil) => false,

            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[allow(unused, dead_code)]
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
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
