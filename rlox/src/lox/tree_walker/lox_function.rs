use crate::{
    frontend::{lox_callable::LoxCallable, lox_value::LoxValue},
    tree_walker::environment::Environment,
    LoxError,
};
use std::rc::Rc;

use super::{interpreter::Interpreter, parser::FunctionDecl};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub declaration: FunctionDecl,
}

impl LoxFunction {
    pub fn new(declaration: FunctionDecl) -> Self {
        Self { declaration }
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
        let mut env = Environment::new_inner_environment(Rc::clone(&interpreter.globals));
        for (parameter, value) in self.declaration.parameters.iter().zip(args.iter()) {
            env.define(&parameter.clone().lexeme, value.clone());
        }

        let result = interpreter.execute_block(&self.declaration.body, env);

        // TODO: I don't like that we wrap the return value in an error!!
        match result {
            Ok(()) => Ok(LoxValue::Nil),
            Err(LoxError::Return(value)) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
}
