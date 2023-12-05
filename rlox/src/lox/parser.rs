use crate::expr::BinaryExpr;
use crate::token_type::TokenType::{self, *};
use crate::{expr::Expr, token::Token};

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code, unused_variables)]
impl Parser {
    fn build_parser() -> Self {
        Parser {
            tokens: Vec::new(),
            current: 0,
        }
    }

    // expression     → equality ;
    // todo() : convert to Result<Expr, LoxError> and fix along the chain
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token_types(&[BangEqual, EqualEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison();
            // todo! () : create build function for BinaryExpr
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&self) -> Expr {
        todo!()
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn match_token_types(&mut self, token_types: &[crate::token_type::TokenType; 2]) -> bool {
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
        self.previous().unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.peek().unwrap().token_type == Eof
    }
}
