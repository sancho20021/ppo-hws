use crate::token::{Operator, Paren, Token, TokenListVisitor};

pub struct ParserVisitor {
    output: Vec<Token>,
    stack: Vec<Token>,
}

type RVoid = Result<(), String>;
impl TokenListVisitor for ParserVisitor {
    type Output = Vec<Token>;
    type Error = String;

    fn new() -> ParserVisitor {
        ParserVisitor {
            output: vec![],
            stack: vec![],
        }
    }

    fn visit_left(&mut self) -> RVoid {
        self.stack.push(Token::Paren(Paren::Left));
        Ok(())
    }

    fn visit_right(&mut self) -> RVoid {
        while let Some(op) = self.stack.last() {
            if *op == Token::Paren(Paren::Right) {
                self.stack.pop();
                return Ok(());
            }
            self.output.push(self.stack.pop().unwrap());
        }
        return Err("Unexpected ')'".to_string());
    }

    fn visit_number(&mut self, x: u32) -> RVoid {
        self.output.push(Token::Number(x));
        Ok(())
    }

    fn visit_op(&mut self, op: Operator) -> RVoid {
        while let Some(token) = self.stack.last() {
            match token {
                Token::Operator(op_on_stack) => {
                    if op.left_associative() && op.precedence() <= op_on_stack.precedence()
                        || !op.left_associative() && op.precedence() < op_on_stack.precedence()
                    {
                        self.output.push(self.stack.pop().unwrap());
                        continue;
                    }
                    break;
                }
                _ => break,
            }
        }
        self.stack.push(Token::Operator(op));
        Ok(())
    }

    fn get_result(mut self) -> Result<Vec<Token>, String> {
        while let Some(token) = self.stack.pop() {
            self.output.push(token);
        }
        Ok(self.output)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::fmt::Debug;

    use crate::{
        token::{Operator, Token, TokenListVisitor},
        tokenizer::Tokenizer,
    };

    use super::ParserVisitor;

    pub fn check_equal<Visitor>(input: &str, output_expected: Visitor::Output)
    where
        Visitor: TokenListVisitor,
        Visitor::Output: Eq + Debug,
        Visitor::Error: Debug,
    {
        let tokens = Tokenizer::tokenize(input.to_string()).unwrap();
        println!("{:?}", tokens);
        let out = Visitor::visit_tokens(tokens).unwrap();
        assert_eq!(output_expected, out);
    }

    pub fn expect_fails<Visitor>(input: &str)
    where
        Visitor: TokenListVisitor,
        Visitor::Output: Debug,
    {
        let tokens = Tokenizer::tokenize(input.to_string()).unwrap();
        Visitor::visit_tokens(tokens).expect_err("expected error, but result found");
    }

    #[test]
    fn test1() {
        check_equal::<ParserVisitor>("", vec![]);
    }

    #[test]
    fn test2() {
        expect_fails::<ParserVisitor>(")")
    }

    #[test]
    fn test3() {
        expect_fails::<ParserVisitor>("3(+-4)")
    }

    #[test]
    fn test4() {
        check_equal::<ParserVisitor>(
            "2 + 3",
            vec![
                Token::Number(2),
                Token::Number(3),
                Token::Operator(Operator::Plus),
            ],
        )
    }

    #[test]
    fn test5() {
        check_equal::<ParserVisitor>(
            "3 - 4 + 5 / 2",
            vec![
                Token::Number(3),
                Token::Number(4),
                Token::Operator(Operator::Minus),
                Token::Number(5),
                Token::Number(2),
                Token::Operator(Operator::Slash),
                Token::Operator(Operator::Plus),
            ],
        )
    }
}
