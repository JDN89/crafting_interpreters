use crate::expr::Expr;
use crate::lox_error::LoxError;
use crate::token:: Token;
use std::fmt;

#[derive(Debug)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &dyn StmtVisitor<R>) -> Result<R, LoxError> {
        match self {
            Stmt::Expression(stmt) => visitor.visit_expression(&stmt),
            Stmt::Print(stmt) => visitor.visit_print(&stmt),
            Stmt::Var(stmt) => visitor.visit_var(&stmt),
        }
    }
}


impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expression(stmt) => write!(f, "{}", stmt),
            Stmt::Print(stmt) => write!(f, "{}", stmt),
            Stmt::Var(stmt) => write!(f, "{}", stmt),
        }
    }
}


#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

impl fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expression: {}", self.expression)
    }
}
#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Expr,
}
impl fmt::Display for PrintStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Print: {}", self.expression)
    }
}
#[derive(Debug)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}
impl fmt::Display for VarStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.initializer {
            Some(expr) => write!(f, "Var: {} = {}", self.name.lexeme, expr),
            None => write!(f, "Var: {}", self.name.lexeme),
        }
    }
}
pub trait StmtVisitor<R> {
    fn visit_expression(&self, stmt: &ExpressionStmt) -> Result<R, LoxError>;
    fn visit_print(&self, stmt: &PrintStmt) -> Result<R, LoxError>;
    fn visit_var(&self, stmt: &VarStmt) -> Result<R, LoxError>;
}
