use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct ParserErr {
    message: String
}

impl ParserErr {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message)
        }
    }
}

impl Default for ParserErr {
    fn default() -> Self {
        Self::new("")
    }
}

impl Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}