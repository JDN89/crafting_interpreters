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

    // We get the current key and reasign a new value to it
    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), LoxError> {
        match self.values.contains_key(&name.lexeme) {
            true => {
                self.values.insert(name.lexeme.to_owned(), value.clone());
                Ok(())
            }

            false => Err(LoxError::Runtime(RuntimeError::throw(format!(
                "Undefined variable: {}",
                name.lexeme
            )))),
        }
    }
}
