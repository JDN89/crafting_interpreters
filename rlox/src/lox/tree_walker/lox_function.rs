use crate::{
    frontend::{lox_callable::LoxCallable, lox_value::LoxValue},
    tree_walker::environment::Environment,
    LoxError,
};
use std::rc::Rc;

use super::{interpreter::Interpreter, parser::FunctionDecl};

#[derive(Debug)]
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
        self.declaration.parameters.len().clone()
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
        let mut env = Environment::new_inner_environment(Rc::clone(&interpreter.globals));
        for (parameter, value) in self.declaration.parameters.iter().zip(args.iter()) {
            env.define(&parameter.clone().lexeme, value.clone());
        }

        let _ = interpreter.execute_block(&self.declaration.body, env);

        return Ok(LoxValue::Nil);
    }

    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
}
