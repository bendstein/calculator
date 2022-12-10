use super::Interpreter;

/**
 * The provided input, should be parsed and evaluated
 * to equal to the expected value
 */
fn default_test(input: &str, expected: f32) {

    match Interpreter::default().evaluate_string(input) {
        Ok(result) => {
            assert!((result - expected).abs() < f32::EPSILON, "Testing equality of {result} and {expected}.")
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
            assert!(result.is_infinite(), "Testing that {result} = f32::inf")
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
    let expected: f32 = 0_f32;
    let input: &str = "";
    default_test(input, expected);
}

#[test]
/**
 * Test that a single number evaluates to the same number,
 * when an integer is provided
 */
fn integer_number() {
    let expected: f32 = 15_f32;
    let input: &str = "15";
    default_test(input, expected);
}

#[test]
/**
 * Test that a single number evaluates to the same number,
 * when a floating point number is provided
 */
fn floating_point_number() {
    let expected: f32 = 12.531_f32;
    let input: &str = "12.531";
    default_test(input, expected);
}

#[test]
/**
 * Test that a negative number is evaluated to the same
 * number
 */
fn negative_number() {
    let expected: f32 = -15_f32;
    let input: &str = "-15";
    default_test(input, expected);
}

#[test]
/**
 * Test that a number followed by a factorial is evaluated
 * to that number's factorial for a positive number
 */
fn positive_factorial() {
    let expected: f32 = 120_f32;
    let input: &str = "5!";
    default_test(input, expected);
}

#[test]
/**
 * Test that a number followed by a factorial is evaluated
 * to that number's factorial for a 0
 */
fn zero_factorial() {
    let expected: f32 = 1_f32;
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
    let expected: f32 = 3_f32;
    let input: &str = "1 + 2";
    default_test(input, expected);
}

#[test]
/**
 * Test that addition operator evaluates as expected
 */
fn addition_negative_result() {
    let expected: f32 = -1_f32;
    let input: &str = "1 + -2";
    default_test(input, expected);
}

#[test]
/**
 * Test that addition operator evaluates as expected
 */
fn addition_identity_result() {
    let expected: f32 = 3_f32;
    let input: &str = "3 + 0";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_positive_result() {
    let expected: f32 = 5_f32;
    let input: &str = "6 - 1";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_zero_result() {
    let expected: f32 = 0_f32;
    let input: &str = "5 - 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_negative_result() {
    let expected: f32 = -12_f32;
    let input: &str = "3 - 15";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_identity_result() {
    let expected: f32 = 5_f32;
    let input: &str = "5 - 0";
    default_test(input, expected);
}

#[test]
/**
 * Test that the subtraction operator evaluates as expected
 */
fn subtraction_double_negative_result() {
    let expected: f32 = -2_f32;
    let input: &str = "-5 - -3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_positive_result() {
    let expected: f32 = 12_f32;
    let input: &str = "3 * 4";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_zero_result() {
    let expected: f32 = 0_f32;
    let input: &str = "123124234.128912 * 0";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_negative_result() {
    let expected: f32 = -30_f32;
    let input: &str = "15 * -2";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_identity_result() {
    let expected: f32 = 15_f32;
    let input: &str = "15 * 1";
    default_test(input, expected);
}

#[test]
/**
 * Test that the multiplication operator evaluates as expected
 */
fn multiplication_double_negative_result() {
    let expected: f32 = 45_f32;
    let input: &str = "-15 * -3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_integer_result() {
    let expected: f32 = 5_f32;
    let input: &str = "15 / 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_fractional_result() {
    let expected: f32 = 0.2_f32;
    let input: &str = "3 / 15";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_negative_result() {
    let expected: f32 = -5.6_f32;
    let input: &str = "-14 / 2.5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the division operator evaluates as expected
 */
fn division_double_negative_result() {
    let expected: f32 = 0.25_f32;
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
    let expected: f32 = 0_f32;
    let input: &str = "0 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_1() {
    let expected: f32 = 3_f32;
    let input: &str = "3 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_2() {
    let expected: f32 = 0_f32;
    let input: &str = "5 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_3() {
    let expected: f32 = 2_f32;
    let input: &str = "7 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_4() {
    let expected: f32 = -2_f32;
    let input: &str = "-2 % 5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the remainder operator evaluates as expected
 */
fn remainder_5() {
    let expected: f32 = 0_f32;
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
    let expected: f32 = 1_f32;
    let input: &str = "0 ^ 0";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_1() {
    let expected: f32 = 0_f32;
    let input: &str = "0 ^ 15";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_2() {
    let expected: f32 = 1_f32;
    let input: &str = "1 ^ 12";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_3() {
    let expected: f32 = 8_f32;
    let input: &str = "2 ^ 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_4() {
    let expected: f32 = 6.25_f32;
    let input: &str = "2.5 ^ 2";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_5() {
    let expected: f32 = -8_f32;
    let input: &str = "-2^3";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_6() {
    let expected: f32 = 0.04_f32;
    let input: &str = "5^-2";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_7() {
    let expected: f32 = 6_f32;
    let input: &str = "36^0.5";
    default_test(input, expected);
}

#[test]
/**
 * Test that the exponent operator evaluates as expected
 */
fn exponent_8() {
    let expected: f32 = 0.0625_f32;
    let input: &str = "256^-0.5";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_0() {
    let expected: f32 = 8_f32;
    let input: &str = "5 + 6 - 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_1() {
    let expected: f32 = 23_f32;
    let input: &str = "5 + 6 * 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_2() {
    let expected: f32 = 33_f32;
    let input: &str = "(5 + 6) * 3";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_3() {
    let expected: f32 = 26_f32;
    let input: &str = "1 + 5 ^ (3 - 1)";
    default_test(input, expected);
}

#[test]
/**
 * Test that less trivial expressions evaluate as expected
 */
fn expression_4() {
    let expected: f32 = 11_f32;
    let input: &str = "((3 * 8 / 6 + 1)! + 1) ^ (3 / 6)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sqrt function evaluates as expected
 */
fn func_sqrt() {
    let expected: f32 = 5_f32;
    let input: &str = "sqrt(5 * 5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that log function evaluates as expected
 */
fn func_log() {
    let expected: f32 = f32::log10(16_f32);
    let input: &str = "log(16)";
    default_test(input, expected);
}

#[test]
/**
 * Test that log base b function evaluates as expected
 */
fn func_logb() {
    let expected: f32 = f32::log(16_f32, 3_f32);
    let input: &str = "logb(16, 3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that log base 2 function evaluates as expected
 */
fn func_log2() {
    let expected: f32 = f32::log(16_f32, 2_f32);
    let input: &str = "log2(16)";
    default_test(input, expected);
}


#[test]
/**
 * Test that log base e function evaluates as expected
 */
fn func_loge() {
    let expected: f32 = f32::log(19_f32, std::f32::consts::E);
    let input: &str = "loge(19)";
    default_test(input, expected);
}

#[test]
/**
 * Test that add function evaluates as expected
 */
fn func_add() {
    let expected: f32 = 7_f32;
    let input: &str = "add(5, 2)";
    default_test(input, expected);
}

#[test]
/**
 * Test that sub function evaluates as expected
 */
fn func_sub() {
    let expected: f32 = 19_f32;
    let input: &str = "sub(15, -4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that mult function evaluates as expected
 */
fn func_mult() {
    let expected: f32 = 24_f32;
    let input: &str = "mult(8, 3)";
    default_test(input, expected);
}

#[test]
/**
 * Test that div function evaluates as expected
 */
fn func_div() {
    let expected: f32 = 7.75_f32;
    let input: &str = "div(124, 16)";
    default_test(input, expected);
}

#[test]
/**
 * Test that rem function evaluates as expected
 */
fn func_rem() {
    let expected: f32 = 4_f32;
    let input: &str = "rem(9, 5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that pow function evaluates as expected
 */
fn func_pow() {
    let expected: f32 = 16_f32;
    let input: &str = "pow(4, 2)";
    default_test(input, expected);
}

#[test]
/**
 * Test that exp function evaluates as expected
 */
fn func_exp() {
    let expected: f32 = f32::exp(4_f32);
    let input: &str = "exp(4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that exp2 function evaluates as expected
 */
fn func_exp2() {
    let expected: f32 = f32::exp2(5_f32);
    let input: &str = "exp2(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that fac function evaluates as expected
 */
fn func_fac() {
    let expected: f32 = 120_f32;
    let input: &str = "fac(5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that neg function evaluates as expected
 */
fn func_neg() {
    let expected: f32 = -13_f32;
    let input: &str = "neg(13)";
    default_test(input, expected);
}

#[test]
/**
 * Test that ceil function evaluates as expected
 */
fn func_ceil() {
    let expected: f32 = 6_f32;
    let input: &str = "ceil(5.4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that floor function evaluates as expected
 */
fn func_floor() {
    let expected: f32 = 6_f32;
    let input: &str = "floor(6.6)";
    default_test(input, expected);
}

#[test]
/**
 * Test that round function evaluates as expected
 */
fn func_round_0() {
    let expected: f32 = 6_f32;
    let input: &str = "round(6.4)";
    default_test(input, expected);
}

#[test]
/**
 * Test that round function evaluates as expected
 */
fn func_round_1() {
    let expected: f32 = 7_f32;
    let input: &str = "round(6.5)";
    default_test(input, expected);
}

#[test]
/**
 * Test that round function evaluates as expected
 */
fn func_round_2() {
    let expected: f32 = 7_f32;
    let input: &str = "round(6.6)";
    default_test(input, expected);
}

#[test]
/**
 * Test that fract function evaluates as expected
 */
fn func_fract_0() {
    let expected: f32 = 0_f32;
    let input: &str = "fract(15)";
    default_test(input, expected);
}

#[test]
/**
 * Test that fract function evaluates as expected
 */
fn func_fract_1() {
    let expected: f32 = f32::fract(12.128);
    let input: &str = "fract(12.128)";
    default_test(input, expected);
}

#[test]
/**
 * Test that fract function evaluates as expected
 */
fn func_fract_2() {
    let expected: f32 = f32::fract(-91.93);
    let input: &str = "fract(-91.93)";
    default_test(input, expected);
}