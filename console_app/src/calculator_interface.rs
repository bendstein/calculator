use std::io::Write;

use crate::calculator_logic;

const EXIT_COMMAND: &str = ":exit";
const CLEAR_COMMAND: &str = ":clear";
const CLEAR_HISTORY_COMMAND: &str = ":clear-hist";
const CLEAR_MEMORY_COMMAND: &str = ":clear-mem";

#[derive(Debug, Clone, Default)]
pub struct ConsoleUI {
    calculator: calculator_logic::calculator::Calculator
}

impl ConsoleUI {
    pub fn new(calculator: calculator_logic::calculator::Calculator) -> Self {
        Self {
            calculator
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("Enter the expression to evaluate, '{CLEAR_COMMAND}' to clear the screen, '{CLEAR_HISTORY_COMMAND}' to clear result history, '{CLEAR_MEMORY_COMMAND}' to clear calculator memory, or '{EXIT_COMMAND}' to exit.");

        loop {
            print!("> ");
            
            match std::io::stdout().flush() {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Failed to flush stdout: {err}");
                    continue;
                }
            }

            let mut input = String::new();

            match std::io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Failed to read input: {err}");
                    continue;
                }
            }

            input = String::from(input.trim_end());

            if input.eq_ignore_ascii_case(EXIT_COMMAND) {
                println!("Exiting...");
                break;
            }
            else if input.eq_ignore_ascii_case(CLEAR_COMMAND) {
                print!("{esc}c", esc = 27 as char);
                continue;
            }
            else if input.eq_ignore_ascii_case(CLEAR_HISTORY_COMMAND) {
                self.calculator.clear_stack();
                println!("Cleared calculator history.");
                continue;
            }
            else if input.eq_ignore_ascii_case(CLEAR_MEMORY_COMMAND) {
                self.calculator.clear_mem();
                println!("Cleared calculator memory.");
                continue;
            }

            let evaluated = match self.calculator.evaluate(&input) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
            };

            println!("{evaluated}");
        };

        Ok(())
    }
}