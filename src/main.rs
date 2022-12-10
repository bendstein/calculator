#[macro_use]
extern crate lazy_static;

pub mod calculator_parser;

fn main() {
    let tests = vec![
        String::from(""),
        String::from("         "),
        String::from("5 + 5"),
        String::from("1     + 2*3 ^4*5"),
        String::from("1+2*3^(4*5)"),
        String::from("sqrt(5)"),
        String::from("sum(5, 6)"),
        String::from("sqrt(10) +5*2^ f(6+       1)"),
        String::from("f(sqrt(1+2   *    3^(4*5)),   5)"),
        String::from(" f * n ( (5 + 5) , (6 ^ 7     )) "),
        String::from("fn(_x2+1)"),
        String::from("-2"),
        String::from("2!"),
        String::from("-2!"),
        String::from("-5 + 6"),
        String::from("-f(15 + 2) * 5!"),
        String::from("5 + 7 + 8 + -9 + (sqrt(15) - 24 / 6)^17^3^12!"),
        String::from("5 + + 6"),
        String::from("+  "),
        String::from("5n"),
        String::from("-f-(5)"),
        String::from("5 * (6 + 7"),
        String::from("&"),
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