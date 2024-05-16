mod error;
mod parser;
mod scanner;
mod syntax;
mod token;
mod interpreter;

use interpreter::Interpreter;
use error::Error;
use lazy_static::lazy_static;
use parser::Parser;
use scanner::Scanner;
use std::collections::HashMap;
use std::io;
use std::process::exit;
use std::{env, fs};
use syntax::AstPrinter;
use token::TokenType;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let keywords: HashMap<&'static str, TokenType> = HashMap::from([
            ("ng", TokenType::And),
            ("tnak", TokenType::Class),
            ("minjengte", TokenType::Else),
            ("ort", TokenType::False),
            ("mukngea", TokenType::Fun),
            ("somhab", TokenType::For),
            ("ber", TokenType::If),
            ("sone", TokenType::Nil),
            ("reu", TokenType::Or),
            ("jongyeytha", TokenType::Print),
            ("morvenh", TokenType::Return),
            ("super", TokenType::Super),
            ("nis", TokenType::This),
            ("ok", TokenType::True),
            ("akthe", TokenType::Var),
            ("nvpeldae", TokenType::While),
        ]);
        keywords
    };
}


struct Bts {
    interpreter: Interpreter
}


impl Bts {
    fn new() -> Self {
        Bts {
            interpreter: Interpreter,
        }
    }

    fn run_file(&self, path: &str) -> Result<(), Error> {
        let source = fs::read_to_string(path)?;
        self.run(source)
    }

    fn run_prompt(&self) -> Result<(), Error> {
        let stdin = io::stdin();
        println!("> bts_interacc");
        for line in stdin.lines() {
            self.run(line?).expect("error reading line!");
            println!("> bts_interacc");
        }
        Ok(())
    }


    fn run(&self, source: String) -> Result<(), Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", &token);
        }
        let mut parser = Parser::new(tokens);
        if let Some(expression) = parser.parse() {
            println!("{}", self.interpreter.interpret(&expression)?);
        }
        Ok(())
    }
}



fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let lox = Bts::new();
    match args.as_slice() {
        [_, file] => match lox.run_file(file) {
            Ok(_) => (),
            Err(Error::Runtime { .. }) => exit(70),
            Err(Error::Parse) => exit(65),
            Err(Error::Io(_)) => unimplemented!(),
        },
        [_] => lox.run_prompt()?,
        _ => {
            eprintln!("Usage: bts [script]");
            exit(64)
        }
    }
    Ok(())
}
