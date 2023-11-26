use crate::{expr::*, LoxError};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expression: &Expr) -> Result<String, LoxError> {
        expression.accept(self)
    }
    pub fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();
        builder.push_str("(");
        builder.push_str(&name);
        for ex in exprs {
            builder.push_str(" ");
            builder.push_str(&ex.accept(self).unwrap());
        }
        builder.push_str(")");
        builder
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        Ok(self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]))
    }

    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<String, crate::LoxError> {
        Ok(self.parenthesize("group", &[&expr.expression]))
    }

    fn visit_literal(&self, expr: &LiteralExpr) -> Result<String, crate::LoxError> {
        Ok(expr.value.to_string())
    }

    fn visit_unary(&self, expr: &UnaryExpr) -> Result<String, crate::LoxError> {
        Ok(self.parenthesize(&expr.operator.lexeme, &[&expr.right]))
    }
}
