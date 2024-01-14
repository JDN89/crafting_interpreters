use crate::frontend::token::{Literal, Token};

// https://doc.rust-lang.org/book/ch15-01-box.html
// A value of recursive type can have another value of the same type as part of itself. 
// Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up.
// However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs.
// Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

// STATEMENTS
#[derive(Debug)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Var(VarStmt),
    If(IfStmt),
    Print(PrintStmt),
    Block(BlockStmt),
    While(WhileStmt),
}

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub expr: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
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

// EXPRESSIONS

#[derive(Debug)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
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
pub struct LogicalExpr {
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
