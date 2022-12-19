pub mod calculator_err;

use super::{calculator_interpreter::{interpreter::Interpreter}, calculator_parser};
pub use super::calculator_interpreter::interpreter::EvaluateOptions as InterpreterOptions;
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

#[derive(Debug, PartialEq, Clone)]
pub struct Calculator {
    interpreter: Interpreter
}

impl Calculator {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter
        }
    }

    pub fn clone_current_state(&self) -> CalculatorState {
        CalculatorState::new(self.interpreter.clone_mem(), self.interpreter.clone_stack())
    }

    pub fn evaluate_with_options(&self, expression: &str, options: EvaluateOptions) -> Result<(f64, CalculatorState), CalculatorErr> {
        let prepared_expression = Calculator::prepare_string(expression);

        let mut parser = calculator_parser::parser::Parser::new(prepared_expression);

        let parsed = match parser.parse() {
            Ok(value) => value,
            Err(e) => Err(CalculatorErr::err(format!("An error occurred while parsing expression '{prepared_expression}'. At {}: {e}", parser.lah()).as_str()))?
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

    fn prepare_string(expression: &str) -> &str {
        expression.trim()
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new(Interpreter::default())
    }
}