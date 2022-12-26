pub mod parsererr;
pub mod parserinner;

use parsererr::*;
use parserinner::*;
use super::expression as xpr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParserSettings {
    
}

impl ParserSettings {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl Default for ParserSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Parser {
    settings: ParserSettings
}

impl Parser {
    fn new(settings: ParserSettings) -> Self {
        Self {
            settings
        }
    }

    fn create_parser<'a>(&self, input: &'a str) -> ParserInner<'a> {
        ParserInner::new(self.settings, input)
    }

    pub fn parse(&self, input: &str) -> Result<xpr::Expr, ParserErr> {
        self.create_parser(input)
            .parse_expression()
    }

    pub fn parse_expression<TExpr: Parsable>(&self, input: &str) -> Result<TExpr, ParserErr> {
        self.create_parser(input)
            .parse_expression()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(ParserSettings::default())
    }
}