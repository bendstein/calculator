use std::ops::RangeBounds;

use crate::yew_app;

use super::calculator::{*, calculator_parser, calculator_interpreter::interpreter::EvaluateOptions as InterpreterOptions, calculator_err::CalculatorErr};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum CalculatorAction {
    #[default] None,
    Insert(String, bool),
    Backspace(bool),
    Clear,
    Submit
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Default)]
pub enum CalculatorResult {
    #[default] None,
    RefreshDisplay,
    Number(f64),
    State(CalculatorState),
    NumberAndState(f64, CalculatorState),
    PreviewNumberAndState(Result<(f64, CalculatorState), CalculatorErr>)
}

#[derive(Debug, Clone)]
pub struct GraphicalUI 
{ 
    calculator: Calculator,
    buffer: String,
    cursor: usize
}

impl GraphicalUI {
    pub fn new(calculator: Calculator) -> Self {
        Self {
            calculator,
            buffer: String::from(""),
            cursor: 0
        }
    }
}

impl Default for GraphicalUI {
    fn default() -> Self {
        Self::new(Calculator::default())
    }
}

impl GraphicalUI {
    pub fn start(&mut self) -> Result<(), String> {
        yew_app::run();
        Ok(())
    }

    fn buffer_append(&mut self, content: &str) {
        self.buffer.push_str(content);
    }

    fn buffer_clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
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

    fn buffer_remove_end(&mut self, count: usize, trim_end: bool) {
        if trim_end {
            let trimmed = self.buffer.trim_end();
            self.buffer = String::from(trimmed);
        }

        if count == 0 {}
        else if count == 1 {
            self.buffer.pop();
        }
        else {
            self.buffer_remove_range(self.buffer.len() - count..)
        }
    }

    fn evaluate_buffer(&self) -> Result<CalculatorResult, calculator_err::CalculatorErr> {
        self.calculator
            .evaluate_with_options(&self.buffer, EvaluateOptions::default())
            .map(|(n, state)| CalculatorResult::NumberAndState(n, state))
    }

    fn evaluate_buffer_preview(&self) -> CalculatorResult {
        CalculatorResult::PreviewNumberAndState(self.calculator
            .evaluate_with_options(&self.buffer, EvaluateOptions::new(InterpreterOptions::new(true))))
    }

    fn insert_at_cursor(&mut self, content: &str) {
        if self.cursor == self.buffer.len() {
            if !content.is_empty() {
                self.buffer_append(content);
                self.cursor += content.len();
            }
        }
        else if content.is_empty() {
            self.buffer.remove(self.cursor);
            if self.cursor > self.buffer.len() {
                self.cursor = self.buffer.len();
            }
        }
        else {
            let before: &str = if self.cursor == 0_usize {
                ""
            }
            else {
                &self.buffer[0..self.cursor]
            };

            let after: &str = if self.cursor + content.len() >= self.buffer.len() {
                ""
            }
            else {
                &self.buffer[self.cursor + content.len() + 1..self.buffer.len()]
            };

            self.buffer = format!("{before}{content}{after}");
            self.cursor = self.cursor + content.len() + 1;
        }
    }

    pub fn do_action(&mut self, action: CalculatorAction) -> Result<CalculatorResult, calculator_err::CalculatorErr> {
        match action {
            CalculatorAction::None => Ok(CalculatorResult::None),
            CalculatorAction::Insert(content, preview) => {
                let mut new_content = content;
                
                let parser = self.calculator.parser();
                let interpreter = self.calculator.interpreter();

                /*
                    * If buffer is empty, and history is not, and content is an infix or suffix operator,
                    * prepend with $0.
                    */
                if self.buffer.is_empty() && self.calculator.has_history()
                    && parser.parse_expression::<calculator_parser::expression::UnopPrefix>(new_content.as_str()).is_err() {
                    if parser.parse_expression::<calculator_parser::expression::BinopInfix>(new_content.as_str()).is_ok() {
                        new_content = format!("$0 {new_content} ");
                    }
                    else if parser.parse_expression::<calculator_parser::expression::UnopSuffix>(new_content.as_str()).is_ok() {
                        new_content = format!("$0{new_content}");
                    }
                    else if let Ok(id) = parser.parse_expression::<calculator_parser::expression::IdToken>(new_content.as_str()) {
                        let func = interpreter.get_func_by_name(id.value.as_str());

                        if func.is_some() {
                            new_content = format!("$0 {new_content} ");
                        }
                    }
                }
                /*
                    * If buffer isn't empty, and appending to end of buffer, and content is an infix operator,
                    * surround operator with spaces
                    */
                else if !self.buffer.is_empty() && self.cursor == self.buffer.len() 
                    && (parser.parse_expression::<calculator_parser::expression::BinopInfix>(new_content.as_str()).is_ok() 
                        || parser.parse_expression::<calculator_parser::expression::IdToken>(new_content.as_str()).is_ok()) {
                    new_content = format!("{}{new_content} ", if self.buffer.ends_with(' ') { "" } else { " " });
                }
                /*
                    * If buffer isn't empty, and appending to end of buffer, and content is a suffix operator,
                    * put a space after it, and remove any spaces before it
                    */
                else if !self.buffer.is_empty() && self.cursor == self.buffer.len() 
                    && parser.parse_expression::<calculator_parser::expression::UnopSuffix>(new_content.as_str()).is_ok() {
                        if self.buffer.ends_with(' ') {
                            let trimmed = self.buffer.trim_end();
                            self.buffer = String::from(trimmed);
                            self.cursor = self.cursor.min(self.buffer.len());
                        }
                        new_content = format!("{new_content} ");
                    }

                self.insert_at_cursor(new_content.as_str());

                if preview {
                    Ok(self.evaluate_buffer_preview())
                }
                else {
                    Ok(CalculatorResult::RefreshDisplay)
                }
            },
            CalculatorAction::Backspace(preview) => {
                if self.cursor > 0 {
                    self.buffer_remove_end(1_usize, true);
                    self.cursor = self.cursor.min(self.buffer.len());

                    if preview {
                        Ok(self.evaluate_buffer_preview())
                    }
                    else {
                        Ok(CalculatorResult::RefreshDisplay)
                    }
                }
                else {
                    Ok(CalculatorResult::None)
                }
            },
            CalculatorAction::Clear => {
                self.buffer_clear();
                Ok(CalculatorResult::RefreshDisplay)
            }
            CalculatorAction::Submit => self.evaluate_buffer(),
        }
    }
}