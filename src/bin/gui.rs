#![windows_subsystem = "windows"]

use calculator::calculator_logic;
use calculator::calculator_interface::{gui::GraphicalUI as UI, ui_trait::CalculatorUI};

fn main() {
    //Create the calculator
    let calculator = calculator_logic::calculator::Calculator::default();
    
    //Create the UI instance
    let mut ui = UI::default();
    
    //Start the UI
    match ui.start(calculator) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("A fatal error occurred: {e}");
        }
    };
}