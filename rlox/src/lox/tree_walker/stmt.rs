use crate::frontend::token::Token;
use crate::tree_walker::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
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
