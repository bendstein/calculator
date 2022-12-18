use crate::calculator_logic;
use super::ui_trait::*;

slint::include_modules!();

#[derive(Debug, PartialEq, Clone, Default)]
pub struct GraphicalUI {
    calculator: calculator_logic::calculator::Calculator
}

impl CalculatorUI for GraphicalUI {
    fn attach_calculator(&mut self, calculator: calculator_logic::calculator::Calculator) {
        self.calculator = calculator;
    }

    fn start(&mut self) -> Result<(), &str> {
        HelloWorld::new().run();
        Ok(())
    }
}