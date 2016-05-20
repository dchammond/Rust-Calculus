extern crate regex;
use regex::Regex;

use std::io;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Unknown, // Invalid test (basically non-ascii)
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
    Sqrt,
    Ln,
    Log,
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Constant {
	Pi,
	E,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Operator {
    Add=0,
    Sub=1,
    Div=2,
    Mul=3,
    Pow=4, // Right
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Assoc {
    Left=0,
    Right=1,
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
		ExpressionIter {expr: &self, count: 0}
	}
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
        let expr = match parse_input(&input, &numeric_regex, &function_regex) {
        		Ok(x) => x,
        		Err(x) => {println!("Encountered an error while parsing: {:?}", x); print!("Exiting..."); break;},
        	};
        println!("{:?}", &expr);
    }
}

fn strip_white_space(input: &String) -> String {
	input.split_whitespace().collect::<Vec<&str>>().join("")
}

fn parse_input(input: &String, numeric_regex: &Regex, function_regex: &Regex) -> Result<Expression, String> {
	// 1. Replace everthing except letters/numbers with their enums
	// 2. Then go through and replace things with Literals or functions
	let mut expr: Expression = Expression::new(Vec::new(), 0.0);
	let mut op_stack: Vec<Token> = Vec::with_capacity(input.len());
	let mut out_queue: Vec<Token> = Vec::with_capacity(input.len());
	let mut builder: String = String::new();
	for c in input.chars() {
		match c {
		    '(' => {
		    	if builder.len() > 0 {
		    		if numeric_regex.is_match(&builder[..]) {
		    			expr.push(Token::Literal(builder.clone()));
		    		} else if function_regex.is_match(&builder[..]) {
		    			expr.push(map_string_to_func(&builder));
		    		}
		    		builder = String::new();
		    	}
		    	expr.push(Token::Open);
		    },
		    ')' => {
		    	if builder.len() > 0 {
		    		if numeric_regex.is_match(&builder[..]) {
		    			expr.push(Token::Literal(builder.clone()));
		    		} else if function_regex.is_match(&builder[..]) {
		    			expr.push(map_string_to_func(&builder));
		    		}
		    		builder = String::new();
		    	}
		    	expr.push(Token::Close);
		    },
		    '+' => {
		    	if builder.len() > 0 {
		    		if numeric_regex.is_match(&builder[..]) {
		    			expr.push(Token::Literal(builder.clone()));
		    		} else if function_regex.is_match(&builder[..]) {
		    			expr.push(map_string_to_func(&builder));
		    		}
		    		builder = String::new();
		    	}
		    	expr.push(Token::Op(Operator::Add));
		    },
		    '-' => {
		    	if builder.len() > 0 {
		    		if numeric_regex.is_match(&builder[..]) {
		    			expr.push(Token::Literal(builder.clone()));
		    		} else if function_regex.is_match(&builder[..]) {
		    			expr.push(map_string_to_func(&builder));
		    		}
		    		builder = String::new();
		    	}
		    	expr.push(Token::Op(Operator::Sub));
		    },
		    '*' => {
		    	if builder.len() > 0 {
		    		if numeric_regex.is_match(&builder[..]) {
		    			expr.push(Token::Literal(builder.clone()));
		    		} else if function_regex.is_match(&builder[..]) {
		    			expr.push(map_string_to_func(&builder));
		    		}
		    		builder = String::new();
		    	}
		    	expr.push(Token::Op(Operator::Mul));
		    },
		    '/' => {
		    	if builder.len() > 0 {
		    		if numeric_regex.is_match(&builder[..]) {
		    			expr.push(Token::Literal(builder.clone()));
		    		} else if function_regex.is_match(&builder[..]) {
		    			expr.push(map_string_to_func(&builder));
		    		}
		    		builder = String::new();
		    	}
		    	expr.push(Token::Op(Operator::Div));
		    },
		    '^' => {
		    	if builder.len() > 0 {
		    		if numeric_regex.is_match(&builder[..]) {
		    			expr.push(Token::Literal(builder.clone()));
		    		} else if function_regex.is_match(&builder[..]) {
		    			expr.push(map_string_to_func(&builder));
		    		}
		    		builder = String::new();
		    	}
		    	expr.push(Token::Op(Operator::Pow));
		    },
		    ',' => {
		    	if builder.len() > 0 {
		    		if numeric_regex.is_match(&builder[..]) {
		    			expr.push(Token::Literal(builder.clone()));
		    		} else if function_regex.is_match(&builder[..]) {
		    			expr.push(map_string_to_func(&builder));
		    		}
		    		builder = String::new();
		    	}
		    	expr.push(Token::Comma);
		    },
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
		}
		builder = String::new();
	}
	println!("EXPR {:?}", &expr);
	for i in 0..expr.len() {
		match expr.get_token(i) {
			&Token::Literal(ref x) => out_queue.push(Token::Literal(x.clone())),
			&Token::Func(ref x) => op_stack.push(Token::Func(x.clone())),
			&Token::Comma => loop {
				let token = op_stack.pop().unwrap();
				match token {
					Token::Open => {op_stack.push(Token::Open);break},
					_ => out_queue.push(token),
				}
			},
			&Token::Open => op_stack.push(Token::Open),
			&Token::Close => loop {
				let token = op_stack.pop().unwrap();
				let mut flag = false;
				match token {
					Token::Open => {flag = true},
					_ => out_queue.push(token),
				}
				if flag {
					let token = op_stack.pop().unwrap();
					match token {
						Token::Func(ref x) => out_queue.push(Token::Func(x.clone())),
						_ => break,
					}
					break;
				}
			},
			&Token::Op(ref x) => loop {
				if op_stack.len() < 1 {
					op_stack.push(Token::Op(x.clone()));
					break;
				}
				let mut flag = false;
				let token = op_stack.get(op_stack.len()-1).unwrap().clone();
				let order: usize = match token {
				    Token::Op(Operator::Pow) => 4,
				    Token::Op(Operator::Mul) => 3,
				    Token::Op(Operator::Div) => 2,
				    Token::Op(Operator::Sub) => 1,
				    Token::Op(Operator::Add) => 0,
				    _ => {flag = true;0},
				};
				if flag {
					op_stack.push(Token::Op(x.clone()));
					break;
				}
				match x {
				    &Operator::Pow => if 4 < order {
				    	out_queue.push(op_stack.pop().unwrap());
				    },
				    &Operator::Mul | &Operator::Div => if 2 <= order {
				    	out_queue.push(op_stack.pop().unwrap());
				    },
				    &Operator::Add | &Operator::Sub => if 1 <= order {
				    	out_queue.push(op_stack.pop().unwrap());
				    },
				};
			},
			_ => continue,
		}
	}
	while op_stack.len() > 0 {
	    out_queue.push(op_stack.pop().unwrap());
	}
	Ok(Expression::new(out_queue, 0.0))
}

fn map_string_to_func(input: &String) -> Token {
	match &(input.to_lowercase())[..] {
		"abs" => Token::Func(Function::Abs),
	    "sqrt" => Token::Func(Function::Sqrt),
	    "ln" => Token::Func(Function::Ln),
	    "log" => Token::Func(Function::Log),
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
	    _ => Token::Unknown,
	}
}

#[test]
fn test_parse() {
	let numeric_regex: Regex = Regex::new(r"\d+\.\d+|\d+").unwrap();
	let function_regex: Regex = Regex::new(r"\w{2,}").unwrap();
	let input: String = String::from("22+(3.14-3)");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Add)], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("+");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Add)], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("-");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Sub)], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("/");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Div)], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("*");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Mul)], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("^");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Pow)], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("()");
	assert_eq!(Expression::new(vec![Token::Open, Token::Close], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("(+)");
	assert_eq!(Expression::new(vec![Token::Open, Token::Op(Operator::Add), Token::Close], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("(-)");
	assert_eq!(Expression::new(vec![Token::Open, Token::Op(Operator::Sub), Token::Close], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("(*)");
	assert_eq!(Expression::new(vec![Token::Open, Token::Op(Operator::Mul), Token::Close], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("(/)");
	assert_eq!(Expression::new(vec![Token::Open, Token::Op(Operator::Div), Token::Close], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("(^)");
	assert_eq!(Expression::new(vec![Token::Open, Token::Op(Operator::Pow), Token::Close], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("(())");
	assert_eq!(Expression::new(vec![Token::Open, Token::Open, Token::Close, Token::Close], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
	let input: String = String::from("+(+(-)*(+))+(/)^");
	assert_eq!(Expression::new(vec![Token::Op(Operator::Add), Token::Open, Token::Op(Operator::Add), Token::Open, Token::Op(Operator::Sub),
									Token::Close, Token::Op(Operator::Mul), Token::Open, Token::Op(Operator::Add), Token::Close, Token::Close,
									Token::Op(Operator::Add), Token::Open, Token::Op(Operator::Div), Token::Close, Token::Op(Operator::Pow)], 0.0), parse_input(&input, &numeric_regex, &function_regex).unwrap());
}