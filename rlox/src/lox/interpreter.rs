use crate::token_type::TokenType;
use crate::{expr::*, token::Literal};
use crate::{token, InterpreterError, LoxError};

#[derive(Debug)]
pub struct Interpreter {}

// We rely on this helper method that sends the expression back into the interpreter's visitor
// pattern
impl Interpreter {
    pub fn interpret(&self, expression: &Box<Expr>) -> Result<(), LoxError> {
        match self.evaluate(expression) {
            Ok(value) => {
                // TODO: implement fmt for Literal so we can print the Literal values!!
                println!("{:?}", value);
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

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

#[allow(dead_code, unused_variables)]
impl ExprVisitor<Literal> for Interpreter {
    // we start with the arithimic operators and cover the other binary operators in a later
    // chapter

    fn visit_binary(&self, expr: &BinaryExpr) -> Result<Literal, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match (&left, &right) {
            (Literal::Integer(left_value), Literal::Integer(right_value)) => {
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
                    _ => Err(LoxError::Interpreter(InterpreterError::throw(
                        None,
                        Some(expr.operator.clone()),
                        "operator is not supported for Number values",
                    ))),
                }
            }
            (Literal::String(left_value), Literal::Integer(right_value)) => {
                return Err(LoxError::Interpreter(InterpreterError::throw(
                    Some(vec![Literal::String(left_value.to_string()), Literal::Integer(*right_value)]),
                    Some(expr.operator.clone()),
                    "operator is not supported for the combination of Integer and String values",
                )))
            }
            (Literal::Integer(left_value), Literal::String(right_value)) => {
                return Err(LoxError::Interpreter(InterpreterError::throw(
                    Some(vec![Literal::String(right_value.to_string()), Literal::Integer(*left_value)]),
                    Some(expr.operator.clone()),
                    "operator is not supported for the combination of Integer and String values",
                )))
            }

            (Literal::String(left_value), Literal::String(right_value)) => {
                let mut left_value = left_value.clone();
                match expr.operator.token_type {
                    TokenType::Plus => {
                        left_value.push_str(&right_value);
                        Ok(Literal::String(left_value.to_string()))
                    }
                    TokenType::EqualEqual => Ok(Literal::Boolean(left == right)),
                    TokenType::BangEqual => Ok(Literal::Boolean(left != right)),
                    _ => Err(LoxError::Interpreter(InterpreterError::throw(
                        None,
                        Some(expr.operator.clone()),
                        "operator is not supported for String values",
                    ))),
                }
            }
            (Literal::Nil, Literal::Nil) => match expr.operator.token_type {
                TokenType::EqualEqual => Ok(Literal::Boolean(true)),
                _ => Err(LoxError::Interpreter(InterpreterError::throw(
                    None,
                    Some(expr.operator.clone()),
                    "operator is not supported for Nil values",
                ))),
            },
            (Literal::Nil, _) | (_, Literal::Nil) => match expr.operator.token_type {
                TokenType::EqualEqual => Ok(Literal::Boolean(false)),
                _ => Err(LoxError::Interpreter(InterpreterError::throw(
                    None,
                    Some(expr.operator.clone()),
                    "operator is not supported for combination Nil and other operand",
                ))),
            },
            _ => Err(LoxError::Interpreter(InterpreterError::throw(
                Some(vec![left, right]),
                None,
                "combination of operands is not supported in Lox",
            ))),
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
            } else {
                InterpreterError::throw(Some(vec![right]), None, "Operand must be a number.");
            }
        } else if expr.operator.token_type == TokenType::Bang {
            let bool = self.is_truthy(right);
            return Ok(Literal::Boolean(bool));
        }
        // unreachable
        return Ok(Literal::Nil);
    }
}

#[test]
fn test_bang_equals() {
    let interpreter = Interpreter {};
    let bin_exp = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Integer(123.00),
        })),
        operator: crate::token::Token {
            token_type: TokenType::BangEqual,
            lexeme: "!=".to_string(),
            literal: None,
            line: 123,
        },

        right: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Integer(124.00),
        })),
    };
    let result = interpreter.visit_binary(&bin_exp);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_equals_equals_integers() {
    let interpreter = Interpreter {};
    let bin_exp = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Integer(123.00),
        })),
        operator: crate::token::Token {
            token_type: TokenType::EqualEqual,
            lexeme: "==".to_string(),
            literal: None,
            line: 123,
        },

        right: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Integer(123.00),
        })),
    };
    let result = interpreter.visit_binary(&bin_exp);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_equals_equals_strings() {
    let interpreter = Interpreter {};
    let bin_exp = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::String("yolo".to_string()),
        })),
        operator: crate::token::Token {
            token_type: TokenType::EqualEqual,
            lexeme: "==".to_string(),
            literal: None,
            line: 123,
        },

        right: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::String("yolo".to_string()),
        })),
    };
    let result = interpreter.visit_binary(&bin_exp);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_bang_equals_strings() {
    let interpreter = Interpreter {};
    let bin_exp = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::String("yolo".to_string()),
        })),
        operator: crate::token::Token {
            token_type: TokenType::BangEqual,
            lexeme: "!=".to_string(),
            literal: None,
            line: 123,
        },

        right: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::String("tralala".to_string()),
        })),
    };
    let result = interpreter.visit_binary(&bin_exp);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));

    let bin_exp_equal_operands = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::String("yolo".to_string()),
        })),
        operator: crate::token::Token {
            token_type: TokenType::BangEqual,
            lexeme: "!=".to_string(),
            literal: None,
            line: 123,
        },

        right: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::String("yolo".to_string()),
        })),
    };
    let result = interpreter.visit_binary(&bin_exp_equal_operands);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(false));
}

// I think object in the java code can be null but we create a token Literal nil in case of a null value in the source code.
#[test]
fn test_equals_equals_literal_nill() {
    let interpreter = Interpreter {};
    let bin_exp = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Nil,
        })),
        operator: token::Token {
            token_type: TokenType::EqualEqual,
            lexeme: "==".to_string(),
            literal: None,
            line: 123,
        },

        right: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Nil,
        })),
    };
    let result = interpreter.visit_binary(&bin_exp);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_equals_equals_nil_and_operand() {
    let interpreter = Interpreter {};
    let bin_exp = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Nil,
        })),
        operator: token::Token {
            token_type: TokenType::EqualEqual,
            lexeme: "==".to_string(),
            literal: None,
            line: 123,
        },

        right: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::String("yolo".to_string()),
        })),
    };
    let result = interpreter.visit_binary(&bin_exp);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(false));

    let bin_exp = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::String("yolo".to_string()),
        })),
        operator: token::Token {
            token_type: TokenType::EqualEqual,
            lexeme: "==".to_string(),
            literal: None,
            line: 123,
        },

        right: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Nil,
        })),
    };
    let result = interpreter.visit_binary(&bin_exp);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(false));
}
