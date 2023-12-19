use crate::expr::Expr;
use crate::lox_error::LoxError;

#[derive(Debug)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &dyn StmtVisitor<R>) -> Result<R, LoxError> {
        match self {
            Stmt::Expression(stmt) => visitor.visit_expression(&stmt),
            Stmt::Print(stmt) => visitor.visit_print(&stmt),
        }
    }
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Expr,
}

pub trait StmtVisitor<R> {
    fn visit_expression(&self, stmt: &ExpressionStmt) -> Result<R, LoxError>;
    fn visit_print(&self, stmt: &PrintStmt) -> Result<R, LoxError>;
}
