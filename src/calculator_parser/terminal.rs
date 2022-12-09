use regex::Regex;

pub mod terminals;

#[derive(Clone)]
pub enum Terminal {
    Epsilon,
    Literal(String),
    RegularExpresion(Regex)
}

impl Terminal {
    pub fn match_symbol(&self, input: &str) -> bool {
        match self {
            Self::Epsilon => true,
            Self::Literal(s) => input.eq(s),
            Self::RegularExpresion(r) => r.is_match(input)
        }
    }
}

impl ToString for Terminal {
    fn to_string(&self) -> String {
        match self {
            Self::Epsilon => String::from(""),
            Self::Literal(s) => s.to_string(),
            Self::RegularExpresion(r) => r.to_string()
        }
    }
}