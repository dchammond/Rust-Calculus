use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to Rust-Calculus!");
    println!("To evaluate an expression, simply type one in and hit RETURN.");
    println!("To set a variable, simply type VAR_NAME=EXPRESSION and hit RETURN.");
    println!("Valid commands are: sym_int, int, sym_def, and def.");
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
    	print!(">>>> ");
    	stdout.flush().ok();
        stdin.read_line(&mut input).unwrap();
        println!("You typed: {}", input.trim());
        input.clear();
    }
}
