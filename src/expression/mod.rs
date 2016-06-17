pub mod enums;

pub type Token = enums::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    tokens: Vec<Token>,
}

pub struct ExpressionIter<'a> {
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

impl Expression {
    pub fn new(tokens: Vec<Token>) -> Self {
         Expression { tokens: tokens }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn get_tokens(&self) -> &[Token] {
        self.tokens.as_slice()
    }

    pub fn get_mut_tokens(&mut self) -> &mut [Token] {
        self.tokens.as_mut_slice()
    }

    pub fn get_token(&self, index: usize) -> &Token {
        self.tokens.get(index).unwrap()
    }

    pub fn get_mut_token(&mut self, index: usize) -> &mut Token {
        self.tokens.get_mut(index).unwrap()
    }

    pub fn replace_token(&mut self, index: usize, new_token: Token) {
        self.tokens.remove(index);
        self.tokens.insert(index, new_token);
    }

    pub fn replace_all_tokens(&mut self, new_token: Vec<Token>) {
        self.tokens = new_token
    }

    pub fn find_first(&self, token: &Token) -> Option<usize> {
        for i in 0..self.tokens.len() {
            if self.tokens.get(i).unwrap() == token {
                return Some(i);
            }
        }
        None
    }

    pub fn find_last(&self, token: &Token) -> Option<usize> {
        for i in (0..self.tokens.len()).rev() {
            if self.tokens.get(i).unwrap() == token {
                return Some(i);
            }
        }
        None
    }

    pub fn split_at(&self, index: usize) -> (&[Token], &[Token]) {
        self.tokens.split_at(index)
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn iter(&self) -> ExpressionIter {
        ExpressionIter {
            expr: &self,
            count: 0,
        }
    }
}