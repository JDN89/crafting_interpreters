use crate::RuntimeError;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::{expr::*, token::Literal};

#[derive(Debug)]
struct Interpreter {}

// impl interpret for Interpreter {
//     pub fn interpret(expr: Expr) {
//         match evaluate {
//             
//         }
//
//     }
//     
// }

#[allow(dead_code, unused_variables)]
impl ExprVisitor<Literal> for Interpreter {
    // we start with the arithimic operators and cover the other binary operators in a later
    // chapter

    fn visit_binary(&self, expr: &BinaryExpr) -> Result<Literal, RuntimeError> {
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
                    _ => Err( RuntimeError::throw(None,Some( expr.operator.token_type.clone()), "operator is not supported for Number values"))
                }
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
                    _ => Err( RuntimeError::throw(None,Some( expr.operator.token_type.clone()), "operator is not supported for String values"))
                }
            }
            (Literal::Nil, Literal::Nil) => match expr.operator.token_type {
                TokenType::EqualEqual => Ok(Literal::Boolean(true)),
                    _ => Err( RuntimeError::throw(None,Some( expr.operator.token_type.clone()), "operator is not supported for Nil values"))
            },
            (Literal::Nil, _) | (_, Literal::Nil) => match expr.operator.token_type {
                TokenType::EqualEqual => Ok(Literal::Boolean(false)),
                    _ => Err( RuntimeError::throw(None,Some( expr.operator.token_type.clone()), "operator is not supported for combination Nil and other operand"))
            },
            _ => Err(RuntimeError::throw(Some(vec![left,right]), None, "combination of operands is not supported in Lox"))

                
        }
    }

    // To evaluate the grouping expression itself, we recursively evaluate that subexpression and return it.
    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<Literal, RuntimeError> {
        return self.evaluate(&expr.expression);
    }

    fn visit_literal(&self, expr: &LiteralExpr) -> Result<Literal, RuntimeError> {
        return Ok(expr.value.clone());
    }

    fn visit_unary(&self, expr: &UnaryExpr) -> Result<Literal, RuntimeError> {
        // first evauluate the operand subexpression before we evaluate the unary operator
        // recursevly walk the AST
        let right = self.evaluate(&expr.right)?;

        if expr.operator.token_type == TokenType::Minus {
            if let Literal::Integer(number) = right {
                return Ok(Literal::Integer(-number));
            }
            else {
                RuntimeError::throw(Some( vec![right]),None, "Operand must be a number.");
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
    fn evaluate(&self, expression: &Box<Expr>) -> Result<Literal, RuntimeError> {
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
    let interpreter = Interpreter {};
    let bin_exp = BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Integer(123.00),
        })),
        operator: Token {
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
        operator: Token {
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
        operator: Token {
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
        operator: Token {
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
        operator: Token {
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
        operator: Token {
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
        operator: Token {
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
        operator: Token {
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
