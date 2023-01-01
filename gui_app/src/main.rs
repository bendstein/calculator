#![windows_subsystem = "windows"]

pub use calculator::calculator;

mod calculator_interface;

fn main() {
    //Create the calculator
    let calculator = calculator::Calculator::default();
    
    //Initialize WASM logging
    wasm_logger::init(wasm_logger::Config::default());

    //Create the UI instance
    let mut ui = calculator_interface::GraphicalUI::new(calculator);
    
    //Start the UI
    match ui.start() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("A fatal error occurred: {e}");
        }
    };
}