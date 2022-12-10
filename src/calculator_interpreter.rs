use super::calculator_parser::{parser, expression};
use std::{collections::HashMap};

pub mod interpreter_err;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FunctionArgs {
    None(fn () -> f32),
    One(fn (f32) -> f32),
    Two(fn (f32, f32) -> f32),
    Three(fn (f32, f32, f32) -> f32),
    Four(fn (f32, f32, f32, f32) -> f32),
    Five(fn (f32, f32, f32, f32, f32) -> f32),
    Variable(fn (Vec<f32>) -> f32),
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
        Self::new(FunctionArgs::None(|| 0_f32))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interpreter {
    functions: HashMap<String, Function>
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            functions: vec![
                ("ADD".to_string(), ADD.clone()),
                ("SUB".to_string(), SUB.clone()),
                ("MULT".to_string(), MULT.clone()),
                ("DIV".to_string(), DIV.clone()),
                ("MOD".to_string(), MOD.clone()),
                ("NEG".to_string(), NEG.clone()),
                ("MAX".to_string(), MAX.clone()),
                ("MIN".to_string(), MIN.clone()),
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
                ("SINH".to_string(), SINH.clone()),
                ("COSH".to_string(), COSH.clone()),
                ("TANH".to_string(), TANH.clone()),
                ("LOG".to_string(), LOG.clone()),
                ("LOG10".to_string(), LOG10.clone()),
                ("LOG2".to_string(), LOG2.clone()),
                ("LOGE".to_string(), LOGE.clone()),
                ("E".to_string(), E.clone()),
                ("PI".to_string(), PI.clone()),
            ].into_iter()
            .collect()
        }
    }
}

impl Interpreter {
    pub fn evaluate(&self, expression: expression::Expr) -> Result<f32, interpreter_err::InterpreterErr> {
        match expression {
            expression::Expr::None => Ok(0_f32),
            expression::Expr::ExprPrime(expr_prime) => self.evaluate_expr_prime(*expr_prime)
        }
    }

    pub fn evaluate_string(&self, line: &str) -> Result<f32, interpreter_err::InterpreterErr> {
        let expr = match parser::Parser::parse_line(line) {
            Ok(parse_tree) => Ok(parse_tree),
            Err(parse_err) => Err(interpreter_err::InterpreterErr::new(&parse_err.message))
        }?;

        self.evaluate(expr)
    }

    fn evaluate_expr_prime(&self, expression: expression::ExprPrime) -> Result<f32, interpreter_err::InterpreterErr> {
        match expression {
            expression::ExprPrime::Number(n) => self.evaluate_number(n),
            expression::ExprPrime::Func(f) => self.evaluate_func(f),
            expression::ExprPrime::Id(_id) => todo!(),
            expression::ExprPrime::UnopPrefixesExpression(prefix, subexpr) => self.evaluate_unary_prefixes(prefix, *subexpr),
            expression::ExprPrime::UnopSuffixesExpression(subexpr, suffixes) => self.evaluate_unary_suffixes(*subexpr, suffixes),
            expression::ExprPrime::ParenthesesExpression(subexpr) => self.evaluate_expr_prime(*subexpr),
            expression::ExprPrime::BinaryInfixExpression(first_child, siblings) => self.evaluate_binary_infix_expression(*first_child, siblings)
        }
    }

    fn evaluate_number(&self, expression: expression::NumberToken) -> Result<f32, interpreter_err::InterpreterErr> {
        Ok(expression.value)
    }

    fn evaluate_func(&self, expression: expression::Func) -> Result<f32, interpreter_err::InterpreterErr> {
        let id: String;
        let args: Vec<expression::ExprPrime>;

        match expression {
            expression::Func::EmptyFunc(name) => {
                id = name.value;
                args = Vec::new();
            },
            expression::Func::FuncWithArgs(name, f_args) => {
                id = name.value;
                args = f_args;
            }
        };

        let matching: Vec<(&String, &Function)> = self.functions.iter()
        .filter(|(name, _)| name.eq_ignore_ascii_case(id.as_str()))
        .collect();

        if matching.is_empty() {
            return Err(interpreter_err::InterpreterErr::new(format!("No such function '{id}'.").as_str()))
        }

        let (_, function) = matching.first().unwrap();

        fn validate_args_count(name: &str, expected: usize, actual: usize) -> Result<(), interpreter_err::InterpreterErr> {
            if actual != expected {
                Err(interpreter_err::InterpreterErr::new(format!("Function '{name}' expected {expected} arguments; got {actual}.").as_str()))
            }
            else {
                Ok(())
            }
        }

        fn evaluate_args(interpreter: &Interpreter, args: Vec<expression::ExprPrime>) -> Result<Vec<f32>, interpreter_err::InterpreterErr> {
            let mut evaluated: Vec<f32> = Vec::new();

            for arg in args {
                let val = interpreter.evaluate_expr_prime(arg)?;
                evaluated.push(val);
            }

            Ok(evaluated)
        }

        Ok(match function.args {
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
            FunctionArgs::Variable(func) => {
                let evaluated_args = evaluate_args(self, args)?;
                func(evaluated_args)
            },
        })
    }

    // fn evaluate_id(&self, expression: expression::IdToken) -> Result<f32, interpreter_err::InterpreterErr> {
    //     unimplemented!()
    // }

    fn evaluate_unary_prefixes(&self, prefixes: Vec<expression::UnopPrefix>, expression: expression::ExprPrime) -> Result<f32, interpreter_err::InterpreterErr> {
        let mut subvalue = self.evaluate_expr_prime(expression)?;

        for prefix in prefixes {
            match prefix {
                expression::UnopPrefix::Neg => subvalue *= -1_f32
            };
        };

        Ok(subvalue)
    }

    fn evaluate_unary_suffixes(&self, expression: expression::ExprPrime, suffixes: Vec<expression::UnopSuffix>) -> Result<f32, interpreter_err::InterpreterErr> {
        let mut subvalue = self.evaluate_expr_prime(expression)?;

        for suffix in suffixes {
            match suffix {
                expression::UnopSuffix::Fac => {
                    if subvalue < 0_f32 {
                        return Err(interpreter_err::InterpreterErr::new("Cannot apply factorial operator to negative value."));
                    }
                    else if subvalue != subvalue.round() {
                        return Err(interpreter_err::InterpreterErr::new("Cannot apply factorial operator to floating point value."));
                    }

                    subvalue = factorial(subvalue as u32) as f32;
                }
            };
        };

        Ok(subvalue)
    }

    fn evaluate_binary_infix_expression(&self, first_child: expression::ExprPrime, siblings: Vec<(expression::BinopInfix, Box<expression::ExprPrime>)>) -> Result<f32, interpreter_err::InterpreterErr> {
        let mut value: f32 = self.evaluate_expr_prime(first_child)?;

        for (operator, sibling_expr) in siblings {
            let sibling_value = self.evaluate_expr_prime(*sibling_expr)?;

            match operator {
                expression::BinopInfix::Exp => value = value.powf(sibling_value),
                expression::BinopInfix::Mult => value *= sibling_value,
                expression::BinopInfix::Div => value /= sibling_value,
                expression::BinopInfix::Mod => value %= sibling_value,
                expression::BinopInfix::Add => value += sibling_value,
                expression::BinopInfix::Sub => value -= sibling_value,
            };
        };

        Ok(value)
    }

}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1_u32
    }
    else {
        n * factorial(n - 1)
    }
}

lazy_static! {
    static ref ADD: Function = Function::new(FunctionArgs::Two(|a: f32, b: f32| a + b));
    static ref SUB: Function = Function::new(FunctionArgs::Two(|a: f32, b: f32| a - b));
    static ref MULT: Function = Function::new(FunctionArgs::Two(|a: f32, b: f32| a * b));
    static ref DIV: Function = Function::new(FunctionArgs::Two(|a: f32, b: f32| a / b));
    static ref MOD: Function = Function::new(FunctionArgs::Two(|a: f32, b: f32| a % b));

    static ref NEG: Function = Function::new(FunctionArgs::One(|n: f32| -n));

    static ref MAX: Function = Function::new(FunctionArgs::Two(f32::max));
    static ref MIN: Function = Function::new(FunctionArgs::Two(f32::min));
    
    static ref CEIL: Function = Function::new(FunctionArgs::One(f32::ceil));
    static ref FLOOR: Function = Function::new(FunctionArgs::One(f32::floor));
    static ref ROUND: Function = Function::new(FunctionArgs::One(f32::round));
    
    static ref FRACT: Function = Function::new(FunctionArgs::One(f32::fract));

    static ref SQRT: Function = Function::new(FunctionArgs::One(f32::sqrt));
    static ref EXP: Function = Function::new(FunctionArgs::One(f32::exp));
    static ref EXP2: Function = Function::new(FunctionArgs::One(f32::exp2));
    static ref POW: Function = Function::new(FunctionArgs::Two(f32::powf));

    static ref SIN: Function = Function::new(FunctionArgs::One(f32::sin));
    static ref COS: Function = Function::new(FunctionArgs::One(f32::cos));
    static ref TAN: Function = Function::new(FunctionArgs::One(f32::tan));

    static ref ASIN: Function = Function::new(FunctionArgs::One(f32::asin));
    static ref ACOS: Function = Function::new(FunctionArgs::One(f32::acos));
    static ref ATAN: Function = Function::new(FunctionArgs::One(f32::atan));

    static ref SINH: Function = Function::new(FunctionArgs::One(f32::sinh));
    static ref COSH: Function = Function::new(FunctionArgs::One(f32::cosh));
    static ref TANH: Function = Function::new(FunctionArgs::One(f32::tanh));

    static ref LOG: Function = Function::new(FunctionArgs::Two(f32::log));
    static ref LOG10: Function = Function::new(FunctionArgs::One(f32::log10));
    static ref LOG2: Function = Function::new(FunctionArgs::One(f32::log2));
    static ref LOGE: Function = Function::new(FunctionArgs::One(|n: f32| n.log(std::f32::consts::E)));

    static ref E: Function = Function::new(FunctionArgs::None(|| std::f32::consts::E));
    static ref PI: Function = Function::new(FunctionArgs::None(|| std::f32::consts::PI));
}