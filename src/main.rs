use std::io;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
enum Token<'a> {
    Unknown(&'a str), // Invalid test (basically non-ascii)
    Literal(&'a str), // Numeric literal number
    Function(Function<'a>), // Pre-defined function (like cos() or a user defined f(x)=... )
    Constant(Constant), // Constant like pi or e
    Variable(&'a str), // str re3presenting an arbitary varible name (no space)
    Operator(Operator), // Any of the 4 operators (+-*/)
    Open(Vec<Token<'a>>), // Open parens '(' followed by vector of tokens and a Token::Close
    Close, // Closing parens ')'
}
#[derive(Debug, PartialEq, Clone)]
enum Function<'a> {
    Sqrt(Vec<Token<'a>>),
    Ln(Vec<Token<'a>>),
    Log(f64, Vec<Token<'a>>),
    Sin(Vec<Token<'a>>),
    Cos(Vec<Token<'a>>),
    Tan(Vec<Token<'a>>),
    Asin(Vec<Token<'a>>),
    Acos(Vec<Token<'a>>),
    Atan(Vec<Token<'a>>),
    Sinh(Vec<Token<'a>>),
    Cosh(Vec<Token<'a>>),
    Tanh(Vec<Token<'a>>),
    Asinh(Vec<Token<'a>>),
    Acosh(Vec<Token<'a>>),
    Atanh(Vec<Token<'a>>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Constant {
	Pi,
	E,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
enum Assoc {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
enum Order {
    AddSub = 1,
    MulDiv = 2,
    IMul = 3, // Implicit multiplication
    Pow = 4,
    Paren = 5,
}

fn main() {
    println!("Welcome to Rust-Calculus!");
    println!("To evaluate an expression, simply type one in and hit RETURN.");
    println!("To set a variable, simply type VAR_NAME=EXPRESSION and hit RETURN.");
    println!("Valid commands are: sym_int, int, sym_def, and def.");
    println!("Type 'quit' to exit.");
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
    	input.clear();
    	print!(">>>> ");
    	stdout.flush().ok();
        stdin.read_line(&mut input).unwrap();
        println!("You typed: {}", input.trim());
        input = strip_white_space(&input);
        match input.to_lowercase().as_ref() {
            "quit" => {print!("Exiting..."); break;},
            _ => {println!("You typed: {}", input.trim());},
        }
    }
}

fn strip_white_space(input: &String) -> String {
	input.split_whitespace().collect::<Vec<&str>>().join("")
}
