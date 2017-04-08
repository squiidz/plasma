extern crate plasma;

#[allow(unused_imports)]
use plasma::token::{Token, TokenType};
#[allow(unused_imports)]
use plasma::ast::{Node, Statement, NodeType};
use plasma::lexer::Lexer;
use plasma::parser::Parser;
use plasma::evaluator::*;
use plasma::environment::Environment;

fn main() {
    let lex = Lexer::new("let a = 5;");
    let mut parser = Parser::new(lex);
    let prog = parser.parse_program();
    let mut env = Environment::new();
    eval(&NodeType::Program(prog), &mut env);
    //println!("{}", prog.to_string());
}
