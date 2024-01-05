use std::collections::HashMap;

use crate::{LoxError, RuntimeError};
use crate::frontend::token::{Literal, Token};

#[derive(Debug,Clone)]

pub struct Environment {
    enclosing: Option<Box< Environment >>,
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn new_inner_environment(env:&Environment) -> Self {
             Environment {
                enclosing: Some(Box::new(env.clone())),
                values: HashMap::new(),
            }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    // If the variable isn't found in this environment, we simply try the enclosing one
pub fn get_literal(&self, name: &Token) -> Result<Literal, LoxError> {
    if let Some(value) = self.values.get(&name.lexeme) {
        Ok(value.clone())
    } else {
        self.enclosing
            .as_ref()
            .map_or_else(
                || Err(LoxError::Runtime(RuntimeError::throw(format!(
                    "undefined variable: {}",
                    name.lexeme
                )))),
                |enclosed| enclosed.as_ref().get_literal(name),
            )
    }
}

    // We get the current key and reassign a new value to it
    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value.clone());
            Ok(())
        } else if let Some(ref mut enclosed) = self.enclosing {
            enclosed.assign(name, value)
        } else {
            Err(LoxError::Runtime(RuntimeError::throw(format!(
                "Undefined variable: {}",
                name.lexeme
            ))))
        }
    }
}
