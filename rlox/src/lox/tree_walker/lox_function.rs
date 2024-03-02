use crate::{
    frontend::{lox_callable::LoxCallable, lox_value::LoxValue},
    tree_walker::environment::Environment,
    LoxError,
};
use std::{cell::RefCell, rc::Rc};

use super::{interpreter::Interpreter, parser::FunctionDecl};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub declaration: FunctionDecl,
    pub closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: FunctionDecl, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
    ) -> Result<LoxValue, LoxError> {
        //with each function call we create a new code env
        //pass the env to execute code block where the code gets executed with the vars bounded to
        //the environement.
        //the parent env gets restored after the function environment has been interpreted
        let mut env = Environment::new_inner_environment(self.closure.clone());

        for (parameter, value) in self.declaration.parameters.iter().zip(args.iter()) {
            env.define(&parameter.clone().lexeme, value.clone());
        }

        let result = interpreter.execute_block(&self.declaration.body, RefCell::new(env))?;

        // TODO: I don't like that we wrap the return value in an error!!
        match result {
            Some(result) => Ok(result),
            None => Ok(LoxValue::Nil),
            // Ok(()) => Ok(Some(LoxValue::Nil)),
            // Err(LoxError::Return(value)) => Ok(value),
            // Err(e) => Err(e),
        }
    }

    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
}
