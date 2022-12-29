use super::{*, super::function::function_impl::*};

lazy_static! {
    pub(in crate::calculator::calculator_interpreter) static ref ADD: Function = Function::new(FunctionArgs::Variable(add_all));
    pub(in crate::calculator::calculator_interpreter) static ref SUB: Function = Function::new(FunctionArgs::Variable(sub_all));
    pub(in crate::calculator::calculator_interpreter) static ref MULT: Function = Function::new(FunctionArgs::Variable(mult_all));
    pub(in crate::calculator::calculator_interpreter) static ref DIV: Function = Function::new(FunctionArgs::Variable(div_all));
    pub(in crate::calculator::calculator_interpreter) static ref REM: Function = Function::new(FunctionArgs::Variable(rem_all));

    pub(in crate::calculator::calculator_interpreter) static ref MAX: Function = Function::new(FunctionArgs::Variable(max_all));
    pub(in crate::calculator::calculator_interpreter) static ref MIN: Function = Function::new(FunctionArgs::Variable(min_all));

    pub(in crate::calculator::calculator_interpreter) static ref MOD: Function = Function::new(FunctionArgs::Two(modulo));

    pub(in crate::calculator::calculator_interpreter) static ref NEG: Function = Function::new(FunctionArgs::One(|n: f64| Ok(-n)));
    pub(in crate::calculator::calculator_interpreter) static ref FAC: Function = Function::new(FunctionArgs::One(factorial));
    
    pub(in crate::calculator::calculator_interpreter) static ref CEIL: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::ceil(n))));
    pub(in crate::calculator::calculator_interpreter) static ref FLOOR: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::floor(n))));
    pub(in crate::calculator::calculator_interpreter) static ref ROUND: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::round(n))));
    
    pub(in crate::calculator::calculator_interpreter) static ref FRACT: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::fract(n))));

    pub(in crate::calculator::calculator_interpreter) static ref SQRT: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::sqrt(n))));
    pub(in crate::calculator::calculator_interpreter) static ref EXP: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::exp(n))));
    pub(in crate::calculator::calculator_interpreter) static ref EXP2: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::exp2(n))));
    pub(in crate::calculator::calculator_interpreter) static ref POW: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| Ok(f64::powf(a, b))));

    pub(in crate::calculator::calculator_interpreter) static ref SIN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::sin(n))));
    pub(in crate::calculator::calculator_interpreter) static ref COS: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::cos(n))));
    pub(in crate::calculator::calculator_interpreter) static ref TAN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::tan(n))));

    pub(in crate::calculator::calculator_interpreter) static ref ASIN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::asin(n))));
    pub(in crate::calculator::calculator_interpreter) static ref ACOS: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::acos(n))));
    pub(in crate::calculator::calculator_interpreter) static ref ATAN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::atan(n))));

    pub(in crate::calculator::calculator_interpreter) static ref CSC: Function = Function::new(FunctionArgs::One(|n: f64| Ok(1_f64 / f64::sin(n))));
    pub(in crate::calculator::calculator_interpreter) static ref SEC: Function = Function::new(FunctionArgs::One(|n: f64| Ok(1_f64 / f64::cos(n))));
    pub(in crate::calculator::calculator_interpreter) static ref COT: Function = Function::new(FunctionArgs::One(|n: f64| Ok(1_f64 / f64::tan(n))));

    pub(in crate::calculator::calculator_interpreter) static ref ACSC: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::asin(1_f64 / n))));
    pub(in crate::calculator::calculator_interpreter) static ref ASEC: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::acos(1_f64 / n))));
    pub(in crate::calculator::calculator_interpreter) static ref ACOT: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::atan(1_f64 / n))));

    pub(in crate::calculator::calculator_interpreter) static ref SINH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::sinh(n))));
    pub(in crate::calculator::calculator_interpreter) static ref COSH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::cosh(n))));
    pub(in crate::calculator::calculator_interpreter) static ref TANH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::tanh(n))));

    pub(in crate::calculator::calculator_interpreter) static ref ASINH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::asinh(n))));
    pub(in crate::calculator::calculator_interpreter) static ref ACOSH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::acosh(n))));
    pub(in crate::calculator::calculator_interpreter) static ref ATANH: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::atanh(n))));

    pub(in crate::calculator::calculator_interpreter) static ref LOG: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::log10(n))));
    pub(in crate::calculator::calculator_interpreter) static ref LOG2: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::log2(n))));
    pub(in crate::calculator::calculator_interpreter) static ref LN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(n.log(std::f64::consts::E))));
    pub(in crate::calculator::calculator_interpreter) static ref LOGB: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| Ok(f64::log(a, b))));

    pub(in crate::calculator::calculator_interpreter) static ref SIGN: Function = Function::new(FunctionArgs::One(|n: f64| Ok(f64::signum(n))));
    pub(in crate::calculator::calculator_interpreter) static ref COND: Function = Function::new(FunctionArgs::Four(|a: f64, b: f64, c: f64, d: f64| {
        Ok(if a == b {
            c
        }
        else {
            d
        })
    }));

    pub(in crate::calculator::calculator_interpreter) static ref FRAND: Function = Function::new(FunctionArgs::None(random::<f64>));
    pub(in crate::calculator::calculator_interpreter) static ref RFRAND: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| random_range::<f64>(a..b)));
    pub(in crate::calculator::calculator_interpreter) static ref RFRANDI: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| random_range_inc::<f64>(a..=b)));

    pub(in crate::calculator::calculator_interpreter) static ref RAND: Function = Function::new(FunctionArgs::None(random::<i32>));
    pub(in crate::calculator::calculator_interpreter) static ref RRAND: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| random_range::<i32>(a.ceil() as i32..b.floor() as i32)));
    pub(in crate::calculator::calculator_interpreter) static ref RRANDI: Function = Function::new(FunctionArgs::Two(|a: f64, b: f64| random_range_inc::<i32>(a.ceil() as i32..=b.floor() as i32)));

    pub(in crate::calculator::calculator_interpreter) static ref E: Function = Function::new(FunctionArgs::None(|| Ok(std::f64::consts::E)));
    pub(in crate::calculator::calculator_interpreter) static ref PI: Function = Function::new(FunctionArgs::None(|| Ok(std::f64::consts::PI)));
}