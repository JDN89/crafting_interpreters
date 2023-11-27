use std::fmt::Display;

use crate::lox_error::LoxError;
use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, LoxError> {
        match self {
            Expr::Binary(expr) => visitor.visit_binary(&expr),
            Expr::Grouping(expr) => visitor.visit_grouping(&expr),
            Expr::Literal(expr) => visitor.visit_literal(&expr),
            Expr::Unary(expr) => visitor.visit_unary(&expr),
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>, // left can be: literal/ unary/ grouping/ binary
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: Literal,
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Integer(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<R> {
    fn visit_binary(&self, expr: &BinaryExpr) -> Result<R, LoxError>;
    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<R, LoxError>;
    fn visit_literal(&self, expr: &LiteralExpr) -> Result<R, LoxError>;
    fn visit_unary(&self, expr: &UnaryExpr) -> Result<R, LoxError>;
}
