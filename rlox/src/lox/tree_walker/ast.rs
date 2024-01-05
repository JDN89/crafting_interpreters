use crate::frontend::token::{Literal, Token};

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
#[derive(Debug)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
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

