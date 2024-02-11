use super::token::Literal;
use crate::{tree_walker::interpreter::Interpreter, RuntimeError};

pub trait LoxCallable {
    fn call(self, interpreter: &Interpreter, args: Vec<Literal>) -> Result<Literal, RuntimeError>;
}
