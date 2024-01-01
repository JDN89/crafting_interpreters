use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::token_type::TokenType;
use crate::{expr::*, token::*};
use crate::{InterpreterError, LoxError};

#[derive(Debug, Clone)]
pub struct Interpreter {
    // We store env as a field directly in Interpreter so that the variables stay in memory as long as the interpreter is still running.
    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T> to fix
    // BorrowMutError
    environment: Box<Environment>,
}

// We rely on this helper method that sends the expression back into the interpreter's visitor
// pattern
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Box::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(&statement)?;
        }
        return Ok(());
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Block(stmt) => {
                let _ = self.execute_block(
                    &stmt.statements,
                    Environment::new_inner_environment(&self.environment.as_ref()),
                );
                Ok(())
            }
            Stmt::Expression(stmt) => {
                self.evaluate_expression(&stmt.expression)?;
                return Ok(());
            }
            Stmt::Print(stmt) => {
                let value = self.evaluate_expression(&stmt.expression)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var(stmt) => {
                let value: Literal;
                match &stmt.initializer {
                    Some(expression) => value = self.evaluate_expression(&expression)?,
                    None => value = Literal::Nil,
                }
                self.environment.define(&stmt.name.lexeme, value);
                Ok(())
            }
        }

        // return statement.accept(self);
    }

    // we create a new env for blocks scope and pass it to this funciton
    fn execute_block(&mut self, statements: &[Stmt], env: Environment) -> Result<(), LoxError> {
        //outer environment
        let previous = &self.environment.clone();
        self.environment = Box::new(env);
        //set inner environment
        for stmt in statements {
            self.execute(stmt)?;
        }

        //reset outer environment
        self.environment = previous.clone();
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: &Expr) -> Result<Literal, LoxError> {
        match expression {
            Expr::Assign(expr) => {
                let value = self.evaluate_expression(&expr.value)?;
                self.environment.assign(&expr.name, &value)?;
                return Ok(value);
            }
            Expr::Binary(expr) => {
                let left = self.evaluate_expression(&expr.left)?;
                let right = self.evaluate_expression(&expr.right)?;

                match (&left, &right) {
                    (Literal::Integer(left_value), Literal::Integer(right_value)) => {
                        match expr.operator.token_type {
                            TokenType::Minus => Ok(Literal::Integer(left_value - right_value)),
                            TokenType::Slash => Ok(Literal::Integer(left_value / right_value)),
                            TokenType::Star => Ok(Literal::Integer(left_value * right_value)),
                            TokenType::Plus => Ok(Literal::Integer(left_value + right_value)),
                            TokenType::Greater => Ok(Literal::Boolean(left_value > right_value)),
                            TokenType::GreaterEqual => {
                                Ok(Literal::Boolean(left_value >= right_value))
                            }
                            TokenType::Less => Ok(Literal::Boolean(left_value < right_value)),
                            TokenType::LessEqual => Ok(Literal::Boolean(left_value <= right_value)),
                            TokenType::EqualEqual => {
                                Ok(Literal::Boolean(left_value == right_value))
                            }
                            TokenType::BangEqual => Ok(Literal::Boolean(left_value != right_value)),
                            _ => Err(self.create_interpreter_error(
                                expr.operator.line,
                                &expr.operator.token_type,
                                left,
                                right,
                            )),
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
                            _ => Err(self.create_interpreter_error(
                                expr.operator.line,
                                &expr.operator.token_type,
                                left,
                                right,
                            )),
                        }
                    }

                    (Literal::Nil, Literal::Nil) => match expr.operator.token_type {
                        TokenType::EqualEqual => Ok(Literal::Boolean(true)),
                        _ => Err(self.create_interpreter_error(
                            expr.operator.line,
                            &expr.operator.token_type,
                            left,
                            right,
                        )),
                    },
                    (Literal::Nil, _) | (_, Literal::Nil) => match expr.operator.token_type {
                        TokenType::EqualEqual => Ok(Literal::Boolean(false)),
                        _ => Err(self.create_interpreter_error(
                            expr.operator.line,
                            &expr.operator.token_type,
                            left,
                            right,
                        )),
                    },
                    _ => Err(self.create_interpreter_error(
                        expr.operator.line,
                        &expr.operator.token_type,
                        left,
                        right,
                    )),
                }
            }
            Expr::Grouping(expr) => {
                return self.evaluate_expression(&expr.expression);
            }
            Expr::Literal(expr) => {
                return Ok(expr.value.clone());
            }
            Expr::Unary(expr) => {
                // first evauluate the operand subexpression before we evaluate the unary operator
                // recursevly walk the AST
                let right = self.evaluate_expression(&expr.right)?;

                if expr.operator.token_type == TokenType::Minus {
                    if let Literal::Integer(number) = right {
                        return Ok(Literal::Integer(-number));
                    } else {
                        return Err(LoxError::Interpreter(InterpreterError::throw(
                            expr.operator.line,
                            format!("Operand: {:?} must be a number", right),
                        )));
                    }
                } else if expr.operator.token_type == TokenType::Bang {
                    let bool = self.is_truthy(right);
                    return Ok(Literal::Boolean(bool));
                }
                // unreachable
                return Ok(Literal::Nil);
            }
            Expr::Variable(expr) => {
                return Ok(self.environment.get_literal(&expr.name)?.clone());
            }
        }

        // return expression.accept(self);
    }
    fn is_truthy(&self, right: Literal) -> bool {
        match right {
            Literal::Nil | Literal::Boolean(false) => false,
            _ => true,
        }
    }

    fn create_interpreter_error(
        &self,
        location: usize,
        token_type: &TokenType,
        left: Literal,
        right: Literal,
    ) -> LoxError {
        return LoxError::Interpreter(InterpreterError::throw(
            location,
            format!(
                "Execution of {:?} operator, is not supporterd for values: {}, {}",
                token_type, left, right
            ),
        ));
    }
}

#[test]
fn test_bang_equals() {
    let mut interpreter = Interpreter::new();
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
    let result = interpreter.evaluate_expression(&Expr::Binary(bin_exp));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_equals_equals_integers() {
    let mut interpreter = Interpreter::new();
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
    let result = interpreter.evaluate_expression(&Expr::Binary(bin_exp));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_equals_equals_strings() {
    let mut interpreter = Interpreter::new();
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
    let result = interpreter.evaluate_expression(&Expr::Binary(bin_exp));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_bang_equals_strings() {
    let mut interpreter = Interpreter::new();
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
    let result = interpreter.evaluate_expression(&Expr::Binary(bin_exp));
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
    let result = interpreter.evaluate_expression(&Expr::Binary(bin_exp_equal_operands));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(false));
}

// I think object in the java code can be null but we create a token Literal nil in case of a null value in the source code.
#[test]
fn test_equals_equals_literal_nill() {
    let mut interpreter = Interpreter::new();
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
    let result = interpreter.evaluate_expression(&Expr::Binary(bin_exp));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(true));
}

#[test]
fn test_equals_equals_nil_and_operand() {
    let mut interpreter = Interpreter::new();
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
    let result = interpreter.evaluate_expression(&Expr::Binary(bin_exp));
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
    let result = interpreter.evaluate_expression(&Expr::Binary(bin_exp));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Literal::Boolean(false));
}
