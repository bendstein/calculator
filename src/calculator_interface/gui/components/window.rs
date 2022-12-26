use kas::prelude::*;

#[allow(unused_imports)]
use kas::{impl_default, Widget};

use kas::{impl_scope, widgets::EditBox};
use std::ops::RangeBounds;
use crate::calculator_logic::{self, calculator_parser};
use crate::calculator_logic::calculator::calculator_err::CalculatorErr;
use super::buttons::Buttons;
use calculator_logic::calculator::*;

impl_scope! {
    #[widget {
        layout = column: [
            self.display,
            Buttons::default()
        ];
    }]

    #[impl_default]
    #[derive(Debug, Clone)]
    pub (in crate::calculator_interface::gui) struct Window {
        core: widget_core!(),
        #[widget] display: EditBox = EditBox::new("")
            .with_editable(false)
            .with_multi_line(true)
            .with_lines(2, 4)
            .with_width_em(5_f32, 10_f32),
        calculator: Option<calculator_logic::calculator::Calculator> = None,
        buffer: String = "".to_string(),
        cursor: usize = 0_usize
    }

    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(action) = mgr.try_pop_msg::<CalculatorAction>() {
               match self.do_action(action) {
                    Ok(result) => {
                        let buffer_contents = self.buffer.clone();

                        match result {
                            CalculatorResult::None => (),
                            CalculatorResult::RefreshDisplay => {
                                *mgr |= self.display.set_string(String::from(""));
                            },
                            CalculatorResult::Number(n) => {
                                self.buffer_clear();
                                *mgr |= self.display.set_string(format!("{n}"));
                            },
                            CalculatorResult::State(_calculator_state) => {
                                *mgr |= self.display.set_string(format!("\r\n\r\n{buffer_contents}"));
                            },
                            CalculatorResult::NumberAndState(n, _calculator_state) => {
                                self.buffer_clear();
                                *mgr |= self.display.set_string(format!("{n}"));
                            },
                            CalculatorResult::PreviewNumberAndState(preview_result) => {
                                let preview_message = match preview_result {
                                    Ok((n, _calculator_state)) => {
                                        format!("{n}")
                                    },
                                    Err(_e) => {
                                        String::from("")
                                    }
                                };

                                *mgr |= self.display.set_string(format!("\r\n{preview_message}\r\n{buffer_contents}"));
                            },
                        }
                    },
                    Err(e) => {
                        self.buffer_clear();
                        *mgr |= self.display.set_string(format!("{e}"));
                    }
               }
            }
        }
    }

    impl kas::Window for Self {
        fn title(&self) -> &str { "Calculator" }
    }
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
// pub enum CursorDirection {
//     #[default] Up,
//     Left,
//     Down,
//     Right
// }

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum CalculatorAction {
    #[default] None,
    Insert(String, bool),
    //Cursor(CursorDirection),
    //Delete(bool),
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

#[allow(dead_code)]
impl Window {
    pub fn attach_calculator(&mut self, calculator: calculator_logic::calculator::Calculator) {
        self.calculator = Some(calculator);
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

    fn buffer_remove_end(&mut self, count: usize) {
        if count == 0 {}
        else if count == 1 {
            self.buffer.pop();
        }
        else {
            self.buffer_remove_range(self.buffer.len() - count..)
        }
    }

    fn evaluate_buffer(&self) -> Result<CalculatorResult, calculator_err::CalculatorErr> {
        match &self.calculator {
            None => Err(CalculatorErr::err("Calculator Not Initialized")),
            Some(calculator) => calculator
                .evaluate_with_options(&self.buffer, EvaluateOptions::default())
                .map(|(n, state)| CalculatorResult::NumberAndState(n, state))
        }
    }

    fn evaluate_buffer_preview(&self) -> Result<CalculatorResult, calculator_err::CalculatorErr> {
        match &self.calculator {
            None => Err(CalculatorErr::err("Calculator Not Initialized")),
            Some(calculator) => Ok(CalculatorResult::PreviewNumberAndState(calculator
                .evaluate_with_options(&self.buffer, EvaluateOptions::new(InterpreterOptions::new(true)))))
        }
    }

    fn increment_cursor(&mut self) {
        if self.cursor <= self.buffer.len() {
            self.cursor += 1;
        }
    }

    fn decrement_cursor(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
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
                if let Some(calculator) = &self.calculator {
                    let mut new_content = content;
                
                    let parser = calculator.parser();
                    let interpreter = calculator.interpreter();
    
                    /*
                     * If buffer is empty, and history is not, and content is an infix or suffix operator,
                     * prepend with $0.
                     */
                    if self.buffer.is_empty() && calculator.has_history() {
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
                    else if !self.buffer.is_empty() && self.cursor == self.buffer.len() 
                        && (parser.parse_expression::<calculator_parser::expression::BinopInfix>(new_content.as_str()).is_ok() 
                            || parser.parse_expression::<calculator_parser::expression::IdToken>(new_content.as_str()).is_ok()) {
                        new_content = format!("{}{new_content} ", if self.buffer.ends_with(' ') { "" } else { " " });
                    }
    
                    self.insert_at_cursor(new_content.as_str());

                    if preview {
                        self.evaluate_buffer_preview()
                    }
                    else {
                        Ok(CalculatorResult::RefreshDisplay)
                    }
                }
                else {
                    Err(CalculatorErr::err("Calculator not initialized!"))
                }
            },
            // CalculatorAction::Cursor(direction) => {
            //     match direction {
            //         CursorDirection::Up => (),
            //         CursorDirection::Down => (),
            //         CursorDirection::Left => self.decrement_cursor(),
            //         CursorDirection::Right => self.increment_cursor(),
            //     };

            //     Ok(CalculatorResult::None)
            // },
            CalculatorAction::Backspace(preview) => {
                if self.cursor > 0 {
                    self.buffer_remove_range(self.cursor - 1..=self.cursor - 1);

                    if preview {
                        self.evaluate_buffer_preview()
                    }
                    else {
                        Ok(CalculatorResult::RefreshDisplay)
                    }
                }
                else {
                    Ok(CalculatorResult::None)
                }
            },
            // CalculatorAction::Delete(preview) => {
            //     if self.cursor < self.buffer.len() {
            //         self.buffer_remove_range(self.cursor..=self.cursor);
                    
            //         if preview {
            //             self.evaluate_buffer_preview()
            //         }
            //         else {
            //             Ok(CalculatorResult::RefreshDisplay)
            //         }
            //     }
            //     else {
            //         Ok(CalculatorResult::None)
            //     }
            // },
            CalculatorAction::Clear => {
                self.buffer_clear();
                Ok(CalculatorResult::RefreshDisplay)
            }
            CalculatorAction::Submit => self.evaluate_buffer(),
        }
    }
}
