pub mod parsererr;

use unicode_segmentation::{self, UnicodeSegmentation};
use super::terminal::*;
use super::expression as xpr;
use parsererr::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parser<'a> {
    lah: usize,
    tokens: Vec<&'a str>
}

#[allow(clippy::question_mark)]
impl<'a> Parser<'a> {
    pub fn new(line: &'a str) -> Self {
        Self {
            lah: 0,
            tokens: line.trim().graphemes(true)
                .collect()
        }
    }

    pub fn parse_line(line: &'a str) -> Result<xpr::Expr, ParserErr> {
        let mut parser = Self::new(line);
        parser.parse()
    }

    pub fn parse(&mut self) -> Result<xpr::Expr, ParserErr> {
        if self.tokens.is_empty() {
            Ok(xpr::Expr::None)
        }
        else {
            let expression_result = self.expr_prime();

            match expression_result {
                Err(err) => Err(err),
                Ok(expr) => {

                    if self.lah < self.tokens.len() {
                        Err(ParserErr::new(""))
                    }
                    else {
                        Ok(xpr::Expr::ExprPrime(Box::new(expr)))
                    }
                }
            }
        }
    }

    fn token_at(&self, ndx: usize) -> &str {
        if ndx >= self.tokens.len() {
            ""
        }
        else {
            self.tokens[ndx]
        }
    }

    fn get_and_increment(&mut self) -> &str {
        self.lah += 1;
        self.token_at(self.lah - 1)
    }

    fn expr_prime(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        //Optional whitespace
        self.whitespace();

        //Handle in steps for each priority, starting with the step with the lowest priority to account for operator precedence
        let result = self.expr_2();

        //Optional whitespace
        self.whitespace();

        result
    }

    fn expr_2(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        //First handle operators of higher priority to account for operator precedence
        let expr_1_result = self.expr_1();

        //expr_1 is required. Return error.
        if expr_1_result.is_err() {
            return expr_1_result;
        }

        let expr_1 = expr_1_result.unwrap();

        let mut children: Vec<(xpr::BinopInfix, Box<xpr::ExprPrime>)> = Vec::new();
        let mut current_lah = self.lah;

        loop {
            //Match optional whitespace
            self.whitespace();

            //Check for binary infix operator with priority 2
            let binop_in_2_result = self.binop_in_2();

            //Not followed by a binary infix operator with priority 2. Rollback lah and break from loop.
            if binop_in_2_result.is_err() {
                self.lah = current_lah;
                break;
            }

            let binop_in_2 = binop_in_2_result.unwrap();

            //Match optional whitespace
            self.whitespace();

            //Check for another expression of priority 1
            let expr_1_suffix_result = self.expr_1();

            //Not followed by an expression of priority 1. Rollback lah and break from loop.
            if expr_1_suffix_result.is_err() {
                self.lah = current_lah;
                break;
            }

            let expr_1_suffix = expr_1_suffix_result.unwrap();

            //Successfully matched section. Record results and update current_lah before repeating.
            children.push((xpr::BinopInfix::from(binop_in_2), Box::new(expr_1_suffix)));
            current_lah = self.lah;
        };

        if children.is_empty() {
            Ok(expr_1)
        }
        else {
            Ok(xpr::ExprPrime::BinaryInfixExpression(Box::new(expr_1), children))
        }
    }

    fn expr_1(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        //First handle operators of higher priority to account for operator precedence
        let expr_0_result = self.expr_0();

        //expr_0 is required. Return error.
        if expr_0_result.is_err() {
            return expr_0_result;
        }

        let expr_0 = expr_0_result.unwrap();

        let mut children: Vec<(xpr::BinopInfix, Box<xpr::ExprPrime>)> = Vec::new();
        let mut current_lah = self.lah;

        loop {
            //Match optional whitespace
            self.whitespace();

            //Check for binary infix operator with priority 1
            let binop_in_1_result = self.binop_in_1();

            //Not followed by a binary infix operator with priority 1. Rollback lah and break from loop.
            if binop_in_1_result.is_err() {
                self.lah = current_lah;
                break;
            }

            let binop_in_1 = binop_in_1_result.unwrap();

            //Match optional whitespace
            self.whitespace();

            //Check for another expression of priority 0
            let expr_0_suffix_result = self.expr_0();

            //Not followed by an expression of priority 0. Rollback lah and break from loop.
            if expr_0_suffix_result.is_err() {
                self.lah = current_lah;
                break;
            }

            let expr_0_suffix = expr_0_suffix_result.unwrap();

            //Successfully matched section. Record results and update current_lah before repeating.
            children.push((xpr::BinopInfix::from(binop_in_1), Box::new(expr_0_suffix)));
            current_lah = self.lah;
        };

        if children.is_empty() {
            Ok(expr_0)
        }
        else {
            Ok(xpr::ExprPrime::BinaryInfixExpression(Box::new(expr_0), children))
        }
    }

    fn expr_0(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        //First handle operators of higher priority to account for operator precedence
        let expr_base_result = self.expr_base();

        //expr_base is required. Return error.
        if expr_base_result.is_err() {
            return expr_base_result;
        }

        let expr_base = expr_base_result.unwrap();

        let mut children: Vec<(xpr::BinopInfix, Box<xpr::ExprPrime>)> = Vec::new();
        let mut current_lah = self.lah;

        loop {
            //Match optional whitespace
            self.whitespace();

            //Check for binary infix operator with priority 0
            let binop_in_0_result = self.binop_in_0();

            //Not followed by a binary infix operator with priority 0. Rollback lah and break from loop.
            if binop_in_0_result.is_err() {
                self.lah = current_lah;
                break;
            }

            let binop_in_0 = binop_in_0_result.unwrap();

            //Match optional whitespace
            self.whitespace();
            
            //Check for another expression of same priority
            let expr_0_suffix_result = self.expr_0();

            //Not followed by another expression of priority 0. Rollback lah and break from loop.
            if expr_0_suffix_result.is_err() {
                self.lah = current_lah;
                break;
            }

            let expr_0_suffix = expr_0_suffix_result.unwrap();

            //Successfully matched section. Record results and update current_lah before repeating.
            children.push((xpr::BinopInfix::from(binop_in_0), Box::new(expr_0_suffix)));
            current_lah = self.lah;
        };

        if children.is_empty() {
            Ok(expr_base)
        }
        else {
            Ok(xpr::ExprPrime::BinaryInfixExpression(Box::new(expr_base), children))
        }
    }

    fn expr_base(&mut self) -> Result<xpr::ExprPrime, ParserErr> {     
        let initial_lah = self.lah;

        let mut result: Option<Result<xpr::ExprPrime, ParserErr>> = None;

        let mut unop_prefixes: Vec<xpr::UnopPrefix> = Vec::new();
        let mut unop_suffixes: Vec<xpr::UnopSuffix> = Vec::new();

        //Match 0+ unary prefix operators
        loop {
            let unop_prefix_result = self.unop_pre();

            if let Ok(unop_prefix) = unop_prefix_result {
                unop_prefixes.push(unop_prefix);
                continue;
            }

            break;
        };

        //Try to match number.
        let number_result = self.number();

        if let Ok(num) = number_result {
            result = Some(Ok(xpr::ExprPrime::Number(num)));
        }

        if result.is_none() {
            //Failed to match. Try to match function.
            let func_result = self.func();

            if let Ok(func) = func_result {
                result = Some(Ok(xpr::ExprPrime::Func(func)));
            }
        }

        //Disallow variables for now
        // if result.is_none() {
        //     //Failed to match. Try to match id.
        //     let id_result = self.id();

        //     if let Ok(id) = id_result {
        //         result = Some(Ok(xpr::ExprPrime::Id(id)));
        //     }
        // }

        if result.is_none() {
            //Failed to match. Try to match an expression in parentheses.
            let paren_expression_paren_result = self.paren_expression_paren();

            if paren_expression_paren_result.is_ok() {
                result = Some(paren_expression_paren_result);
            }
        }

        if let Some(some_result) = result {
            //Match 0+ unary suffix operators
            loop {
                let unop_suffix_result = self.unop_suf();

                if let Ok(unop_suffix) = unop_suffix_result {
                    unop_suffixes.push(unop_suffix);
                    continue;
                }

                break;
            };

            if let Ok(ok_result) = some_result {
                if unop_prefixes.is_empty() && unop_suffixes.is_empty() {
                    Ok(ok_result)
                }
                else if unop_prefixes.is_empty() && !unop_suffixes.is_empty() {
                    Ok(xpr::ExprPrime::UnopSuffixesExpression(Box::new(ok_result), unop_suffixes))
                }
                else if !unop_prefixes.is_empty() && unop_suffixes.is_empty() {
                    Ok(xpr::ExprPrime::UnopPrefixesExpression(unop_prefixes, Box::new(ok_result)))
                }
                else {
                    Ok(xpr::ExprPrime::UnopPrefixesExpression(unop_prefixes, Box::new(xpr::ExprPrime::UnopSuffixesExpression(Box::new(ok_result), unop_suffixes))))
                }
            }
            else {
                some_result
            }
        }
        else {
            //Failed to match. Rollback lah and return error
            self.lah = initial_lah;
            Err(ParserErr::new(""))
        }
    }

    fn number(&mut self) -> Result<xpr::NumberToken, ParserErr> {
        let mut current_lah = self.lah;

        let mut collected: Vec<&str> = Vec::new();

        //Try to match 1 or more digits
        loop {
            let current_token = self.token_at(current_lah);

            if terminals::DIGIT.match_symbol(current_token) {
                current_lah += 1;
                collected.push(current_token);
                continue;
            }

            break;
        };

        //Make sure at least one digit is present
        if collected.is_empty() {
            return Err(ParserErr::new(""));
        }

        //Check if the next symbol is a decimal point
        let radix_pt_token = self.token_at(current_lah);

        if terminals::RADIX_PT.match_symbol(radix_pt_token) {
            let mut current_lah_1 = current_lah;
            let mut collected_1: Vec<&str> = Vec::new();

            current_lah_1 += 1;

            //Try to match 1 or more digits
            loop {
                let current_token = self.token_at(current_lah_1);

                if terminals::DIGIT.match_symbol(current_token) {
                    current_lah_1 += 1;
                    collected_1.push(current_token);
                    continue;
                }

                break;
            };

            //Make sure at least one digit is present
            if !collected.is_empty() {
                //Successfully matched. Record progress in outer state
                current_lah = current_lah_1;
                collected.push(radix_pt_token);
                for term in collected_1 {
                    collected.push(term)
                }
            }
        }

        let concatenated = collected.join("");

        let parsed= concatenated.parse::<f32>();

        if let Err(_parse_int_err) = parsed {
            return Err(ParserErr::new("Failed to parse number."));
        }

        self.lah = current_lah;
        Ok(xpr::NumberToken::new(parsed.unwrap()))
    }

    fn func(&mut self) -> Result<xpr::Func, ParserErr> {
        let initial_lah = self.lah;
        
        //Try to match id
        let id_result = self.id();

        if let Err(id_err) = id_result {
            return Err(id_err);
        }

        let id = id_result.unwrap();

        //Optional whitespace
        self.whitespace();

        //Try to match an opening paren
        let token = self.get_and_increment();

        //Opening paren is required. Rollback and return error if not present.
        if !xpr::Token::OpParO.get_terminal().match_symbol(token) {
            self.lah = initial_lah;
            return Err(ParserErr::new(""));
        }

        //Optional whitespace
        self.whitespace();

        //Try to match function arguments
        let mut current_lah = self.lah;
        let mut func_args: Vec<xpr::ExprPrime> = Vec::new();

        loop {
            let mut inner_lah = current_lah;
            
            //If at least one argument is present, match the argument delimiter
            if !func_args.is_empty() {
                let token = self.token_at(inner_lah);

                if xpr::Token::Delimiter.get_terminal().match_symbol(token) {
                    inner_lah += 1;
                }
                //Delimiter is required. Break from loop if not present.
                else {
                    break;
                }
            }

            self.lah = inner_lah;

            //Match optional whitespace
            self.whitespace();

            //inner_lah = self.lah;

            //Match expression
            let expr_prime_result = self.expr_prime();

            //Expression is required. Rollback lah and break from loop if not present.
            if expr_prime_result.is_err() {
                self.lah = current_lah;
                break;
            }
            
            let expr_prime = expr_prime_result.unwrap();

            //Optional whitespace
            self.whitespace();

            //Match was success. Record progress in outer loop.
            current_lah = self.lah;
            func_args.push(expr_prime);
        }

        //Try to match a closing paren
        let token = self.get_and_increment();

        //Closing paren is required. Rollback and return error if not present.
        if !xpr::Token::OpParC.get_terminal().match_symbol(token) {
            self.lah = initial_lah;
            return Err(ParserErr::new(""));
        }

        if func_args.is_empty() {
            Ok(xpr::Func::EmptyFunc(id))
        }
        else {
            Ok(xpr::Func::FuncWithArgs(id, func_args))
        }
    }

    fn id(&mut self) -> Result<xpr::IdToken, ParserErr> {
        let mut current_lah = self.lah;
        
        let mut symbols: Vec<&str> = Vec::new();

        let mut seen_letter: bool = false;

        loop {
            let current_token = self.token_at(current_lah);

            //Try to match letter
            if terminals::LETTER.match_symbol(current_token) {
                current_lah += 1;
                symbols.push(current_token);
                seen_letter = true;
                continue;
            }

            //If not letter, try to match underscore
            if terminals::UNDERSCORE.match_symbol(current_token) {
                current_lah += 1;
                symbols.push(current_token);
                continue;
            }

            //If neither a letter nor underscore, and a letter has been seen, try to match a digit
            if seen_letter && terminals::DIGIT.match_symbol(current_token) {
                current_lah += 1;
                symbols.push(current_token);
                continue;
            }

            //Match failed, break from loop
            break;
        };

        if !seen_letter {
            return Err(ParserErr::new(""));
        }

        //Record lah progress and return id
        let concatenated = symbols.join("");
        self.lah = current_lah;

        Ok(xpr::IdToken::new(concatenated.as_str()))
    }

    fn paren_expression_paren(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        let initial_lah = self.lah;

        //Try to match an opening paren
        let token = self.get_and_increment();

        //Opening paren is required. Rollback and return error if not present.
        if !xpr::Token::OpParO.get_terminal().match_symbol(token) {
            self.lah = initial_lah;
            return Err(ParserErr::new(""));
        }

        //Optional whitespace
        self.whitespace();

        //Try to match the inner expression
        let expr_prime_result = self.expr_prime();

        //Inner expression is required. Rollback and return error if not present.
        if expr_prime_result.is_err() {
            self.lah = initial_lah;
            return expr_prime_result;
        }

        let expr_prime = expr_prime_result.unwrap();

        //Optional whitespace
        self.whitespace();

        //Try to match a closing paren
        let token = self.get_and_increment();

        //Closing paren is required. Rollback and return error if not present.
        if !xpr::Token::OpParC.get_terminal().match_symbol(token) {
            self.lah = initial_lah;
            return Err(ParserErr::new(""));
        }

        Ok(xpr::ExprPrime::ParenthesesExpression(Box::new(expr_prime)))
    }

    fn unop_pre(&mut self) -> Result<xpr::UnopPrefix, ParserErr> {
        let initial_lah = self.lah;
        let current = self.get_and_increment();

        let tokens = vec![
            xpr::UnopPrefix::Neg
        ];

        let mut found: Option<xpr::UnopPrefix> = None;

        for token in tokens {
            if xpr::Token::from(token).get_terminal().match_symbol(current) {
                found = Some(token);
                break;
            }
        }

        if let Some(found_token) = found {
            return Ok(found_token);
        }

        //Match failed. Rollback and return error.
        self.lah = initial_lah;
        Err(ParserErr::new(""))
    }

    fn unop_suf(&mut self) -> Result<xpr::UnopSuffix, ParserErr> {
        let initial_lah = self.lah;
        let current = self.get_and_increment();

        let tokens = vec![
            xpr::UnopSuffix::Fac
        ];

        let mut found: Option<xpr::UnopSuffix> = None;

        for token in tokens {
            if xpr::Token::from(token).get_terminal().match_symbol(current) {
                found = Some(token);
                break;
            }
        }

        if let Some(found_token) = found {
            return Ok(found_token);
        }

        //Match failed. Rollback and return error.
        self.lah = initial_lah;
        Err(ParserErr::new(""))
    }

    fn binop_in_0(&mut self) -> Result<xpr::BinopInfix0, ParserErr> {
        let initial_lah = self.lah;
        let current = self.get_and_increment();

        let tokens = vec![
            xpr::BinopInfix0::Exp
        ];

        let mut found: Option<xpr::BinopInfix0> = None;

        for token in tokens {
            if xpr::Token::from(xpr::BinopInfix::from(token)).get_terminal().match_symbol(current) {
                found = Some(token);
                break;
            }
        }

        if let Some(found_token) = found {
            return Ok(found_token);
        }

        //Match failed. Rollback and return error.
        self.lah = initial_lah;
        Err(ParserErr::new(""))
    }

    fn binop_in_1(&mut self) -> Result<xpr::BinopInfix1, ParserErr> {
        let initial_lah = self.lah;
        let current = self.get_and_increment();

        let tokens = vec![
            xpr::BinopInfix1::Div,
            xpr::BinopInfix1::Mult,
            xpr::BinopInfix1::Mod
        ];

        let mut found: Option<xpr::BinopInfix1> = None;

        for token in tokens {
            if xpr::Token::from(xpr::BinopInfix::from(token)).get_terminal().match_symbol(current) {
                found = Some(token);
                break;
            }
        }

        if let Some(found_token) = found {
            return Ok(found_token);
        }

        //Match failed. Rollback and return error.
        self.lah = initial_lah;
        Err(ParserErr::new(""))
    }

    fn binop_in_2(&mut self) -> Result<xpr::BinopInfix2, ParserErr> {
        let initial_lah = self.lah;
        let current = self.get_and_increment();

        let tokens = vec![
            xpr::BinopInfix2::Add,
            xpr::BinopInfix2::Sub
        ];

        let mut found: Option<xpr::BinopInfix2> = None;

        for token in tokens {
            if xpr::Token::from(xpr::BinopInfix::from(token)).get_terminal().match_symbol(current) {            
                found = Some(token);
                break;
            }
        }

        if let Some(found_token) = found {
            return Ok(found_token);
        }

        //Match failed. Rollback and return error.
        self.lah = initial_lah;
        Err(ParserErr::new(""))
    }

    fn whitespace(&mut self) {
        let mut current_lah = self.lah;

        let whitespace_terminal: &Terminal = &terminals::WHITESPACE;

        loop {
            let current_token = self.token_at(current_lah);

            if whitespace_terminal.match_symbol(current_token) {
                current_lah += 1;
                continue;
            }

            break;
        };      

        self.lah = current_lah;
    }
}