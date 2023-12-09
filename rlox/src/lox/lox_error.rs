// todo: look up how to create custom errors


#[derive(Debug)]
pub struct LoxError {
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
impl LoxError {
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
