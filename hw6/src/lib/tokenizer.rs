use crate::token::{self, Token};

type RVoid = Result<(), String>;

enum TokenizerState {
    WhiteSpace,
    Operator,
    Number(String),
    Paren,
    Start,
}

pub struct Tokenizer {
    tokens: Vec<Token>,
    state: TokenizerState,
}

impl Tokenizer {
    /// If the tokenizer's state is number then
    /// try to parse it and push it into tokens list
    fn end_number_if_necessary(&mut self) -> RVoid {
        match &self.state {
            TokenizerState::Number(x) => {
                let num = x.parse::<u32>().map_err(|e| e.to_string())?;
                self.tokens.push(Token::Number(num));
            }
            _ => {}
        }
        Ok(())
    }

    fn whitespace(&mut self) -> RVoid {
        self.end_number_if_necessary()?;
        self.state = TokenizerState::WhiteSpace;
        Ok(())
    }
    fn operation(&mut self, op: token::Operator) -> RVoid {
        self.end_number_if_necessary()?;
        self.state = TokenizerState::Operator;
        self.tokens.push(Token::Operator(op));
        Ok(())
    }
    fn digit(&mut self, digit: char) -> RVoid {
        match &mut self.state {
            TokenizerState::Number(s) => {
                s.push(digit);
            }
            _ => {
                self.state = TokenizerState::Number(digit.to_string());
            }
        }
        Ok(())
    }
    fn paren(&mut self, paren: token::Paren) -> RVoid {
        self.end_number_if_necessary()?;
        self.state = TokenizerState::Paren;
        self.tokens.push(Token::Paren(paren));
        Ok(())
    }
    fn get_tokens(mut self) -> Result<Vec<Token>, String> {
        self.end_number_if_necessary()?;
        return Ok(self.tokens);
    }

    pub fn tokenize(input: String) -> Result<Vec<Token>, String> {
        let mut tokenizer = Tokenizer {
            tokens: vec![],
            state: TokenizerState::Start,
        };
        for char in input.chars() {
            if char.is_digit(10) {
                tokenizer.digit(char)?
            } else if char.is_whitespace() {
                tokenizer.whitespace()?
            } else {
                match char {
                    '(' => tokenizer.paren(token::Paren::Left)?,
                    ')' => tokenizer.paren(token::Paren::Right)?,
                    '+' => tokenizer.operation(token::Operator::Plus)?,
                    '-' => tokenizer.operation(token::Operator::Minus)?,
                    '*' => tokenizer.operation(token::Operator::Star)?,
                    '/' => tokenizer.operation(token::Operator::Slash)?,
                    _ => return Err(format!("Unexpected token '{}'", char)),
                }
            }
        }
        tokenizer.get_tokens()
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Operator, Paren, Token};

    use super::Tokenizer;

    fn check_equal(str: &str, expected: Vec<Token>) {
        assert_eq!(expected, Tokenizer::tokenize(str.to_string()).unwrap());
    }

    #[test]
    fn test1() {
        check_equal("", vec![]);
    }

    #[test]
    fn test2() {
        check_equal(
            "()",
            vec![Token::Paren(Paren::Left), Token::Paren(Paren::Right)],
        );
    }

    #[test]
    fn test3() {
        check_equal(
            "    1  2 3 + 4 // *-",
            vec![
                Token::Number(1),
                Token::Number(2),
                Token::Number(3),
                Token::Operator(Operator::Plus),
                Token::Number(4),
                Token::Operator(Operator::Slash),
                Token::Operator(Operator::Slash),
                Token::Operator(Operator::Star),
                Token::Operator(Operator::Minus),
            ],
        )
    }
}
