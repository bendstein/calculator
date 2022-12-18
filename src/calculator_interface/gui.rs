use crate::calculator_logic;
use super::ui_trait::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct GraphicalUI {
    calculator: calculator_logic::calculator::Calculator
}

impl CalculatorUI for GraphicalUI {
    fn attach_calculator(&mut self, calculator: calculator_logic::calculator::Calculator) {
        self.calculator = calculator;
    }

    fn start(&mut self) -> Result<(), &str> {
        Ok(())
    }
}