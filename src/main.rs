#[macro_use]
extern crate lazy_static;

pub mod calculator_parser;

fn main() {
    let tests = vec![
        String::from("")
    ];

    for test in tests {
        println!("Test: '{test}'");

        let mut parser = calculator_parser::parser::Parser::new(&test);
        match parser.parse() {
            Ok(parse_tree) => println!(" -- Result: '{parse_tree}' --"),
            Err(parse_err) => eprintln!(" -- Error: '{parse_err}' --")
        }
    }
}