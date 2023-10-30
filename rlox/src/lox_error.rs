// todo: look up how to create custom errors
pub struct LoxError {
    line: usize,
    location: usize,
    message: String,
}

#[allow(dead_code)]
impl LoxError {
    pub fn new(line: usize, location: usize, message: &str) -> Self {
        Self {
            line,
            location,
            message: message.to_string(),
        }
    }

    pub fn report(&self) {
        eprintln!(
            "[line {}, position {}] Error: {}",
            self.line, self.location, self.message
        );
    }
}
