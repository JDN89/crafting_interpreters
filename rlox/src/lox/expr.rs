use crate::lox_error::LoxError;
use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    expression: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    value: Literal,
}

#[derive(Debug)]
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub trait ExprVisitor<R> {
    fn visit_binary(&self, expr: &BinaryExpr) -> Result<R, LoxError>;
    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<R, LoxError>;
    fn visit_literal(&self, expr: &LiteralExpr) -> Result<R, LoxError>;
    fn visit_unary(&self, expr: &UnaryExpr) -> Result<R, LoxError>;
}
impl BinaryExpr {
    pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, LoxError> {
        visitor.visit_binary(self)
    }
}

impl GroupingExpr {
    pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, LoxError> {
        visitor.visit_grouping(self)
    }
}

impl LiteralExpr {
    pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, LoxError> {
        visitor.visit_literal(self)
    }
}

impl UnaryExpr {
    pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, LoxError> {
        visitor.visit_unary(self)
    }
}
