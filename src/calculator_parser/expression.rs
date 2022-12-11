use std::fmt::Display;

use super::terminal::{Terminal, terminals};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpAdd,
    OpSub,
    OpMult,
    OpDiv,
    OpRem,
    OpExp,
    OpFac,
    OpParO,
    OpParC,
    Delimiter,
    Number(f32),
    Id(String)
}

impl Token {
    pub fn get_terminal(&self) -> &Terminal {
        match self {
            Token::OpAdd => &terminals::OP_ADD,
            Token::OpSub => &terminals::OP_SUB,
            Token::OpMult => &terminals::OP_MULT,
            Token::OpDiv => &terminals::OP_DIV,
            Token::OpRem => &terminals::OP_REM,
            Token::OpExp => &terminals::OP_EXP,
            Token::OpFac => &terminals::OP_FAC,
            Token::OpParO => &terminals::OP_PAR_O,
            Token::OpParC => &terminals::OP_PAR_C,
            Token::Delimiter => &terminals::DELIMITER,
            Token::Number(_) => &terminals::DIGIT,
            Token::Id(_) => &terminals::LETTER
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let terminal = self.get_terminal();
        let to_display: String = match terminal {
            Terminal::Epsilon => String::from(""),
            Terminal::Literal(s) => s.to_string(),
            Terminal::RegularExpresion(_) => {
                match self {
                    Token::Number(n) => format!("{n}"),
                    Token::Id(id) => id.to_string(),
                    _ => panic!("Cannot get display value for token!")
                }
            }
        };

        f.write_str(to_display.as_str())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    None,
    ExprPrime(Box<ExprPrime>)
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_print = match self {
            Self::None => String::from(""),
            Self::ExprPrime(sub_expr) => sub_expr.to_string()
        };

        f.write_str(to_print.as_str())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprPrime {
    Number(NumberToken),
    Func(Func),
    Id(IdToken),
    UnopPrefixesExpression(Vec<UnopPrefix>, Box<ExprPrime>),
    UnopSuffixesExpression(Box<ExprPrime>, Vec<UnopSuffix>),
    ParenthesesExpression(Box<ExprPrime>),
    BinaryInfixExpression(Box<ExprPrime>, Vec<(BinopInfix, Box<ExprPrime>)>),
    BinaryInfixFunctionExpression(Box<ExprPrime>, Vec<(IdToken, Box<ExprPrime>)>),
}

impl Display for ExprPrime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        enum SubexprStrParentType {
            //None,
            UnopPrefix,
            UnopSuffix,
            Parentheses,
            BinaryInfix
        }

        fn subexpr_str(subexpr: &ExprPrime, _parent_type: SubexprStrParentType) -> String {
            if !matches!(subexpr, ExprPrime::Number(_) | ExprPrime::Func(_) | ExprPrime::Id(_) | ExprPrime::ParenthesesExpression(_)) {

                // match parent_type {
                //     SubexprStrParentType::UnopPrefixesExpression => {
                //         if matches!(**subexpr2, ExprPrime::Number(_) | ExprPrime::Func(_) | ExprPrime::Id(_) | ExprPrime::ParenthesesExpression(_)) {
                //             return format!("{subexpr}");
                //         }
                //     }
                // };

                // if let ExprPrime::UnopPrefixesExpression(_, subexpr2) = subexpr {
                //     if matches!(**subexpr2, ExprPrime::Number(_) | ExprPrime::Func(_) | ExprPrime::Id(_) | ExprPrime::ParenthesesExpression(_)) {
                //         return format!("{subexpr}");
                //     }
                // }
                // else if let ExprPrime::UnopSuffixesExpression(subexpr2, _) = subexpr {
                //     if matches!(**subexpr2, ExprPrime::Number(_) | ExprPrime::Func(_) | ExprPrime::Id(_) | ExprPrime::ParenthesesExpression(_)) {
                //         return format!("{subexpr}");
                //     }
                // }

                format!("[{subexpr}]")
            }
            else {
                subexpr.to_string()
            }
        }

        let to_print = match self {
            Self::Number(n) => n.get_token().to_string(),
            Self::Func(func) => func.to_string(),
            Self::Id(id) => id.get_token().to_string(),
            Self::UnopPrefixesExpression(prefix, expr) => {
                let prefix_strings: Vec<String> = prefix.iter()
                    .map(|op| op.to_string())
                    .collect();
                let prefix_concatenated = prefix_strings.join("");

                format!("{}{}", prefix_concatenated, subexpr_str(expr, SubexprStrParentType::UnopPrefix))
            },
            Self::UnopSuffixesExpression(expr, suffix) => {
                let suffix_strings: Vec<String> = suffix.iter()
                    .map(|op| op.to_string())
                    .collect();
                let suffix_concatenated = suffix_strings.join("");

                format!("{}{}", subexpr_str(expr, SubexprStrParentType::UnopSuffix), suffix_concatenated)
            },
           Self::ParenthesesExpression(subexpr) => subexpr_str(subexpr, SubexprStrParentType::Parentheses),
           Self::BinaryInfixExpression(subexpr, suffix) => {
                let suffix_strings: Vec<String> = suffix.iter()
                    .map(|(binop, suffix_expr)| format!("{} {}", binop, subexpr_str(suffix_expr, SubexprStrParentType::BinaryInfix)))
                    .collect();
                let concatenated = suffix_strings.join(" ");

                let space_between = match suffix.is_empty() {
                    true => "".to_string(),
                    false => " ".to_string()
                };

                format!("{}{}{}", subexpr_str(subexpr, SubexprStrParentType::BinaryInfix), space_between, concatenated)
           },
           Self::BinaryInfixFunctionExpression(subexpr, suffix) => {
            let suffix_strings: Vec<String> = suffix.iter()
                .map(|(binfunc, suffix_expr)| format!("{} {}", binfunc.value, subexpr_str(suffix_expr, SubexprStrParentType::BinaryInfix)))
                .collect();
            let concatenated = suffix_strings.join(" ");

            let space_between = match suffix.is_empty() {
                true => "".to_string(),
                false => " ".to_string()
            };

            format!("{}{}{}", subexpr_str(subexpr, SubexprStrParentType::BinaryInfix), space_between, concatenated)
       },
        };

        f.write_str(to_print.as_str())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Func {
    EmptyFunc(IdToken),
    FuncWithArgs(IdToken, Vec<ExprPrime>),
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
        let to_display = match self {
            Func::EmptyFunc(id) => format!("{}{}{}", id.get_token(), Token::OpParO, Token::OpParC),
            Func::FuncWithArgs(id, args) => {
                let args_strings: Vec<String> = args.iter()
                    .map(|expr| expr.to_string())
                    .collect();
                let concatenated = args_strings.join(", ");
                format!("{}{}{}{}", id.get_token(), Token::OpParO, concatenated, Token::OpParC)
            }
        };
        
        f.write_str(to_display.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinopInfix {
    Exp,
    Mult,
    Div,
    Rem,
    Add,
    Sub
}

impl From<BinopInfix> for Token {
    fn from(value: BinopInfix) -> Self {
        match value {
            BinopInfix::Exp => Self::OpExp,
            BinopInfix::Mult => Self::OpMult,
            BinopInfix::Div => Self::OpDiv,
            BinopInfix::Rem => Self::OpRem,
            BinopInfix::Add => Self::OpAdd,
            BinopInfix::Sub => Self::OpSub
        }
    }
}

impl TryFrom<Token> for BinopInfix {
    type Error = &'static str;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::OpExp => Ok(Self::Exp),
            Token::OpMult => Ok(Self::Mult),
            Token::OpDiv => Ok(Self::Div),
            Token::OpRem => Ok(Self::Rem),
            Token::OpAdd => Ok(Self::Add),
            Token::OpSub => Ok(Self::Sub),
            _ => Err("The passed value is not a valid binary infix operator.")
        }
    }
}

impl Display for BinopInfix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
        let to_display = Token::from(*self).to_string();
        f.write_str(to_display.as_str())
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinopInfix2 {
    Add,
    Sub
}

impl From<BinopInfix2> for BinopInfix {
    fn from(value: BinopInfix2) -> Self {
        match value {
            BinopInfix2::Add => Self::Add,
            BinopInfix2::Sub => Self::Sub
        }
    }
}

impl TryFrom<BinopInfix> for BinopInfix2 {
    type Error = &'static str;

    fn try_from(value: BinopInfix) -> Result<Self, Self::Error> {
        match value {
            BinopInfix::Add => Ok(Self::Add),
            BinopInfix::Sub => Ok(Self::Sub),
            _ => Err("The passed value is not a valid priority 2 binary infix operator.")
        }
    }
}

impl Display for BinopInfix2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
        let to_display = BinopInfix::from(*self).to_string();
        f.write_str(to_display.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinopInfix1 {
    Mult,
    Div,
    Rem
}

impl From<BinopInfix1> for BinopInfix {
    fn from(value: BinopInfix1) -> Self {
        match value {
            BinopInfix1::Mult => Self::Mult,
            BinopInfix1::Div => Self::Div,
            BinopInfix1::Rem => Self::Rem
        }
    }
}

impl TryFrom<BinopInfix> for BinopInfix1 {
    type Error = &'static str;

    fn try_from(value: BinopInfix) -> Result<Self, Self::Error> {
        match value {
            BinopInfix::Mult => Ok(Self::Mult),
            BinopInfix::Div => Ok(Self::Div),
            BinopInfix::Rem => Ok(Self::Rem),
            _ => Err("The passed value is not a valid priority 1 binary infix operator.")
        }
    }
}

impl Display for BinopInfix1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
        let to_display = BinopInfix::from(*self).to_string();
        f.write_str(to_display.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinopInfix0 {
    Exp
}

impl From<BinopInfix0> for BinopInfix {
    fn from(value: BinopInfix0) -> Self {
        match value {
            BinopInfix0::Exp => Self::Exp
        }
    }
}

impl TryFrom<BinopInfix> for BinopInfix0 {
    type Error = &'static str;

    fn try_from(value: BinopInfix) -> Result<Self, Self::Error> {
        match value {
            BinopInfix::Exp => Ok(Self::Exp),
            _ => Err("The passed value is not a valid priority 0 binary infix operator.")
        }
    }
}

impl Display for BinopInfix0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
        let to_display = BinopInfix::from(*self).to_string();
        f.write_str(to_display.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinopInfixPriority {
    Priority0(BinopInfix0),
    Priority1(BinopInfix1),
    Priority2(BinopInfix2)
}

impl BinopInfix {
    pub fn get_priority(&self) -> BinopInfixPriority {
        let priority_0_result: Result<BinopInfix0, _> = BinopInfix0::try_from(*self);

        if let Ok(priority_0) = priority_0_result {
            return BinopInfixPriority::Priority0(priority_0);
        }

        let priority_1_result: Result<BinopInfix1, _> = BinopInfix1::try_from(*self);
        if let Ok(priority_1) = priority_1_result {
            return BinopInfixPriority::Priority1(priority_1);
        }

        let priority_2_result: Result<BinopInfix2, _> = BinopInfix2::try_from(*self);
        if let Ok(priority_2) = priority_2_result {
            return BinopInfixPriority::Priority2(priority_2);
        }

        //Should never happen
        panic!("Binary operator is not assigned a priority.");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnopPrefix {
    Neg
}

impl From<UnopPrefix> for Token {
    fn from(value: UnopPrefix) -> Self {
        match value {
            UnopPrefix::Neg => Self::OpSub
        }
    }
}

impl TryFrom<Token> for UnopPrefix {
    type Error = &'static str;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::OpSub => Ok(Self::Neg),
            _ => Err("The passed value is not a valid unary prefix operator.")
        }
    }
}

impl Display for UnopPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
        let to_display = Token::from(*self).to_string();
        f.write_str(to_display.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnopSuffix {
    Fac
}

impl From<UnopSuffix> for Token {
    fn from(value: UnopSuffix) -> Self {
        match value {
            UnopSuffix::Fac => Self::OpFac
        }
    }
}

impl TryFrom<Token> for UnopSuffix {
    type Error = &'static str;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::OpFac => Ok(Self::Fac),
            _ => Err("The passed value is not a valid unary suffix operator.")
        }
    }
}

impl Display for UnopSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
        let to_display = Token::from(*self).to_string();
        f.write_str(to_display.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpAddToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpSubToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpMultToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpDivToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpRemToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpExpToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpFacToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpParOToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OpParCToken {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DelimiterToken {}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NumberToken {
    pub value: f32
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IdToken {
    pub value: String
}

impl OpAddToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpAdd
    }
}

impl Default for OpAddToken {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpAddToken> for Token {
	fn from(_: OpAddToken) -> Self {
		Token::OpAdd
	}
}

impl TryFrom<Token> for OpAddToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::OpAdd => Ok(OpAddToken {}),
			_ => Err("The passed value is not an OpAdd token.")
		}
	}
}

impl OpSubToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpSub
    }
}

impl Default for OpSubToken {
    fn default() -> Self {
        Self::new()
    }
}


impl From<OpSubToken> for Token {
	fn from(_: OpSubToken) -> Self {
	    Token::OpSub
	}
}

impl TryFrom<Token> for OpSubToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::OpSub => Ok(OpSubToken {}),
            _ => Err("The passed value is not an OpSub token.")
		}
	}
}

impl OpMultToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpMult
    }
}

impl Default for OpMultToken {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpMultToken> for Token {
	fn from(_: OpMultToken) -> Self {
	    Token::OpMult
	}
}

impl TryFrom<Token> for OpMultToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::OpMult => Ok(OpMultToken {}),
			_ => Err("The passed value is not an OpMult token.")
		}
	}
}

impl OpDivToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpDiv
    }
}

impl Default for OpDivToken {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpDivToken> for Token {
	fn from(_: OpDivToken) -> Self {
		Token::OpDiv
	}
}

impl TryFrom<Token> for OpDivToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::OpDiv => Ok(OpDivToken {}),
			_ => Err("The passed value is not an OpDiv token.")
		}
	}
}

impl OpRemToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpRem
    }
}

impl Default for OpRemToken {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpRemToken> for Token {
	fn from(_: OpRemToken) -> Self {
		Token::OpRem
	}
}

impl TryFrom<Token> for OpRemToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::OpRem => Ok(OpRemToken {}),
			_ => Err("The passed value is not an OpRem token.")
		}
	}
}

impl OpExpToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpExp
    }
}

impl Default for OpExpToken {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpExpToken> for Token {
	fn from(_: OpExpToken) -> Self {
		Token::OpExp
	}
}

impl TryFrom<Token> for OpExpToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::OpExp => Ok(OpExpToken {}),
			_ => Err("The passed value is not an OpExp token.")
		}
	}
}

impl OpFacToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpFac
    }
}

impl Default for OpFacToken {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpFacToken> for Token {
	fn from(_: OpFacToken) -> Self {
		Token::OpFac
	}
}

impl TryFrom<Token> for OpFacToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::OpFac => Ok(OpFacToken {}),
			_ => Err("The passed value is not an OpFac token.")
		}
	}
}

impl OpParOToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpParO
    }
}

impl Default for OpParOToken {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpParOToken> for Token {
	fn from(_: OpParOToken) -> Self {
		Token::OpParO
	}
}

impl TryFrom<Token> for OpParOToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::OpParO => Ok(OpParOToken {}),
			_ => Err("The passed value is not an OpParO token.")
		}
	}
}

impl Default for OpParCToken {
    fn default() -> Self {
        Self::new()
    }
}

impl OpParCToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::OpParC
    }
}

impl From<OpParCToken> for Token {
	fn from(_: OpParCToken) -> Self {
		Token::OpParC
	}
}

impl TryFrom<Token> for OpParCToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::OpParC => Ok(OpParCToken {}),
			_ => Err("The passed value is not an OpParC token.")
		}
	}
}

impl DelimiterToken {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_token(&self) -> Token {
        Token::Delimiter
    }
}

impl Default for DelimiterToken {
    fn default() -> Self {
        Self::new()
    }
}

impl From<DelimiterToken> for Token {
	fn from(_: DelimiterToken) -> Self {
		Token::Delimiter
	}
}

impl TryFrom<Token> for DelimiterToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::Delimiter => Ok(DelimiterToken {}),
			_ => Err("The passed value is not an Delimiter token.")
		}
	}
}

impl NumberToken {
    pub fn new(value: f32) -> Self {
        Self {
            value
        }
    }
    pub fn get_token(&self) -> Token {
        Token::Number(self.value)
    }
}

impl Default for NumberToken {
    fn default() -> Self {
        Self::new(0_f32)
    }
}

impl From<NumberToken> for Token {
	fn from(value: NumberToken) -> Self {
		Token::Number(value.value)
	}
}

impl TryFrom<Token> for NumberToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::Number(value) => Ok(NumberToken { value }),
			_ => Err("The passed value is not an Number token.")
		}
	}
}

impl IdToken {
    pub fn new(value: &str) -> Self {
        Self {
            value: String::from(value)
        }
    }
    pub fn get_token(&self) -> Token {
        Token::Id(self.value.clone())
    }
}

impl Default for IdToken {
    fn default() -> Self {
        Self::new("")
    }
}

impl From<IdToken> for Token {
	fn from(value: IdToken) -> Self {
		Token::Id(value.value)
	}
}

impl TryFrom<Token> for IdToken {
	type Error = &'static str;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::Id(value) => Ok(IdToken { value }),
			_ => Err("The passed value is not an Id token.")
		}
	}
}
