#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Paren(Paren),
    Number(u32),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

impl Operator {
    pub fn left_associative(&self) -> bool {
        true
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Plus => 1,
            Operator::Minus => 1,
            Operator::Star => 2,
            Operator::Slash => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Paren {
    Left,
    Right,
}

pub trait TokenListVisitor: Sized {
    type Output;
    type Error: Sized;

    fn new() -> Self;
    fn visit_left(&mut self) -> Result<(), Self::Error>;
    fn visit_right(&mut self) -> Result<(), Self::Error>;
    fn visit_number(&mut self, x: u32) -> Result<(), Self::Error>;
    fn visit_op(&mut self, op: Operator) -> Result<(), Self::Error>;
    fn visit(&mut self, token: Token) -> Result<(), Self::Error> {
        match token {
            Token::Paren(p) => match p {
                Paren::Left => self.visit_left(),
                Paren::Right => self.visit_right(),
            },
            Token::Number(x) => self.visit_number(x),
            Token::Operator(op) => self.visit_op(op),
        }
    }
    fn get_result(self) -> Result<Self::Output, Self::Error>;
    fn visit_tokens(tokens: impl IntoIterator<Item = Token>) -> Result<Self::Output, Self::Error> {
        let mut visitor = Self::new();
        for token in tokens {
            visitor.visit(token)?;
        }
        visitor.get_result()
    }
}
