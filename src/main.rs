#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap};

pub mod calculator_parser;
pub mod calculator_interpreter;

fn main() {
    //Read in command line arguments
    let args = get_args_map();

    //If help flag is present, print help
    if args.contains_key(HELP_KEY) && String::from(args.get(HELP_KEY).unwrap()).eq(true.to_string().as_str()) {      
        print_help();
        return;
    }

    //Get expression from arguments
    let expr = match args.get(EXPRESSION_KEY) {
        None => "",
        Some(value) => value.as_str()
    };

    //Get action from arguments
    let action = match args.get(ACTION_KEY) {
        None => ACTION_DEFAULT,
        Some(value) => {
            if !vec![ACTION_EVAL, ACTION_PARSE, ACTION_BOTH].iter().any(|s| s.eq_ignore_ascii_case(value))
            {
                eprintln!("{value} is not a valid action. Please run with \"{ARGUMENT_PREFIX}{HELP_KEY}\" for help.");
                return;
            }

            value
        }
    };

    let print_parse = vec![ACTION_PARSE, ACTION_BOTH].iter().any(|s| s.eq_ignore_ascii_case(action));
    let do_eval = vec![ACTION_EVAL, ACTION_BOTH].iter().any(|s| s.eq_ignore_ascii_case(action));

    let parsed = match calculator_parser::parser::Parser::parse_line(expr) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("An error occurred while parsing expression '{expr}': {e}");
            return;
        }
    };

    if print_parse {
        println!("Parsed: {parsed}");
    }

    if do_eval {
        let evaluated = match calculator_interpreter::Interpreter::default().evaluate(parsed) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("An error occurred while evaluating expression '{expr}': {e}");
                return;
            }
        };

        println!("{evaluated}");
    }
}

///
/// Command line argument key to print help docs.
/// 
pub const HELP_KEY: &str = "help";

///
/// Command line argument key to provide expression.
/// 
pub const EXPRESSION_KEY: &str = "exp";

///
/// Command line argument key to provide action.
/// 
pub const ACTION_KEY: &str = "action";

///
/// Value for ACTION_KEY indicating that the expression should be
/// parsed and printed
/// 
pub const ACTION_PARSE: &str = "parse";

///
/// Value for ACTION_KEY indicating that the expression should be
/// evaluated
/// 
pub const ACTION_EVAL: &str = "evaluate";

///
/// Value for ACTION_KEY indicating that the expression should be
/// parsed, printed and evaluated
/// 
pub const ACTION_BOTH: &str = "both";

///
/// Default value for ACTION_KEY
/// 
pub const ACTION_DEFAULT: &str = ACTION_EVAL;

///
/// Prefix for command line arguments.
/// 
pub const ARGUMENT_PREFIX: &str = "/";

///
/// Delimiter to split command line arguments
/// as key to value.
/// 
pub const ARGUMENT_DELIMITER: &str = ":";

///
/// Get command line arguments
/// as a map from key to value.
/// 
fn get_args_map() -> HashMap<String, String> {
    let mut rv = HashMap::new(); 
    
    match parse_args::argparser::parse_args_with_opts(
        std::env::args(), 
        parse_args::argparser::ParseArgsSettings::init(
            String::from(ARGUMENT_PREFIX), 
            String::from(ARGUMENT_DELIMITER))
        ) {
            Err(msgs) => {
                panic!("Failed to parse arguments: {}", msgs.join(", "));
            },
            Ok(args) => args
        }
        .iter()
        .for_each(|arg| {
            let kvp = arg.to_key_value_pair();
            rv.insert(kvp.0, kvp.1);
        })
    ;

    rv
}

///
/// Print help docs
/// 
fn print_help() {
    fn flag_example(key: &str) -> String {
        let temp = format!("{ARGUMENT_PREFIX}{key}");
        temp
    }

    fn pair_example(key: &str) -> String {
        let temp = format!("{ARGUMENT_PREFIX}{key}{ARGUMENT_DELIMITER}{{VALUE}}");
        temp
    }

    let flag_key_restriction = "If used as a key-value argument, rather than a flag argument, must be either true or false.";

    let arg_info = vec![
        (
            HELP_KEY,
            "Display application help.".to_string(),
            flag_example(HELP_KEY),
            "".to_string(),
            flag_key_restriction.to_string(),
            None
        ),
        (
            EXPRESSION_KEY,
            "The expression to perform the action on.".to_string(),
            pair_example(EXPRESSION_KEY),
            "".to_string(),
            "".to_string(),
            None
        ),
        (
            ACTION_KEY,
            format!("The action to perform on the expression. ({ACTION_PARSE}: Parse the expression, and print the formatted parse tree;\r\n        {ACTION_EVAL}: Evaluate the expression and print the result;\r\n        {ACTION_BOTH}: Parse the expression, print the formatted parse tree, evaluate the expression, and print the result.)"),
            pair_example(ACTION_KEY),
            format!("[{ACTION_PARSE}, {ACTION_EVAL}, {ACTION_BOTH}]"),
            "".to_string(),
            Some(ACTION_DEFAULT)
        )
    ];

    println!("\r\nApplication arguments must be of one of the following forms:\r\n  -For a key/value: {}\r\n  -For a flag: {}\r\n\r\nThe following are acceptable arguments:", pair_example("{KEY}"), flag_example("{FLAG}"));

    arg_info.iter()
    .for_each(|info| {
        println!();
        println!("  -{}", info.0);
        println!("    -Description: {}", info.1);
        println!("    -Usage: {}", info.2);

        if !info.3.is_empty() {
            println!("    -Restrictions: {}", info.3);
        }

        if !info.4.is_empty() {
            println!("    -Exmaple: {}", info.4);
        }

        if let Some(default_value) = info.5 {
            println!("    -Default Value: {}", default_value);
        }
    });

    println!();
}