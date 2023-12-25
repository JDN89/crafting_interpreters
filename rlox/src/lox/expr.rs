use crate::lox_error::LoxError;
use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, LoxError> {
        match self {
            Expr::Assign(expr) => visitor.visit_assign(&expr),
            Expr::Binary(expr) => visitor.visit_binary(&expr),
            Expr::Grouping(expr) => visitor.visit_grouping(&expr),
            Expr::Literal(expr) => visitor.visit_literal(&expr),
            Expr::Unary(expr) => visitor.visit_unary(&expr),
            Expr::Variable(expr) => visitor.visit_variable(&expr),
        }
    }
}
#[derive(Debug)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Literal,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct VariableExpr {
    pub name: Token,
}

pub trait ExprVisitor<R> {
    fn visit_assign(&self, expr: &AssignExpr) -> Result<R, LoxError>;
    fn visit_binary(&self, expr: &BinaryExpr) -> Result<R, LoxError>;
    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<R, LoxError>;
    fn visit_literal(&self, expr: &LiteralExpr) -> Result<R, LoxError>;
    fn visit_unary(&self, expr: &UnaryExpr) -> Result<R, LoxError>;
    fn visit_variable(&self, expr: &VariableExpr) -> Result<R, LoxError>;
}
