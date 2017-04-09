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
    let lex = Lexer::new("
            var test = 5 + 5 - 2;
            var new = test + 5;
    ");
    let mut parser = Parser::new(lex);
    let prog = parser.parse_program();
    let mut env = Environment::new();

    println!("{}", prog.to_string());
    println!("{:?}", eval(&NodeType::Program(prog), &mut env));
}
