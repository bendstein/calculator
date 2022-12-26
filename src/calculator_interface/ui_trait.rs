use crate::calculator_logic::calculator;

pub trait CalculatorUI {
    fn start(&mut self, calculator: calculator::Calculator) -> Result<(), String>;
}