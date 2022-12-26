use super::super::interpreter_err::InterpreterErr;
use rand::Rng;

pub fn factorial(n: f64) -> Result<f64, InterpreterErr> {
    if n == 0_f64 {
        Ok(1_f64)
    }
    else if n < 0_f64 {
        Err(InterpreterErr::new("Cannot apply factorial operator to negative value."))
    }
    else if n != n.round() {
        Err(InterpreterErr::new("Cannot apply factorial operator to floating point value."))
    }
    else {
        let rounded = n.round() as i32;
        let mut agg = 1;
        for i in 0..rounded {
            if let Some(product) = i32::checked_mul(agg, rounded - i) {
                agg = product;
            }
            else {
                Err(InterpreterErr::new("Integer Overflow"))?
            }
        }
        Ok(agg as f64)
    }
}

pub fn modulo(a: f64, b: f64) -> Result<f64, InterpreterErr> {
    //If b is 0, a mod b is undefined
    if b.abs() < f64::EPSILON {
        Ok(f64::NAN)
    }
    //If a is an integer b is -1, a mod b is 0
    else if (a.fract().abs() < f64::EPSILON) && (b + 1_f64).abs() < f64::EPSILON {
        Ok(0_f64)
    }
    else {
        let rem = a % b;

        //If a and b have the same sign, a mod b = a % b
        if a.signum() == b.signum() {
            Ok(rem)
        }
        //Otherwise, a mod b = (a % b) + b
        else {
            Ok(rem + b)
        }
    }
}

pub fn random<T>() -> Result<f64, InterpreterErr>
    where T : Into<f64>, 
    rand::distributions::Standard: rand::distributions::Distribution<T>
{
    Ok(rand::thread_rng().gen::<T>().into())
}

pub fn random_range<T>(range: std::ops::Range<T>) -> Result<f64, InterpreterErr>
    where T : Into<f64> + std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform, 
    rand::distributions::Standard: rand::distributions::Distribution<T>
{
    Ok(rand::thread_rng().gen_range(range).into())
}

pub fn random_range_inc<T>(range: std::ops::RangeInclusive<T>) -> Result<f64, InterpreterErr>
    where T : Into<f64> + std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform, 
    rand::distributions::Standard: rand::distributions::Distribution<T>
{
    Ok(rand::thread_rng().gen_range(range).into())
}

pub fn add_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a + b);

    Ok(maybe_value.unwrap_or(0_f64))
}

pub fn sub_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a - b);

    Ok(maybe_value.unwrap_or(0_f64))
}

pub fn mult_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a * b);

    Ok(maybe_value.unwrap_or(0_f64))
}

pub fn div_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a / b);

    Ok(maybe_value.unwrap_or(0_f64))
}

pub fn rem_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(|a, b| a % b);

    Ok(maybe_value.unwrap_or(0_f64))
}

pub fn max_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(f64::max);

    Ok(maybe_value.unwrap_or(0_f64))
}

pub fn min_all(values: Vec<f64>) -> Result<f64, InterpreterErr> {
    let maybe_value: Option<f64> = values.iter()
    .copied()
    .reduce(f64::min);

    Ok(maybe_value.unwrap_or(0_f64))
}