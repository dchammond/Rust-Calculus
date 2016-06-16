#![allow(dead_code)]

extern crate regex;
use regex::Regex;

use std::io;
use std::io::Write;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Unknown(String), // Invalid test (basically non-ascii)
    Literal(String), // Numeric literal number
    Func(Function), // Pre-defined function (like cos() )
    Const(Constant), // Constant like pi or e
    Var(String), // str arbitrary single char variable name
    Op(Operator), // Any of the 4 operators (+-*/)
    Open, // Open parens '('
    Close, // Closing parens ')'
    Comma,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    Abs,
    Exp,
    Sqrt,
    Ln,
    Log,
    LogBase,
    Sin,
    Csc,
    Cos,
    Sec,
    Tan,
    Cot,
    Asin,
    Acsc,
    Acos,
    Asec,
    Atan,
    Acot,
    Sinh,
    Csch,
    Cosh,
    Sech,
    Tanh,
    Coth,
    Asinh,
    Acsch,
    Acosh,
    Asech,
    Atanh,
    Acoth,
    Max,
    Recip,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Constant {
    Pi,
    E,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Operator {
    Add = 0,
    Sub = 1,
    Div = 2,
    Mul = 3,
    Pow = 4, // Right
    Negate = 5, // Right
}

#[derive(Debug, PartialEq, Clone)]
struct Expression {
    tokens: Vec<Token>,
}

struct ExpressionIter<'a> {
    expr: &'a Expression,
    count: usize,
}

impl<'a> Iterator for ExpressionIter<'a> {
    type Item = &'a Token;
    fn next(&mut self) -> Option<&'a Token> {
        let token = self.expr.get_tokens().get(self.count);
        self.count += 1;
        token
    }
}

impl Expression {
    fn new(tokens: Vec<Token>) -> Self {
        Expression { tokens: tokens }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn get_tokens(&self) -> &[Token] {
        self.tokens.as_slice()
    }

    fn get_mut_tokens(&mut self) -> &mut [Token] {
        self.tokens.as_mut_slice()
    }

    fn get_token(&self, index: usize) -> &Token {
        self.tokens.get(index).unwrap()
    }

    fn get_mut_token(&mut self, index: usize) -> &mut Token {
        self.tokens.get_mut(index).unwrap()
    }

    fn replace_token(&mut self, index: usize, new_token: Token) {
        self.tokens.remove(index);
        self.tokens.insert(index, new_token);
    }

    fn replace_all_tokens(&mut self, new_token: Vec<Token>) {
        self.tokens = new_token
    }

    fn find_first(&self, token: &Token) -> Option<usize> {
        for i in 0..self.tokens.len() {
            if self.tokens.get(i).unwrap() == token {
                return Some(i);
            }
        }
        None
    }

    fn find_last(&self, token: &Token) -> Option<usize> {
        for i in (0..self.tokens.len()).rev() {
            if self.tokens.get(i).unwrap() == token {
                return Some(i);
            }
        }
        None
    }

    fn split_at(&self, index: usize) -> (&[Token], &[Token]) {
        self.tokens.split_at(index)
    }

    fn len(&self) -> usize {
        self.tokens.len()
    }

    fn iter(&self) -> ExpressionIter {
        ExpressionIter {
            expr: &self,
            count: 0,
        }
    }
}

fn strip_white_space(input: &String) -> String {
    input.split_whitespace().collect::<Vec<&str>>().join("")
}

fn parse_input(input: &String,
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
                        expr.push(Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(map_string_to_func(&builder));
                    } else {
                        expr.push(Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(Token::Open);
            }
            ')' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(map_string_to_func(&builder));
                    } else {
                        expr.push(Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(Token::Close);
            }
            '+' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(map_string_to_func(&builder));
                    } else {
                        expr.push(Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(Token::Op(Operator::Add));
            }
            '-' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(map_string_to_func(&builder));
                    } else {
                        expr.push(Token::Var(builder.clone()));
                    }
                    builder = String::new();
                    expr.push(Token::Op(Operator::Sub));
                } else if builder.len() == 0 {
                    expr.push(Token::Op(Operator::Negate));
                }
            }
            '*' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(map_string_to_func(&builder));
                    } else {
                        expr.push(Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(Token::Op(Operator::Mul));
            }
            '/' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(map_string_to_func(&builder));
                    } else {
                        expr.push(Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(Token::Op(Operator::Div));
            }
            '^' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(map_string_to_func(&builder));
                    } else {
                        expr.push(Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(Token::Op(Operator::Pow));
            }
            ',' => {
                if builder.len() > 0 {
                    if numeric_regex.is_match(&builder[..]) {
                        expr.push(Token::Literal(builder.clone()));
                    } else if function_regex.is_match(&builder[..]) {
                        expr.push(map_string_to_func(&builder));
                    } else {
                        expr.push(Token::Var(builder.clone()));
                    }
                    builder = String::new();
                }
                expr.push(Token::Comma);
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
            expr.push(Token::Literal(builder.clone()));
        } else if function_regex.is_match(&builder[..]) {
            expr.push(map_string_to_func(&builder));
        } else {
            expr.push(Token::Var(builder.clone()));
        }
    }
    let mut op_stack: Vec<Token> = Vec::with_capacity(input.len());
    let mut out_queue: Vec<Token> = Vec::with_capacity(input.len());
    for i in 0..expr.len() {
        let current_token = expr.get_token(i);
        match current_token {
            &Token::Literal(ref x) => out_queue.push(Token::Literal(x.clone())),
            &Token::Func(ref x) => op_stack.push(Token::Func(x.clone())),
            &Token::Comma => {
                loop {
                    let stack_token = op_stack.pop();
                    if !stack_token.is_some() {
                        return (variable,
                                Err("Malformed Expression, comma but no Parenthesis".to_owned()));
                    }
                    let stack_token = stack_token.unwrap();
                    match stack_token {
                        Token::Open => {
                            op_stack.push(stack_token);
                            break;
                        }
                        _ => out_queue.push(stack_token),
                    }
                }
            }
            &Token::Op(ref o1) => {
                loop {
                    if op_stack.len() < 1 {
                        break;
                    }
                    let o2 = op_stack.pop().unwrap(); // top of stack, must exist based off of previous if
                    match o1 {
                        &Operator::Negate => {
                            op_stack.push(o2);
                            break;
                        }
                        &Operator::Pow => {
                            match o2 {
                                Token::Op(Operator::Negate) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                        &Operator::Mul | &Operator::Div => {
                            match o2 {
                                Token::Op(Operator::Negate) |
                                Token::Op(Operator::Pow) |
                                Token::Op(Operator::Mul) |
                                Token::Op(Operator::Div) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                        &Operator::Add | &Operator::Sub => {
                            match o2 {
                                Token::Op(Operator::Negate) |
                                Token::Op(Operator::Pow) |
                                Token::Op(Operator::Mul) |
                                Token::Op(Operator::Div) |
                                Token::Op(Operator::Add) |
                                Token::Op(Operator::Sub) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                    }
                }
                op_stack.push(Token::Op(o1.clone()));
            }
            &Token::Open => op_stack.push(Token::Open),
            &Token::Close => {
                loop {
                    let stack_token = op_stack.pop();
                    if !stack_token.is_some() {
                        return (variable,
                                Err("Malformed Expression, found a ) without (".to_owned()));
                    }
                    let stack_token = stack_token.unwrap();
                    match stack_token {
                        Token::Open => break,
                        _ => out_queue.push(stack_token),
                    }
                }
                if op_stack.len() > 0 {
                    let next_stack_token = op_stack.pop().unwrap(); // must exist based off of previous if
                    match next_stack_token {
                        Token::Func(ref x) => out_queue.push(Token::Func(x.clone())),
                        _ => op_stack.push(next_stack_token),
                    }
                }
            }
            &Token::Var(ref x) => out_queue.push(Token::Var(x.clone())),
            &Token::Unknown(ref x) => {
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

fn map_string_to_func(input: &String) -> Token {
    match &(input.to_lowercase())[..] {
        "abs" => Token::Func(Function::Abs),
        "exp" => Token::Func(Function::Exp),
        "sqrt" => Token::Func(Function::Sqrt),
        "ln" => Token::Func(Function::Ln),
        "log" => Token::Func(Function::Log),
        "logbase" => Token::Func(Function::LogBase),
        "sin" => Token::Func(Function::Sin),
        "csc" => Token::Func(Function::Csc),
        "cos" => Token::Func(Function::Cos),
        "sec" => Token::Func(Function::Sec),
        "tan" => Token::Func(Function::Tan),
        "cot" => Token::Func(Function::Cot),
        "asin" => Token::Func(Function::Asin),
        "acsc" => Token::Func(Function::Acsc),
        "acos" => Token::Func(Function::Acos),
        "asec" => Token::Func(Function::Asec),
        "atan" => Token::Func(Function::Atan),
        "acot" => Token::Func(Function::Acot),
        "sinh" => Token::Func(Function::Sinh),
        "csch" => Token::Func(Function::Csch),
        "cosh" => Token::Func(Function::Cosh),
        "sech" => Token::Func(Function::Sech),
        "tanh" => Token::Func(Function::Tanh),
        "coth" => Token::Func(Function::Coth),
        "asinh" => Token::Func(Function::Asinh),
        "acsch" => Token::Func(Function::Acsch),
        "acosh" => Token::Func(Function::Acosh),
        "asech" => Token::Func(Function::Asech),
        "atanh" => Token::Func(Function::Atanh),
        "acoth" => Token::Func(Function::Acoth),
        "max" => Token::Func(Function::Max),
        "recip" => Token::Func(Function::Recip),
        _ => Token::Var(input.clone()),
    }
}

fn eval_postfix_expr(expr: &Expression, vars: &HashMap<String, f64>) -> f64 {
    let mut stack: Vec<f64> = Vec::with_capacity(expr.len() / 2);
    for token in expr.iter() {
        match token {
            &Token::Literal(ref x) => stack.push(x.parse::<f64>().unwrap()),
            &Token::Const(ref x) => {
                match x {
                    &Constant::Pi => stack.push(std::f64::consts::PI),
                    &Constant::E => stack.push(std::f64::consts::E),
                }
            }
            &Token::Op(ref x) => {
                let arg: f64 = stack.pop().unwrap();
                match x {
                    &Operator::Negate => stack.push(-1.0f64 * arg),
                    _ => {
                        let arg2 = arg;
                        let arg1 = stack.pop().unwrap();
                        match x {
                            &Operator::Add => stack.push(arg1 + arg2),
                            &Operator::Sub => stack.push(arg1 - arg2),
                            &Operator::Div => stack.push(arg1 / arg2),
                            &Operator::Mul => stack.push(arg1 * arg2),
                            &Operator::Pow => stack.push(arg1.powf(arg2)),
                            _ => continue, // Should never hit here
                        }
                    }
                }
            }
            &Token::Func(ref x) => {
                let arg: f64 = stack.pop().unwrap();
                match x {
                    &Function::Abs => stack.push(f64::abs(arg)),
                    &Function::Sqrt => stack.push(f64::sqrt(arg)),
                    &Function::Ln => stack.push(f64::ln(arg)),
                    &Function::Log => stack.push(f64::log10(arg)),
                    &Function::Exp => stack.push(f64::exp(arg)),
                    &Function::Sin => stack.push(f64::sin(arg)),
                    &Function::Csc => stack.push(f64::recip(f64::sin(arg))),
                    &Function::Cos => stack.push(f64::cos(arg)),
                    &Function::Sec => stack.push(f64::recip(f64::cos(arg))),
                    &Function::Tan => stack.push(f64::tan(arg)),
                    &Function::Cot => stack.push(f64::recip(f64::tan(arg))),
                    &Function::Asin => stack.push(f64::asin(arg)),
                    &Function::Acsc => stack.push(f64::recip(f64::asin(arg))),
                    &Function::Acos => stack.push(f64::acos(arg)),
                    &Function::Asec => stack.push(f64::recip(f64::acos(arg))),
                    &Function::Atan => stack.push(f64::atan(arg)),
                    &Function::Acot => stack.push(f64::recip(f64::atan(arg))),
                    &Function::Sinh => stack.push(f64::sinh(arg)),
                    &Function::Csch => stack.push(f64::recip(f64::sinh(arg))),
                    &Function::Cosh => stack.push(f64::cosh(arg)),
                    &Function::Sech => stack.push(f64::recip(f64::cosh(arg))),
                    &Function::Tanh => stack.push(f64::tanh(arg)),
                    &Function::Coth => stack.push(f64::recip(f64::tanh(arg))),
                    &Function::Asinh => stack.push(f64::asinh(arg)),
                    &Function::Acsch => stack.push(f64::recip(f64::asinh(arg))),
                    &Function::Acosh => stack.push(f64::acosh(arg)),
                    &Function::Asech => stack.push(f64::recip(f64::acosh(arg))),
                    &Function::Atanh => stack.push(f64::atanh(arg)),
                    &Function::Acoth => stack.push(f64::recip(f64::atanh(arg))),
                    &Function::Recip => stack.push(f64::recip(arg)),
                    &Function::Max => {
                        let arg2 = arg;
                        let arg1 = stack.pop().unwrap();
                        stack.push(arg1.max(arg2));
                    }
                    &Function::LogBase => {
                        let arg2 = arg;
                        let arg1 = stack.pop().unwrap();
                        stack.push(arg1.log(arg2)); // logbase(8,2) == 3
                    }
                }
            }
            &Token::Unknown(ref x) => {
                let _ = x;
                let _ = stack.pop().unwrap();
            }
            &Token::Var(ref x) => {
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

fn main() {
    let numeric_regex: Regex = Regex::new(r"\d+\.\d+|\d+").unwrap();
    let function_regex: Regex = Regex::new(r"\w{2,}").unwrap();
    println!("Welcome to Rust-Calculus!");
    println!("To evaluate an expression, simply type one in and hit RETURN.");
    println!("To set a variable, simply type VAR_NAME=EXPRESSION and hit RETURN.");
    println!("Valid commands are: sym_int, int, sym_def, and def.");
    println!("Type 'quit' to exit.");
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut var_expr: bool;
    loop {
        input.clear();
        print!(">>>> ");
        stdout.flush().ok();
        if let Err(x) = stdin.read_line(&mut input) {
            println!("There was a problem reading stdin: {:?}", x);
            print!("Exiting...");;
            break;
        }
        input = strip_white_space(&input).to_lowercase();
        if input == "quit" {
            println!("Exiting...");
            break;
        }
        let (var, expr) = parse_input(&input, &numeric_regex, &function_regex);
        if var.len() > 0 {
            var_expr = true;
            if !variables.contains_key(&var) {
                variables.insert(var.clone(), 0.0);
            }
        } else {
            var_expr = false;
        }
        if expr.is_ok() {
            let my_expression = expr.unwrap();
            let result = eval_postfix_expr(&my_expression, &variables);
            if var_expr {
                variables.insert(var.clone(), result);
                println!("{} = {}", &var, &result);
            } else {
                println!("{}", &result);
            }
        } else {
            println!("Encountered an error while parsing: {:?}",
                     expr.unwrap_err());
            println!("Try Again...(type 'quit' to exit)");
            continue;
        }
    }
}
