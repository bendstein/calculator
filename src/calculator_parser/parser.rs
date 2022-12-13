pub mod parsererr;

use unicode_segmentation::{self, UnicodeSegmentation};
use super::expression::ExprPrime;
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

    pub fn lah(&self) -> usize {
        self.lah
    }

    pub fn reset(&mut self) {
        self.lah = 0
    }

    pub fn set_line(&mut self, line: &'a str) {
        self.reset();
        self.tokens = line.trim().graphemes(true)
            .collect();
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
                Err(err) => {
                    if err.propagate() {
                        Err(err)
                    }
                    else {
                        Err(ParserErr::err(format!("Unexpected token '{}'.", self.token_at(self.lah)).as_str()))
                    }
                },
                Ok(expr) => {
                    if self.lah < self.tokens.len() {
                        Err(ParserErr::err(format!("Unexpected token '{}'.", self.token_at(self.lah)).as_str()))
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
        let result = self.expr_id_fn();

        //Optional whitespace
        self.whitespace();

        result
    }

    fn expr_id_fn(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        //First handle operators of higher priority to account for operator precedence
        let expr_2_result = self.expr_2();

        //Expression 2 error
        if let Err(expr_2_err) = expr_2_result {
            if expr_2_err.propagate() {
                return Err(expr_2_err);
            }
            else {
                return Err(ParserErr::default())
            }
        }

        let expr_2 = expr_2_result.unwrap();

        let mut children: Vec<(xpr::IdToken, Box<xpr::ExprPrime>)> = Vec::new();
        let mut current_lah = self.lah;

        loop {
            //Match optional whitespace
            self.whitespace();

            //Check for identifier
            let id_result = self.id();

            //Not followed by an id.
            if let Err(id_err) = id_result {
                if id_err.propagate() {
                    return Err(id_err);
                }
                //Rollback lah and break from loop.
                else {
                    self.lah = current_lah;
                    break;
                }
            }

            let id = id_result.unwrap();

            //Match optional whitespace
            self.whitespace();

            //Check for another expression of priority 2
            let expr_2_suffix_result = self.expr_2();

            //Expression 2 is required
            if let Err(expr_2_suffix_err) = expr_2_suffix_result {
                if expr_2_suffix_err.propagate() {
                    return Err(expr_2_suffix_err);
                }
                else {
                    return Err(ParserErr::err(format!("Expected expression after function '{}'.", id.value).as_str()))
                }
            }

            let expr_2_suffix = expr_2_suffix_result.unwrap();

            //Successfully matched section. Record results and update current_lah before repeating.
            children.push((id, Box::new(expr_2_suffix)));
            current_lah = self.lah;
        };

        if children.is_empty() {
            Ok(expr_2)
        }
        else {
            Ok(xpr::ExprPrime::BinaryInfixFunctionExpression(Box::new(expr_2), children))
        }
    }

    fn expr_2(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        //First handle operators of higher priority to account for operator precedence
        let expr_1_result = self.expr_1();

        //expr_1 is required.
        if let Err(expr_1_err) = expr_1_result {
            if expr_1_err.propagate() {
                return Err(expr_1_err);
            }
            else {
                return Err(ParserErr::default())
            }
        }

        let expr_1 = expr_1_result.unwrap();

        let mut children: Vec<(xpr::BinopInfix, Box<xpr::ExprPrime>)> = Vec::new();
        let mut current_lah = self.lah;

        loop {
            //Match optional whitespace
            self.whitespace();

            //Check for binary infix operator with priority 2
            let binop_in_2_result = self.binop_in_2();

            //Not followed by a binary infix operator with priority 2.
            if let Err(binop_in_2_err) = binop_in_2_result {
                if binop_in_2_err.propagate() {
                    return Err(binop_in_2_err);
                }
                //Rollback lah and break from loop.
                else {
                    self.lah = current_lah;
                    break;
                }
            }

            let binop_in_2 = binop_in_2_result.unwrap();

            //Match optional whitespace
            self.whitespace();

            //Check for another expression of priority 1
            let expr_1_suffix_result = self.expr_1();

            //Expression 1 suffix is required
            if let Err(expr_1_suffix_err) = expr_1_suffix_result {
                if expr_1_suffix_err.propagate() {
                    return Err(expr_1_suffix_err);
                }
                else {
                    return Err(ParserErr::err(format!("Expected expression after operator '{binop_in_2}'").as_str()))
                }
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

        //expr_0 is required.
        if let Err(expr_0_err) = expr_0_result {
            if expr_0_err.propagate() {
                return Err(expr_0_err);
            }
            else {
                return Err(ParserErr::default())
            }
        }

        let expr_0 = expr_0_result.unwrap();

        let mut children: Vec<(xpr::BinopInfix, Box<xpr::ExprPrime>)> = Vec::new();
        let mut current_lah = self.lah;

        loop {
            //Match optional whitespace
            self.whitespace();

            //Check for binary infix operator with priority 1
            let binop_in_1_result = self.binop_in_1();

            //Not followed by a binary infix operator with priority 1.
            if let Err(binop_in_1_err) = binop_in_1_result {
                if binop_in_1_err.propagate() {
                    return Err(binop_in_1_err);
                }
                //Rollback lah and break from loop.
                else {
                    self.lah = current_lah;
                    break;
                }
            }

            let binop_in_1 = binop_in_1_result.unwrap();

            //Match optional whitespace
            self.whitespace();

            //Check for another expression of priority 0
            let expr_0_suffix_result = self.expr_0();

            //expr_0 is required
            if let Err(expr_0_suffix_err) = expr_0_suffix_result {
                if expr_0_suffix_err.propagate() {
                    return Err(expr_0_suffix_err);
                }
                else {
                    return Err(ParserErr::err(format!("Expected expression after operator '{binop_in_1}'").as_str()))
                }
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

        //expr_base is required.
        if let Err(expr_base_err) = expr_base_result {
            if expr_base_err.propagate() {
                return Err(expr_base_err);
            }
            else {
                return Err(ParserErr::default())
            }
        }

        let expr_base = expr_base_result.unwrap();

        let mut children: Vec<(xpr::BinopInfix, Box<xpr::ExprPrime>)> = Vec::new();
        let mut current_lah = self.lah;

        loop {
            //Match optional whitespace
            self.whitespace();

            //Check for binary infix operator with priority 0
            let binop_in_0_result = self.binop_in_0();

            //Not followed by a binary infix operator with priority 0.
            if let Err(binop_in_0_err) = binop_in_0_result {
                if binop_in_0_err.propagate() {
                    return Err(binop_in_0_err);
                }
                //Rollback lah and break from loop.
                else {
                    self.lah = current_lah;
                    break;
                }
            }

            let binop_in_0 = binop_in_0_result.unwrap();

            //Match optional whitespace
            self.whitespace();
            
            //Check for another expression of same priority
            let expr_0_suffix_result = self.expr_0();

            //expr 0 suffix is required
            if let Err(expr_0_suffix_err) = expr_0_suffix_result {
                if expr_0_suffix_err.propagate() {
                    return Err(expr_0_suffix_err);
                }
                else {
                    return Err(ParserErr::err(format!("Expected expression after operator {binop_in_0}").as_str()))
                }
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

        let mut result: Option<Result<xpr::ExprPrime, ParserErr>>;

        let mut unop_prefixes: Vec<xpr::UnopPrefix> = Vec::new();
        let mut unop_suffixes: Vec<xpr::UnopSuffix> = Vec::new();

        //Match 0+ unary prefix operators
        loop {
            let unop_prefix_result = self.unop_pre();

            if let Err(unop_prefix_err) = unop_prefix_result {
                if unop_prefix_err.propagate() {
                    return Err(unop_prefix_err);
                }
                else {
                    break;
                }
            }

            let unop_prefix = unop_prefix_result.unwrap();
            unop_prefixes.push(unop_prefix);
            continue;
        };

        //Try to match number.
        let number_result = self.number();

        result = match number_result {
            Ok(num) => Some(Ok(xpr::ExprPrime::Number(num))),
            Err(err) => {
                if err.propagate() {
                    return Err(err);
                }
                else {
                    None
                }
            }
        };

        if result.is_none() {
            //Failed to match. Try to match history stack/memory access.
            let history_memory_result = self.history_memory();

            result = match history_memory_result {
                Ok(hist_mem) => Some(Ok(hist_mem)),
                Err(err) => {
                    if err.propagate() {
                        return Err(err);
                    }
                    else {
                        None
                    }
                }
            };
        }

        if result.is_none() {
            //Failed to match. Try to match function.
            let func_result = self.func();

            result = match func_result {
                Ok(func) => Some(Ok(xpr::ExprPrime::Func(func))),
                Err(err) => {
                    if err.propagate() {
                        return Err(err);
                    }
                    else {
                        None
                    }
                }
            };
        }

        if result.is_none() {
            //Failed to match. Try to match an expression in parentheses.
            let paren_expression_paren_result = self.paren_expression_paren();

            result = match paren_expression_paren_result {
                Ok(paren_expression_paren_result) => Some(Ok(paren_expression_paren_result)),
                Err(err) => {
                    if err.propagate() {
                        return Err(err);
                    }
                    else {
                        None
                    }
                }
            };
        }

        if let Some(some_result) = result {
            //Match 0+ unary suffix operators
            loop {
                let unop_suffix_result = self.unop_suf();

                if let Err(unop_suffix_err) = unop_suffix_result {
                    if unop_suffix_err.propagate() {
                        return Err(unop_suffix_err);
                    }
                    else {
                        break;
                    }
                }

                let unop_suffix = unop_suffix_result.unwrap();

                unop_suffixes.push(unop_suffix);
                continue;
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
            Err(ParserErr::default())
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
            return Err(ParserErr::default());
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
            if collected_1.is_empty() {
                let concatenated = collected.join("");
                return Err(ParserErr::err(format!("Expected digit after '{concatenated}.'").as_str()));
            }

            //Successfully matched. Record progress in outer state
            current_lah = current_lah_1;
            collected.push(radix_pt_token);
            for term in collected_1 {
                collected.push(term)
            }
        }

        let concatenated = collected.join("");

        let parsed= concatenated.parse::<f64>();

        if let Err(_parse_int_err) = parsed {
            return Err(ParserErr::err(format!("Failed to parse number '{concatenated}'.").as_str()));
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

        if !xpr::Token::OpParO.get_terminal().match_symbol(token) {
            //Opening paren is required, unless the id is a predefined constant,
            //in which case it will be treated as a zero-arg function.
            //Descrement lah.
            if let Ok(constant) = xpr::Constant::try_from(id.value) {
                self.lah -= 1;
                return Ok(xpr::Func::ConstantFunc(constant));
            }
            //Not a constant. Rollback and return error if not present.
            else {
                self.lah = initial_lah;
                return Err(ParserErr::default());
            }
        }

        //Optional whitespace
        self.whitespace();

        //Check if next token is closing paren
        let token_closing_paren_no_args = self.token_at(self.lah);

        //If next character is not a closing paren, then the function should have arguments
        let should_have_args: bool = !xpr::Token::OpParC.get_terminal().match_symbol(token_closing_paren_no_args);

        //Try to match function arguments
        let mut current_lah = self.lah;
        let mut func_args: Vec<xpr::ExprPrime> = Vec::new();

        if should_have_args {
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
    
                //Expression is required.
                if let Err(expr_prime_err) = expr_prime_result {
                    if expr_prime_err.propagate() {
                        return Err(expr_prime_err);
                    }
                    else {
                        let concatenated = match func_args.is_empty() {
                            true => String::from(""),
                            false => {
                                if func_args.len() == 1 {
                                    format!("{},", func_args[0])
                                }
                                else {
                                    let arg_strings: Vec<String> = func_args.iter()
                                    .map(|arg| arg.to_string())
                                    .collect();

                                    format!("{},", arg_strings.join(", "))
                                }
                            }
                        };

                        return Err(ParserErr::err(format!("Expected function argument after '{}({concatenated}'.", id.value).as_str()))
                    }
                }

                let expr_prime = expr_prime_result.unwrap();
    
                //Optional whitespace
                self.whitespace();
    
                //Match was success. Record progress in outer loop.
                current_lah = self.lah;
                func_args.push(expr_prime);
            }    
        }

        //Try to match a closing paren
        let token = self.get_and_increment();

        //Closing paren is required.
        if !xpr::Token::OpParC.get_terminal().match_symbol(token) {
            let concatenated = match func_args.is_empty() {
                true => String::from(""),
                false => {
                    if func_args.len() == 1 {
                        func_args[0].to_string()
                    }
                    else {
                        let arg_strings: Vec<String> = func_args.iter()
                        .map(|arg| arg.to_string())
                        .collect();

                        arg_strings.join(", ")
                    }
                }
            };

            return Err(ParserErr::err(format!("Expected closing parenthesis ')' after '{}({concatenated}'.", id.value).as_str()));
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

        let concatenated = symbols.join("");

        if !seen_letter {
            return Err(ParserErr::default());
        }

        //Record lah progress and return id
        self.lah = current_lah;

        Ok(xpr::IdToken::new(concatenated.as_str()))
    }

    fn history_memory(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        let initial_lah = self.lah;
        
        //Try to match the history stack access symbol
        let token = self.token_at(initial_lah);

        //History stack access symbol is required. Rollback and return error if not present.
        if !xpr::Token::History(0_usize).get_terminal().match_symbol(token) {
            self.lah = initial_lah;
            return Err(ParserErr::default());
        }

        let mut current_lah = initial_lah + 1;
        let mut digits: Vec<&str> = Vec::new();

        let maybe_mem_qualifier = self.token_at(current_lah);
        let is_memory: bool = terminals::HISTORY_MEMORY_QUALIFIER.match_symbol(maybe_mem_qualifier);

        if is_memory {
            current_lah += 1;
        }

        //Try to match 1 or more digits
        loop {
            let current_token = self.token_at(current_lah);

            if terminals::DIGIT.match_symbol(current_token) {
                current_lah += 1;
                digits.push(current_token);
                continue;
            }

            break;
        };

        //Make sure at least one digit is present
        if digits.is_empty() {
            if is_memory {
                return Err(ParserErr::err(format!("Expected digit after memory access token '{}'", terminals::MEMORY.to_string()).as_str()));
            }
            else {
                return Err(ParserErr::err(format!("Expected digit after history access token '{}'", terminals::HISTORY.to_string()).as_str()));
            }
        }
        
        let concatenated = digits.join("");

        let parsed= concatenated.parse::<usize>();

        if let Err(_parse_int_err) = parsed {
            return Err(ParserErr::err(format!("Failed to parse number '{concatenated}'.").as_str()));
        }

        self.lah = current_lah;

        let expr: ExprPrime;

        //If parsed memory token, check if next token is the memory assignment operator
        if is_memory {
            let maybe_mem_assignment_token = self.token_at(self.lah);

            if terminals::OP_SETMEM.match_symbol(maybe_mem_assignment_token) {
                self.lah += 1;

                //Try to get assigned expression
                let subexpr_result = self.expr_prime();

                if let Err(subexpr_err) = subexpr_result {
                    if subexpr_err.propagate() {
                        return Err(subexpr_err);
                    }
                    else {
                        return Err(ParserErr::err(format!("Expected expression after memory assignment '{}{concatenated}{}'.", 
                            terminals::MEMORY.to_string(), terminals::OP_SETMEM.to_string()).as_str()));
                    }
                }

                //Return assignment of expression to memory
                let subexpr = subexpr_result.unwrap();
                expr = xpr::ExprPrime::StoreMem(xpr::MemoryToken::new(parsed.unwrap()), Box::new(subexpr));
            }
            else {
                //Return memory access
                expr = xpr::ExprPrime::AccessMem(xpr::MemoryToken::new(parsed.unwrap()));
            }
        }
        else {
            //Return history access
            expr = xpr::ExprPrime::History(xpr::HistoryToken::new(parsed.unwrap()));
        }

        Ok(expr)
    }

    fn paren_expression_paren(&mut self) -> Result<xpr::ExprPrime, ParserErr> {
        let initial_lah = self.lah;

        //Try to match an opening paren
        let token = self.get_and_increment();

        //Opening paren is required. Rollback and return error if not present.
        if !xpr::Token::OpParO.get_terminal().match_symbol(token) {
            self.lah = initial_lah;
            return Err(ParserErr::default());
        }

        //Optional whitespace
        self.whitespace();

        //Try to match the inner expression
        let expr_prime_result = self.expr_prime();

        //Inner expression is required.
        if let Err(expr_prime_err) = expr_prime_result {
            if expr_prime_err.propagate() {
                return Err(expr_prime_err);
            }
            else {
                return Err(ParserErr::err("Exepcted expression after opening parenthesis '('."));
            }
        }

        let expr_prime = expr_prime_result.unwrap();

        //Optional whitespace
        self.whitespace();

        //Try to match a closing paren
        let token = self.get_and_increment();

        //Closing paren is required.
        if !xpr::Token::OpParC.get_terminal().match_symbol(token) {
            return Err(ParserErr::err(format!("Expected closing parenthesis ')' after expression '({expr_prime}'.").as_str()));
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
        Err(ParserErr::default())
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
        Err(ParserErr::default())
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
        Err(ParserErr::default())
    }

    fn binop_in_1(&mut self) -> Result<xpr::BinopInfix1, ParserErr> {
        let initial_lah = self.lah;
        let current = self.get_and_increment();

        let tokens = vec![
            xpr::BinopInfix1::Div,
            xpr::BinopInfix1::Mult,
            xpr::BinopInfix1::Rem
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
        Err(ParserErr::default())
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
        Err(ParserErr::default())
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

impl Default for Parser<'_> {
    fn default() -> Self {
        Self::new("")
    }
}