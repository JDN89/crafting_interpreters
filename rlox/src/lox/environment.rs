use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use crate::token::Token;
use crate::RuntimeError;
use crate::{token::Literal, LoxError};

#[derive(Debug,Clone)]

pub struct Environment {
    enclosing: Option<Rc<RefCell< Environment >>>,
    pub values: HashMap<String, Literal>,
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
                enclosing: Some(env),
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
                |enclosed| enclosed.borrow().get_literal(name),
            )
    }
}

    // We get the current key and reasign a new value to it
    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value.clone());
            Ok(())
        } else if let Some(enclosed) = self.enclosing.as_ref() {
            let mut borrow: RefMut<Environment> = enclosed.borrow_mut();
            borrow.assign(name, value)
        } else {
            Err(LoxError::Runtime(RuntimeError::throw(format!(
                "Undefined variable: {}",
                name.lexeme
            ))))
        }
    }
}
