use std::io;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Unknown(String), // Invalid test (basically non-ascii)
    Literal(String), // Numeric literal number
    Func(Function), // Pre-defined function (like cos() )
    Const(Constant), // Constant like pi or e
    Var(String), // str arbitrary single char variable name
    Op(Operator), // Any of the 4 operators (+-*/)
    Open, // Open parens '(' 
    Close, // Closing parens ')'
    Set(Vec<Token>), // [ (, tokens..., ) ]
    Fake,
}

#[derive(Debug, PartialEq, Clone)]
enum Function {
    Sqrt(Vec<Token>),
    Ln(Vec<Token>),
    Log(f64, Vec<Token>),
    Sin(Vec<Token>),
    Cos(Vec<Token>),
    Tan(Vec<Token>),
    Asin(Vec<Token>),
    Acos(Vec<Token>),
    Atan(Vec<Token>),
    Sinh(Vec<Token>),
    Cosh(Vec<Token>),
    Tanh(Vec<Token>),
    Asinh(Vec<Token>),
    Acosh(Vec<Token>),
    Atanh(Vec<Token>),
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
    Pow,
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
struct Expression {
    tokens: Vec<Token>,
    constant: f64,
}

impl Expression {
	fn new(tokens: Vec<Token>, constant: f64) -> Self {
		Expression {tokens: tokens, constant: constant}
	}

	fn push(&mut self, token: Token) {
		self.tokens.push(token);
	}

	fn update_constant(&mut self, constant: &f64) {
		self.constant += *constant;
	}

	fn get_constant(&self) -> f64 {
		self.constant
	}

	fn get_tokens(&self) -> &[Token] {
		self.tokens.as_slice()
	}

	fn get_mut_tokens(&mut self) -> &mut [Token] {
		self.tokens.as_mut_slice()
	}

	fn replace_token(&mut self, index: usize, newToken: Token) {
		self.tokens.remove(index);
		self.tokens.insert(index, newToken);
	}

	fn replace_all_tokens(&mut self, newTokens: Vec<Token>) {
		self.tokens = newTokens
	}

	fn find_first(&self, token: &Token) -> Option<usize> {
		let mut index: usize = 0;
		for t in &self.tokens {
			if t == token {
				return Some(index);
			}
			index += 1;
		}
		None
	}

	fn find_last(&self, token: &Token) -> Option<usize> {
		let mut index: usize = 0;
		let mut found = false;
		for i in (0..self.tokens.len()).rev() {
			if self.tokens.get(i).unwrap() == token {
				index = i;
				return Some(index);
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
        println!("{:?}", &expr);
    }
}

fn strip_white_space(input: &String) -> String {
	input.split_whitespace().collect::<Vec<&str>>().join("")
}

fn parse_input(input: &String) -> Result<Expression, ()> {
	let mut expr: Expression = Expression::new(Vec::new(), 0.0);
	let mut builder: Vec<Token> = Vec::new();
	for c in input.chars() {
		match c {
			'-' => {
				expr.push(Token::Op(Operator::Sub));
			},
			'+' => {
				expr.push(Token::Op(Operator::Add));
			},
			'/' => {
				expr.push(Token::Op(Operator::Div));
			},
			'*' => {
				expr.push(Token::Op(Operator::Mul));
			},
			'^' => {
				expr.push(Token::Op(Operator::Pow));
			},
			'(' => {
				expr.push(Token::Open);
			},
			')' => {
				expr.push(Token::Close);
			},
			_ => continue,
		}
	}
	if let Some(index) = expr.find_first(&Token::Open) { // If we find an index of an Open
		let endSet: usize = expr.find_last(&Token::Close).unwrap();
		let mut lhs: Vec<Token> = Vec::new();
		let mut rhs: Vec<Token> = Vec::new();
		{
			let (temp_lhs, temp_rhs) = expr.split_at(index); // [0, where set should be placed)
			lhs = temp_lhs.to_vec();
			rhs = temp_rhs.to_vec(); // [where set should be placed, len)
			rhs.remove(0);
			// split rhs at endset - 2
			// Left part gets pushed as a Set to lhs
			// Remove index 0 of right part (this is the close we split on) and add that as a vector to lhs
			let (mut left_rhs, mut right_rhs) = rhs.split_at(endSet-2);
			let (mut left_rhs, mut right_rhs) = (left_rhs.to_vec(), right_rhs.to_vec());
			right_rhs.remove(0);
			//rhs.remove(endSet-2); // Sub one because we removed the first element, need to split at endSet -2 
			lhs.push(Token::Set(left_rhs));
			lhs.append(&mut right_rhs);
		}
		for i in (0..lhs.len()).rev() {
			if lhs.get(i).unwrap() == &Token::Close {
				lhs.remove(i);
				break;
			}
		}
		expr.replace_all_tokens(lhs);
	}
	Ok(expr)
}