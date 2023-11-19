// The subexpressions on either side of the operator are operands.
// We define a base class for expressions
// For each produciton un expression we create a sublass that has fields fo rthe nonterminals
// specific for that rule
//
/* expression      ::= literal | unary | binary | grouping ;
literal         ::= NUMBER | STRING | "true" | "false" | "nil" ;
grouping        ::= "(" expression ")" ;
unary           ::= ( "-" | "!" ) expression ;
binary          ::= expression operator expression ;
operator        ::= "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/" ; */
//
use crate::{
    lox_error::LoxError,
    token::{Literal, Token},
};

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

#[derive(Debug)]
pub struct GroupingExpr {
    expr: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    literal: Literal,
}

#[derive(Debug)]
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

// We use generic R, so the return type for the visitor can be flexible
pub trait ExprVisitor<R> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<R, LoxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<R, LoxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<R, LoxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<R, LoxError>;
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }
}

// create codeGeneration for the above code:
// 1) wirte yourself
// 2) via macros?

// Example implementation of ExprVisitor
// create the above AST objects and then we can add functionality by impl ExprVisitor
struct MyVisitor;

impl ExprVisitor<String> for MyVisitor {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        // Your implementation here
        Ok(format!("BinaryExpr: {:?}", expr))
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        // Your implementation here
        Ok(format!("GroupingExpr: {:?}", expr))
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        // Your implementation here
        Ok(format!("LiteralExpr: {:?}", expr))
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        // Your implementation here
        Ok(format!("UnaryExpr: {:?}", expr))
    }
}
