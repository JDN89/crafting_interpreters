use core::fmt;
use std::fmt::Display;
use std::rc::Rc;

use crate::tree_walker::lox_function::LoxFunction;
use crate::tree_walker::parser::FunctionDecl;

use super::lox_callable::LoxCallable;

#[derive(Debug, Clone)]
pub enum LoxValue {
    String(String),
    Integer(f64),
    Boolean(bool),
    // NOTE: use Rc instead of box? give it some thought
    Function(Box<LoxFunction>),
    Nil,
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxValue::String(s) => s.fmt(f),
            LoxValue::Integer(num) => num.fmt(f),
            LoxValue::Boolean(b) => b.fmt(f),
            LoxValue::Nil => write!(f, "`nil`"),
            LoxValue::Function(fun) => write!(f, "Funciont {:?}", fun),
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
            LoxValue::Function(fun) => String::from(fun.name()),
        }
    }

    pub fn get_callable(&self) -> Option<Box<LoxFunction>> {
        match *self {
            LoxValue::Function(ref func) => Some(func.clone()),
            _ => None,
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
            (LoxValue::String(_), LoxValue::Integer(_)) => todo!(),
            (LoxValue::String(_), LoxValue::Boolean(_)) => todo!(),
            (LoxValue::String(_), LoxValue::Function(_)) => todo!(),
            (LoxValue::Integer(_), LoxValue::String(_)) => todo!(),
            (LoxValue::Integer(_), LoxValue::Boolean(_)) => todo!(),
            (LoxValue::Integer(_), LoxValue::Function(_)) => todo!(),
            (LoxValue::Boolean(_), LoxValue::String(_)) => todo!(),
            (LoxValue::Boolean(_), LoxValue::Integer(_)) => todo!(),
            (LoxValue::Boolean(_), LoxValue::Function(_)) => todo!(),
            (LoxValue::Function(_), LoxValue::String(_)) => todo!(),
            (LoxValue::Function(_), LoxValue::Integer(_)) => todo!(),
            (LoxValue::Function(_), LoxValue::Boolean(_)) => todo!(),
            (LoxValue::Function(_), LoxValue::Function(_)) => todo!(),
            // _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
