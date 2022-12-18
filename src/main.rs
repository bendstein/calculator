#[macro_use]
extern crate lazy_static;

pub mod calculator_logic;
pub mod calculator_interface;
mod handle_args;

use calculator_logic::calculator;
use calculator_interface::{console::ConsoleUI, ui_trait::CalculatorUI};

fn main() {
    //Read in command line arguments
    let args = handle_args::get_args_map();

    //If help flag is present, print help and return
    if args.contains_key(handle_args::HELP_KEY) && String::from(&**args.get(handle_args::HELP_KEY).unwrap()).eq(true.to_string().as_str()) {      
        handle_args::print_help();
        return;
    }

    //Get display type from command line arguments
    let empty_str_box = Box::from("");
    let display_type_arg = &**args.get(handle_args::CALCULATOR_DISPLAY_KEY).unwrap_or(&empty_str_box);
    let display_type = get_display_type(display_type_arg, false);

    //Make sure the display type exists
    if display_type.is_none() {
        eprintln!("Invalid display type {display_type_arg}.");
        return;
    }

    let display_type = display_type.unwrap();

    //Create the calculator
    let calculator = calculator::Calculator::default();
    
    //Create the UI instance
    let mut ui: Box<dyn CalculatorUI> = Box::from(match display_type {
        DisplayType::Console => ConsoleUI::default(),
        DisplayType::Gui => unimplemented!()
    });

    //Attach the calculator to the UI
    ui.take_calculator(calculator);

    //Start the UI
    match ui.start() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("A fatal error occurred: {e}");
        }
    };
}

#[allow(dead_code)]
enum DisplayType {
    Console,
    Gui
}

fn get_display_type(display_type_string: &str, is_default: bool) -> Option<DisplayType> {
    let trimmed = display_type_string.trim();

    let value_console = handle_args::CALCULATOR_DISPLAY_VALUE_CONSOLE.trim();
    let value_gui = handle_args::CALCULATOR_DISPLAY_VALUE_GUI.trim();

    if trimmed.eq_ignore_ascii_case(value_console) {
        Some(DisplayType::Console)
    }
    else if trimmed.eq_ignore_ascii_case(value_gui) {
        Some(DisplayType::Gui)
    }
    else if !is_default {
        get_display_type(handle_args::CALCULATOR_DISPLAY_VALUE_DEFAULT, true)
    }
    else {
        None
    }
}