// todo: look up how to create custom errors
pub struct LoxError {
    line: u32,
    location: String,
    message: String,
}

#[allow(dead_code)]
impl LoxError {
    pub fn new(line: u32, location: String, message: String) -> Self {
        Self {
            line,
            location,
            message,
        }
    }

    pub fn report(&self) {
        eprintln!(
            "[line {}] Error{}: {}",
            self.line, self.location, self.message
        );
    }
}
