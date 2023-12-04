use crate::expr::BinaryExpr;
use crate::token_type::TokenType::*;
use crate::{expr::Expr, token::Token};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn build_parser() -> Self {
        Parser {
            tokens: Vec::new(),
            current: 0,
        }
    }

    // expression     → equality ;
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        // todo: setup patter matching? call appropriate functions in case of certain match satements

        while self.match_token_types(&[BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
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

    fn previous(&self) -> Token {
        todo!()
    }

    fn comparison(&self) -> Expr {
        todo!()
    }

    fn match_token_types(&self, token_types: &[crate::token_type::TokenType; 2]) -> bool {
        todo!()
    }
}
