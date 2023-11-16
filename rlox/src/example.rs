// go over visitor pattern once more
// impl BinaryExpr
// impl GroupingExpr
//
// pub struct BinaryExpr
//
// trait visitor
//
//trait ExprVisitor<T> {
//fn visit_binanry_expr(&self, expr: &BinaryExpr) => Result<T,LoxError>;
//}
//the impl implement the traits
//
use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expr>, // Box<Expr>?
    operator: Token,
    right: Box<Expr>,
}
