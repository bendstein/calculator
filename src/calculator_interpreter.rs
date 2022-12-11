use super::calculator_parser::{parser, expression};
use std::{collections::HashMap, cell::RefCell};

pub mod interpreter_err;

#[cfg(test)]
pub mod tests;

use interpreter_err::InterpreterErr;
use rand::Rng;

pub type Func0 = fn () -> Result<f64, InterpreterErr>;
pub type Func1 = fn (f64) -> Result<f64, InterpreterErr>;
pub type Func2 = fn (f64, f64) -> Result<f64, InterpreterErr>;
pub type Func3 = fn (f64, f64, f64) -> Result<f64, InterpreterErr>;
pub type Func4 = fn (f64, f64, f64, f64) -> Result<f64, InterpreterErr>;
pub type Func5 = fn (f64, f64, f64, f64, f64) -> Result<f64, InterpreterErr>;
pub type Func6 = fn (f64, f64, f64, f64, f64, f64) -> Result<f64, InterpreterErr>;
pub type Func7 = fn (f64, f64, f64, f64, f64, f64, f64) -> Result<f64, InterpreterErr>;
pub type Func8 = fn (f64, f64, f64, f64, f64, f64, f64, f64) -> Result<f64, InterpreterErr>;
pub type Func9 = fn (f64, f64, f64, f64, f64, f64, f64, f64, f64) -> Result<f64, InterpreterErr>;
pub type FuncVar = fn (Vec<f64>) -> Result<f64, InterpreterErr>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FunctionArgs {
    None(Func0),
    One(Func1),
    Two(Func2),
    Three(Func3),
    Four(Func4),
    Five(Func5),
    Six(Func6),
    Seven(Func7),
    Eight(Func8),
    Nine(Func9),
    Variable(FuncVar),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    pub args: FunctionArgs
}

impl Function {
    pub fn new(args: FunctionArgs) -> Self {
        Self {
            args
        }
    }
}

impl Default for Function {
    fn default() -> Self {
        Self::new(FunctionArgs::None(|| Ok(0_f64)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Interpreter {
    functions: HashMap<String, Function>,
    history: RefCell<Vec<f64>>
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            functions: vec![
                ("ADD".to_string(), ADD.clone()),
                ("SUB".to_string(), SUB.clone()),
                ("MULT".to_string(), MULT.clone()),
                ("DIV".to_string(), DIV.clone()),
                ("REM".to_string(), REM.clone()),
                ("NEG".to_string(), NEG.clone()),
                ("FAC".to_string(), FAC.clone()),
                ("MAX".to_string(), MAX.clone()),
                ("MIN".to_string(), MIN.clone()),
                ("MOD".to_string(), MOD.clone()),
                ("CEIL".to_string(), CEIL.clone()),
                ("FLOOR".to_string(), FLOOR.clone()),
                ("ROUND".to_string(), ROUND.clone()),
                ("FRACT".to_string(), FRACT.clone()),
                ("SQRT".to_string(), SQRT.clone()),
                ("EXP".to_string(), EXP.clone()),
                ("EXP2".to_string(), EXP2.clone()),
                ("POW".to_string(), POW.clone()),
                ("SIN".to_string(), SIN.clone()),
                ("COS".to_string(), COS.clone()),
                ("TAN".to_string(), TAN.clone()),
                ("ASIN".to_string(), ASIN.clone()),
                ("ACOS".to_string(), ACOS.clone()),
                ("ATAN".to_string(), ATAN.clone()),
                ("CSC".to_string(), CSC.clone()),
                ("SEC".to_string(), SEC.clone()),
                ("COT".to_string(), COT.clone()),
                ("ACSC".to_string(), ACSC.clone()),
                ("ASEC".to_string(), ASEC.clone()),
                ("ACOT".to_string(), ACOT.clone()),
                ("SINH".to_string(), SINH.clone()),
                ("COSH".to_string(), COSH.clone()),
                ("TANH".to_string(), TANH.clone()),
                ("ASINH".to_string(), ASINH.clone()),
                ("ACOSH".to_string(), ACOSH.clone()),
                ("ATANH".to_string(), ATANH.clone()),
                ("LOG".to_string(), LOG.clone()),
                ("LOGB".to_string(), LOGB.clone()),
                ("LOG2".to_string(), LOG2.clone()),
                ("LN".to_string(), LN.clone()),
                ("FRAND".to_string(), FRAND.clone()),
                ("RFRAND".to_string(), RFRAND.clone()),
                ("RFRANDI".to_string(), RFRANDI.clone()),
                ("RAND".to_string(), RAND.clone()),
                ("RRAND".to_string(), RRAND.clone()),
                ("RRANDI".to_string(), RRANDI.clone()),
                ("SIGN".to_string(), SIGN.clone()),
                ("COND".to_string(), COND.clone()),
                ("E".to_string(), E.clone()),
                ("PI".to_string(), PI.clone()),
            ].into_iter()
            .collect(),
            history: RefCell::new(Vec::new())
        }
    }
}

impl Interpreter {
    pub fn clear_stack(&self) {
        let mut history = self.history.borrow_mut();
        history.clear();
    }

    pub fn evaluate(&self, expression: expression::Expr) -> Result<f64, InterpreterErr> {
        let evaluated_result = match expression {
            expression::Expr::None => Ok(0_f64),
            expression::Expr::ExprPrime(expr_prime) => self.evaluate_expr_prime(*expr_prime)
        };

        if let Ok(evaluated) = evaluated_result {
            let mut history = self.history.borrow_mut();

            if history.is_empty() || *history.last().unwrap() != evaluated {
                history.push(evaluated);
            }

            Ok(evaluated)
        }
        else {
            evaluated_result
        }
    }

    pub fn evaluate_string(&self, line: &str) -> Result<f64, InterpreterErr> {
        let expr = match parser::Parser::parse_line(line) {
            Ok(parse_tree) => Ok(parse_tree),
            Err(parse_err) => Err(InterpreterErr::new(parse_err.message()))
        }?;

        self.evaluate(expr)
    }

    fn evaluate_expr_prime(&self, expression: expression::ExprPrime) -> Result<f64, InterpreterErr> {
        match expression {
            expression::ExprPrime::Number(n) => self.evaluate_number(n),
            expression::ExprPrime::History(h) => self.evaluate_hist(h),
            expression::ExprPrime::Func(f) => self.evaluate_func(f),
            expression::ExprPrime::Id(_id) => todo!(),
            expression::ExprPrime::UnopPrefixesExpression(prefix, subexpr) => self.evaluate_unary_prefixes(prefix, *subexpr),
            expression::ExprPrime::UnopSuffixesExpression(subexpr, suffixes) => self.evaluate_unary_suffixes(*subexpr, suffixes),
            expression::ExprPrime::ParenthesesExpression(subexpr) => self.evaluate_expr_prime(*subexpr),
            expression::ExprPrime::BinaryInfixExpression(first_child, siblings) => self.evaluate_binary_infix_expression(*first_child, siblings),
            expression::ExprPrime::BinaryInfixFunctionExpression(first_child, siblings) => self.evaluate_binary_infix_function_expression(*first_child, siblings),
        }
    }

    fn evaluate_number(&self, expression: expression::NumberToken) -> Result<f64, InterpreterErr> {
        Ok(expression.value)
    }

    fn evaluate_func(&self, expression: expression::Func) -> Result<f64, InterpreterErr> {
        let id: String;
        let args: Vec<expression::ExprPrime>;

        match expression {
            expression::Func::EmptyFunc(name) => {
                id = name.value;
                args = Vec::new();
            },
            expression::Func::ConstantFunc(constant) => {
                id = constant.to_string();
                args = Vec::new();
            }
            expression::Func::FuncWithArgs(name, f_args) => {
                id = name.value;
                args = f_args;
            }
        };

        let matching: Vec<(&String, &Function)> = self.functions.iter()
        .filter(|(name, _)| name.eq_ignore_ascii_case(id.as_str()))
        .collect();

        if matching.is_empty() {
            return Err(InterpreterErr::new(format!("No such function '{id}'.").as_str()))
        }

        let (_, function) = matching.first().unwrap();

        fn validate_args_count(name: &str, expected: usize, actual: usize) -> Result<(), InterpreterErr> {
            if actual != expected {
                Err(InterpreterErr::new(format!("Function '{name}' expected {expected} arguments; got {actual}.").as_str()))
            }
            else {
                Ok(())
            }
        }

        fn evaluate_args(interpreter: &Interpreter, args: Vec<expression::ExprPrime>) -> Result<Vec<f64>, InterpreterErr> {
            let mut evaluated: Vec<f64> = Vec::new();

            for arg in args {
                let val = interpreter.evaluate_expr_prime(arg)?;
                evaluated.push(val);
            }

            Ok(evaluated)
        }

        match function.args {
            FunctionArgs::None(func) => {
                validate_args_count(id.as_str(), 0, args.len())?;
                func()
            },
            FunctionArgs::One(func) => {
                validate_args_count(id.as_str(), 1, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0])
            },
            FunctionArgs::Two(func) => {
                validate_args_count(id.as_str(), 2, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0], evaluated_args[1])
            },
            FunctionArgs::Three(func) => {
                validate_args_count(id.as_str(), 3, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0], evaluated_args[1], evaluated_args[2])
            },
            FunctionArgs::Four(func) => {
                validate_args_count(id.as_str(), 4, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0], evaluated_args[1], evaluated_args[2], evaluated_args[3])
            },
            FunctionArgs::Five(func) => {
                validate_args_count(id.as_str(), 5, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0], evaluated_args[1], evaluated_args[2], evaluated_args[3], evaluated_args[4])
            },
            FunctionArgs::Six(func) => {
                validate_args_count(id.as_str(), 6, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0], evaluated_args[1], evaluated_args[2], evaluated_args[3], evaluated_args[4], 
                    evaluated_args[5])
            },
            FunctionArgs::Seven(func) => {
                validate_args_count(id.as_str(), 7, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0], evaluated_args[1], evaluated_args[2], evaluated_args[3], evaluated_args[4], 
                    evaluated_args[5], evaluated_args[6])
            },
            FunctionArgs::Eight(func) => {
                validate_args_count(id.as_str(), 8, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0], evaluated_args[1], evaluated_args[2], evaluated_args[3], evaluated_args[4], 
                    evaluated_args[5], evaluated_args[6], evaluated_args[7])
            },
            FunctionArgs::Nine(func) => {
                validate_args_count(id.as_str(), 9, args.len())?;
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args[0], evaluated_args[1], evaluated_args[2], evaluated_args[3], evaluated_args[4], 
                    evaluated_args[5], evaluated_args[6], evaluated_args[7], evaluated_args[8])
            },
            FunctionArgs::Variable(func) => {
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args)
            },
        }
    }

    fn evaluate_hist(&self, expression: expression::HistoryToken) -> Result<f64, InterpreterErr> {
        match self.history.try_borrow() {
            Err(borrow_error) => Err(InterpreterErr::new(format!("Failed to access past results: {borrow_error}").as_str())),
            Ok(history) => {
                if history.len() <= expression.value {
                    Err(InterpreterErr::new(format!("History entry {} does not exist.", expression.value).as_str()))
                }
                else {
                    Ok(history[history.len() - (expression.value + 1)])
                }
            }
        }
    }

    // fn evaluate_id(&self, expression: expression::IdToken) -> Result<f64, InterpreterErr> {
    //     unimplemented!()
    // }

    fn evaluate_unary_prefixes(&self, prefixes: Vec<expression::UnopPrefix>, expression: expression::ExprPrime) -> Result<f64, InterpreterErr> {
        let mut subvalue = self.evaluate_expr_prime(expression)?;

        for prefix in prefixes {
            match prefix {
                expression::UnopPrefix::Neg => subvalue *= -1_f64
            };
        };

        Ok(subvalue)
    }

    fn evaluate_unary_suffixes(&self, expression: expression::ExprPrime, suffixes: Vec<expression::UnopSuffix>) -> Result<f64, InterpreterErr> {
        let mut subvalue = self.evaluate_expr_prime(expression)?;

        for suffix in suffixes {
            match suffix {
                expression::UnopSuffix::Fac => subvalue = factorial(subvalue)?
            };
        };

        Ok(subvalue)
    }

    fn evaluate_binary_infix_expression(&self, first_child: expression::ExprPrime, siblings: Vec<(expression::BinopInfix, Box<expression::ExprPrime>)>) -> Result<f64, InterpreterErr> {
        let mut value: f64 = self.evaluate_expr_prime(first_child)?;

        for (operator, sibling_expr) in siblings {
            let sibling_value = self.evaluate_expr_prime(*sibling_expr)?;

            match operator {
                expression::BinopInfix::Exp => value = value.powf(sibling_value),
                expression::BinopInfix::Mult => value *= sibling_value,
                expression::BinopInfix::Div => value /= sibling_value,
                expression::BinopInfix::Rem => value %= sibling_value,
                expression::BinopInfix::Add => value += sibling_value,
                expression::BinopInfix::Sub => value -= sibling_value,
            };
        };

        Ok(value)
    }

    fn evaluate_binary_infix_function_expression(&self, first_child: expression::ExprPrime, siblings: Vec<(expression::IdToken, Box<expression::ExprPrime>)>) -> Result<f64, InterpreterErr> {
        let mut value: f64 = self.evaluate_expr_prime(first_child)?;

        for (binfunc, sibling_expr) in siblings {
            value = self.evaluate_func(expression::Func::FuncWithArgs(binfunc, vec![
                expression::ExprPrime::Number(expression::NumberToken::new(value)), 
                *sibling_expr]
            ))?
        };

        Ok(value)
    }

}

fn factorial(n: f64) -> Result<f64, InterpreterErr> {
    if n == 0_f64 {
        Ok(1_f64)
    }
    else if n < 0_f64 {
        Err(InterpreterErr::new("Cannot apply factorial operator to negative value."))
    }
    else if n != n.round() {
        Err(InterpreterErr::new("Cannot apply factorial operator to floating point value."))
    }
    else {
        Ok(n * factorial(n - 1_f64)?)
    }
}


fn modulo(a: f64, b: f64) -> Result<f64, InterpreterErr> {
    //If b is 0, a mod b is undefined
    if b.abs() < f64::EPSILON {
        Ok(f64::NAN)
    }
    //If a is an integer b is -1, a mod b is 0
    else if (a.fract().abs() < f64::EPSILON) && (b + 1_f64).abs() < f64::EPSILON {
        Ok(0_f64)
    }
    else {
        let rem = a % b;

        //If a and b have the same sign, a mod b = a % b
        if a.signum() == b.signum() {
            Ok(rem)
        }
        //Otherwise, a mod b = (a % b) + b
        else {
            Ok(rem + b)
        }
    }
}

fn random<T>() -> Result<f64, InterpreterErr>
    where T : Into<f64>, 
    rand::distributions::Standard: rand::distributions::Distribution<T>
{
    Ok(rand::thread_rng().gen::<T>().into())
}

fn random_range<T>(range: std::ops::Range<T>) -> Result<f64, InterpreterErr>
    where T : Into<f64> + std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform, 
    rand::distributions::Standard: rand::distributions::Distribution<T>
{
    Ok(rand::thread_rng().gen_range(range).into())
}

fn random_range_inc<T>(range: std::ops::RangeInclusive<T>) -> Result<f64, InterpreterErr>
    where T : Into<f64> + std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform, 
    rand::distributions::Standard: rand::distributions::Distribution<T>
{
    Ok(rand::thread_rng().gen_range(range).into())
}

fn add_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a + b);

    Ok(maybe_value.unwrap_or(0_f64))
}

fn sub_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a - b);

    Ok(maybe_value.unwrap_or(0_f64))
}

fn mult_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a * b);

    Ok(maybe_value.unwrap_or(0_f64))
}

fn div_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a / b);

    Ok(maybe_value.unwrap_or(0_f64))
}

fn rem_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a % b);

    Ok(maybe_value.unwrap_or(0_f64))
}

fn max_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(f64::max);

    Ok(maybe_value.unwrap_or(0_f64))
}

fn min_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(f64::min);

    Ok(maybe_value.unwrap_or(0_f64))
}

lazy_static! {
    static ref ADD: Function = Function::new(FunctionArgs::Variable(add_all));
    static ref SUB: Function = Function::new(FunctionArgs::Variable(sub_all));
    static ref MULT: Function = Function::new(FunctionArgs::Variable(mult_all));
    static ref DIV: Function = Function::new(FunctionArgs::Variable(div_all));
    static ref REM: Function = Function::new(FunctionArgs::Variable(rem_all));

    static ref MAX: Function = Function::new(FunctionArgs::Variable(max_all));
    static ref MIN: Function = Function::new(FunctionArgs::Variable(min_all));

    static ref MOD: Function = Function::new(FunctionArgs::Two(modulo));

    static ref NEG: Function = Function::new(FunctionArgs::One(|n: f64| Ok(-n)));
    static ref FAC: Function = Function::new(FunctionArgs::One(factorial));
    
    static ref CEIL: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::ceil(n))));
    static ref FLOOR: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::floor(n))));
    static ref ROUND: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::round(n))));
    
    static ref FRACT: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::fract(n))));

    static ref SQRT: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::sqrt(n))));
    static ref EXP: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::exp(n))));
    static ref EXP2: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::exp2(n))));
    static ref POW: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| Ok(f64::powf(a, b))));

    static ref SIN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::sin(n))));
    static ref COS: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::cos(n))));
    static ref TAN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::tan(n))));

    static ref ASIN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::asin(n))));
    static ref ACOS: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::acos(n))));
    static ref ATAN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::atan(n))));

    static ref CSC: Function = Function::new(FunctionArgs::One(|n: f64| Ok(1_f64 / f64::sin(n))));
    static ref SEC: Function = Function::new(FunctionArgs::One(|n: f64| Ok(1_f64 / f64::cos(n))));
    static ref COT: Function = Function::new(FunctionArgs::One(|n: f64| Ok(1_f64 / f64::tan(n))));

    static ref ACSC: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::asin(1_f64 / n))));
    static ref ASEC: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::acos(1_f64 / n))));
    static ref ACOT: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::atan(1_f64 / n))));

    static ref SINH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::sinh(n))));
    static ref COSH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::cosh(n))));
    static ref TANH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::tanh(n))));

    static ref ASINH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::asinh(n))));
    static ref ACOSH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::acosh(n))));
    static ref ATANH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::atanh(n))));

    static ref LOG: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::log10(n))));
    static ref LOG2: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::log2(n))));
    static ref LN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(n.log(std::f64::consts::E))));
    static ref LOGB: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| Ok(f64::log(a, b))));

    static ref SIGN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::signum(n))));
    static ref COND: Function = Function::new(FunctionArgs::Four(|a: f64, b: f64, c: f64, d: f64| {
        Ok(if a == b {
            c
        }
        else {
            d
        })
    }));

    static ref FRAND: Function = Function::new(FunctionArgs::None(random::<f64>));
    static ref RFRAND: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| random_range::<f64>(a..b)));
    static ref RFRANDI: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| random_range_inc::<f64>(a..=b)));

    static ref RAND: Function = Function::new(FunctionArgs::None(random::<i32>));
    static ref RRAND: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| random_range::<i32>(a.ceil() as i32..b.floor() as i32)));
    static ref RRANDI: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| random_range_inc::<i32>(a.ceil() as i32..=b.floor() as i32)));

    static ref E: Function = Function::new(FunctionArgs::None(|| Ok(std::f64::consts::E)));
    static ref PI: Function = Function::new(FunctionArgs::None(|| Ok(std::f64::consts::PI)));
}