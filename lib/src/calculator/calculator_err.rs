use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum CalculatorErr {
    #[default] General,
    EvaluateErr { message: String },
    ParseErr { message: String, propagate: bool, lah: usize },
    InterpretErr { message: String }
}

impl CalculatorErr {
    pub fn message(&self) -> Option<String> {
        match self {
            Self::General => None,
            Self::EvaluateErr { message } => Some(message.clone()),
            Self::ParseErr { message, ..} => Some(message.clone()),
            Self::InterpretErr { message } => Some(message.clone())
        }
    } 

    pub fn lah(&self) -> usize {
        if let Self::ParseErr { lah, .. } = self {
            *lah
        }
        else {
            0_usize
        }
    }

    pub fn propagate(&self) -> bool {
        if let Self::ParseErr { propagate, .. } = self {
            *propagate
        }
        else {
            false
        }
    }

    pub fn eval_err(message: &str) -> Self {
        Self::EvaluateErr { message: String::from(message) }
    }

    pub fn interp_err(message: &str) -> Self {
        Self::InterpretErr { message: String::from(message) }
    }

    pub fn parse_err(message: &str, propagate: bool, lah: usize) -> Self {
        Self::ParseErr { message: String::from(message), propagate, lah }
    }

}

impl Display for CalculatorErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::General => "An error occurred.",
            Self::EvaluateErr { message } => message,
            Self::ParseErr { message, .. } => message,
            Self::InterpretErr { message } => message
        };

        f.write_str(message)
    }
}
