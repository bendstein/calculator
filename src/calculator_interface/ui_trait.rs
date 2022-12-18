use crate::calculator_logic::calculator;

pub trait CalculatorUI: Default {
    type Error;
    fn take_calculator(&mut self, calculator: calculator::Calculator);
    fn start(&mut self) -> Result<(), Self::Error>;
}