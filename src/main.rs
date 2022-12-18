#[macro_use]
extern crate lazy_static;

pub mod calculator_logic;
pub mod calculator_interface;

use calculator_logic::calculator;
use calculator_interface::{console::ConsoleUI, ui_trait::CalculatorUI};

fn main() {
    let calculator = calculator::Calculator::default();
    let mut ui = ConsoleUI::default();
    ui.take_calculator(calculator);

    _ = ui.start();
}