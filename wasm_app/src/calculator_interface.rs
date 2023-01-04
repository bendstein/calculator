use std::ops::RangeBounds;
use yew::prelude::*;
use bitflags::bitflags;

use super::calculator::{*, calculator_parser, calculator_interpreter::interpreter::EvaluateOptions as InterpreterOptions, calculator_err::CalculatorErr};

bitflags! {
    pub struct ClearType: u32 {
        const ENTRY = 0b00000001;
        const HISTORY = 0b00000010;
        const MEMORY = 0b00000100;
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum CalculatorAction {
    #[default] None,
    Insert(String, bool),
    Surround { prefix: Option<String>, open: String, close: Option<String>, suffix: Option<String>, preview: bool },
    Backspace(bool),
    Clear(ClearType),
    Cursor(bool),
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

#[derive(Properties, Default, Debug, PartialEq, Clone)]
pub struct CalculatorButton {
    callback_click: Option<Callback<()>>,
    display: AttrValue
}

impl Component for CalculatorButton {
    type Message = ();
    type Properties = CalculatorButton;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            callback_click: ctx.props().callback_click.clone(),
            display: ctx.props().display.clone()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log::info!("InnerUpdate");

        if let Some(ref mut callback) = self.callback_click {
            log::info!("Emit");
            callback.emit(());
            true
        }
        else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {       
        let onclick = ctx.link().callback(move |_: MouseEvent| {
            log::debug!("Click");
        });

        html! {
            <button {onclick} title={&self.display} data-display={&self.display}>{ &self.display }</button>
        }
    }
}

#[derive(Properties, Default, Debug, PartialEq, Clone)]
pub struct CalculatorBase {
    calculator: Calculator,
    buffer: String,
    cursor: usize,
    result: Option<f64>,
    preview: Option<Result<(f64, CalculatorState), CalculatorErr>>
}

impl CalculatorBase {
    fn buffer_append(&mut self, content: &str) {
        self.buffer.push_str(content);
    }

    fn buffer_insert_at_cursor(&mut self, content: &str) {
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
}

impl Component for CalculatorBase {
    type Message = CalculatorAction;
    type Properties = CalculatorBase;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            calculator: ctx.props().calculator.clone(),
            buffer: ctx.props().buffer.clone(),
            cursor: ctx.props().cursor,
            result: ctx.props().result,
            preview: ctx.props().preview.clone()
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Update; Buffer: {}; Cursor: {}", &self.buffer, self.cursor);
        log::debug!("{msg:?}");

        let result = match msg {
            CalculatorAction::None => CalculatorResult::None,
            CalculatorAction::Insert(symbol, preview) => {
                log::info!("Insert '{symbol}'; preview: {}", preview);

                let mut new_content = symbol;
                
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

                self.buffer_insert_at_cursor(&new_content);

                log::info!("Buffer: {}; Cursor: {}", &self.buffer, self.cursor);
                
                if preview {
                    self.evaluate_buffer_preview()
                }
                else {
                    CalculatorResult::RefreshDisplay
                }
            },
            CalculatorAction::Surround { prefix, open, close, suffix, preview } => {
                let prefix = prefix.unwrap_or_default();
                let close = close.unwrap_or_else(|| open.clone());
                let suffix = suffix.unwrap_or_default();

                log::info!("Surround; {prefix}, {open}, {close}, {suffix}; preview: {}", preview);

                if self.buffer.is_empty() && self.calculator.has_history() {
                    self.buffer = format!("{prefix}{open}$0{close}{suffix}");
                    self.cursor = self.buffer.len();
                }
                else {
                    self.buffer = format!("{prefix}{open}{}{close}{suffix}", self.buffer);
                    self.cursor = (self.cursor + prefix.len() + open.len() + close.len() + suffix.len()).min(self.buffer.len());
                }

                if preview {
                    self.evaluate_buffer_preview()
                }
                else {
                    CalculatorResult::RefreshDisplay
                }
            },
            CalculatorAction::Cursor(direction) => {
                if direction {
                    self.cursor = (self.cursor + 1).min(self.buffer.len());
                }
                else {
                    self.cursor = (self.cursor.max(1) - 1).min(self.buffer.len());
                }

                CalculatorResult::None
            },
            CalculatorAction::Backspace(preview) => {
                log::info!("Backspace; preview: {}", preview);
                self.buffer_remove_end(1, true);
                self.cursor = (self.cursor.max(1) - 1).min(self.buffer.len());
                
                if preview {
                    self.evaluate_buffer_preview()
                }
                else {
                    CalculatorResult::RefreshDisplay
                }
            },
            CalculatorAction::Clear(clear_type) => {
                log::info!("Clear; {}", clear_type.bits);

                if clear_type.contains(ClearType::ENTRY) {
                    log::info!("Clearing buffer.");
                    self.buffer_clear();
                }

                if clear_type.contains(ClearType::HISTORY) {
                    log::info!("Clearing history.");
                    self.calculator.clear_stack();
                }

                if clear_type.contains(ClearType::MEMORY) {
                    log::info!("Clearing memory.");
                    self.calculator.clear_mem();
                }

                CalculatorResult::RefreshDisplay
            },
            CalculatorAction::Submit => {
                log::info!("Submit");
                let result = self.evaluate_buffer();

                match result {
                    Ok(evaluated) => {
                        evaluated
                    },
                    Err(err) => {
                        log::error!("{err}");
                        CalculatorResult::RefreshDisplay
                    }
                }
            }
        };

        let rerender = match result {
            CalculatorResult::None => {
                log::info!("None");
                false
            },
            CalculatorResult::RefreshDisplay => {
                log::info!("Refresh Display");
                self.result = None;
                self.preview = None;
                true
            },
            CalculatorResult::Number(n) => {
                log::info!("Number ({n}).");
                self.result = Some(n);
                self.preview = None;
                self.buffer_clear();
                true
            },
            CalculatorResult::State(_state) => {
                log::info!("State");
                true
            },
            CalculatorResult::NumberAndState(n, _state) => {
                log::info!("Number ({n}) And State");
                self.result = Some(n);
                self.preview = None;
                self.buffer_clear();
                true
            },
            CalculatorResult::PreviewNumberAndState(preview) => {
                self.preview = Some(preview.clone());
                self.result = None;

                match preview {
                    Ok((n, _state)) => {
                        log::info!("Preview Number ({n}) And State");
                    },
                    Err(e) => {
                        log::warn!("Preview Number And State: Error: {e}");
                    },
                };

                true
            },
        };

        log::debug!("Debug: {self:#?}");

        rerender
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let buffer = if self.buffer.is_empty() {
            self.result.as_ref().map(|n| n.to_string())
        }
        else {
            Some(self.buffer.clone())
        };

        let buffer_segments: Option<Vec<yew::virtual_dom::VNode>> = buffer.as_ref().map(|buf| buf.split_whitespace()
            .map(|segment| {
                let tooltip = match self.calculator.evaluate_with_options(segment, EvaluateOptions::new(InterpreterOptions::new(true))) {
                    Err(_) => None,
                    Ok((n, _)) => Some(n.to_string())
                };

                html! {
                    <span>
                        <span title={tooltip}>{ segment }</span><span>{ " " }</span>
                    </span>
                }
            })
            .collect());

        let buffer_segment_iter = buffer_segments.map(|bs| bs.into_iter());

        let preview = match &self.preview {
            None => None,
            Some(result) => {
                let (content, class) = match result {
                    Ok((n, _)) => (n.to_string(), None),
                    Err(e) => (e.to_string(), Some("error"))
                };

                let content_truncated = if content.len() > 10 {
                    let truncated = &content[0..10];
                    format!("{truncated}...")
                }
                else {
                    content.clone()
                };

                Some(html! {
                    <span title={content} class={class}>{ content_truncated }</span>
                })
            }
        };

        let calculator_state = self.calculator.clone_current_state();
        let history = calculator_state.history;

        let history_rows = history.iter().rev().map(|n| {
            //format!("<div class=\"history\">= {n}</div>")
            html! {
                <div class="history">{"= "}{n}</div>
            }
        });

        html! {
            <>
                <div id="calculator" class="calculator">
                    <div class="calculator-screen">
                        <div class="calculator-screen-inner">
                            <div id="buffer">             
                            if let Some(iter) = buffer_segment_iter {
                                { for iter }
                            }
                            </div>
                            <div id="preview">{ preview }</div>
                            { for history_rows }
                        </div>
                    </div>
                    <table class="calculator-buttons">
                         <tr>
                            <td></td>
                            <td><CalculatorButton display="π" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("pi"), true))} /></td>
                            <td><CalculatorButton display="e" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("e"), true))} /></td>
                            <td><CalculatorButton display="C" callback_click={ctx.link().callback(move |_| CalculatorAction::Clear(ClearType::all()))} /></td>
                            <td><CalculatorButton display="CE" callback_click={ctx.link().callback(move |_| CalculatorAction::Clear(ClearType::ENTRY))} /></td>
                            <td><CalculatorButton display="BK" callback_click={ctx.link().callback(move |_| CalculatorAction::Backspace(true))} /></td>
                         </tr>
                        <tr>
                            <td><CalculatorButton display="(" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("("), true))} /></td>
                            <td><CalculatorButton display=")" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from(")"), true))} /></td>
                            <td></td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td><CalculatorButton display="|x|" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: Some(String::from("abs")), open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td><CalculatorButton display="(x)" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: None, open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td><CalculatorButton display="^" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("^"), true))} /></td>
                            <td><CalculatorButton display="%" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("%"), true))} /></td>
                            <td><CalculatorButton display="!" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("!"), true))} /></td>
                            <td><CalculatorButton display="÷" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("/"), true))} /></td>
                        </tr>
                        <tr>
                            <td><CalculatorButton display="√x" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: Some(String::from("sqrt")), open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td><CalculatorButton display="1/x" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: Some(String::from("1 / ")), open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td><CalculatorButton display="7" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("7"), true))} /></td>
                            <td><CalculatorButton display="8" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("8"), true))} /></td>
                            <td><CalculatorButton display="9" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("9"), true))} /></td>
                            <td><CalculatorButton display="×" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("*"), true))} /></td>
                        </tr>
                        <tr>
                            <td><CalculatorButton display="x^2" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: None, open: String::from("("), close: Some(String::from(")")), suffix: Some(String::from(" ^ 2")), preview: true })} /></td>
                            <td><CalculatorButton display="2^x" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: Some(String::from("2 ^ ")), open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td><CalculatorButton display="4" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("4"), true))} /></td>
                            <td><CalculatorButton display="5" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("5"), true))} /></td>
                            <td><CalculatorButton display="6" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("6"), true))} /></td>
                            <td><CalculatorButton display="-" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("-"), true))} /></td>
                        </tr>
                        <tr>
                            <td><CalculatorButton display="e^x" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: Some(String::from("e ^ ")), open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td><CalculatorButton display="10^x" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: Some(String::from("10 ^ ")), open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td><CalculatorButton display="1" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("1"), true))} /></td>
                            <td><CalculatorButton display="2" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("2"), true))} /></td>
                            <td><CalculatorButton display="3" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("3"), true))} /></td>
                            <td><CalculatorButton display="+" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("+"), true))} /></td>
                        </tr>
                        <tr>
                            <td><CalculatorButton display="log" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: Some(String::from("log")), open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td><CalculatorButton display="ln" callback_click={ctx.link().callback(move |_| CalculatorAction::Surround { prefix: Some(String::from("ln")), open: String::from("("), close: Some(String::from(")")), suffix: None, preview: true })} /></td>
                            <td></td>
                            <td><CalculatorButton display="0" callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("0"), true))} /></td>
                            <td><CalculatorButton display="." callback_click={ctx.link().callback(move |_| CalculatorAction::Insert(String::from("."), true))} /></td>
                            <td><CalculatorButton display="=" callback_click={ctx.link().callback(move |_| CalculatorAction::Submit)} /></td>
                        </tr>
                    </table>
                </div>   
            </>
        }
    }
}