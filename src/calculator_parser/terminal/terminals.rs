use super::Terminal;
use regex::Regex;

/**
 * A radix/decimal point
 */
const RADIX_PT_STR: &str = ".";
const OP_ADD_STR: &str = "+";
const OP_SUB_STR: &str = "-";
const OP_MULT_STR: &str = "*";
const OP_DIV_STR: &str = "/";
const OP_REM_STR: &str = "%";
const OP_EXP_STR: &str = "^";
const OP_FAC_STR: &str = "!";
const OP_PAR_O_STR: &str = "(";
const OP_PAR_C_STR: &str = ")";
const DELIMITER_STR: &str = ",";
const UNDERSCORE_STR: &str = "_";
const DIGIT_REG_STR: &str = r#"[0-9]"#;
const LETTER_REG_STR: &str = r#"[a-zA-Z]"#;
const WHITESPACE_REG_STR: &str = r#"\s"#;

lazy_static! {
    /**
     * An empty terminal
     */
    pub static ref EPSILON: Terminal = Terminal::Epsilon;

    /**
     * A radix/decimal point
     */
    pub static ref RADIX_PT: Terminal = Terminal::Literal(String::from(RADIX_PT_STR));

    /**
     * Addition operator
     */
    pub static ref OP_ADD: Terminal = Terminal::Literal(String::from(OP_ADD_STR));

    /**
     * Subtraction/Negation operator
     */
    pub static ref OP_SUB: Terminal = Terminal::Literal(String::from(OP_SUB_STR));

    /**
     * Multiplication operator
     */
    pub static ref OP_MULT: Terminal = Terminal::Literal(String::from(OP_MULT_STR));

    /**
     * Division operator
     */
    pub static ref OP_DIV: Terminal = Terminal::Literal(String::from(OP_DIV_STR));

    /**
     * Remainder (modulus) operator
     */
    pub static ref OP_REM: Terminal = Terminal::Literal(String::from(OP_REM_STR));

    /**
     * Exponentiation operator
     */
    pub static ref OP_EXP: Terminal = Terminal::Literal(String::from(OP_EXP_STR));

    /**
     * Factorial operator
     */
    pub static ref OP_FAC: Terminal = Terminal::Literal(String::from(OP_FAC_STR));

    /**
     * Opening parenthesis
     */
    pub static ref OP_PAR_O: Terminal = Terminal::Literal(String::from(OP_PAR_O_STR));

    /**
     * Closing parenthesis
     */
    pub static ref OP_PAR_C: Terminal = Terminal::Literal(String::from(OP_PAR_C_STR));

    /**
     * Function argument delimiter
     */
    pub static ref DELIMITER: Terminal = Terminal::Literal(String::from(DELIMITER_STR));

    /**
     * Function argument delimiter
     */
    pub static ref UNDERSCORE: Terminal = Terminal::Literal(String::from(UNDERSCORE_STR));

    /**
     * A single digit, 0-9
     */
    pub static ref DIGIT: Terminal = Terminal::RegularExpresion(Regex::new(DIGIT_REG_STR).unwrap());

    /**
     * A single alphabetic letter
     */
    pub static ref LETTER: Terminal = Terminal::RegularExpresion(Regex::new(LETTER_REG_STR).unwrap());

    /**
     * One or more characters of whitespace
     */
    pub static ref WHITESPACE: Terminal = Terminal::RegularExpresion(Regex::new(WHITESPACE_REG_STR).unwrap());
}