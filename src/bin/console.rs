use calculator::calculator_logic;
use calculator::calculator_interface::{console::ConsoleUI as UI, ui_trait::CalculatorUI};

fn main() {
    //Create the calculator
    let calculator = calculator_logic::calculator::Calculator::default();
    
    //Create the UI instance
    let mut ui = UI::default();

    //Attach the calculator to the UI
    ui.attach_calculator(calculator);

    //Start the UI
    match ui.start() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("A fatal error occurred: {e}");
        }
    };
}