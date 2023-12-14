use crate::lox_error::LoxError;
use crate::token_type::*;
use crate::{expr::*, token::Literal};

#[derive(Debug)]
struct Interpreter {}

#[allow(dead_code, unused_variables)]
impl ExprVisitor<Literal> for Interpreter {
    // we start with the arithimic operators and cover the other binary operators in a later
    // chapter

    fn visit_binary(&self, expr: &BinaryExpr) -> Result<Literal, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => {
                if let (Literal::Integer(left_value), Literal::Integer(right_value)) = (left, right)
                {
                    let result = left_value - right_value;
                    return Ok(Literal::Integer(result));
                }
                else {
                    // return interpreter error
                    todo!()
                }
            }
            TokenType::Slash => {
                if let (Literal::Integer(left), Literal::Integer(right)) = (left,right) {
                    let devision = left / right;
                    return Ok(Literal::Integer(devision));
                }
                else {
                    todo!()
                }
            }
            TokenType::Star => {

                if let (Literal::Integer(left), Literal::Integer(right)) = (left,right) {
                    let result = left * right;
                    return Ok(Literal::Integer(result));
                }
                else {
                    todo!()
                }
            }
            // handle the other BinaryExpr operators later
            _ => todo!()
        }
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
