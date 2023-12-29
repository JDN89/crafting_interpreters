use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::stmt::StmtVisitor;
use crate::token_type::TokenType;
use crate::{expr::*, token::*};
use crate::{InterpreterError, LoxError};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Interpreter {
    // We store env as a field directly in Interpreter so that the variables stay in memory as long as the interpreter is still running.
    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T> to fix
    // BorrowMutError
    environment: Rc< RefCell<Environment> >,
}

// We rely on this helper method that sends the expression back into the interpreter's visitor
// pattern
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new( Environment::new())),
        }
    }
   

    pub fn interpret(&self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(&statement)?;
        }
        return Ok(());
    }

    fn execute(&self, statement: &Stmt) -> Result<(), LoxError> {
        return statement.accept(self);
    }

    // we create a new env for blocks scope and pass it to this funciton
    fn execute_block(&self, statements: &[Stmt], env: Environment) -> Result<(), LoxError> {
        //outer environment
    let previous = self.environment.replace(env);
            //set inner environment
            for stmt in statements {
                self.execute(stmt)?;
            }

        //reset outer environment
        let _  = self.environment.replace(previous);
        Ok(())
    }

    fn evaluate(&self, expression: &Expr) -> Result<Literal, LoxError> {
        return expression.accept(self);
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

// Statements produce no values
impl StmtVisitor<()> for Interpreter {
    fn visit_expression(&self, stmt: &crate::stmt::ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        return Ok(());
    }

    fn visit_print(&self, stmt: &crate::stmt::PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var(&self, stmt: &crate::stmt::VarStmt) -> Result<(), LoxError> {
        let value: Literal;
        match &stmt.initializer {
            Some(expression) => value = self.evaluate(&expression)?,
            None => value = Literal::Nil,
        }
        self.environment
            .borrow_mut()
            .define(&stmt.name.lexeme, value);
        Ok(())
    }

    fn visit_block(&self, stmt: &crate::stmt::BlockStmt) -> Result<(), LoxError> {
        let _ = self.execute_block(
            &stmt.statements,
            Environment::new_inner_environment(Rc::clone(&self.environment)),
        );
        Ok(())
    }
}

#[allow(dead_code, unused_variables)]
impl ExprVisitor<Literal> for Interpreter {
    fn visit_assign(&self, expr: &AssignExpr) -> Result<Literal, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.borrow_mut().assign(&expr.name, &value)?;
        return Ok(value);
    }

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

    fn visit_variable(&self, expr: &VariableExpr) -> Result<Literal, LoxError> {
        return Ok(self
            .environment
            .borrow_mut()
            .get_literal(&expr.name)?
            .clone());
    }
}

#[test]
fn test_bang_equals() {
    let interpreter = Interpreter::new();
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
    let interpreter = Interpreter::new();
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
    let interpreter = Interpreter::new();
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
    let interpreter = Interpreter::new();
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
    let interpreter = Interpreter::new();
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
    let interpreter = Interpreter::new();
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
