use crate::calculator_logic::calculator;

pub trait CalculatorUI {
    fn take_calculator(&mut self, calculator: calculator::Calculator);
    fn start(&mut self) -> Result<(), &str>;
}