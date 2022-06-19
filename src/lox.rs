use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;

use self::scanner::Scanner;
use self::token::Token;

mod error;
mod scanner;
mod token;

pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn run_file(&mut self, path: String) -> &mut Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        println!("{}", contents);

        self
    }

    pub fn run(&mut self, source: String) -> &mut Self {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        match tokens {
            Ok(tokens) => {
                for token in tokens {
                    println!("{:?}", token);
                }
            }
            Err(error) => println!("Error: {:?}", error),
        }

        self
    }
}
