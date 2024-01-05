use std::io::{BufRead, Write};

pub use lox_error::*;


pub mod user_interface;
pub mod tree_walker;
mod frontend;
mod lox_error;

// possible need of converson to Box<dyn Error>

