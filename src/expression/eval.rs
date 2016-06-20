use expression;
use expression::enums;

pub type Expression = expression::Expression;

use std::collections::HashMap;
use std::f64;

pub fn eval_postfix_expr(expr: &Expression, vars: &HashMap<String, f64>) -> f64 {
    let mut stack: Vec<f64> = Vec::with_capacity(expr.len() / 2);
    for token in expr.iter() {
        match token {
            &enums::Token::Literal(ref x) => stack.push(x.parse::<f64>().unwrap()),
            &enums::Token::Const(ref x) => {
                match x {
                    &enums::Constant::Pi => stack.push(f64::consts::PI),
                    &enums::Constant::E => stack.push(f64::consts::E),
                }
            }
            &enums::Token::Op(ref x) => {
                let arg: f64 = stack.pop().unwrap();
                match x {
                    &enums::Operator::Negate => stack.push(-1.0f64 * arg),
                    _ => {
                        let arg2 = arg;
                        let arg1 = stack.pop().unwrap();
                        match x {
                            &enums::Operator::Add => stack.push(arg1 + arg2),
                            &enums::Operator::Sub => stack.push(arg1 - arg2),
                            &enums::Operator::Div => stack.push(arg1 / arg2),
                            &enums::Operator::Mul => stack.push(arg1 * arg2),
                            &enums::Operator::Mod => stack.push(arg1 % arg2),
                            &enums::Operator::Pow => stack.push(arg1.powf(arg2)),
                            _ => continue, // Should never hit here
                        }
                    }
                }
            }
            &enums::Token::Func(ref x) => {
                let arg: f64 = stack.pop().unwrap();
                match x {
                    &enums::Function::Abs => stack.push(f64::abs(arg)),
                    &enums::Function::Sqrt => stack.push(f64::sqrt(arg)),
                    &enums::Function::Ln => stack.push(f64::ln(arg)),
                    &enums::Function::Log => stack.push(f64::log10(arg)),
                    &enums::Function::Exp => stack.push(f64::exp(arg)),
                    &enums::Function::Sin => stack.push(f64::sin(arg)),
                    &enums::Function::Csc => stack.push(f64::recip(f64::sin(arg))),
                    &enums::Function::Cos => stack.push(f64::cos(arg)),
                    &enums::Function::Sec => stack.push(f64::recip(f64::cos(arg))),
                    &enums::Function::Tan => stack.push(f64::tan(arg)),
                    &enums::Function::Cot => stack.push(f64::recip(f64::tan(arg))),
                    &enums::Function::Asin => stack.push(f64::asin(arg)),
                    &enums::Function::Acsc => stack.push(f64::recip(f64::asin(arg))),
                    &enums::Function::Acos => stack.push(f64::acos(arg)),
                    &enums::Function::Asec => stack.push(f64::recip(f64::acos(arg))),
                    &enums::Function::Atan => stack.push(f64::atan(arg)),
                    &enums::Function::Acot => stack.push(f64::recip(f64::atan(arg))),
                    &enums::Function::Sinh => stack.push(f64::sinh(arg)),
                    &enums::Function::Csch => stack.push(f64::recip(f64::sinh(arg))),
                    &enums::Function::Cosh => stack.push(f64::cosh(arg)),
                    &enums::Function::Sech => stack.push(f64::recip(f64::cosh(arg))),
                    &enums::Function::Tanh => stack.push(f64::tanh(arg)),
                    &enums::Function::Coth => stack.push(f64::recip(f64::tanh(arg))),
                    &enums::Function::Asinh => stack.push(f64::asinh(arg)),
                    &enums::Function::Acsch => stack.push(f64::recip(f64::asinh(arg))),
                    &enums::Function::Acosh => stack.push(f64::acosh(arg)),
                    &enums::Function::Asech => stack.push(f64::recip(f64::acosh(arg))),
                    &enums::Function::Atanh => stack.push(f64::atanh(arg)),
                    &enums::Function::Acoth => stack.push(f64::recip(f64::atanh(arg))),
                    &enums::Function::Recip => stack.push(f64::recip(arg)),
                    &enums::Function::Max => {
                        let arg2 = arg;
                        let arg1 = stack.pop().unwrap();
                        stack.push(arg1.max(arg2));
                    }
                    &enums::Function::LogBase => {
                        let arg2 = arg;
                        let arg1 = stack.pop().unwrap();
                        stack.push(arg1.log(arg2)); // logbase(8,2) == 3
                    }
                }
            }
            &enums::Token::Unknown(ref x) => {
                let _ = x;
                let _ = stack.pop().unwrap();
            }
            &enums::Token::Var(ref x) => {
                let value = vars.get(x);
                if value.is_some() {
                    stack.push(*value.unwrap());
                } else {
                    stack.push(0.0);
                }
            }
            _ => continue,
        }
    }
    stack.pop().unwrap()
}
