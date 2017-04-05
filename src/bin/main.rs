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
    for _ in 0..6 {
        parser.next_token();
        println!("{:?}", parser.cur_token);   
    }
}