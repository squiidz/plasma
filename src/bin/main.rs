extern crate plasma;
extern crate clap;

use plasma::interpreter::Executor;
use clap::{App, Arg};

use std::io::{self, BufRead, Read, Write};
use std::fs::File;

const ERROR: &'static str = "[Error]";

fn main() {
    let matches = App::new("brain")
        .arg(Arg::with_name("file")
            .takes_value(true)
            .index(1)
        ).get_matches();

    if matches.is_present("file") {
        let file_name = match matches.value_of("file") {
            Some(v) => v,
            None => {
                println!("{} Source file needed.", ERROR);
                return
            },
        };

        let mut file = match File::open(file_name) {
            Ok(f) => f,
            Err(_) => {
                println!("{} Source file not found.", ERROR);
                return
            },
        };

        let mut code = String::new();
        match file.read_to_string(&mut code) {
            Ok(_) => { },
            Err(_) => {
                println!("{} Source invalid.", ERROR);
                return
            },
        }
        exec_file(&code);
    } else {
        repl();
    }
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

fn exec_file(code: &str) {
    let mut exec = Executor::new();

    match exec.execute(code) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}