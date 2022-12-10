#[allow(unused_imports)]
use super::{terminal, expression, parser};

/**
 * The provided input, should parse into an Expr
 * whose to_string method equals the expected string.
 */
fn default_test(input: &str, expected: &str) {
    let mut parser = parser::Parser::new(input);

    match parser.parse() {
        Ok(result) => {
            let result_string = result.to_string();
            assert_eq!(expected, result_string.as_str());
        },
        Err(err) => {
            panic!("{err}")
        }
    };
}

#[test]
/**
 * Test that an empty string parses to an
 * empty expression
 */
fn empty_string() {
    const EXPECTED: &str = "";
    let input: &str = "";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that a whitespace string parses to an
 * empty expression
 */
fn whitespace_string() {
    const EXPECTED: &str = "";
    let input: &str = "     ";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that numbers are read in
 */
fn number() {
    const EXPECTED: &str = "0";
    let input: &str = "0";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that decimal numbers are
 * read in
 */
fn decimal_number() {
    const EXPECTED: &str = "0.1";
    let input: &str = "0.1";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that binary infix expressions
 * are read in for +,-
 */
fn number_addition() {
    const EXPECTED: &str = "0 + 1";
    let input: &str = "0+1";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that binary infix expressions
 * are read in for *,/,%
 */
fn number_multiplication() {
    const EXPECTED: &str = "0 * 1";
    let input: &str = "0*1";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that binary infix expressions
 * are read in for ^
 */
fn number_exponentiation() {
    const EXPECTED: &str = "0 ^ 1";
    let input: &str = "0^1";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that +,- are read in on the same
 * precedence level
 */
fn binary_expression_order_of_operations_test_1() {
    const EXPECTED: &str = "0 + 1 - 2";
    let input: &str = "0 + 1 - 2";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that *,/,% are read in on the same
 * precedence level
 */
fn binary_expression_order_of_operations_test_2() {
    const EXPECTED: &str = "0 * 1 % 3 / 2";
    let input: &str = "0 * 1 % 3 / 2";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that ^ takes precedence over *,/,%.
 */
fn binary_expression_order_of_operations_test_3() {
    const EXPECTED: &str = "0 % [1 ^ 2] / 3";
    let input: &str = "0 % 1 ^ 2 / 3";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that *,/,% take precedence over +/-
 */
fn binary_expression_order_of_operations_test_4() {
    const EXPECTED: &str = "0 + [1 * 2] - 3";
    let input: &str = "0 + 1 * 2 - 3";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that ^ takes precedence over +/-
 */
fn binary_expression_order_of_operations_test_5() {
    const EXPECTED: &str = "5 + [2 ^ 3] - 5";
    let input: &str = "5 + 2 ^ 3 - 5";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that *,/,% can take precedence over ^ using parentheses.
 */
fn binary_expression_order_of_operations_test_6() {
    const EXPECTED: &str = "[0 % 1] ^ [2 / 3]";
    let input: &str = "(0 % 1) ^ (2 / 3)";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that +/- can take precedence over *,/,% using parentheses.
 */
fn binary_expression_order_of_operations_test_7() {
    const EXPECTED: &str = "[[0 + 1] * 2] - 3";
    let input: &str = "(0 + 1) * 2 - 3";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that +/- can take precedence over ^ using parentheses.
 */
fn binary_expression_order_of_operations_test_8() {
    const EXPECTED: &str = "5 + [2 ^ [3 - 5]]";
    let input: &str = "5 + 2 ^ (3 - 5)";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that unary prefixes are read in
 */
fn unary_prefix_expression() {
    const EXPECTED: &str = "-5";
    let input: &str = "-5";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that unary suffixes are read in
 */
fn unary_suffix_expression() {
    const EXPECTED: &str = "5!";
    let input: &str = "5!";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that unary prefixes in binary expressions are read in
 */
fn unary_prefix_in_binary_expression() {
    const EXPECTED: &str = "-5 + -6";
    let input: &str = "-5 + -6";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that unary suffixes in binary expressions are read in
 */
fn unary_suffix_in_binary_expression() {
    const EXPECTED: &str = "5! / 6!";
    let input: &str = "5! / 6!";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that both unary prefixes and suffixes can apply
 * to the same expression
 */
fn unary_prefix_and_suffix_in_expression() {
    const EXPECTED: &str = "-5!";
    let input: &str = "-5!";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that unary prefixes can be repeated
 */
fn unary_repeated_prefix() {
    const EXPECTED: &str = "--5";
    let input: &str = "--5";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that unary suffixes can be repeated
 */
fn unary_repeated_suffix() {
    const EXPECTED: &str = "5!!";
    let input: &str = "5!!";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that both unary prefixes and suffixes can be repeated
 */
fn unary_repeated_prefix_and_suffix() {
    const EXPECTED: &str = "---5!!";
    let input: &str = "---5!!";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that both unary prefixes and suffixes can be applied
 * seperately in the same binary expression
 */
fn unary_prefix_and_suffix_in_binary_expression() {
    const EXPECTED: &str = "-5 ^ 3!!";
    let input: &str = "-5 ^ 3!!";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that unary prefixes work on parentheses
 * expressions
 */
fn unary_prefix_parentheses() {
    const EXPECTED: &str = "-[1 + 2]";
    let input: &str = "-(1+2)";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that unary suffixes work on parentheses
 * expressions
 */
fn unary_suffixes_parentheses() {
    const EXPECTED: &str = "[1 + 2]!";
    let input: &str = "(1+2)!";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that no-arg functions can be read in
 */
fn empty_function() {
    const EXPECTED: &str = "fx()";
    let input: &str = "fx()";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that single-arg functions can be read in
 */
fn one_arg_function_0() {
    const EXPECTED: &str = "fx(5)";
    let input: &str = "fx(5)";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that single-arg functions can be read in
 * when the arg is a more complex expression
 */
fn one_arg_function_1() {
    const EXPECTED: &str = "fx([5 ^ 2] - [5 + 1])";
    let input: &str = "fx(5^2-(5+1))";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that multiple-arg functions can be read in
 */
fn multiple_arg_function_0() {
    const EXPECTED: &str = "fx(5, 3, 1, 2)";
    let input: &str = "fx(5, 3, 1, 2)";
    default_test(input, EXPECTED);
}

#[test]
/**
 * Test that multiple-arg functions can be read in
 * when the some args are a more complex expressions
 */
fn multiple_arg_function_1() {
    const EXPECTED: &str = "fx(1, 2 + 3, 4, 3 - [1 ^ [6 - 2]])";
    let input: &str = "fx(1, 2 + 3, 4, 3 - 1^(6-2))";
    default_test(input, EXPECTED);
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_0() {
    let input: &str = "+";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_1() {
    let input: &str = "5 *";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_2() {
    let input: &str = "+ 5";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_3() {
    let input: &str = "5 * (6 - 3";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_4() {
    let input: &str = "5(6 - 3)";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_5() {
    let input: &str = "#";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_6() {
    let input: &str = "func)";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_7() {
    let input: &str = "5()";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_8() {
    let input: &str = "()";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_9() {
    let input: &str = "-";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_10() {
    let input: &str = "!";
    default_test(input, "");
}

#[test]
#[should_panic]
/**
 * Test that the following is invalid syntax
 */
fn invalid_syntax_11() {
    let input: &str = "-!";
    default_test(input, "");
}