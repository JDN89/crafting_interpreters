use crate::token::Token;
use crate::literal::Literal;

#[derive(Debug)]
pub enum expr {
    Binary(Box<Expr> left, Token operator, Box<Expr> right),
    Grouping(Box<Expr> expression),
    Literal(Literal literal),
    Unary(Token operator, Box<Expr> right),
}

pub struct Binary {
    pub fn new(Box<Expr> left, Token operator, Box<Expr> right) -> Self {
        left: Box<Expr> left,
        operator: Token operator,
        right: Box<Expr> right,
        Binary { Box<Expr> left, Token operator, Box<Expr> right }
    }

    pub fn accept<R>(&self, visitor: &dyn exprVisitor<R>) -> Result<R, LoxError> {
        visitor.visit_binary(self)
    }

    pub Box<Expr> left,
    pub Token operator,
    pub Box<Expr> right,
}

pub struct Grouping {
    pub fn new(Box<Expr> expression) -> Self {
        expression: Box<Expr> expression,
        Grouping { Box<Expr> expression }
    }

    pub fn accept<R>(&self, visitor: &dyn exprVisitor<R>) -> Result<R, LoxError> {
        visitor.visit_grouping(self)
    }

    pub Box<Expr> expression,
}

pub struct Literal {
    pub fn new(Literal literal) -> Self {
        literal: Literal literal,
        Literal { Literal literal }
    }

    pub fn accept<R>(&self, visitor: &dyn exprVisitor<R>) -> Result<R, LoxError> {
        visitor.visit_literal(self)
    }

    pub Literal literal,
}

pub struct Unary {
    pub fn new(Token operator, Box<Expr> right) -> Self {
        operator: Token operator,
        right: Box<Expr> right,
        Unary { Token operator, Box<Expr> right }
    }

    pub fn accept<R>(&self, visitor: &dyn exprVisitor<R>) -> Result<R, LoxError> {
        visitor.visit_unary(self)
    }

    pub Token operator,
    pub Box<Expr> right,
}

pub trait exprVisitor<R> {
    fn visit_binary(&self, exprexpr: &Binary) -> Result<R, LoxError>;
    fn visit_grouping(&self, exprexpr: &Grouping) -> Result<R, LoxError>;
    fn visit_literal(&self, exprexpr: &Literal) -> Result<R, LoxError>;
    fn visit_unary(&self, exprexpr: &Unary) -> Result<R, LoxError>;
}
