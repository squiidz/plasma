extern crate plasma;

use plasma::interpreter::Executor;

use std::io::{self, BufRead, Write};

fn main() {
    repl();
}

fn repl() {
    let inp = io::stdin();
    let mut inp = inp.lock();
    let mut exec = Executor::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut buff = String::new();
        match inp.read_line(&mut buff) {
            Ok(_) => {
                match exec.execute(&buff) {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("{}", e),
                }
            },
            Err(e) => println!("{}", e),
        };
        buff.clear();
    }
}
