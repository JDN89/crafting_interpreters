use crate::frontend::{self, lox_callable::LoxCallable, token::Literal};

use chrono::offset::Utc;

use super::interpreter;

#[derive(Debug)]
pub struct Clock {}

impl LoxCallable for Clock {
    fn arity(&self) -> u8 {
        0
    }

    fn call(
        self,
        _: &interpreter::Interpreter,
        _: Vec<Literal>,
    ) -> Result<Literal, crate::LoxError> {
        return Ok(Literal::Integer(Utc::now().timestamp() as f64));
    }
}
