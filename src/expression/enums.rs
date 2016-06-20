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

pub fn map_string_to_func(input: &String) -> Token {
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
    Mod,
    Mul,
    Pow, // Right
    Negate, // Right
}
