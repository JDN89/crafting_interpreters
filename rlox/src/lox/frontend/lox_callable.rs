use super::token::{Function, Literal};
use crate::{tree_walker::interpreter::Interpreter, LoxError};

pub trait LoxCallable {
    fn arity(&self) -> u8;
    fn call(self, interpreter: &Interpreter, args: Vec<Literal>) -> Result<Literal, LoxError>;
}

impl LoxCallable for Function {
    fn call(self, interpreter: &Interpreter, args: Vec<Literal>) -> Result<Literal, LoxError> {
        todo!()
    }

    fn arity(&self) -> u8 {
        todo!()
    }
}
