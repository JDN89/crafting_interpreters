use core::fmt;
use std::fmt::Display;
use std::rc::Rc;

use crate::tree_walker::lox_function::LoxFunction;

#[derive(Debug, Clone)]
pub enum LoxValue {
    String(String),
    Integer(f64),
    Boolean(bool),
    Function(Rc<LoxFunction>),
    Nil,
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxValue::String(s) => s.fmt(f),
            LoxValue::Integer(num) => num.fmt(f),
            LoxValue::Boolean(b) => b.fmt(f),
            LoxValue::Nil => write!(f, "`nil`"),
            LoxValue::Function(_) => todo!(),
        }
    }
}

impl LoxValue {
    pub fn as_str(&self) -> String {
        match self {
            LoxValue::String(s) => s.clone(),
            LoxValue::Integer(num) => num.to_string(),
            LoxValue::Boolean(b) => b.to_string(),
            LoxValue::Nil => String::from("nil"),
            LoxValue::Function(_) => todo!(),
        }
    }
}

// Implement custom equality impl because equality for lox is laxer than equality for rust and we
// can have nill types
impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Nil, Self::Nil) => true,
            (Self::Nil, _) => false,
            (_, Self::Nil) => false,

            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
