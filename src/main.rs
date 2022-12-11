#[macro_use]
extern crate lazy_static;

pub mod calculator_parser;
pub mod calculator_interpreter;

fn main() {
    // let parser_tests = vec![
    //     String::from("")
    // ];

    // for test in parser_tests {
    //     println!("Parser Test: '{test}'");

    //     match calculator_parser::parser::Parser::parse_line(&test) {
    //         Ok(parse_tree) => println!(" -- Result: '{parse_tree}' --"),
    //         Err(parse_err) => eprintln!(" -- Error: '{parse_err}' --")
    //     }
    // }

    // let interpreter_tests = vec![
    //     String::from("")
    // ];

    // for test in interpreter_tests {
    //     println!("Interpreter Test: '{test}'");

    //     match calculator_interpreter::Interpreter::default().evaluate_string(&test) {
    //         Ok(value) => println!(" -- Result: '{value}' --"),
    //         Err(interpreter_err) => eprintln!(" -- Error: '{interpreter_err}' --")
    //     }
    // }
}