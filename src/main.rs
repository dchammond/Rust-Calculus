use std::io;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
enum Token<'a> {
    Unknown(&'a str), // Invalid test (basically non-ascii)
    Literal(&'a str), // Numeric literal number
    Function(Function<'a>), // Pre-defined function (like cos() )
    Constant(Constant), // Constant like pi or e
    Variable(&'a str), // str arbitrary single char variable name
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

#[derive(Debug, PartialEq, Clone)]
struct Expression<'a> {
    tokens: Vec<Token<'a>>,
    constant: f64,
}

impl<'a> Expression<'a> {
	fn new(tokens: Vec<Token<'a>>, constant: f64) -> Self {
		Expression {tokens: tokens, constant: constant}
	}

	fn push(&mut self, token: Token<'a>) {
		self.tokens.push(token);
	}

	fn update_constant(&mut self, constant: &f64) {
		self.constant += *constant;
	}
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
        input = strip_white_space(&input);
        input = input.to_lowercase();
        if input == "quit" {
        	print!("Exiting...");
        	break;
        }
        let mut expr = parse_input(&input).unwrap();
    }
}

fn strip_white_space(input: &String) -> String {
	input.split_whitespace().collect::<Vec<&str>>().join("")
}

fn parse_input(input: &String) -> Result<Expression, ()> {
	unimplemented!()
}