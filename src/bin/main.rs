extern crate plasma;

#[allow(unused_imports)]
use plasma::token::{Token, TokenType};
#[allow(unused_imports)]
use plasma::ast::{Node, Statement};
use plasma::lexer::Lexer;
use plasma::parser::Parser;

fn main() {
    let lex = Lexer::new("var num = 45 + 5 / 6;");
    let mut parser = Parser::new(lex);
    let prog = parser.parse_program();
    println!("{:?}", prog.to_string());
}