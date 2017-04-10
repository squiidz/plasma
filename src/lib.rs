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

    #[derive(Default)]
    pub struct Executor {
        variables: Environment,
    }

    impl Executor {
        pub fn new() -> Executor {
            Executor{ variables: Environment::default() }
        }

        pub fn execute(&mut self, code: &str) -> Result<String, String> {
            let lex = Lexer::new(code);
            let mut parser = Parser::new(lex);
            let prog = parser.parse_program();

            match eval(&NodeType::Program(prog), &mut self.variables) {
                Some(res) => Ok(res.inspect()),
                None => Err("empty result".to_owned()),
            }
        }

    }
}
