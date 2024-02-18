use core::fmt;

use super::lox_value::LoxValue;
use crate::{tree_walker::interpreter::Interpreter, LoxError};

pub trait LoxCallable: fmt::Debug {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
    ) -> Result<LoxValue, LoxError>;
    fn name(&self) -> &str;
}
impl fmt::Display for dyn LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn >",)
    }
}
