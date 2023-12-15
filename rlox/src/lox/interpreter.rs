use crate::lox_error::LoxError;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::{expr::*, token::Literal};

#[derive(Debug)]
struct Interpreter {}

#[allow(dead_code, unused_variables)]
impl ExprVisitor<Literal> for Interpreter {
    // we start with the arithimic operators and cover the other binary operators in a later
    // chapter

    fn visit_binary(&self, expr: &BinaryExpr) -> Result<Literal, LoxError> {
        // evaluate the opareands before executing the Bin operator
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        if let (Literal::Integer(left_value), Literal::Integer(right_value)) = (&left, &right) {
            // toto impl partialOrd for Literal Struct
            match expr.operator.token_type {
                TokenType::Minus => Ok(Literal::Integer(left_value - right_value)),
                TokenType::Slash => Ok(Literal::Integer(left_value / right_value)),
                TokenType::Star => Ok(Literal::Integer(left_value * right_value)),
                TokenType::Plus => Ok(Literal::Integer(left_value + right_value)),
                TokenType::Greater => Ok(Literal::Boolean(left_value > right_value)),
                TokenType::GreaterEqual => Ok(Literal::Boolean(left_value >= right_value)),
                TokenType::Less => Ok(Literal::Boolean(left_value < right_value)),
                TokenType::LessEqual => Ok(Literal::Boolean(left_value <= right_value)),
                TokenType::EqualEqual => Ok(Literal::Boolean(left_value == right_value)),
                TokenType::BangEqual => Ok(Literal::Boolean(left_value != right_value)),
                // Handle other BinaryExpr operators later
                _ => todo!(),
            }
        } else if let (Literal::String(ref mut left_value), Literal::String(right_value)) =
            (left.clone(), &right)
        {
            match expr.operator.token_type {
                TokenType::Plus => {
                    left_value.push_str(&right_value);

                    return Ok(Literal::String(left_value.to_string()));
                }
                TokenType::EqualEqual => {
                    return Ok(Literal::Boolean(left == right))
                }
                _ => todo!(),
            }
        } else {
            return Ok(Literal::Nil);
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
            if let Literal::Integer(number) = right {
                return Ok(Literal::Integer(-number));
            }
        } else if expr.operator.token_type == TokenType::Bang {
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
            Literal::Nil | Literal::Boolean(false) => false,
            _ => true,
        }
    }
}

#[test]
fn test_bang_equals() {
    let interpreter = Interpreter{};
    let bin_exp = BinaryExpr{
       left: Box::new(Expr::Literal(LiteralExpr {value: Literal::Integer(123.00)  })),
        operator: Token { token_type: TokenType::BangEqual,
            lexeme: "!=".to_string(),
            literal: None, line: 123 },

       right: Box::new(Expr::Literal(LiteralExpr {value: Literal::Integer(124.00)  })),

    };
    let result = interpreter.visit_binary(&bin_exp) ;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}


#[test]
fn test_equals_equals() {
    let interpreter = Interpreter{};
    let bin_exp = BinaryExpr{
       left: Box::new(Expr::Literal(LiteralExpr {value: Literal::Integer(123.00)  })),
        operator: Token { token_type: TokenType::EqualEqual,
            lexeme: "==".to_string(),
            literal: None, line: 123 },

       right: Box::new(Expr::Literal(LiteralExpr {value: Literal::Integer(123.00)  })),

    };
    let result = interpreter.visit_binary(&bin_exp) ;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_equals_equals_strings() {
    let interpreter = Interpreter{};
    let bin_exp = BinaryExpr{
       left: Box::new(Expr::Literal(LiteralExpr {value: Literal::String("yolo".to_string())  })),
        operator: Token { token_type: TokenType::EqualEqual,
            lexeme: "==".to_string(),
            literal: None, line: 123 },

       right: Box::new(Expr::Literal(LiteralExpr {value: Literal::String("yolo".to_string())  })),

    };
    let result = interpreter.visit_binary(&bin_exp) ;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}
