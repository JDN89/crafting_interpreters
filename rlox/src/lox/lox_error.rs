use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum LoxError {
   Interpreter(InterpreterError),
    ParserError(ParserError),
    ScannerError(ParserError),

}

#[derive(Debug)]
pub struct InterpreterError {
    literal: Option<Vec<Literal>> ,
    operator_token:Option<Token>,
    message:String
}
impl InterpreterError {
    pub fn throw(literal: Option< Vec<Literal>>,token:Option<Token>, message:&str) -> Self{
        InterpreterError { literal,operator_token: token,
            message: message.to_string() }
    }

    //Todo get access to token so we can get the line where the error originated
    pub fn report (&self) {
        if let Some(value) = &self.operator_token {

        eprintln!("Location: {:?} literal: {:?}, operator: {:?}, message: {}",value.line, self.literal,value.token_type,self.message);
        }
    }
    
}

#[derive(Debug)]
pub struct ParserError {
    line: usize,
    location: Loc,
    message: String,
}

#[derive(Debug)]
pub enum Loc {
    Lexeme(String),
    Pos(usize)

}

#[allow(dead_code)]
impl ParserError {
    pub fn new(line: usize, location: Loc,message: &str) -> Self {
        Self {
            line,
            location,
            message: message.to_string(),
        }
    }

   pub fn report(&self) {
        match &self.location {
            Loc::Lexeme(lexeme) => {
                eprintln!("[line {}, lexeme '{}'] Error: {}", self.line, lexeme, self.message);
            }
            Loc::Pos(position) => {
                eprintln!("[line {}, position {}] Error: {}", self.line, position, self.message);
            }
        }
    }
}
