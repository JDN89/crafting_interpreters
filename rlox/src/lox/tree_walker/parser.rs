use std::rc::Rc;

use crate::frontend::lox_value::LoxValue;
use crate::frontend::token::Token;
use crate::frontend::token_type::TokenType::{self, *};
use crate::{Loc, LoxError, ParserError};

const PARAM_LIMIT: usize = 255;

// STATEMENTS
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Function(FunctionDecl),
    Var(VarStmt),
    If(IfStmt),
    Print(PrintStmt),
    Return(ReturnStmt),
    Block(BlockStmt),
    While(WhileStmt),
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Box<Vec<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

// EXPRESSIONS

#[derive(Debug, Clone)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Call(FunctionCallExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct FunctionCallExpr {
    pub callee: Rc<Expr>,
    pub paren: Token,
    pub arguments: Vec<Rc<Expr>>,
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: LoxValue,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct VariableExpr {
    pub name: Token,
}

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

#[allow(dead_code, unused_variables)]
impl<'a> Parser<'a> {
    pub fn build_parser(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    // program        → statement* EOF ;
    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();

        // TODO with if let we can call synchronize() in case of an error
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => {
                    println!("Error: {:?}", error);
                    let _ = self.synchronize();
                    return Err(error);
                }
            }
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token_types(&[Fun]) {
            Ok(self.parse_function_statement("function")?)
        } else if self.match_token_types(&[Var]) {
            Ok(self.var_declaration()?)
        } else {
            Ok(self.statement()?)
        }
    }

    //parse statement syntax trees
    // statement      → exprStmt | printStmt ;
    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token_types(&[For]) {
            return Ok(self.parse_for_statement()?);
        }
        if self.match_token_types(&[If]) {
            return Ok(self.parse_if_statement()?);
        }

        if self.match_token_types(&[Print]) {
            return Ok(self.parse_print_statement()?);
        }
        if self.match_token_types(&[Return]) {
            return Ok(self.return_statment()?);
        }
        if self.match_token_types(&[While]) {
            return Ok(self.parse_while_statement())?;
        }
        if self.match_token_types(&[LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }));
        }

        Ok(self.expression_statement()?)
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(LeftParen, "Expect '(', after 'if'")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect '(', after if condition '")?;
        let then_branch = Box::new(self.statement()?);
        // Eagerly check fo an else statement so it belongs to the closest if statement
        let else_branch = if self.match_token_types(&[Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }
    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        // Separate scope for the mutable borrow of `self` for `name`
        //consume token and store in name
        let name = {
            // Inner scope starts here
            let name_token = self.consume(Identifier, "expect variable name.")?;
            // Rust doesn't allow moving out of borrowed content within the same scope
            // Creating a new scope allows the borrow of `name_token` to end before
            // attempting to move or clone it
            name_token.clone()
            // Inner scope ends here
        };

        let mut initializer: Option<Expr> = None;
        if self.match_token_types(&[Equal]) {
            initializer = Some(self.expression()?);
        }

        let _consumed_semicolon = self.consume(Semicolon, "Expect ';' after variable declaration.");
        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(LeftParen, "Expect '(' after 'while'.)")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after condition. ")?;
        let body = self.statement()?;
        Ok(Stmt::While(WhileStmt {
            condition,
            body: Box::new(body),
        }))
    }

    // printStmt      → "print" expression ";" ;
    fn parse_print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    // returnStmt     → "return" expression? ";" ;
    fn return_statment(&mut self) -> Result<Stmt, LoxError> {
        let expr = if !self.check(&Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        let token = self.previous().unwrap().clone();
        self.consume(Semicolon, "Expect ';' after return value")?;
        Ok(Stmt::Return(ReturnStmt {
            keyword: token,
            value: expr,
        }))
    }

    // exprStmt       → expression ";" ;
    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expression = self.expression()?;
        self.consume(Semicolon, "Expect ';' expression.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression }))
    }

    fn parse_function_statement(&mut self, kind: &str) -> Result<Stmt, LoxError> {
        let name = self.consume(Identifier, &format!("Expect {} name.", kind).as_str())?;
        let _ignore = self.consume(
            LeftParen,
            format!("Expect ( after {} name.", &kind).as_str(),
        );

        let (parameters, body) = self.parse_fun_parameters_and_body()?;

        Ok(Stmt::Function(FunctionDecl {
            name,
            parameters,
            body: Box::new(body),
        }))
    }

    fn parse_fun_parameters_and_body(&mut self) -> Result<(Vec<Token>, Vec<Stmt>), LoxError> {
        let mut parameters = Vec::new();
        if !self.check(&RightParen) {
            // todo: you should always consume first identifier if there is one

            loop {
                if parameters.len() >= PARAM_LIMIT {
                    return Err(LoxError::ParserError(ParserError::new(
                        self.peek().unwrap().line,
                        Loc::Lexeme(self.peek().unwrap().lexeme.to_owned()),
                        "Can't have more than 255 parameters",
                    )));
                }

                parameters.push(self.consume(Identifier, "Expect parameter name.")?);

                if !self.match_token_types(&[Comma]) {
                    break;
                }
            }
        }
        let _ = self.consume(RightParen, "Expect ')' after parameters!");

        let _ = self.consume(LeftBrace, "Expect { after function body");
        let body = self.block()?;

        Ok((parameters, body))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.check(&RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        let _ = self.consume(RightBrace, "Expect '}' after block");
        Ok(statements)
    }

    // expression     → assingment ;
    fn expression(&mut self) -> Result<Expr, LoxError> {
        Ok(self.assignment()?)
    }

    // assignment     → IDENTIFIER "=" assignment | equality ;
    // recursion cause assignment is right associative. For the other binary operators we loop as
    // long as we match the same operator type because the are left associative
    fn assignment(&mut self) -> Result<Expr, LoxError> {
        // store Assing Expr in expr
        let assing_expr = self.parse_or()?;
        if self.match_token_types(&[Equal]) {
            let equals = self.previous();
            // we call assginement again because we can have var a = 1 = 2 = 3
            let literal_expr = self.assignment()?;

            if let Expr::Variable(var) = &assing_expr {
                let name = &var.name;
                return Ok(Expr::Assign(AssignExpr {
                    name: name.clone(),
                    value: Box::new(literal_expr),
                }));
            }
        }
        Ok(assing_expr)
    }

    fn parse_or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.parse_and()?;
        while self.match_token_types(&[Or]) {
            let operator = self.previous().unwrap().clone();
            let right = self.parse_and()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;
        while self.match_token_types(&[And]) {
            let operator = self.previous().unwrap().clone();
            let right = self.equality()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&[BangEqual, EqualEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.match_token_types(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;
        while self.match_token_types(&[Minus, Plus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_token_types(&[Slash, Star]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    // unary          → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token_types(&[Bang, Minus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        Ok(self.call()?)
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token_types(&[LeftParen]) {
                expr = self.finish_call(expr)?
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut arguments = Vec::new();
        if !self.check(&RightParen) {
            loop {
                if arguments.len() >= 255 {
                    let token = self.peek().unwrap();
                    return Err(LoxError::ParserError(ParserError::new(
                        token.line,
                        Loc::Lexeme(token.lexeme.to_owned()),
                        "Can't have more than 255 arguments",
                    )));
                }
                arguments.push(Rc::new(self.expression()?));
                if !self.match_token_types(&[Comma]) {
                    break;
                }
            }
        }
        let parenthesis = self.consume(RightParen, "Expect ')' after arguments.");
        Ok(Expr::Call(FunctionCallExpr {
            callee: Rc::new(callee.clone()),
            paren: parenthesis.unwrap().clone(),
            arguments,
        }))
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token_types(&[False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: LoxValue::Boolean(false),
            }));
        }
        if self.match_token_types(&[True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: LoxValue::Boolean(true),
            }));
        }
        if self.match_token_types(&[Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: LoxValue::Nil,
            }));
        }
        if self.match_token_types(&[Number, String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().unwrap().literal.clone().unwrap(),
            }));
        }

        if self.match_token_types(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "expect ')' after expression")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        if self.match_token_types(&[Identifier]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous().unwrap().clone(),
            }));
        }
        // If none of the cases in there match, it means we are sitting on a token that can’t start an expression. We need to handle that error too.
        else {
            Err(LoxError::ParserError(ParserError::new(
                self.peek().unwrap().line,
                Loc::Lexeme(self.peek().unwrap().lexeme.to_owned()),
                "Expected expression.",
            )))
        }
    }

    // returning the just consumed token makes it easier to use match_token_types
    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn match_token_types(&mut self, token_types: &[TokenType]) -> bool {
        token_types.iter().any(|ttype| {
            if self.check(ttype) {
                self.advance();
                true
            } else {
                false
            }
        })
    }

    // todo instead of unwrap return error -> unwrap_or_else
    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().unwrap().token_type == *ttype
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).clone().cloned()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous().unwrap().clone();
    }

    fn is_at_end(&self) -> bool {
        self.peek().unwrap().token_type == Eof
    }

    // We clone token to take ownershiop of the value and avoid borrower issues later on
    fn consume(&mut self, ttype: TokenType, error_message: &str) -> Result<Token, LoxError> {
        if self.check(&ttype) {
            return Ok(self.advance());
        }

        let curr_token = self.peek().unwrap();
        Err(LoxError::ParserError(ParserError::new(
            curr_token.line,
            Loc::Lexeme(curr_token.lexeme.to_owned()),
            error_message,
        )))
    }

    fn synchronize(&mut self) -> Result<(), LoxError> {
        self.advance();
        while !self.is_at_end() {
            if self.previous().unwrap().token_type == Semicolon {
                return Ok(());
            }
            match self.peek().unwrap().token_type {
                TokenType::Class => {
                    // Implement the logic for handling 'Class'
                }
                TokenType::Fun => {
                    // Implement the logic for handling 'Fun'
                }
                TokenType::Var => {
                    // Implement the logic for handling 'Var'
                }
                TokenType::For => {
                    // Implement the logic for handling 'For'
                }
                TokenType::If => {
                    // Implement the logic for handling 'If'
                }
                TokenType::While => {
                    // Implement the logic for handling 'While'
                }
                TokenType::Print => {
                    // Implement the logic for handling 'Print'
                }
                TokenType::Return => {
                    // Implement the logic for handling 'Return'
                }
                _ => unreachable!(), // Marking other cases as unreachable
            }
            self.advance();
        }
        Ok(())
    }

    fn parse_for_statement(&mut self) -> Result<Stmt, LoxError> {
        let _ = self.consume(LeftParen, "Expect '(' after 'for'.");
        // parse intializer of for loop
        let initializer: Option<Stmt>;
        if self.match_token_types(&[Semicolon]) {
            initializer = None;
        } else if self.match_token_types(&[Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }
        // parse loop condition
        let mut condition: Option<Expr> = None;
        if !self.check(&Semicolon) {
            condition = Some(self.expression()?);
        }
        let _ = self.consume(Semicolon, "Expected ';' after loop condition");

        // parse increment

        let mut increment: Option<Expr> = None;
        if !self.check(&RightParen) {
            increment = Some(self.expression()?);
        }
        let _ = self.consume(RightParen, "Expected '(' after for clause)");

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![body, Stmt::Expression(ExpressionStmt { expression: inc })],
            })
        }

        if let None = condition {
            condition = Some(Expr::Literal(LiteralExpr {
                value: LoxValue::Boolean(true),
            }));
        }
        body = Stmt::While(WhileStmt {
            condition: condition.expect("A condition should be present!"),
            body: Box::new(body),
        });

        if let Some(initializer) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: vec![initializer, body],
            })
        }

        Ok(body)
    }
}
