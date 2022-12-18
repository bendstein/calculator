use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct CalculatorErr {
    message: String,
    propagate: bool
}

impl CalculatorErr {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
            propagate: false
        }
    }

    pub fn err(message: &str) -> Self {
        Self {
            message: String::from(message),
            propagate: true
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn propagate(&self) -> bool {
        self.propagate
    }
}

impl Default for CalculatorErr {
    fn default() -> Self {
        Self::new("")
    }
}

impl Display for CalculatorErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}