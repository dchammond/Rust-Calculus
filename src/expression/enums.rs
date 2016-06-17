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