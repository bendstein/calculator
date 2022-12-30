pub mod function_impl;
pub mod function_lazy_static;

use crate::calculator::CalculatorErr;

pub type Func0 = fn () -> Result<f64, CalculatorErr>;
pub type Func1 = fn (f64) -> Result<f64, CalculatorErr>;
pub type Func2 = fn (f64, f64) -> Result<f64, CalculatorErr>;
pub type Func3 = fn (f64, f64, f64) -> Result<f64, CalculatorErr>;
pub type Func4 = fn (f64, f64, f64, f64) -> Result<f64, CalculatorErr>;
pub type Func5 = fn (f64, f64, f64, f64, f64) -> Result<f64, CalculatorErr>;
pub type Func6 = fn (f64, f64, f64, f64, f64, f64) -> Result<f64, CalculatorErr>;
pub type Func7 = fn (f64, f64, f64, f64, f64, f64, f64) -> Result<f64, CalculatorErr>;
pub type Func8 = fn (f64, f64, f64, f64, f64, f64, f64, f64) -> Result<f64, CalculatorErr>;
pub type Func9 = fn (f64, f64, f64, f64, f64, f64, f64, f64, f64) -> Result<f64, CalculatorErr>;
pub type FuncVar = fn (Vec<f64>) -> Result<f64, CalculatorErr>;

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