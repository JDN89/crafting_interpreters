use std::collections::HashMap;

use crate::token::Token;
use crate::RuntimeError;
use crate::{token::Literal, LoxError};

#[derive(Debug)]

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    // If the variable isn't found in this environment, we simply try the enclosing one
    pub fn get_literal(&self, name: &Token) -> Result<&Literal, LoxError> {
        self.values
            .get(&name.lexeme)
            .ok_or_else(|| {
                LoxError::Runtime(RuntimeError::throw(format!(
                    "undefined variable: {}",
                    name.lexeme
                )))
            })
            .or_else(|_| {
                self.enclosing.as_ref().map_or_else(
                    || self.get_literal(name),
                    |_| {
                        Err(LoxError::Runtime(RuntimeError::throw(format!(
                            "undefined varialbe: {}",
                            name.lexeme
                        ))))
                    },
                )
            })
    }

    // We get the current key and reasign a new value to it
    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), LoxError> {

        if  self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value.clone());
            Ok(())
        } else if let Some(enclosed) = &mut self.enclosing {
            enclosed.assign(name, value)
        } else {
            Err(LoxError::Runtime(RuntimeError::throw(format!(
                "Undefined variable: {}",
                name.lexeme
            ))))
        }
    }
}
