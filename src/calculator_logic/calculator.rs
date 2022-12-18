pub mod calculator_err;

use super::{calculator_interpreter::{interpreter::Interpreter}, calculator_parser};
use calculator_err::CalculatorErr;

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

    pub fn evaluate(&self, expression: &str) -> Result<f64, CalculatorErr> {
        let prepared_expression = Calculator::prepare_string(expression);

        let mut parser = calculator_parser::parser::Parser::new(prepared_expression);

        let parsed = match parser.parse() {
            Ok(value) => value,
            Err(e) => Err(CalculatorErr::err(format!("An error occurred while parsing expression '{prepared_expression}'. At {}: {e}", parser.lah()).as_str()))?
        };

        let evaluated = match self.interpreter.evaluate(parsed) {
            Ok(value) => value,
            Err(e) => Err(CalculatorErr::err(format!("An error occurred while evaluating expression '{prepared_expression}': {e}").as_str()))?
        };

        Ok(evaluated)
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