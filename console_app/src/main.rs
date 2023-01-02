use std::io::BufRead;

pub use calculator::calculator;

mod calculator_interface;

fn main() -> Result<(), String> {
    //Set to use virtual terminal so that control characters work on windows
    _ = colored::control::set_virtual_terminal(true);

    //Check if piped input

    //Input not piped; don't read from stdin
    let mut inputs: Vec<String> = if atty::is(atty::Stream::Stdin) {
        Vec::new()
    }
    //Input was piped; read content from stdin
    else {
        std::io::stdin()
        .lock()
        .lines()
        .filter_map(|l| {
            if let Ok(line) = l {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    None
                }
                else {
                    Some(String::from(trimmed))
                }
            }
            else {
                None
            }
        })
        .collect()
    };

    //Get arguments
    let args: Vec<String> = std::env::args().collect();

    //Push all arguments (except for path) to input
    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            inputs.push(arg.clone());
        }
    }

    //Create the calculator
    let calculator = calculator::Calculator::default();

    if !inputs.is_empty() {
        for expression in inputs {
            match calculator.evaluate(&expression) {
                Ok(result) => {
                    println!("{result}");
                    Ok(())
                },
                Err(e) => Err(e.to_string())
            }?;
        }

        Ok(())
    }
    else {
        //Create the UI instance
        let mut ui = calculator_interface::ConsoleUI::new(calculator);

        //Start the UI
        ui.start()
    }
}