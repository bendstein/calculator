use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct InterpreterErr {
    message: String
}

impl InterpreterErr {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message)
        }
    }
}

impl Default for InterpreterErr {
    fn default() -> Self {
        Self::new("")
    }
}

impl Display for InterpreterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}