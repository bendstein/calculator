use std::ops::RangeBounds;

use crate::calculator_logic;
use super::ui_trait::*;

use calculator_logic::calculator::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct GraphicalUI {
    calculator: calculator_logic::calculator::Calculator,
    buffer: String
}

#[allow(dead_code)]
impl GraphicalUI {
    fn buffer_append(&mut self, content: &str) {
        self.buffer.push_str(content);
    }

    fn buffer_clear(&mut self) {
        self.buffer.clear();
    }

    fn buffer_remove_range<R: RangeBounds<usize>>(&mut self, range: R) {
        //Extract range start and end
        let mut start_bound = match range.start_bound() {
            std::ops::Bound::Unbounded => None,
            std::ops::Bound::Excluded(n) => Some(*n + 1),
            std::ops::Bound::Included(n) => Some(*n)
        };

        let mut end_bound = match range.start_bound() {
            std::ops::Bound::Unbounded => None,
            std::ops::Bound::Excluded(n) => Some(*n - 1),
            std::ops::Bound::Included(n) => Some(*n)
        };

        //Reverse range direction if necessary
        if start_bound.is_some() && end_bound.is_some() && start_bound.unwrap() > end_bound.unwrap() {
            std::mem::swap(&mut start_bound, &mut end_bound)
        }

        //Get the inverse of the range
        let start_range = start_bound.map(|n| 0_usize..=n);
        let end_range = end_bound.map(|n| n..self.buffer.len());

        //Get the substrings corresponding to the start and end range
        let start = start_range.map_or("",|r| &self.buffer[r]);
        let end = end_range.map_or("", |r| &self.buffer[r]);

        //Concatenate substrings
        self.buffer = format!("{start}{end}");
    }

    fn buffer_undo(&mut self, count: usize) {
        if count == 0 {}
        else if count == 1 {
            self.buffer.pop();
        }
        else {
            self.buffer_remove_range(self.buffer.len() - count..)
        }
    }

    fn evaluate_buffer(&self) -> Result<f64, calculator_err::CalculatorErr> {
        self.calculator.evaluate(&self.buffer)
    }

    fn evaluate_buffer_preview(&self) -> Result<(f64, CalculatorState), calculator_err::CalculatorErr> {
        self.calculator.evaluate_with_options(&self.buffer, EvaluateOptions::new(InterpreterOptions::new(true)))
    }
}

impl CalculatorUI for GraphicalUI {
    fn attach_calculator(&mut self, calculator: Calculator) {
        self.calculator = calculator;
    }

    fn start(&mut self) -> Result<(), &str> {

        Ok(())
    }
}