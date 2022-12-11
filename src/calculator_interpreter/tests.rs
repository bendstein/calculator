use std::ops::{Range, RangeInclusive};

use super::Interpreter;

/**
 * The provided input, should be parsed and evaluated
 * to equal to the expected value
 */
fn default_test(input: &str, expected: f64) {
    const THRESHOLD: f64 = 4_f64 * f64::EPSILON;

    match Interpreter::default().evaluate_string(input) {
        Ok(result) => {
            assert!((result - expected).abs() < THRESHOLD, "Testing equality of {result} and {expected}.")
        },
        Err(err) => {
            panic!("{err}")
        }
    }
}

/**
 * The provided input, should be parsed and evaluated
 * to inf
 */
fn inf_test(input: &str) {

    match Interpreter::default().evaluate_string(input) {
        Ok(result) => {
            assert!(result.is_infinite(), "Testing that {result} = f64::inf")
        },
        Err(err) => {
            panic!("{err}")
        }
    }
}

/**
 * The provided input, should be parsed and evaluated
 * to NaN
 */
fn nan_test(input: &str) {

    match Interpreter::default().evaluate_string(input) {
        Ok(result) => {
            assert!(result.is_nan(), "Testing that {result} is not a number")
        },
        Err(err) => {
            panic!("{err}")
        }
    }
}

fn in_range_test(input: &str, range: Range<f64>, repeat: usize) {
    let interpreter = Interpreter::default();

    for _ in 0_usize..repeat {
        match interpreter.evaluate_string(input) {
            Ok(result) => {
                assert!(range.contains(&result), "Testing that {result} is in the range [{}, {}).", range.start, range.end)
            },
            Err(err) => {
                panic!("{err}")
            }
        }
    }   
}

fn in_range_inc_test(input: &str, range: RangeInclusive<f64>, repeat: usize) {
    let interpreter = Interpreter::default();

    for _ in 0_usize..repeat {
        match interpreter.evaluate_string(input) {
            Ok(result) => {
                assert!(range.contains(&result), "Testing that {result} is in the range [{}, {}].", range.start(), range.end())
            },
            Err(err) => {
                panic!("{err}")
            }
        }
    }  
}

/**
 * Test if the given input is an integer
 */
fn is_integer_test(input: &str, repeat: usize) {
    let interpreter = Interpreter::default();

    for _ in 0_usize..repeat {
        match interpreter.evaluate_string(input) {
            Ok(result) => {
                assert!(result.fract().abs() < f64::EPSILON, "Testing that {input} is an integer.")
            },
            Err(err) => {
                panic!("{err}")
            }
        }
    }   
}

/**
 * The provided input should panic
 */
fn panic_test(input: &str) {
    match Interpreter::default().evaluate_string(input) {
        Ok(result) => println!("{result}"),
        Err(err) => {
            panic!("{err}")
        }
    }
}

#[test]
/**
 * Test that an empty string evaluates to 0
 */
fn empty_string() {
    let expected: f64 = 0_f64;
    let input: &str = "";
    default_test(input, expected);
}

#[test]
/**
 * Test that a single number evaluates to the same number,
 * when an integer is provided
 */
fn integer_number() {
    let expected: f64 = 15_f64;
    let input: &str = "15";
    default_test(input, expected);
}

#[test]
/**
 * Test that a single number evaluates to the same number,
 * when a floating point number is provided
 */
fn floating_point_number() {
    let expected: f64 = 12.531_f64;
    let input: &str = "12.531";
    default_test(input, expected);
}

#[test]
/**
 * Test that a negative number is evaluated to the same
 * number
 */
fn negative_number() {
    let expected: f64 = -15_f64;
    let input: &str = "-15";
    default_test(input, expected);
}

#[test]
/**
 * Test that a number followed by a factorial is evaluated
 * to that number's factorial for a positive number
 */
fn positive_factorial() {
    let expected: f64 = 120_f64;
    let input: &str = "5!";
    default_test(input, expected);
}

#[test]
/**
 * Test that a number followed by a factorial is evaluated
 * to that number's factorial for a 0
 */
fn zero_factorial() {
    let expected: f64 = 1_f64;
    let input: &str = "0!";
    default_test(input, expected);
}

#[test]
#[should_panic]
/**
 * Test that a negative number followed by a factorial returns
 * an error
 */
fn negative_factorial() {
    let input: &str = "(-1)!";
    panic_test(input);
}

#[test]
/**
 * Test that addition operator evaluates as expected
 */
fn addition_positive_result() {
    let expected: f64 = 1_f64 + 2_f64;
    let input: &str = "1 + 2";
    default_test(input, expected);
}

#[test]
/**
 * Test that addition operator evaluates as expected
 */
fn addition_negative_result() {
    let expected: f64 = 1_f64 + -2_f64;
    let input: &str = "1 + -2";
    default_test(input, expected);
}

#[test]
/**
 * Test that addition operator evaluates as expected
 */
fn addition_identity_result() {
    let expected: f64 = 3_f64 + 0_f64;
    let input: &str = "3 + 0";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_positive_result() {
    let expected: f64 = 6_f64 - 1_f64;
    let input: &str = "6 - 1";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_zero_result() {
    let expected: f64 = 5_f64 - 5_f64;
    let input: &str = "5 - 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_negative_result() {
    let expected: f64 = 3_f64 - 15_f64;
    let input: &str = "3 - 15";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_identity_result() {
    let expected: f64 = 5_f64 - 0_f64;
    let input: &str = "5 - 0";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_double_negative_result() {
    let expected: f64 = -5_f64 - -3_f64;
    let input: &str = "-5 - -3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_positive_result() {
    let expected: f64 = 3_f64 * 4_f64;
    let input: &str = "3 * 4";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_zero_result() {
    let expected: f64 = 123124234.128912_f64 * 0_f64;
    let input: &str = "123124234.128912 * 0";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_negative_result() {
    let expected: f64 = 15_f64 * -2_f64;
    let input: &str = "15 * -2";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_identity_result() {
    let expected: f64 = 15_f64 * 1_f64;
    let input: &str = "15 * 1";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_double_negative_result() {
    let expected: f64 = -15_f64 * -3_f64;
    let input: &str = "-15 * -3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_integer_result() {
    let expected: f64 = 15_f64 / 3_f64;
    let input: &str = "15 / 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_fractional_result() {
    let expected: f64 = 3_f64 / 15_f64;
    let input: &str = "3 / 15";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_negative_result() {
    let expected: f64 = -14_f64 / 2.5_f64;
    let input: &str = "-14 / 2.5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_double_negative_result() {
    let expected: f64 = -4_f64 / -16_f64;
    let input: &str = "-4 / -16";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_divide_by_zero_result() {
    let input: &str = "15 / 0";
    inf_test(input);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_0() {
    let expected: f64 = 0_f64 % 5_f64;
    let input: &str = "0 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_1() {
    let expected: f64 = 3_f64 % 5_f64;
    let input: &str = "3 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_2() {
    let expected: f64 = 5_f64 % 5_f64;
    let input: &str = "5 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_3() {
    let expected: f64 = 7_f64 % 5_f64;
    let input: &str = "7 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_4() {
    let expected: f64 = -2_f64 % 5_f64;
    let input: &str = "-2 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_5() {
    let expected: f64 = -5_f64 % 5_f64;
    let input: &str = "-5 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_6() {
    let input: &str = "5 % 0";
    nan_test(input);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_0() {
    let expected: f64 = 0_f64.powf(0_f64);
    let input: &str = "0 ^ 0";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_1() {
    let expected: f64 = 0_f64.powf(15_f64);
    let input: &str = "0 ^ 15";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_2() {
    let expected: f64 = 1_f64.powf(12_f64);
    let input: &str = "1 ^ 12";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_3() {
    let expected: f64 = 2_f64.powf(3_f64);
    let input: &str = "2 ^ 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_4() {
    let expected: f64 = 2.5_f64.powf(2_f64);
    let input: &str = "2.5 ^ 2";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_5() {
    let expected: f64 = (-2_f64).powf(3_f64);
    let input: &str = "-2^3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_6() {
    let expected: f64 = 5_f64.powf(-2_f64);
    let input: &str = "5^-2";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_7() {
    let expected: f64 = 36_f64.powf(0.5_f64);
    let input: &str = "36^0.5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_8() {
    let expected: f64 = 256_f64.powf(-0.5_f64);
    let input: &str = "256^-0.5";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_0() {
    let expected: f64 = 5_f64 + 6_f64 - 3_f64;
    let input: &str = "5 + 6 - 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_1() {
    let expected: f64 = 5_f64 + 6_f64 * 3_f64;
    let input: &str = "5 + 6 * 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_2() {
    let expected: f64 = (5_f64 + 6_f64) * 3_f64;
    let input: &str = "(5 + 6) * 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_3() {
    let expected: f64 = 1_f64 + 5_f64.powf(3_f64 - 1_f64);
    let input: &str = "1 + 5 ^ (3 - 1)";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_4() {
    let expected: f64 = 11_f64;
    let input: &str = "((3 * 8 / 6 + 1)! + 1) ^ (3 / 6)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sqrt function evaluates as expected
 */
fn func_sqrt() {
    let expected: f64 = (5_f64 * 5_f64).sqrt();
    let input: &str = "sqrt(5 * 5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that log function evaluates as expected
 */
fn func_log() {
    let expected: f64 = f64::log10(16_f64);
    let input: &str = "log(16)";
    default_test(input, expected);
}

#[test]
/**
 * Test that log base b function evaluates as expected
 */
fn func_logb() {
    let expected: f64 = f64::log(16_f64, 3_f64);
    let input: &str = "logb(16, 3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that log base 2 function evaluates as expected
 */
fn func_log2() {
    let expected: f64 = f64::log(16_f64, 2_f64);
    let input: &str = "log2(16)";
    default_test(input, expected);
}


#[test]
/**
 * Test that log base e function evaluates as expected
 */
fn func_ln() {
    let expected: f64 = f64::log(19_f64, std::f64::consts::E);
    let input: &str = "ln(19)";
    default_test(input, expected);
}

#[test]
/**
 * Test that add function evaluates as expected
 */
fn func_add() {
    let expected: f64 = 5_f64 + 2_f64;
    let input: &str = "add(5, 2)";
    default_test(input, expected);
}

#[test]
/**
 * Test that add function evaluates as expected when provided several arguments
 */
fn func_add_var() {
    let expected: f64 = -2_f64 + 4_f64 + 1.01_f64 + 11_f64;
    let input: &str = "add(-2, 4, 1.01, 11)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sub function evaluates as expected
 */
fn func_sub() {
    let expected: f64 = 15_f64 - -4_f64;
    let input: &str = "sub(15, -4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sub function evaluates as expected when provided several arguments
 */
fn func_sub_var() {
    let expected: f64 = 15_f64 - -2_f64 - 3.1_f64 - 0_f64 - 1_f64;
    let input: &str = "sub(15, -2, 3.1, 0, 1)";
    default_test(input, expected);
}

#[test]
/**
 * Test that mult function evaluates as expected
 */
fn func_mult() {
    let expected: f64 = 8_f64 * 3_f64;
    let input: &str = "mult(8, 3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that mult function evaluates as expected when provided several arguments
 */
fn func_mult_var() {
    let expected: f64 = 4_f64 * 2_f64 * 6_f64 * 0.4_f64;
    let input: &str = "mult(4, 2, 6, 0.4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that div function evaluates as expected
 */
fn func_div() {
    let expected: f64 = 124_f64 / 16_f64;
    let input: &str = "div(124, 16)";
    default_test(input, expected);
}

#[test]
/**
 * Test that div function evaluates as expected when provided several arguments
 */
fn func_div_var() {
    let expected: f64 = 25_f64 / 10_f64 / 0.25_f64 / 8_f64;
    let input: &str = "div(25, 10, 0.25, 8)";
    default_test(input, expected);
}

#[test]
/**
 * Test that rem function evaluates as expected
 */
fn func_rem() {
    let expected: f64 = 9_f64 % 5_f64;
    let input: &str = "rem(9, 5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that rem function evaluates as expected when
 * provided several arguments
 */
fn func_rem_var() {
    let expected: f64 = 9_f64 % 5_f64 % 6_f64 % 3_f64;
    let input: &str = "rem(9, 5, 6, 3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that pow function evaluates as expected
 */
fn func_pow() {
    let expected: f64 = 4_f64.powf(2_f64);
    let input: &str = "pow(4, 2)";
    default_test(input, expected);
}

#[test]
/**
 * Test that exp function evaluates as expected
 */
fn func_exp() {
    let expected: f64 = 4_f64.exp();
    let input: &str = "exp(4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that exp2 function evaluates as expected
 */
fn func_exp2() {
    let expected: f64 = 5_f64.exp2();
    let input: &str = "exp2(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that fac function evaluates as expected
 */
fn func_fac() {
    let expected: f64 = 120_f64;
    let input: &str = "fac(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that neg function evaluates as expected
 */
fn func_neg() {
    let expected: f64 = -13_f64;
    let input: &str = "neg(13)";
    default_test(input, expected);
}

#[test]
/**
 * Test that ceil function evaluates as expected
 */
fn func_ceil() {
    let expected: f64 = 5.4_f64.ceil();
    let input: &str = "ceil(5.4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that floor function evaluates as expected
 */
fn func_floor() {
    let expected: f64 = 6.6_f64.floor();
    let input: &str = "floor(6.6)";
    default_test(input, expected);
}

#[test]
/**
 * Test that round function evaluates as expected
 */
fn func_round_0() {
    let expected: f64 = 6.4_f64.round();
    let input: &str = "round(6.4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that round function evaluates as expected
 */
fn func_round_1() {
    let expected: f64 = 6.5_f64.round();
    let input: &str = "round(6.5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that round function evaluates as expected
 */
fn func_round_2() {
    let expected: f64 = 6.6_f64.round();
    let input: &str = "round(6.6)";
    default_test(input, expected);
}

#[test]
/**
 * Test that fract function evaluates as expected
 */
fn func_fract_0() {
    let expected: f64 = 15_f64.fract();
    let input: &str = "fract(15)";
    default_test(input, expected);
}

#[test]
/**
 * Test that fract function evaluates as expected
 */
fn func_fract_1() {
    let expected: f64 = 12.128_f64.fract();
    let input: &str = "fract(12.128)";
    default_test(input, expected);
}

#[test]
/**
 * Test that fract function evaluates as expected
 */
fn func_fract_2() {
    let expected: f64 = -91.93_f64.fract();
    let input: &str = "fract(-91.93)";
    default_test(input, expected);
}

#[test]
/**
 * Test that max function evaluates as expected
 */
fn fn_max_0() {
    let expected: f64 = 5_f64.max(3_f64);
    let input: &str = "max(5, 3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that max function evaluates as expected
 */
fn fn_max_1() {
    let expected: f64 = (-3_f64).max(-5_f64);
    let input: &str = "max(-5, -3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that max function evaluates as expected when
 * several args are provided
 */
fn fn_max_var() {
    let expected: f64 = 5_f64.max(4.99_f64.max((-2_f64).max((-2.01_f64).max(3_f64.max(4_f64)))));
    let input: &str = "max(5, 4.99, -2, -2.01, 3, 4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that min function evaluates as expected
 */
fn fn_min_0() {
    let expected: f64 = 5_f64.min(3_f64);
    let input: &str = "min(5, 3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that min function evaluates as expected
 */
fn fn_min_1() {
    let expected: f64 = (-5_f64).min(-3_f64);
    let input: &str = "min(-5, -3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that min function evaluates as expected when
 * several args are provided
 */
fn fn_min_var() {
    let expected: f64 = 5_f64.min(4.99_f64.min((-2_f64).min((-2.01_f64).min(3_f64.min(4_f64)))));
    let input: &str = "min(5, 4.99, -2, -2.01, 3, 4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sign function evaluates as expected
 */
fn fn_sign_0() {
    let expected: f64 = 15_f64.signum();
    let input: &str = "sign(15)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sign function evaluates as expected
 */
fn fn_sign_1() {
    let expected: f64 = (-12.4_f64).signum();
    let input: &str = "sign(-12.4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that cond function evaluates as expected
 */
fn fn_cond_0() {
    let expected: f64 = match 5_f64 == 1_f64 {
        true => 1_f64,
        false => 0_f64
    };
    let input: &str = "cond(5, 1, 1, 0)";
    default_test(input, expected);
}

#[test]
/**
 * Test that cond function evaluates as expected
 */
fn fn_cond_1() {
    let expected: f64 = match 1_f64 == 1_f64 {
        true => 1_f64,
        false => 0_f64
    };
    let input: &str = "cond(1, 1, 1, 0)";
    default_test(input, expected);
}

#[test]
/**
 * Test that PI function evaluates as expected
 */
fn fn_pi() {
    let expected: f64 = std::f64::consts::PI;
    let input: &str = "pi()";
    default_test(input, expected);
}

#[test]
/**
 * Test that E function evaluates as expected
 */
fn fn_e() {
    let expected: f64 = std::f64::consts::E;
    let input: &str = "e()";
    default_test(input, expected);
}

#[test]
/**
 * Test that PI constant evaluates as expected
 */
fn constant_pi() {
    let expected: f64 = std::f64::consts::PI;
    let input: &str = "pi";
    default_test(input, expected);
}

#[test]
/**
 * Test that E constant evaluates as expected
 */
fn constant_e() {
    let expected: f64 = std::f64::consts::E;
    let input: &str = "e";
    default_test(input, expected);
}


#[test]
/**
 * Test that PI constant evaluates as expected when nested in a function
 */
fn constant_pi_nested() {
    let expected: f64 = std::f64::consts::PI.sqrt();
    let input: &str = "sqrt(pi)";
    default_test(input, expected);
}

#[test]
/**
 * Test that E constant evaluates as expected when nested in a function
 */
fn constant_e_nested() {
    let expected: f64 = std::f64::consts::E.sqrt();
    let input: &str = "sqrt(e)";
    default_test(input, expected);
}
#[test]
/**
 * Test that sin function evaluates as expected
 */
fn fn_sin() {
    let expected: f64 = 5_f64.sin();
    let input: &str = "sin(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that cos function evaluates as expected
 */
fn fn_cos() {
    let expected: f64 = 5_f64.cos();
    let input: &str = "cos(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that tan function evaluates as expected
 */
fn fn_tan() {
    let expected: f64 = 5_f64.tan();
    let input: &str = "tan(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that asin function evaluates as expected
 */
fn fn_asin() {
    let expected: f64 = 0.5_f64.asin();
    let input: &str = "asin(0.5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that acos function evaluates as expected
 */
fn fn_acos() {
    let expected: f64 = 0.5_f64.acos();
    let input: &str = "acos(0.5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that atan function evaluates as expected
 */
fn fn_atan() {
    let expected: f64 = 0.5_f64.atan();
    let input: &str = "atan(0.5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that csc function evaluates as expected
 */
fn fn_csc() {
    let expected: f64 = 1_f64 / 5_f64.sin();
    let input: &str = "csc(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sec function evaluates as expected
 */
fn fn_sec() {
    let expected: f64 = 1_f64 / 5_f64.cos();
    let input: &str = "sec(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that cot function evaluates as expected
 */
fn fn_cot() {
    let expected: f64 = 1_f64 / 5_f64.tan();
    let input: &str = "cot(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that acsc function evaluates as expected
 */
fn fn_acsc() {
    let expected: f64 = (1_f64 / 5_f64).asin();
    let input: &str = "acsc(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that asec function evaluates as expected
 */
fn fn_asec() {
    let expected: f64 = (1_f64 / 5_f64).acos();
    let input: &str = "asec(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that acot function evaluates as expected
 */
fn fn_acot() {
    let expected: f64 = (1_f64 / 5_f64).atan();
    let input: &str = "acot(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sinh function evaluates as expected
 */
fn fn_sinh() {
    let expected: f64 = 5_f64.sinh();
    let input: &str = "sinh(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that cosh function evaluates as expected
 */
fn fn_cosh() {
    let expected: f64 = 5_f64.cosh();
    let input: &str = "cosh(5)";
    default_test(input, expected);
}
#[test]

/**
 * Test that tanh function evaluates as expected
 */
fn fn_tanh() {
    let expected: f64 = 5_f64.tanh();
    let input: &str = "tanh(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that asinh function evaluates as expected
 */
fn fn_asinh() {
    let expected: f64 = 0.5_f64.asinh();
    let input: &str = "asinh(0.5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that acosh function evaluates as expected
 */
fn fn_acosh() {
    let expected: f64 = 1_f64.acosh();
    let input: &str = "acosh(1)";
    default_test(input, expected);
}

#[test]
/**
 * Test that atanh function evaluates as expected
 */
fn fn_atanh() {
    let expected: f64 = 0.5_f64.atanh();
    let input: &str = "atanh(0.5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that infix functions evaluate as expected
 */
fn infix_function_0() {
    let expected: f64 = 5_f64 * 6_f64;
    let input: &str = "5 mult 6";
    default_test(input, expected);
}

#[test]
/**
 * Test that infix functions evaluate as expected
 */
fn infix_function_1() {
    let expected: f64 = 5_f64.powf(2_f64) + 6_f64;
    let input: &str = "5^2 add 3!";
    default_test(input, expected);
}

#[test]
/**
 * Test that infix functions evaluate as expected
 */
fn infix_function_2() {
    let expected: f64 = 24_f64;
    let input: &str = "(sqrt(2^(2 add 4)) div 2)!";
    default_test(input, expected);
}

#[test]
/**
 * Test that random float function returns a number between 0 and 1
 */
fn frand_0() {
    let repeat: usize = 500_usize;
    let range = 0_f64..1_f64;
    let input: &str = "frand()";
    in_range_test(input, range, repeat)
}

#[test]
/**
 * Test that ranged random float function returns a number
 * in the range
 */
fn frand_1() {
    let repeat: usize = 500_usize;
    let range = 5_f64..12.5_f64;
    let input: &str = "rfrand(5, 12.5)";
    in_range_test(input, range, repeat)
}

#[test]
/**
 * Test that inclusive ranged random float function returns a number
 * in the range
 */
fn frand_2() {
    let repeat: usize = 500_usize;
    let range = 12.05_f64..=19_f64;
    let input: &str = "rfrandi(12.05, 19)";
    in_range_inc_test(input, range, repeat)
}

#[test]
/**
 * Test that random int function returns an integer number
 */
fn rand_0() {
    let repeat: usize = 500_usize;
    let input: &str = "rand()";
    is_integer_test(input, repeat)
}

#[test]
/**
 * Test that ranged random int function returns an integer number
 * in the range
 */
fn rand_1() {
    let repeat: usize = 500_usize;
    let range = 5_f64..12.5_f64;
    let input: &str = "rrand(5, 12.5)";
    in_range_test(input, range, repeat);
    is_integer_test(input, repeat);
}

#[test]
/**
 * Test that inclusive ranged random int function returns an integer number
 * in the range
 */
fn rand_2() {
    let repeat: usize = 500_usize;
    let range = 12.05_f64..=19_f64;
    let input: &str = "rrandi(12.05, 19)";
    in_range_inc_test(input, range, repeat);
    is_integer_test(input, repeat);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_0() {
    let expected: f64 = 1_f64;
    let input: &str = "5 mod 4";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_1() {
    let expected: f64 = 4_f64;
    let input: &str = "4 mod 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_2() {
    let expected: f64 = 0_f64;
    let input: &str = "5 mod 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_3() {
    let expected: f64 = 2_f64;
    let input: &str = "-3 mod 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_4() {
    let expected: f64 = -2_f64;
    let input: &str = "3 mod -5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_5() {
    let expected: f64 = -3_f64;
    let input: &str = "-3 mod -5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_6() {
    let expected: f64 = 0.5_f64;
    let input: &str = "0.5 mod 2.1";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_7() {
    let expected: f64 = 0_f64;
    let input: &str = "0.5 mod 0.25";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_8() {
    let input: &str = "0.5 mod 0";
    nan_test(input)
}


#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_9() {
    let expected: f64 = -0.5_f64;
    let input: &str = "0.5 mod -1";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_10() {
    let expected: f64 = 0_f64;
    let input: &str = "3 mod -1";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_11() {
    let expected: f64 = -0.8_f64;
    let input: &str = "5.2 mod -3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_12() {
    let expected: f64 = 0.8_f64;
    let input: &str = "-5.2 mod 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_13() {
    let expected: f64 = -2.2_f64;
    let input: &str = "-5.2 mod -3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the modulo function outputs as expected
 */
fn mod_14() {
    let expected: f64 = 2.2_f64;
    let input: &str = "5.2 mod 3";
    default_test(input, expected);
}