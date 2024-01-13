use crate::frontend::token::{Literal, Token};
use crate::frontend::token_type::TokenType::{self, *};
use crate::tree_walker::ast::*;
use crate::{Loc, LoxError, ParserError};

// don't forget to compare this to the PRAT parser:
// https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/
//When you’re writing a parser, recursive descent is as easy as spreading peanut butter.
// It excels when you can figure out what to parse based on the next bit of code you’re looking at.
// That’s usually true at the declaration and statement levels of a language’s grammar since most syntax there starts with keywords—class, if, for, while, etc.

//Parsing gets trickier when you get to expressions.
// When it comes to infix operators like +, postfix ones like ++, and even mixfix expressions like ?:,
// it can be hard to tell what kind of expression you’re parsing until you’re halfway through it.
// You can do this with recursive descent, but it’s a chore.
// You have to write separate functions for each level of precedence (JavaScript has 17 of them, for example),
// manually handle associativity, and smear your grammar across a bunch of parsing code until it’s hard to see.

//
// In a top-down parser, you reach the lowest-precedence expressions first because they may in turn contain subexpressions of higher precedence.
// each precedence calls a following function that deals with a higher precedence level
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

//  STATEMENT GRAMAR RULES
// | Production     | Expansion                                      |
// |----------------|------------------------------------------------|
// | program        | declaration* EOF ;                             |
// | declaration    | varDecel | statement;                          |
// | statement      | exprStmt |ifStmt | printStmt | block;          |
// | ifStmt         | "if" "("  expression ")" statement ("else" statement)?|
// | block          | "{" declaration* "}";                          |
// | exprStmt       | expression ";"                                 |
// | printStmt      | "print" expression ";"                         |
//
// the tricky part here is the GROUPING which in turn can call parse expression again
// EXAMPLE ( 1 + 2) * 3
//
//      *
//     / \
//    +   3
//   / \
//  1   2
//
// we go down the recursive chain of call until we hit ( here we call recursively call expression,
// which in turn parses 1 + 2 in a binary expression and wraps it in a grouping expression
// we go up the chain of calls until we hit *.
// The while loop places the previous grouping expression on the left side of the tree and the
// remaining 3 on the right side of the tree.
//
// The recrusive calls build up the right leaning tree and the while statements build up the left
// leaning tree. Everytime you hit a while statement the previously parsed expr get placed to the
// left operand side

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
        if self.match_token_types(&[If]) {
            return Ok(self.parse_if_statement()?);
        }

        if self.match_token_types(&[Print]) {
            return Ok(self.print_statement()?);
        }
        if self.match_token_types(&[LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }));
        }

        return Ok(self.expression_statement()?);
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(&LeftBrace, "Expect '(', after 'if'")?;
        let condition = self.expression()?;
        self.consume(&RightBrace, "Expect '(', after if condition '")?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token_types(&[Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        return Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }));
    }

    // printStmt      → "print" expression ";" ;
    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(&Semicolon, "Expect ';' after value.")?;
        return Ok(Stmt::Print(PrintStmt { expression: value }));
    }

    // exprStmt       → expression ";" ;
    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expression = self.expression()?;
        self.consume(&Semicolon, "Expect ';' expression.")?;
        return Ok(Stmt::Expression(ExpressionStmt { expression }));
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.check(&RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        let _ = self.consume(&RightBrace, "Expect '}' after block");
        return Ok(statements);
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
        let assing_expr = self.equality()?;
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

    fn match_token_types(
        &mut self,
        token_types: &[TokenType],
    ) -> bool {
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
