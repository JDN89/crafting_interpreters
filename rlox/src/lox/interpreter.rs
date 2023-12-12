use crate::lox_error::LoxError;
use crate::{expr::*, token::Literal};
use crate::token_type::*;

#[derive(Debug)]
struct Interpreter {}

#[allow(dead_code, unused_variables)]
impl ExprVisitor<Literal> for Interpreter {

    fn visit_binary(&self, expr: &BinaryExpr) -> Result<Literal, LoxError> {
        todo!()
    }

    // To evaluate the grouping expression itself, we recursively evaluate that subexpression and return it.
    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<Literal, LoxError> {
        return self.evaluate(&expr.expression);
    }

    fn visit_literal(&self, expr: &LiteralExpr) -> Result<Literal, LoxError> {
        return Ok(expr.value.clone());
    }

    fn visit_unary(&self, expr: &UnaryExpr) -> Result<Literal, LoxError> {
        // first evauluate the operand subexpression before we evaluate the unary operator
        // recursevly walk the AST
        let right = self.evaluate(&expr.right)?;

        if expr.operator.token_type == TokenType::Minus {
            // in case of minus the subExpression must be a number
            let result = match right {
                Literal::Integer(number) => Ok(Literal::Integer(-number)),
                _ => Ok(Literal::Nil),
            };
            return result;
        }

        if expr.operator.token_type == TokenType::Bang {
            let bool = self.is_truthy(right);
            return Ok(Literal::Boolean(bool));
        }
        // unreachable
        return Ok(Literal::Nil);
    }
}

// We rely on this helper method that sends the expression back into the interpreter's visitor
// pattern
impl Interpreter {
    fn evaluate(&self, expression: &Box<Expr>) -> Result<Literal, LoxError> {
        return expression.accept(self);
    }
    fn is_truthy(&self, right: Literal) -> bool {
        match right {
            Literal::Nil => false,
            Literal::Boolean(false) => false,
            _ => true,
        }
    }
}
