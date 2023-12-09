// todo: look up how to create custom errors

use std::usize;

use crate::token::Token;

#[derive(Debug)]
pub struct LoxError {
    line: usize,
    location: Location,
    message: String,
}

#[derive(Debug)]
struct Location {
    position_source_code: Option<usize>,
    token: Option<Token>
}

#[allow(dead_code)]
impl LoxError {
    pub fn new(line: usize, location: Location,message: &str) -> Self {
        Self {
            line,
            location,
            message: message.to_string(),
        }
    }

    pub fn report(&self) {
        if let Some(position) =  self.location.position_source_code {
        eprintln!(
            "[line {}, position {:?}] Error: {}",
            self.line, position, self.message
        );
            if let Some(token) = self.location.token {

        eprintln!(
            "[line {}, position {:?}] Error: {}",
            self.line, token, self.message
        );
            }
        }
    }
}
