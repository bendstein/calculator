use std::collections::HashMap;

///
/// Command line argument key to print help docs.
/// 
pub(crate) const HELP_KEY: &str = "help";

///
/// Command line argument key for display type
/// 
pub(crate) const CALCULATOR_DISPLAY_KEY: &str = "d";

///
/// Command line argument value for console display type
/// 
pub(crate) const CALCULATOR_DISPLAY_VALUE_CONSOLE: &str = "console";

///
/// Command line argument value for gui display type
/// 
pub(crate) const CALCULATOR_DISPLAY_VALUE_GUI: &str = "gui";


///
/// Command line argument value for default display type
/// 
pub(crate) const CALCULATOR_DISPLAY_VALUE_DEFAULT: &str = CALCULATOR_DISPLAY_VALUE_GUI;

///
/// Prefix for command line arguments.
/// 
const ARGUMENT_PREFIX: &str = "/";

///
/// Delimiter to split command line arguments
/// as key to value.
/// 
const ARGUMENT_DELIMITER: &str = ":";

///
/// Get command line arguments
/// as a map from key to value.
/// 
pub(crate) fn get_args_map() -> HashMap<String, Box<str>> {
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
            rv.insert(kvp.0, Box::from(kvp.1.as_str()));
        })
    ;

    rv
}

///
/// Print help docs
/// 
pub(crate) fn print_help() {
    fn flag_example(key: &str) -> String {
        let temp = format!("{ARGUMENT_PREFIX}{key}");
        temp
    }

    fn pair_example(key: &str) -> String {
        let temp = format!("{ARGUMENT_PREFIX}{key}{ARGUMENT_DELIMITER}{{VALUE}}");
        temp
    }

    let flag_key_restriction = "If used as a key-value argument, rather than a flag argument, must be either true or false.";

    type HelpTuple<'a> = (&'a str, String, String, String, String, Option<&'a str>);

    let arg_info: Vec<HelpTuple> = vec![
        (
            HELP_KEY,
            "Display application help.".to_string(),
            flag_example(HELP_KEY),
            flag_key_restriction.to_string(),
            "".to_string(),
            None
        ),
        (
            CALCULATOR_DISPLAY_KEY,
            "Indicate the UI type for the calculator.".to_string(),
            pair_example(CALCULATOR_DISPLAY_KEY),
            format!("[{CALCULATOR_DISPLAY_VALUE_CONSOLE}, {CALCULATOR_DISPLAY_VALUE_GUI}]"),
            "".to_string(),
            Some(CALCULATOR_DISPLAY_VALUE_GUI)
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
            println!("    -Example: {}", info.4);
        }

        if let Some(default_value) = info.5 {
            println!("    -Default Value: {}", default_value);
        }
    });

    println!();
}