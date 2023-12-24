use std::collections::HashMap;

use crate::token::Token;
use crate::RuntimeError;
use crate::{token::Literal, LoxError};

#[derive(Debug)]

pub struct Environment {
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get_literal(&self, name: &Token) -> Result<&Literal, LoxError> {
        self.values.get(&name.lexeme).ok_or_else(|| {
            LoxError::Runtime(RuntimeError::throw(format!(
                "Undefined variable: {}",
                name.lexeme
            )))
        })
    }
}
