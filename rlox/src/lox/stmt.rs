use crate::expr::Expr;
use crate::lox_error::LoxError;
use crate::token::Token;

#[derive(Debug)]
pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &dyn StmtVisitor<R>) -> Result<R, LoxError> {
        match self {
            Stmt::Block(stmt) => visitor.visit_block(&stmt),
            Stmt::Expression(stmt) => visitor.visit_expression(&stmt),
            Stmt::Print(stmt) => visitor.visit_print(&stmt),
            Stmt::Var(stmt) => visitor.visit_var(&stmt),
        }
    }
}

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Expr,
}
#[derive(Debug)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait StmtVisitor<R> {
    fn visit_block(&self, stmt: &BlockStmt) -> Result<R, LoxError>;
    fn visit_expression(&self, stmt: &ExpressionStmt) -> Result<R, LoxError>;
    fn visit_print(&self, stmt: &PrintStmt) -> Result<R, LoxError>;
    fn visit_var(&self, stmt: &VarStmt) -> Result<R, LoxError>;
}
