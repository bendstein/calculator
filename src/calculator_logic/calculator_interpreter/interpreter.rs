pub mod interpreter_err;
pub mod function;

use interpreter_err::InterpreterErr;
use function::{*, function_impl::*, function_lazy_static::*};
use super::super::calculator_parser::expression;
use std::{collections::HashMap, cell::RefCell};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EvaluateOptions {
    pub preview: bool
}

impl EvaluateOptions {
    pub fn new(preview: bool) -> Self {
        Self {
            preview
        }
    }
}

impl Default for EvaluateOptions {
    fn default() -> Self {
        Self::new(false)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Interpreter {
    functions: HashMap<String, Function>,
    history: RefCell<Vec<f64>>,
    memory: RefCell<Vec<f64>>
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
            history: RefCell::new(Vec::new()),
            memory: RefCell::new(vec![0_f64; u8::MAX as usize])
        }
    }
}

impl Interpreter {
    /**
     * Reset calculator memory to its original state
     */
    pub fn clear_mem(&mut self) {
        let mut memory = self.memory.borrow_mut();
        memory.clear();
        memory.resize(u8::MAX as usize, 0_f64);
    }

    /**
     * Result calculator history to empty
     */
    pub fn clear_stack(&mut self) {
        let mut history = self.history.borrow_mut();
        history.clear();
    }

    /**
     * Create a clone of the calculator's current memory
     */
    pub fn view_mem(&self) -> Vec<f64> {
        self.memory.borrow().clone()
    }

    /**
     * Create a clone of the calculator's current history
     */
    pub fn view_stack(&self) -> Vec<f64> {
        self.history.borrow().clone()
    }

    /**
     * Evaluate the given expression with the given options
     */
    pub fn evaluate_with_options(&self, expression: expression::Expr, options: EvaluateOptions) -> Result<(f64, Option<Vec<f64>>), InterpreterErr> {
        let (evaluated_result, evaluated_memory) = match expression {
            expression::Expr::None => {
                let result = Ok(0_f64);
                if options.preview {
                    let temp_mem = self.memory.borrow().clone();
                    (result, Some(temp_mem))
                }
                else {
                    (result, None)
                }
            },
            expression::Expr::ExprPrime(expr_prime) => {

                if options.preview {
                    let temp_mem = self.memory.borrow().clone();

                    let result: Result<f64, InterpreterErr> = self.evaluate_expr_prime(*expr_prime);

                    let result_mem = self.memory.borrow().clone();

                    //Reset memory back to its original state (as stored in temp_mem)
                    let mut mem = self.memory.borrow_mut();

                    for (i, elem) in temp_mem.iter().enumerate() {
                        mem[i] = *elem;
                    }

                    (result, Some(result_mem))
                }
                else {
                    (self.evaluate_expr_prime(*expr_prime), None)
                }
            }
        };

        if let Ok(evaluated) = evaluated_result {
            if !options.preview {
                let mut history = self.history.borrow_mut();

                if history.is_empty() || *history.last().unwrap() != evaluated {
                    history.push(evaluated);
                }
            }

            Ok((evaluated, evaluated_memory))
        }
        else {
            Err(evaluated_result.unwrap_err())
        }
    }

    /**
     * Evaluate the given expression
     */
    pub fn evaluate(&self, expression: expression::Expr) -> Result<f64, InterpreterErr> {
        match self.evaluate_with_options(expression, EvaluateOptions::default()) {
            Err(e) => Err(e),
            Ok((result, _)) => Ok(result)
        }
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
            expression::ExprPrime::AccessMem(m) => self.evaluate_mem(m),
            expression::ExprPrime::StoreMem(m, subexpr) => self.evaluate_store_mem(m, *subexpr)
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

    fn evaluate_mem(&self, expression: expression::MemoryToken) -> Result<f64, InterpreterErr> {
        match self.memory.try_borrow() {
            Err(borrow_error) => Err(InterpreterErr::new(format!("Failed to access memory: {borrow_error}").as_str())),
            Ok(memory) => {
                if memory.len() <= expression.value {
                    Err(InterpreterErr::new(format!("Memory entry {} does not exist.", expression.value).as_str()))
                }
                else {
                    Ok(memory[expression.value])
                }
            }
        }
    }

    fn evaluate_store_mem(&self, memory_token: expression::MemoryToken, subexpr: expression::ExprPrime) -> Result<f64, InterpreterErr> {
        let subexpr_value = self.evaluate_expr_prime(subexpr)?;

        match self.memory.try_borrow_mut() {
            Err(borrow_error) => Err(InterpreterErr::new(format!("Failed to access memory: {borrow_error}").as_str())),
            Ok(mut memory) => {
                if memory.len() <= memory_token.value {
                    Err(InterpreterErr::new(format!("Memory entry {} does not exist.", memory_token.value).as_str()))
                }
                else {
                    memory[memory_token.value] = subexpr_value;
                    Ok(subexpr_value)
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