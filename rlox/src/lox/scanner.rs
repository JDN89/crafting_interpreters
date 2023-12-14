use crate::lox_error::Loc;
use crate::lox_error::LoxError;
use crate::token::{Literal, Token};
use crate::token_type::TokenType::{self, *};
use std::string::String;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and".to_string(), TokenType::And);
        map.insert("class".to_string(), TokenType::Class);
        map.insert("else".to_string(), TokenType::Else);
        map.insert("false".to_string(), TokenType::False);
        map.insert("for".to_string(), TokenType::For);
        map.insert("fun".to_string(), TokenType::Fun);
        map.insert("if".to_string(), TokenType::If);
        map.insert("nil".to_string(), TokenType::Nil);
        map.insert("or".to_string(), TokenType::Or);
        map.insert("print".to_string(), TokenType::Print);
        map.insert("return".to_string(), TokenType::Return);
        map.insert("super".to_string(), TokenType::Super);
        map.insert("this".to_string(), TokenType::This);
        map.insert("true".to_string(), TokenType::True);
        map.insert("var".to_string(), TokenType::Var);
        map.insert("while".to_string(), TokenType::While);
        map
    };
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

// self is instance of Scanner, you call instance methods on self.
// Self is the type Scanner
impl Scanner {
    pub fn build_scanner(source: &String) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            // start = 0 current =5, next lexeme start = 5
            self.start = self.current;
            self.scan_token()?;
        }

        // add at the end of source code an EOF when is_at_end is true.
        // Not needed but cleaner
        self.tokens
            .push(Token::new(Eof, "".to_string(), None, self.line));
        // clone so the caller has ownership of the tokens
        // TODO: check if we really need clone
        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        // match and return to scan_tokens until we reach end of source code
        match self.advance()? {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                //current is set to +1 after advance call so we match on the the char after !
                if self.is_match('=') {
                    self.add_token(BangEqual)
                } else {
                    self.add_token(Bang)
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(EqualEqual)
                } else {
                    self.add_token(Equal)
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(LessEqual)
                } else {
                    self.add_token(Less)
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(GreaterEqual)
                } else {
                    self.add_token(Greater)
                }
            }

            '/' => {
                if self.is_match('/') {
                    // A comment goes until the end of the line
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance()?;
                    }
                } else if self.is_match('*') {
                    self.scan_block_comment()?;
                } else {
                    self.add_token(Slash);
                }
            }

            // Ignore whitespaces
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string()?,

            // _ => (),
            c => {
                if self.check_is_digit(Some(c)) {
                    self.consume_number()?;
                } else if self.is_alpha(c) {
                    self.identifier()?;
                } else {
                    return Err(LoxError::new(
                        self.line,
                        Loc::Pos(self.current),
                        "Unexpected character",
                    ));
                }
            }
        }
        Ok(())
    }

    // Consume the current character and return it,
    // increase current with one
    fn advance(&mut self) -> Result<char, LoxError> {
        let current_character = self.source.chars().nth(self.current).ok_or_else(|| {
            LoxError::new(
                self.line,
                Loc::Pos(self.current),
                "Couldn't consume character at this position",
            )
        })?;
        self.current += 1;
        Ok(current_character)
    }

    // we're going ot handle literals here later
    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None);
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Literal>) {
        // Comments get consumed until the end of the line
        // for String the lexeme is different
        let lexeme = &self.source[self.start..self.current];
        let token = match literal {
            None => Token::new(
                ttype,
                lexeme.to_string(),
                Some(Literal::String("".to_string())),
                self.line,
            ),
            Some(Literal::String(value)) => Token::new(
                ttype,
                self.source[self.start + 1..self.current - 1].to_string(),
                Some(Literal::String(value.to_string())),
                self.line,
            ),
            Some(Literal::Integer(value)) => Token::new(
                ttype,
                lexeme.to_string(),
                Some(Literal::Integer(value)),
                self.line,
            ),
            Some(Literal::Boolean(value)) => Token::new(
                ttype,
                lexeme.to_owned(),
                Some(Literal::Boolean(value)),
                self.line,
            ),

            Some(Literal::Nil) => {
                Token::new(ttype, lexeme.to_owned(), Some(Literal::Nil), self.line)
            }
        };
        let tokens = self.tokens.push(token);
        tokens
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        // Current is set to +1 after advance call so we match on the the char after !
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn peek(&self) -> Option<char> {
        return self.source.chars().nth(self.current); // TODO: add error handling
    }

    // TODO fix bug -> lexeme of string literal is fucked up -> start up java program
    fn string(&mut self) -> Result<(), LoxError> {
        // if '"' we skip while loop and jump to self.advance() to consume the closing ".
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance()?;
        }
        if self.is_at_end() {
            return Err(LoxError::new(
                self.line,
                Loc::Pos(self.current),
                "Unterminated tring",
            ));
        }
        self.advance()?; // consume the closing ".

        //Trim the surrounding quotes
        let string_value = &self.source[self.start + 1..self.current - 1];
        self.add_token_object(String, Some(Literal::String(string_value.to_string())));

        Ok(())
    }

    fn check_is_digit(&self, c: Option<char>) -> bool {
        if let Some(ch) = c {
            return ch >= '0' && ch <= '9';
        } else {
            false
        }
    }

    fn consume_number(&mut self) -> Result<(), LoxError> {
        while self.check_is_digit(self.peek()) {
            self.advance()?;
        }

        // Look for a fractional part.
        if self.peek() == Some('.') && self.check_is_digit(self.peek_next()) {
            //consume the "."
            self.advance()?;

            while self.check_is_digit(self.peek()) {
                self.advance()?;
            }
        }
        let num = self.source[self.start..self.current].parse::<f64>();

        match num {
            Ok(num) => self.add_token_object(Number, Some(Literal::Integer(num))),
            Err(_) => {
                return Err(LoxError::new(
                    self.line,
                    Loc::Pos(self.current),
                    "Couldn't parse integer",
                ))
            }
        }
        Ok(())
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return None;
        }
        return self.source.chars().nth(self.current + 1);
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while self.is_alpha_numeric(self.peek()) {
            self.advance()?;
        }
        let txt = self.source[self.start..self.current].to_string();
        let ttype = KEYWORDS.get(&txt);
        match ttype {
            None => self.add_token(Identifier),
            Some(value) => match value {
                False => self.add_token_object(False, Some(Literal::Boolean(false))),
                True => self.add_token_object(True, Some(Literal::Boolean(true))),
                Nil => self.add_token_object(Nil, Some(Literal::Nil)),
                _ => self.add_token(value.clone()),
            },
        }
        Ok(())
    }

    fn is_alpha(&self, c: char) -> bool {
        return c >= 'a' && c <= 'z' || c >= 'A' && c >= 'Z' || c == '_';
    }
    fn is_alpha_numeric(&self, option_c: Option<char>) -> bool {
        if let Some(c) = option_c {
            return self.is_alpha(c) || self.check_is_digit(option_c);
        } else {
            false
        }
    }

    fn scan_block_comment(&mut self) -> Result<(), LoxError> {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance()?;
                    if self.is_match('/') {
                        return Ok(());
                    }
                }
                Some('/') => {
                    self.advance()?;
                    if self.is_match('*') {
                        self.scan_block_comment()?;
                    }
                }
                Some('\n') => {
                    // consume \n and go to the next line and increase line counter with one
                    self.advance()?;
                    self.line += 1;
                }
                None => {
                    return Err(LoxError::new(
                        self.line,
                        Loc::Pos(self.current),
                        "Unterminated comment",
                    ));
                }
                _ => {
                    self.advance()?;
                }
            }
        }
    }
}
