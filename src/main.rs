use std::io;
use std::io::Write;

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
    Set(Vec<Token>), // [ (, tokens..., ) ]
}

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
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
pub enum Constant {
	Pi,
	E,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Order {
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

#[allow(dead_code)]
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
        let expr = parse_input(&input).unwrap();
        println!("{:?}", &expr);
    }
}

fn strip_white_space(input: &String) -> String {
	input.split_whitespace().collect::<Vec<&str>>().join("")
}

fn parse_input(input: &String) -> Result<Expression, ()> {
	let mut expr: Expression = Expression::new(Vec::new(), 0.0);
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
	// Once we find an outermost (), split the rest of the expression after that off into a new vector
	//let mut expr_vecs: Vec<Vec<Token>> = Vec::new();
	while let Some(index) = expr.find_first(&Token::Open) { // If we find an index of an Open
		let end_set: usize = expr.find_last(&Token::Close).unwrap(); // Finds the last Close, need to find the one that actually matches the proper Open (if two Sets are on same depth)
		let mut lhs: Vec<Token> = Vec::new();
		{
			let (temp_lhs, temp_rhs) = expr.split_at(index); // [0, where set should be placed)
			lhs = temp_lhs.to_vec();
			let mut rhs = temp_rhs.to_vec(); // [where set should be placed, len)
			rhs.remove(0);
			let (left_rhs, right_rhs) = rhs.split_at(end_set-1);
			let (left_rhs, mut right_rhs) = (left_rhs.to_vec(), right_rhs.to_vec());
			right_rhs.remove(0);
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

#[test]
fn test_parse() {
	let input: String = String::from("+");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Add)], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("-");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Sub)], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("/");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Div)], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("*");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Mul)], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("^");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Pow)], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("()");
	assert_eq!(Expression::new(vec![Token::Set(Vec::new())], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("(+)");
	assert_eq!(Expression::new(vec![Token::Set(vec![Token::Op(Operator::Add)])], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("(-)");
	assert_eq!(Expression::new(vec![Token::Set(vec![Token::Op(Operator::Sub)])], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("(*)");
	assert_eq!(Expression::new(vec![Token::Set(vec![Token::Op(Operator::Mul)])], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("(/)");
	assert_eq!(Expression::new(vec![Token::Set(vec![Token::Op(Operator::Div)])], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("(^)");
	assert_eq!(Expression::new(vec![Token::Set(vec![Token::Op(Operator::Pow)])], 0.0), parse_input(&input).unwrap());
	let input: String = String::from("(())");
	assert_eq!(Expression::new(vec![Token::Set(vec![Token::Set(Vec::new())])], 0.0), parse_input(&input).unwrap());
}