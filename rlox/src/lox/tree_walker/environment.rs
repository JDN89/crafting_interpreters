use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::frontend::lox_value::LoxValue;
use crate::frontend::token::Token;
use crate::{LoxError, RuntimeError};

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent_env: Option<Rc<RefCell<Environment>>>,
    variables: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            parent_env: None,
            variables: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn new_inner_environment(parent_environment: Rc<RefCell<Environment>>) -> Self {
        let env = Environment {
            parent_env: Some(parent_environment),
            variables: HashMap::new(),
        };
        env
    }

    pub fn define(&mut self, name: &str, value: LoxValue) {
        self.variables.insert(name.to_string(), value);
    }

    // If the variable isn't found in this environment, we simply try the enclosing one
    pub fn get_literal(&self, name: &Token) -> Result<LoxValue, LoxError> {
        let key = &name.lexeme;
        println!("get variable {:?}",key);
        if let Some(value) = self.variables.get(key) {
            println!("get form inner scope");
            Ok(value.clone())
        } else {
            println!("get form parent scope");
            self.parent_env.as_ref().map_or_else(
                || {
                    Err(LoxError::Runtime(RuntimeError::throw(format!(
                        "undefined variable: {}",
                        name.lexeme
                    ))))
                },
                |enclosed| enclosed.borrow().get_literal(name),
            )
        }
    }

    // We get the current key and reassign a new value to it
    pub fn assign(&mut self, name: &Token, value: &LoxValue) -> Result<(), LoxError> {
        if self.variables.contains_key(&name.lexeme) {
            self.variables
                .insert(name.lexeme.to_string(), value.clone());
            Ok(())
        } else if let Some(ref mut enclosed) = self.parent_env {
            enclosed.borrow_mut().assign(name, value)
        } else {
            Err(LoxError::Runtime(RuntimeError::throw(format!(
                "Undefined variable: {}",
                name.lexeme
            ))))
        }
    }
}
