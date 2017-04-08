#[macro_use]
extern crate lazy_static;

pub mod token;
mod types;
mod object;
mod environment;
mod evaluator;
pub mod ast;
pub mod lexer;
pub mod parser;