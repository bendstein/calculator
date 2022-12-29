use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct ParserErr {
    message: String,
    lah: usize,
    propagate: bool
}

impl ParserErr {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
            lah: 0,
            propagate: false
        }
    }

    pub fn err(message: &str, lah: usize) -> Self {
        Self {
            message: String::from(message),
            lah,
            propagate: true
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn lah(&self) -> usize {
        self.lah
    }

    pub fn propagate(&self) -> bool {
        self.propagate
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