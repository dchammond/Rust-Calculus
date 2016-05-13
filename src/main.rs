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
    Open(u8), // Open parens '(' 
    Close(u8), // Closing parens ')'
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
			//if self.tokens.get(i).unwrap() == Token::Set {
				println!("HERE {:?}", self.tokens.get(i).unwrap().0);
			//}
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
		ExpressionIter {expr: &self, count: 0}
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
        if let Err(x) = stdin.read_line(&mut input) {
        	println!("There was a problem reading stdin: {:?}", x);
        	print!("Exiting...");;
        	break;
        }
        input = strip_white_space(&input).to_lowercase();
        if input == "quit" {
        	print!("Exiting...");
        	break;
        }
        let expr = match parse_input(&input) {
        		Ok(x) => x,
        		Err(x) => {println!("Encountered an error while parsing: {:?}", x); print!("Exiting..."); break;},
        	};
        println!("{:?}", &expr);
    }
}

fn strip_white_space(input: &String) -> String {
	input.split_whitespace().collect::<Vec<&str>>().join("")
}

fn parse_input(input: &String) -> Result<Expression, String> {
	let mut expr: Expression = Expression::new(Vec::new(), 0.0);
	let mut depth: u8 = 0;
	let mut lastParens = ' ';
	let mut maxDepth: u8 = 0;
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
				if lastParens == '(' {
					depth += 1;
					if maxDepth < depth {maxDepth = depth}
				}
				expr.push(Token::Open(depth));
				lastParens = '(';
			},
			')' => {
				if lastParens == ')' {
					depth -= 1;
				}
				expr.push(Token::Close(depth));
				lastParens = ')';

			},
			_ => continue,
		}
	}
	assert!(depth == 0); // Depth should always end at 0
	println!("ORIG EXPR {:?}", &expr);
	loop {
		let mut index: Option<usize> = None;
		depth = 0;
		while depth <= maxDepth {
			if let Some(x) = expr.find_first(&Token::Open(depth)) {
				index = Some(x);
				break;
			}
			depth += 1;
		}
		if index == None {break;}
		let index = index.unwrap(); // If we reach here, we know index is not None
		let mut end_set: usize = expr.find_first(&Token::Close(depth)).unwrap();
		let mut lhs: Vec<Token> = Vec::new();
		{
			let (temp_lhs, temp_rhs) = expr.split_at(index); // [0, where set should be placed)
			lhs = temp_lhs.to_vec();
			let mut rhs = temp_rhs.to_vec(); // [where set should be placed, len)
			rhs.remove(0);
			end_set = end_set - (index + 1); // Compensate for part of vec split off and the removal of one element
			let (left_rhs, right_rhs) = rhs.split_at(end_set);
			let (left_rhs, mut right_rhs) = (left_rhs.to_vec(), right_rhs.to_vec());
			right_rhs.remove(0);
			lhs.push(Token::Set(left_rhs));
			lhs.append(&mut right_rhs);
		}

		expr.replace_all_tokens(lhs);
		println!("NEW EXPR {:?}", &expr);
		depth += 1;
		if depth > maxDepth {break;}
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