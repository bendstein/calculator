use super::{ ParserInner, Parsable };
use crate::calculator::calculator_parser::expression as xpr;
use crate::calculator::CalculatorErr;


impl Parsable for xpr::Expr {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Ok(xpr::Expr::None)
        }
        else {
            let expression_result = parser.expr_prime();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else {
                        Ok(xpr::Expr::ExprPrime(Box::new(expr)))
                    }
                }
            }
        }
    }
}

impl Parsable for xpr::BinopInfix {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Err(CalculatorErr::parse_err("Empty string provided!", true, parser.lah))
        }
        else {
            let expression_result = parser.binop_in();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else {
                        Ok(expr)
                    }
                }
            }
        }
    }
}

impl Parsable for xpr::UnopPrefix {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Err(CalculatorErr::parse_err("Empty string provided!", true, parser.lah))
        }
        else {
            let expression_result = parser.unop_pre();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else {
                        Ok(expr)
                    }
                }
            }
        }
    }
}

impl Parsable for xpr::UnopSuffix {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Err(CalculatorErr::parse_err("Empty string provided!", true, parser.lah))
        }
        else {
            let expression_result = parser.unop_suf();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else {
                        Ok(expr)
                    }
                }
            }
        }
    }
}

impl Parsable for xpr::IdToken {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Err(CalculatorErr::parse_err("Empty string provided!", true, parser.lah))
        }
        else {
            let expression_result = parser.id();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else {
                        Ok(expr)
                    }
                }
            }
        }
    }
}

impl Parsable for xpr::Func {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Err(CalculatorErr::parse_err("Empty string provided!", true, parser.lah))
        }
        else {
            let expression_result = parser.func();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else {
                        Ok(expr)
                    }
                }
            }
        }
    }
}

impl Parsable for xpr::NumberToken {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Err(CalculatorErr::parse_err("Empty string provided!", true, parser.lah))
        }
        else {
            let expression_result = parser.number();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else {
                        Ok(expr)
                    }
                }
            }
        }
    }
}

impl Parsable for xpr::HistoryToken {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Err(CalculatorErr::parse_err("Empty string provided!", true, parser.lah))
        }
        else {
            let expression_result = parser.history_memory();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else if let xpr::ExprPrime::History(history) = expr {
                        Ok(history)
                    }
                    else {
                        Err(CalculatorErr::parse_err("Parsed input was not parsed as a history access expression.", true, parser.lah))
                    }
                }
            }
        }
    }
}

impl Parsable for (xpr::MemoryToken, Option<xpr::ExprPrime>) {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        if parser.tokens.is_empty() {
            Err(CalculatorErr::parse_err("Empty string provided!", true, parser.lah))
        }
        else {
            let expression_result = parser.history_memory();

            match expression_result {
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                },
                Ok(expr) => {
                    if parser.lah < parser.tokens.len() {
                        Err(CalculatorErr::parse_err(format!("Unexpected token '{}'.", parser.token_at(parser.lah)).as_str(), true, parser.lah))
                    }
                    else if let xpr::ExprPrime::StoreMem(memory, subexpr) = expr {
                        Ok((memory, Some(*subexpr)))
                    }
                    else if let xpr::ExprPrime::AccessMem(memory) = expr {
                        Ok((memory, None))
                    }
                    else {
                        Err(CalculatorErr::parse_err("Parsed input was not parsed as a memory access/mutate expression.", true, parser.lah))
                    }
                }
            }
        }
    }
}

impl Parsable for xpr::MemoryToken {
    fn parse_from(parser: &mut ParserInner) -> Result<Self, CalculatorErr> {
        type MemoryAssignmentTuple = (xpr::MemoryToken, Option<xpr::ExprPrime>);
        let (memory, _) = MemoryAssignmentTuple::parse_from(parser)?;
        Ok(memory)
    }
}