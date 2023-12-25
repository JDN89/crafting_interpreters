use crate::lox_error::LoxError;
use crate::token::{Literal, Token};
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Assign(expr) => write!(f, "{}", expr),
            Expr::Binary(expr) => write!(f, "{}", expr),
            Expr::Grouping(expr) => write!(f, "{}", expr),
            Expr::Literal(expr) => write!(f, "{}", expr),
            Expr::Unary(expr) => write!(f, "{}", expr),
            Expr::Variable(expr) => write!(f, "{}", expr),
        }
    }
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

impl fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic for AssignExpr
        write!(f, "Assign({} = {})", self.name.lexeme, self.value)
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic for BinaryExpr
        write!(f, "   \n Binary : ( left: {} \n operator:  {}  \n right : {})", self.left, self.operator.lexeme, self.right)
    }
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

impl fmt::Display for GroupingExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic for GroupingExpr
        write!(f, " Grouping:  ({})", self.expression)
    }
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Literal,
}

impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic for LiteralExpr
        write!(f, "  Literal:  ({})", self.value)
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic for UnaryExpr
        write!(f, "  Unary: ({} {})", self.operator.lexeme, self.right)
    }
}

#[derive(Debug)]
pub struct VariableExpr {
    pub name: Token,
}

impl fmt::Display for VariableExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic for VariableExpr
        write!(f, "  \n  Variable:  ({})", self.name.lexeme)
    }
}

pub trait ExprVisitor<R> {
    fn visit_assign(&self, expr: &AssignExpr) -> Result<R, LoxError>;
    fn visit_binary(&self, expr: &BinaryExpr) -> Result<R, LoxError>;
    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<R, LoxError>;
    fn visit_literal(&self, expr: &LiteralExpr) -> Result<R, LoxError>;
    fn visit_unary(&self, expr: &UnaryExpr) -> Result<R, LoxError>;
    fn visit_variable(&self, expr: &VariableExpr) -> Result<R, LoxError>;
}
