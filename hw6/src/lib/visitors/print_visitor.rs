use crate::token::{Operator, TokenListVisitor};

pub struct PrintVisitor;

impl TokenListVisitor for PrintVisitor {
    type Error = &'static str;
    type Output = ();

    fn new() -> Self {
        PrintVisitor
    }

    fn visit_left(&mut self) -> Result<(), Self::Error> {
        Err("There shouldn't be parenthesis in RPN")
    }

    fn visit_right(&mut self) -> Result<(), Self::Error> {
        Err("There shouldn't be parenthesis in RPN")
    }

    fn visit_number(&mut self, x: u32) -> Result<(), Self::Error> {
        print!("{} ", x);
        Ok(())
    }

    fn visit_op(&mut self, op: crate::token::Operator) -> Result<(), Self::Error> {
        let s = match op {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Slash => "/",
        };
        print!("{} ", s);
        Ok(())
    }

    fn get_result(self) -> Result<(), Self::Error> {
        println!();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        token::TokenListVisitor, tokenizer::Tokenizer, visitors::parser_visitor::ParserVisitor,
    };

    use super::PrintVisitor;

    #[test]
    fn test() {
        let tokens = Tokenizer::tokenize("1 + 2 * 3 - 8".to_string()).unwrap();
        let tokens = ParserVisitor::visit_tokens(tokens).unwrap();
        PrintVisitor::visit_tokens(tokens).unwrap();
    }
}
