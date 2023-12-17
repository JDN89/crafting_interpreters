use std::usize;

#[derive(Debug)]
pub enum LoxError {
   Interpreter(InterpreterError),
    ParserError(ParserError),
    ScannerError(ParserError),

}

#[derive(Debug)]
pub struct InterpreterError {
    line:usize,
    message:String
}
impl InterpreterError {
    pub fn throw(line:usize ,message:String) -> Self{
        InterpreterError { line, 
            message: message.to_string() }
    }

    //Todo get access to token so we can get the line where the error originated
    pub fn report (&self) {

        eprintln!("Location: {:?} error: {:?}",self.line, self.message);
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
