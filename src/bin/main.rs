extern crate plasma;

#[allow(unused_imports)]
use plasma::token::{Token, TokenType};
#[allow(unused_imports)]
use plasma::ast::{Node, Statement};
use plasma::lexer::Lexer;
use plasma::parser::Parser;

fn main() {
    let lex = Lexer::new("let n = 5;");
    let mut parser = Parser::new(lex);
    for _ in 0..5 {
        println!("{:?}", parser.cur_token);
        parser.next_token();
    }
}