pub use calculator::calculator;

mod calculator_interface;

fn main() {
    //Set to use virtual terminal so that control characters work on windows
    _ = colored::control::set_virtual_terminal(true);

    //Create the calculator
    let calculator = calculator::Calculator::default();
    
    //Create the UI instance
    let mut ui = calculator_interface::ConsoleUI::new(calculator);

    //Start the UI
    match ui.start() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("A fatal error occurred: {e}");
        }
    };
}