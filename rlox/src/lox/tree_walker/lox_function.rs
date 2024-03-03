use crate::{
    frontend::{lox_callable::LoxCallable, lox_value::LoxValue},
    tree_walker::environment::Environment,
    LoxError,
};
use std::{rc::Rc, cell::RefCell};

use super::{interpreter::Interpreter, parser::FunctionDecl};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub closure: Rc<RefCell<Environment>>,
    pub declaration: FunctionDecl,
}

impl LoxFunction {
    pub fn new(declaration: FunctionDecl, closure: Rc<RefCell<Environment>>) -> Self {
        Self { declaration,closure }
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

        // how do i create a whole new enviroment without mutating the original environment!!
        // but is this the solution to create with each function call a new env?
        // the solution is with recursion to keep track of the levels deep we are?
        // how i don't know
        // should i start with recursion in the parent block?


        // on master branch try to make a deep copy of the environment 
        // or use std;;mem replace 
        // store current env somewhere else
        //
        //
        // or should i create a reset function in here
        // like in execute block
        // we take copy of the current env
        // execute the function call
        // reset to previous env settings after settinf function call
        //
        // DEBUG tomorrow on master and see what happens i need to have a look into Environment and
        // its variables that's the only way!! I can confirm my theory of what is going wrong
        let mut scoped_interpreter = interpreter.fork(Rc::clone(&self.closure));
        for (parameter, value) in self.declaration.parameters.iter().zip(args.iter()) {
          
            println!("define {} {}", &parameter.lexeme, value);
            scoped_interpreter.environment.borrow_mut().define(&parameter.lexeme, value.clone());
        }

let env = scoped_interpreter.environment.borrow_mut().clone();

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
