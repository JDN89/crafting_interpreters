use crate::frontend::lox_callable::LoxCallable;
use std::rc::Rc;
use std::{
    cell::RefCell,
    io::{Cursor, Write},
};

use crate::frontend::lox_value::LoxValue;
use crate::frontend::token_type::TokenType;
use crate::tree_walker::environment::Environment;
use crate::{InterpreterError, LoxError, RuntimeError};

use super::lox_function::LoxFunction;
use super::parser::{Expr, Stmt};

// TODO: read about lifetimes and anonymous lifetimes!!

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    // We store env as a field directly in Interpreter so that the variables stay in memory as long as the interpreter is still running.
    environment: Rc<RefCell<Environment>>,
    //Write to an in memory buffer to test our interpreter:
    output_buffer: RefCell<Cursor<Vec<u8>>>,
}

// We rely on this helper method that sends the expression back into the interpreter's visitor
// pattern
impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        // define the clock
        //env.define(Clock.name().into(), Value::Callable(Rc::new(Clock)));

        Interpreter {
            globals: Rc::clone(&globals),
            environment: Rc::clone(&globals), // Corrected line
            output_buffer: RefCell::new(Cursor::new(Vec::new())),
        }
    }

    pub fn write_to_buffer(&self, text: &str) {
        let mut buffer = self.output_buffer.borrow_mut();
        buffer.write_all(text.as_bytes()).unwrap();
    }

    pub fn get_outpout(&self) -> Vec<u8> {
        self.output_buffer.borrow().get_ref().clone()
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(&statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Block(stmt) => {
                let _block = self.execute_block(
                    &stmt.statements,
                    // create a pointer to the current env
                    Environment::new_inner_environment(Rc::clone(&self.environment)),
                );
                Ok(())
            }
            Stmt::Expression(stmt) => {
                let _expr = self.evaluate_expression(&stmt.expression)?;
                Ok(())
            }
            Stmt::Function(fun) => {
                let function = LoxFunction::new(fun.clone());
                self.environment
                    .borrow_mut()
                    .define(&fun.name.lexeme, LoxValue::Function(Rc::new(function)));

                Ok(())
            }
            Stmt::Print(stmt) => {
                let value = self.evaluate_expression(&stmt.expression)?;
                println!("{:?}", value);
                // write to buffer so you get the output of the buffer for testing
                self.write_to_buffer(&value.as_str());
                Ok(())
            }
            Stmt::Var(stmt) => {
                let value: LoxValue;
                match &stmt.initializer {
                    Some(expression) => value = self.evaluate_expression(&expression)?,
                    None => value = LoxValue::Nil,
                }
                self.environment
                    .borrow_mut()
                    .define(&stmt.name.lexeme, value);
                Ok(())
            }
            Stmt::If(stmt) => {
                let evaluate_if_condition = self.evaluate_expression(&stmt.condition)?;

                if self.is_truthy(&evaluate_if_condition) {
                    self.execute(&stmt.then_branch)
                } else if let Some(else_statement) = &stmt.else_branch {
                    self.execute(else_statement)
                } else {
                    Ok(())
                }
            }
            Stmt::While(stmt) => {
                while {
                    let condition_result = self.evaluate_expression(&stmt.condition)?;

                    self.is_truthy(&condition_result)
                } {
                    let _ = self.execute(&stmt.body);
                }
                Ok(())
            }
            Stmt::Return(stmt) => {
                if let Some(value) = stmt.value.clone() {
                    Err(LoxError::Return(self.evaluate_expression(&value)?))
                } else {
                    Err(LoxError::Return(LoxValue::Nil))
                }
            }
        }

        // return statement.accept(self);
    }

    // we create a new env for blocks scope and pass it to this funciton
    // When i reset the code to the old env i was not keeping up to date with the previous
    // enviroment and this was becasue I was creating a new object via mem::replace so I was no
    // referring to the same enviroment
    // code that messed everyitng up!
    // let previous = std::mem::replace(&mut *self.environment, *Box::new(env));
    // TODO: write in learned and look up details of std::mem::replace!
    pub fn execute_block(&mut self, statements: &[Stmt], env: Environment) -> Result<(), LoxError> {
        // crate a pointer to the parrent env
        let parent_env = self.environment.clone();
        // new env that holds previous env as an enclosing field (BOX ENV)

        {
            self.environment = Rc::new(RefCell::new(env));
            for stmt in statements {
                self.execute(stmt)?;
            }
        }

        self.environment = parent_env;
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: &Expr) -> Result<LoxValue, LoxError> {
        match expression {
            Expr::Assign(expr) => {
                let value = self.evaluate_expression(&expr.value)?;
                self.environment.borrow_mut().assign(&expr.name, &value)?;
                Ok(value)
            }
            // OR and first is truthy return left
            // AND and first is false return left
            Expr::Logical(expr) => {
                let left = self.evaluate_expression(&expr.left)?;
                if expr.operator.token_type == TokenType::Or {
                    if self.is_truthy(&left) {
                        return Ok(left);
                    }
                }
                // Logical AND operator if operand is false return left otherwise return right
                else {
                    if !self.is_truthy(&left) {
                        return Ok(left);
                    }
                }
                let right = self.evaluate_expression(&expr.right)?;
                Ok(right)
            }
            Expr::Binary(expr) => {
                let left = self.evaluate_expression(&expr.left)?;
                let right = self.evaluate_expression(&expr.right)?;

                match (&left, &right) {
                    (LoxValue::Integer(left_value), LoxValue::Integer(right_value)) => {
                        match expr.operator.token_type {
                            TokenType::Minus => Ok(LoxValue::Integer(left_value - right_value)),
                            TokenType::Slash => Ok(LoxValue::Integer(left_value / right_value)),
                            TokenType::Star => Ok(LoxValue::Integer(left_value * right_value)),
                            TokenType::Plus => Ok(LoxValue::Integer(left_value + right_value)),
                            TokenType::Greater => Ok(LoxValue::Boolean(left_value > right_value)),
                            TokenType::GreaterEqual => {
                                Ok(LoxValue::Boolean(left_value >= right_value))
                            }
                            TokenType::Less => Ok(LoxValue::Boolean(left_value < right_value)),
                            TokenType::LessEqual => {
                                Ok(LoxValue::Boolean(left_value <= right_value))
                            }
                            TokenType::EqualEqual => {
                                Ok(LoxValue::Boolean(left_value == right_value))
                            }
                            TokenType::BangEqual => {
                                Ok(LoxValue::Boolean(left_value != right_value))
                            }
                            _ => Err(self.create_interpreter_error(
                                expr.operator.line,
                                &expr.operator.token_type,
                                left,
                                right,
                            )),
                        }
                    }

                    (LoxValue::String(left_value), LoxValue::String(right_value)) => {
                        let mut left_value = left_value.clone();
                        match expr.operator.token_type {
                            TokenType::Plus => {
                                left_value.push_str(&right_value);
                                Ok(LoxValue::String(left_value.to_string()))
                            }
                            TokenType::EqualEqual => Ok(LoxValue::Boolean(left == right)),
                            TokenType::BangEqual => Ok(LoxValue::Boolean(left != right)),
                            _ => Err(self.create_interpreter_error(
                                expr.operator.line,
                                &expr.operator.token_type,
                                left,
                                right,
                            )),
                        }
                    }

                    (LoxValue::Nil, LoxValue::Nil) => match expr.operator.token_type {
                        TokenType::EqualEqual => Ok(LoxValue::Boolean(true)),
                        _ => Err(self.create_interpreter_error(
                            expr.operator.line,
                            &expr.operator.token_type,
                            left,
                            right,
                        )),
                    },
                    (LoxValue::Nil, _) | (_, LoxValue::Nil) => match expr.operator.token_type {
                        TokenType::EqualEqual => Ok(LoxValue::Boolean(false)),
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
            Expr::Grouping(expr) => self.evaluate_expression(&expr.expression),
            Expr::Literal(expr) => Ok(expr.value.clone()),
            Expr::Unary(expr) => {
                // first evauluate the operand subexpression before we evaluate the unary operator
                // recursevly walk the AST
                let right = self.evaluate_expression(&expr.right)?;

                if expr.operator.token_type == TokenType::Minus {
                    if let LoxValue::Integer(number) = right {
                        return Ok(LoxValue::Integer(-number));
                    } else {
                        return Err(LoxError::Interpreter(InterpreterError::throw(
                            expr.operator.line,
                            format!("Operand: {:?} must be a number", right),
                        )));
                    }
                } else if expr.operator.token_type == TokenType::Bang {
                    let bool = self.is_truthy(&right);
                    return Ok(LoxValue::Boolean(bool));
                }
                // unreachable
                Ok(LoxValue::Nil)
            }
            Expr::Variable(expr) => Ok(self
                .environment
                .borrow_mut()
                .get_literal(&expr.name)?
                .clone()),
            Expr::Call(expr) => {
                let callee = self.evaluate_expression(&expr.callee)?;

                let mut arguments: Vec<LoxValue> = Vec::with_capacity(expr.arguments.len());
                for expr in &expr.arguments {
                    arguments.push(self.evaluate_expression(&expr)?);
                }

                println!("args: {:?}", arguments);

                // println!("args:  {:?}", arguments);
                match callee {
                    LoxValue::Function(callee) => {
                        let n_arguments = arguments.len();
                        if callee.arity() != n_arguments {
                            return Err(LoxError::Runtime(RuntimeError::arity_mismatch(
                                callee.arity(),
                                n_arguments,
                            )));
                        }

                        Ok(callee.call(self, arguments)?)
                    }
                    LoxValue::String(_)
                    | LoxValue::Integer(_)
                    | LoxValue::Boolean(_)
                    | LoxValue::Nil => Err(LoxError::Runtime(RuntimeError::throw(format!(
                        "Can only call functions and classes: {:?}",
                        expr.paren
                    )))),
                }
            }
        }

        // return expression.accept(self);
    }
    fn is_truthy(&mut self, right: &LoxValue) -> bool {
        match right {
            LoxValue::Nil | LoxValue::Boolean(false) => false,
            _ => true,
        }
    }

    fn create_interpreter_error(
        &self,
        location: usize,
        token_type: &TokenType,
        left: LoxValue,
        right: LoxValue,
    ) -> LoxError {
        LoxError::Interpreter(InterpreterError::throw(
            location,
            format!(
                "Execution of {:?} operator, is not supporterd for values: {}, {}",
                token_type, left, right
            ),
        ))
    }
}
