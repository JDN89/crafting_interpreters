// use crate::{expr::*, LoxError};
//
// pub struct AstPrinter {}
//
// impl ExprVisitor<String> for AstPrinter {
//     fn visit_binary(&self, expr: &BinaryExpr) -> Result<String, crate::LoxError> {
//         self.parenthesize_bin
//     }
//
//     fn visit_grouping(&self, expr: &GroupingExpr) -> Result<String, crate::LoxError> {
//         todo!()
//     }
//
//     fn visit_literal(&self, expr: &LiteralExpr) -> Result<String, crate::LoxError> {
//         todo!()
//     }
//
//     fn visit_unary(&self, expr: &UnaryExpr) -> Result<String, crate::LoxError> {
//         todo!()
//     }
// }
//
// impl AstPrinter {
//     fn print(&self, expr: &Expr) -> Result<String, LoxError> {
//         expr.(self)
//     }
// fn parenthesize(name: String, exprs: &[&Expr]) -> String {
//     let mut builder = String::new();
//     builder.push_str("(");
//     builder.push_str(name);
//     for ex in exprs {
//         builder.push_str(" ")
//         builder.push_str(ex.acc)
//
//     }
// }
/* } */



