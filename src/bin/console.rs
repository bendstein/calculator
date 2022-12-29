use calculator::calculator_logic;
use calculator::calculator_interface::{console::ConsoleUI as UI, ui_trait::CalculatorUI};

fn main() {
    //Set to use virtual terminal so that control characters work on windows
    _ = colored::control::set_virtual_terminal(true);

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