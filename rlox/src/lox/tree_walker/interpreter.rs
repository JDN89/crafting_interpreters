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
    pub environment: Rc<RefCell<Environment>>,
    //Write to an in memory buffer to test our interpreter:
    output_buffer: Rc<RefCell<Cursor<Vec<u8>>>>,
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
            output_buffer: Rc::new(RefCell::new(Cursor::new(Vec::new()))),
        }
    }

    pub fn fork(&self, environment: Rc<RefCell<Environment>> ) -> Interpreter {
        Interpreter {
            globals: environment.clone(),
            environment:environment.clone(),
            output_buffer: Rc::clone(&self.output_buffer)

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
                let function = LoxFunction::new(fun.clone(),Rc::new(RefCell::new(self.environment.borrow().to_owned())));

        (*self.environment).borrow_mut().define(

                    // TODO: replace loxcallable by lox function
                    // cleanup dynamic dispatch en globals
                    // not going to use it and don't want to use it

                    &function.declaration.name.clone().lexeme,
                    LoxValue::Function(Box::new(function.clone())),
                );
                
                      // We need the function itself to exist in the environment it closes over,
                // otherwise recursion won't work.
                (*function.closure).borrow_mut().define(
                    &function.declaration.name.clone().lexeme,
                    LoxValue::Function(Box::new(function.clone())),
                );

                Ok(())
            }
            Stmt::Print(stmt) => {
                let value = self.evaluate_expression(&stmt.expression)?;
                println!("{:?}\n", value);
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

    // BUG: the issue is that with execute block we keep defining n in the function scope
    // but the scope is a ref to the outer scope -> so when we define in the inner scope we adjust
    // the outer scope!
    // or we retrieve anding a DropBomb initialized with the given message. This is returned from the function.
    // we define recursion mutiple times on the same environemnt
    // run code and check print statments -> when do we lewve execute block? 
    // if you run these print statements in the master branch
    // you'll see taht we get n from the parent
    // for example with fib(2)
    // environment {n : 2 inner {n: 0 inner {n -1}} }
    // we keep creating new inner environments with each functions calls on the master branch and
    // retrieving n from the previous scope! instead of retrieving it from the outer layer (parent
    // scope where it was first defined!
    // how do we keep track of the layers of scopes where we defined the variable the first time?)
    // 
    // now here we don't create a new inner environemnt and we keep overwriting the current env
    //
    // std mem replace in function call?
    pub fn execute_block(&mut self, statements: &[Stmt], env: Environment) -> Result<(), LoxError> {
        let parent_env = self.environment.clone();
        println!("enter execute block");
        {
            self.environment = Rc::new(RefCell::new(env));
            for stmt in statements {
                println!("execute statement");
                self.execute(stmt)?;
            }
        }

        println!("leaving execute block");
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
                Ok(LoxValue::Nil)
            }
            Expr::Variable(expr) => {
                let env_value = self.environment.borrow().get_literal(&expr.name)?.clone();
                println!("getting variable with value ");
                Ok(env_value)
            }
            Expr::Call(expr) => {
                let callable = self
                    .evaluate_expression(&expr.callee)?
                    .get_callable()
                    .ok_or_else(|| {
                        LoxError::Runtime(RuntimeError::throw(format!(
                            "not callable : {:?}",
                            expr.paren
                        )))
                    })?;

                let mut arguments: Vec<LoxValue> = Vec::with_capacity(expr.arguments.len());
                for expr in &expr.arguments {
                    arguments.push(self.evaluate_expression(&expr)?);
                }

                let n_arguments = arguments.len();
                if callable.arity() != arguments.len() {
                    return Err(LoxError::Runtime(RuntimeError::arity_mismatch(
                        callable.arity(),
                        n_arguments,
                    )));
                }

                Ok(callable.call(self, arguments)?)
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
