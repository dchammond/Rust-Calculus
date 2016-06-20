#![allow(dead_code)]

extern crate regex;
use regex::Regex;

use std::io;
use std::io::Write;

use std::collections::HashMap;

mod expression;

type Expression = expression::Expression;

use expression::eval::eval_postfix_expr;
use expression::parse::parse_input;

fn strip_white_space(input: &String) -> String {
    input.split_whitespace().collect::<Vec<&str>>().join("")
}

fn main() {
    let numeric_regex: Regex = Regex::new(r"\d+\.\d+|\d+").unwrap();
    let function_regex: Regex = Regex::new(r"[a-zA-Z]{2,}").unwrap();
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
