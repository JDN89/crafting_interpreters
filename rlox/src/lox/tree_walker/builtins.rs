use chrono::offset::Utc;

use crate::frontend::{lox_callable::LoxCallable, lox_value::LoxValue};

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
        _: Vec<LoxValue>,
    ) -> Result<LoxValue, crate::LoxError> {
        return Ok(LoxValue::Integer(Utc::now().timestamp() as f64));
    }
}
