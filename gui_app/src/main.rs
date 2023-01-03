#![windows_subsystem = "windows"]

pub use calculator::calculator;

mod calculator_interface;

fn main() {
    //Initialize WASM logging
    wasm_logger::init(wasm_logger::Config::default());

    //Create the application
    let _handle = yew::Renderer::<calculator_interface::CalculatorBase>::new().render();
}