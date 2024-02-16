use core::fmt;

use super::lox_value::{Function, LoxValue};
use crate::{tree_walker::interpreter::Interpreter, LoxError};

pub trait LoxCallable: fmt::Debug {
    fn arity(&self) -> u8;
    fn call(self, interpreter: &Interpreter, args: Vec<LoxValue>) -> Result<LoxValue, LoxError>;
}
impl fmt::Display for dyn LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn >",)
    }
}

impl LoxCallable for Function {
    fn call(self, interpreter: &Interpreter, args: Vec<LoxValue>) -> Result<LoxValue, LoxError> {
        todo!()
    }

    fn arity(&self) -> u8 {
        todo!()
    }
}
