use my_lib::{
    token::TokenListVisitor,
    tokenizer::Tokenizer,
    visitors::{
        calc_visitor::CalcVisitor, parser_visitor::ParserVisitor, print_visitor::PrintVisitor,
    },
};
use text_io::read;

fn main() {
    let input: String = read!("{}\n");
    let res = (|| -> Result<(), String> {
        let tokens = Tokenizer::tokenize(input)?;
        let rpn = ParserVisitor::visit_tokens(tokens)?;
        PrintVisitor::visit_tokens(rpn.clone())?;
        let val = CalcVisitor::visit_tokens(rpn)?;
        println!("{}", val);
        Ok(())
    })();
    match res {
        Ok(()) => {}
        Err(message) => {
            println!("Error: {}", message);
            std::process::exit(1);
        }
    };
}
