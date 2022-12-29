pub mod calculator_parser;
pub mod calculator_interpreter;
pub mod calculator_err;

use std::fmt::Debug;

use calculator_interpreter::interpreter::{Interpreter, EvaluateOptions as InterpreterOptions};
use calculator_parser::parser::Parser;
use calculator_err::CalculatorErr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct EvaluateOptions {
    pub interpreter_options: Option<InterpreterOptions>
}

impl EvaluateOptions {
    pub fn new(interpreter_options: InterpreterOptions) -> Self {
        Self {
            interpreter_options: Some(interpreter_options)
        }
    }

    pub fn interpreter(&self) -> InterpreterOptions {
        match self.interpreter_options {
            Some(options) => options,
            None => InterpreterOptions::default()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CalculatorState {
    pub memory: Vec<f64>,
    pub history: Vec<f64>
}

impl CalculatorState {
    pub fn new(memory: Vec<f64>, history: Vec<f64>) -> Self {
        Self {
            memory,
            history
        }
    }
}

#[derive(Clone)]
pub struct Calculator 
{
    interpreter: Interpreter,
    parser: Parser
}

impl Debug for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.interpreter.fmt(f)
    }
}

impl Calculator {
    pub fn new(interpreter: Interpreter, parser: Parser) -> Self {
        Self {
            interpreter,
            parser
        }
    }

    pub fn clone_current_state(&self) -> CalculatorState {
        CalculatorState::new(self.interpreter.clone_mem(), self.interpreter.clone_stack())
    }

    pub fn evaluate_with_options(&self, expression: &str, options: EvaluateOptions) -> Result<(f64, CalculatorState), CalculatorErr> {
        let prepared_expression = Calculator::prepare_string(expression);

        let parsed = match self.parser.parse(prepared_expression) {
            Ok(value) => value,
            Err(e) => Err(CalculatorErr::err(format!("An error occurred while parsing expression '{prepared_expression}'. At {}: {e}", e.lah()).as_str()))?
        };

        let (evaluated, mem) = match self.interpreter.evaluate_with_options(parsed, options.interpreter()) {
            Ok(value) => value,
            Err(e) => Err(CalculatorErr::err(format!("An error occurred while evaluating expression '{prepared_expression}': {e}").as_str()))?
        };

        Ok((evaluated, CalculatorState::new(mem.unwrap_or_else(|| self.interpreter.clone_mem()), self.interpreter.clone_stack())))
    }

    pub fn evaluate(&self, expression: &str) -> Result<f64, CalculatorErr> {
        match self.evaluate_with_options(expression, EvaluateOptions::default()) {
            Err(e) => Err(e),
            Ok((value, _)) => Ok(value)
        }
    }

    pub fn clear_stack(&mut self) {
        self.interpreter.clear_stack()
    }

    pub fn clear_mem(&mut self) {
        self.interpreter.clear_mem()
    }

    pub fn has_history(&self) -> bool {
        self.interpreter.has_history()
    }

    pub fn parser(&self) -> &Parser {
        &self.parser
    }

    pub fn interpreter(&self) -> &Interpreter {
        &self.interpreter
    }

    fn prepare_string(expression: &str) -> &str {
        expression.trim()
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new(Interpreter::default(), Parser::default())
    }
}