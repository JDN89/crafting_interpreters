use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{LoxError, RuntimeError};
use crate::frontend::token::{Literal, Token};

#[derive(Debug,Clone)]

pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
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
    pub fn new_inner_environment(env: Rc<RefCell< Environment >>) -> Self {
             Environment {
                enclosing: Some(env) ,
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
                |enclosed| enclosed.borrow_mut().get_literal(name),
            )
    }
}

    // We get the current key and reassign a new value to it
    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value.clone());
            Ok(())
        } else if let Some(ref mut enclosed) = self.enclosing {
            enclosed.borrow_mut().assign(name, value)
        } else {
            Err(LoxError::Runtime(RuntimeError::throw(format!(
                "Undefined variable: {}",
                name.lexeme
            ))))
        }
    }
}
