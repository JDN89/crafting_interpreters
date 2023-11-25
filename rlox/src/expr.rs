use crate::literal::Literal;
use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

pub struct LiteralExpr {
    literal: Literal,
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub trait ExprVisitor<R> {
    fn visit_binary(&self, exprexpr: &Binary) -> Result<R, LoxError>;
    fn visit_grouping(&self, exprexpr: &Grouping) -> Result<R, LoxError>;
    fn visit_literal(&self, exprexpr: &Literal) -> Result<R, LoxError>;
    fn visit_unary(&self, exprexpr: &Unary) -> Result<R, LoxError>;
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
