use crate::token::{Operator, TokenListVisitor};

pub struct CalcVisitor {
    stack: Vec<i32>,
}

impl TokenListVisitor for CalcVisitor {
    type Error = &'static str;
    type Output = i32;

    fn new() -> Self {
        CalcVisitor { stack: vec![] }
    }

    fn visit_left(&mut self) -> Result<(), Self::Error> {
        Err("No parenthesis in RPN expected")
    }

    fn visit_right(&mut self) -> Result<(), Self::Error> {
        Err("No parenthesis in RPN expected")
    }

    fn visit_number(&mut self, x: u32) -> Result<(), Self::Error> {
        self.stack.push(x as i32);
        Ok(())
    }

    fn visit_op(&mut self, op: crate::token::Operator) -> Result<(), Self::Error> {
        let x2 = self.stack.pop().ok_or("expected argument")?;
        let x1 = self.stack.pop().ok_or("expected argument")?;
        let res = match op {
            Operator::Plus => Ok(x1 + x2),
            Operator::Minus => Ok(x1 - x2),
            Operator::Star => Ok(x1 * x2),
            Operator::Slash => {
                if x2 != 0 {
                    Ok(x1 / x2)
                } else {
                    Err("Division by zero")
                }
            }
        }?;
        self.stack.push(res);
        Ok(())
    }

    fn get_result(mut self) -> Result<Self::Output, Self::Error> {
        let res = self
            .stack
            .pop()
            .ok_or("Unexpected empty evaluation stack")?;
        if !self.stack.is_empty() {
            return Err("Unexpected elements left in evaluation stack");
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::CalcVisitor;
    use crate::visitors::parser_visitor::test::check_equal;

    #[test]
    fn test1() {
        check_equal::<CalcVisitor>("2 3 +", 5)
    }

    #[test]
    fn test2() {
        check_equal::<CalcVisitor>("3 4 - 5 2 / +", 1)
    }
}
