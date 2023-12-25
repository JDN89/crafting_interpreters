use crate::expr::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr};
use crate::stmt::{ExpressionStmt, Stmt};
use crate::stmt::{PrintStmt, VarStmt};
use crate::token::Literal;
use crate::token_type::TokenType::{self, *};
use crate::{expr::Expr, token::Token};
use crate::{Loc, LoxError, ParserError};

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code, unused_variables)]
impl Parser {
    pub fn build_parser(tokens: Vec<Token>) -> Parser {
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
                    let _ = self.synchronize();
                    return Err(error);
                }
            }
        }

        return Ok(statements);
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token_types(&[Var]) {
            return Ok(self.var_declaration()?);
        } else {
            return Ok(self.statement()?);
        };
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        // Separate scope for the mutable borrow of `self` for `name`
        //consume token and store in name
        let name = {
            // Inner scope starts here
            let name_token = self.consume(&Identifier, "expect variable name.")?;
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

        let _consumed_semicolon =
            self.consume(&Semicolon, "Expect ';' after variable declaration.");
        return Ok(Stmt::Var(VarStmt { name, initializer }));
    }

    //parse statement syntax trees
    // statement      → exprStmt | printStmt ;
    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token_types(&[Print]) {
            return Ok(self.print_statement()?);
        }
        return Ok(self.expression_statement()?);
    }

    // printStmt      → "print" expression ";" ;
    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(&Semicolon, "Expect ';' after value.")?;
        return Ok(Stmt::Print(PrintStmt { expression: value }));
    }

    // exprStmt       → expression ";" ;
    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(&Semicolon, "Expect ';' expression.")?;
        return Ok(Stmt::Expression(ExpressionStmt { expression: value }));
    }

    // expression     → equality ;
    // todo() : convert to Result<Expr, LoxError> and fix along the chain
    fn expression(&mut self) -> Result<Expr, LoxError> {
        Ok(self.equality()?)
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

        return Ok(self.primary()?);
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token_types(&[False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Boolean(false),
            }));
        }
        if self.match_token_types(&[True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Boolean(true),
            }));
        }
        if self.match_token_types(&[Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Nil,
            }));
        }
        if self.match_token_types(&[Number, String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().unwrap().literal.clone().unwrap(),
            }));
        }

        if self.match_token_types(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, "expect ')' after expression")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        if self.match_token_types(&[Identifier]) {
            return Ok(Expr::Variable(VariableExpr { name: self.previous().unwrap().clone() }));
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

    fn match_token_types(&mut self, token_types: &[crate::token_type::TokenType]) -> bool {
        let mut found_match = false;

        for ttype in token_types {
            if self.check(&ttype) {
                self.advance();
                found_match = true;
            }
        }
        found_match
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().unwrap().token_type == *ttype
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous().unwrap();
    }

    fn is_at_end(&self) -> bool {
        self.peek().unwrap().token_type == Eof
    }

    fn consume(&mut self, ttype: &TokenType, error_message: &str) -> Result<&Token, LoxError> {
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
}
