#[macro_use]
extern crate lazy_static;

mod token;
mod types;
mod object;
mod environment;
mod evaluator;
mod ast;
mod lexer;
mod parser;

pub mod interpreter {
    use lexer::Lexer;
    use parser::Parser;
    use environment::Environment;
    use evaluator::eval;
    use object::Objecter;
    use ast::NodeType;

    pub fn execute(code: &str) -> Result<String, String> {
        let lex = Lexer::new(code);
        let mut parser = Parser::new(lex);
        let prog = parser.parse_program();
        let mut env = Environment::new();

        match eval(&NodeType::Program(prog), &mut env) {
            Some(res) => Ok(res.inspect()),
            None => Err("Invalid code".to_owned()),
        }
    }
}
