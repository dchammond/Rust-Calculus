pub mod enums;

extern crate regex;
use regex::Regex;

//pub type enums::Token = enums::enums::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    tokens: Vec<enums::Token>,
}

pub struct ExpressionIter<'a> {
    expr: &'a Expression,
    count: usize,
}

impl<'a> Iterator for ExpressionIter<'a> {
    type Item = &'a enums::Token;
    fn next(&mut self) -> Option<&'a enums::Token> {
        let token = self.expr.get_tokens().get(self.count);
        self.count += 1;
        token
    }
}

impl Expression {
    pub fn new(tokens: Vec<enums::Token>) -> Self {
         Expression { tokens: tokens }
    }

    pub fn push(&mut self, token: enums::Token) {
        self.tokens.push(token);
    }

    pub fn get_tokens(&self) -> &[enums::Token] {
        self.tokens.as_slice()
    }

    pub fn get_mut_tokens(&mut self) -> &mut [enums::Token] {
        self.tokens.as_mut_slice()
    }

    pub fn get_token(&self, index: usize) -> &enums::Token {
        self.tokens.get(index).unwrap()
    }

    pub fn get_mut_token(&mut self, index: usize) -> &mut enums::Token {
        self.tokens.get_mut(index).unwrap()
    }

    pub fn replace_token(&mut self, index: usize, new_token: enums::Token) {
        self.tokens.remove(index);
        self.tokens.insert(index, new_token);
    }

    pub fn replace_all_tokens(&mut self, new_token: Vec<enums::Token>) {
        self.tokens = new_token
    }

    pub fn find_first(&self, token: &enums::Token) -> Option<usize> {
        for i in 0..self.tokens.len() {
            if self.tokens.get(i).unwrap() == token {
                return Some(i);
            }
        }
        None
    }

    pub fn find_last(&self, token: &enums::Token) -> Option<usize> {
        for i in (0..self.tokens.len()).rev() {
            if self.tokens.get(i).unwrap() == token {
                return Some(i);
            }
        }
        None
    }

    pub fn split_at(&self, index: usize) -> (&[enums::Token], &[enums::Token]) {
        self.tokens.split_at(index)
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn iter(&self) -> ExpressionIter {
        ExpressionIter {
            expr: &self,
            count: 0,
        }
    }
}

pub fn parse_input(input: &String,
               numeric_regex: &Regex,
               function_regex: &Regex)
               -> (String, Result<Expression, String>) {
    // 1. Replace everthing except letters/numbers with their enums
    // 2. Then go through and replace things with Literals or functions
    let mut variable: String = String::new();
    let mut expr: Expression = Expression::new(Vec::new());
    let mut builder: String = String::new();
    for c in input.chars() {
        match c {
            '(' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(enums::Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(enums::map_string_to_func(&builder));
                    } else {
                        expr.push(enums::Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(enums::Token::Open);
            }
            ')' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(enums::Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(enums::map_string_to_func(&builder));
                    } else {
                        expr.push(enums::Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(enums::Token::Close);
            }
            '+' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(enums::Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(enums::map_string_to_func(&builder));
                    } else {
                        expr.push(enums::Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Add));
            }
            '-' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(enums::Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(enums::map_string_to_func(&builder));
                    } else {
                        expr.push(enums::Token::Var(builder.clone()));
                    }
                    builder = String::new();
                    expr.push(enums::Token::Op(enums::Operator::Sub));
                } else if builder.len() == 0 {
                    expr.push(enums::Token::Op(enums::Operator::Negate));
                }
            }
            '*' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(enums::Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(enums::map_string_to_func(&builder));
                    } else {
                        expr.push(enums::Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Mul));
            }
            '/' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(enums::Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(enums::map_string_to_func(&builder));
                    } else {
                        expr.push(enums::Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Div));
            }
            '^' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(enums::Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(enums::map_string_to_func(&builder));
                    } else {
                        expr.push(enums::Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Pow));
            }
            ',' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(enums::Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(enums::map_string_to_func(&builder));
                    } else {
                        expr.push(enums::Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(enums::Token::Comma);
            }
            '=' => {
                if builder.len() > 0 {
                    variable = builder.clone();
                    builder = String::new();
                }
            }
            _ => {
                builder.push(c);
            }
        }
    }
    if builder.len() > 0 {
        if numeric_regex.is_match(&builder[..]) {
            expr.push(enums::Token::Literal(builder.clone()));
        } else if function_regex.is_match(&builder[..]) {
            expr.push(enums::map_string_to_func(&builder));
        } else {
            expr.push(enums::Token::Var(builder.clone()));
        }
    }
    let mut op_stack: Vec<enums::Token> = Vec::with_capacity(input.len());
    let mut out_queue: Vec<enums::Token> = Vec::with_capacity(input.len());
    for i in 0..expr.len() {
        let current_token = expr.get_token(i);
        match current_token {
            &enums::Token::Literal(ref x) => out_queue.push(enums::Token::Literal(x.clone())),
            &enums::Token::Func(ref x) => op_stack.push(enums::Token::Func(x.clone())),
            &enums::Token::Comma => {
                loop {
                    let stack_token = op_stack.pop();
                    if !stack_token.is_some() {
                        return (variable,
                                Err("Malformed Expression, comma but no Parenthesis".to_owned()));
                    }
                    let stack_token = stack_token.unwrap();
                    match stack_token {
                        enums::Token::Open => {
                            op_stack.push(stack_token);
                            break;
                        }
                        _ => out_queue.push(stack_token),
                    }
                }
            }
            &enums::Token::Op(ref o1) => {
                loop {
                    if op_stack.len() < 1 {
                        break;
                    }
                    let o2 = op_stack.pop().unwrap(); // top of stack, must exist based off of previous if
                    match o1 {
                        &enums::Operator::Negate => {
                            op_stack.push(o2);
                            break;
                        }
                        &enums::Operator::Pow => {
                            match o2 {
                                enums::Token::Op(enums::Operator::Negate) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                        &enums::Operator::Mul | &enums::Operator::Div => {
                            match o2 {
                                enums::Token::Op(enums::Operator::Negate) |
                                enums::Token::Op(enums::Operator::Pow) |
                                enums::Token::Op(enums::Operator::Mul) |
                                enums::Token::Op(enums::Operator::Div) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                        &enums::Operator::Add | &enums::Operator::Sub => {
                            match o2 {
                                enums::Token::Op(enums::Operator::Negate) |
                                enums::Token::Op(enums::Operator::Pow) |
                                enums::Token::Op(enums::Operator::Mul) |
                                enums::Token::Op(enums::Operator::Div) |
                                enums::Token::Op(enums::Operator::Add) |
                                enums::Token::Op(enums::Operator::Sub) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                    }
                }
                op_stack.push(enums::Token::Op(o1.clone()));
            }
            &enums::Token::Open => op_stack.push(enums::Token::Open),
            &enums::Token::Close => {
                loop {
                    let stack_token = op_stack.pop();
                    if !stack_token.is_some() {
                        return (variable,
                                Err("Malformed Expression, found a ) without (".to_owned()));
                    }
                    let stack_token = stack_token.unwrap();
                    match stack_token {
                        enums::Token::Open => break,
                        _ => out_queue.push(stack_token),
                    }
                }
                if op_stack.len() > 0 {
                    let next_stack_token = op_stack.pop().unwrap(); // must exist based off of previous if
                    match next_stack_token {
                        enums::Token::Func(ref x) => out_queue.push(enums::Token::Func(x.clone())),
                        _ => op_stack.push(next_stack_token),
                    }
                }
            }
            &enums::Token::Var(ref x) => out_queue.push(enums::Token::Var(x.clone())),
            &enums::Token::Unknown(ref x) => {
                let mut message: String = "You either misspelled a function, or it is not yet \
                                           implemented. The unknown string was: ".to_owned();
                message.push_str(x);
                return (variable, Err(message));
            }
            _ => break,
        }
    }
    while op_stack.len() > 0 {
        out_queue.push(op_stack.pop().unwrap()); // The item must exist
    }
    (variable, Ok(Expression::new(out_queue)))
}

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