mod env;
mod error;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod syntax;
mod token;

use error::Error;
use interpreter::Interpreter;
use lazy_static::lazy_static;
use parser::Parser;
use scanner::Scanner;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::process::exit;
use syntax::AstPrinter;
use token::TokenType;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let keywords: HashMap<&'static str, TokenType> = HashMap::from([
            ("bongSlanhOun", TokenType::BongSlanhOun),
            ("ng", TokenType::And), // 
            ("tnak", TokenType::Class), // 
            ("minjengte", TokenType::Else), // 
            ("ort", TokenType::False), // 
            ("mukngea", TokenType::Fun), // 
            ("somhab", TokenType::For), // 
            ("ber", TokenType::If), // 
            ("sone", TokenType::Nil), // 
            ("reu", TokenType::Or), // 
            ("jongyeytha", TokenType::Print), // 
            ("morvenh", TokenType::Return), // 
            ("super", TokenType::Super), // 
            ("nis", TokenType::This), //
            ("ok", TokenType::True), // 
            ("akthe", TokenType::Var), // 
            ("nvpeldae", TokenType::While), // 
        ]);
        keywords
    };
}

struct Bts {
    interpreter: Interpreter,
}

impl Bts {
    fn new() -> Self {
        Bts {
            interpreter: Interpreter::new(),
        }
    }

    fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let source = fs::read_to_string(path)?;

        self.run(source, false)
    }

    fn run_prompt(&mut self) -> Result<(), Error> {
        let stdin = io::stdin();
        println!("> bts_interacctive session (yoooo)");
        for line in stdin.lines() {
            self.run(line?, true).expect("error reading line!");
            println!("> bts_interacc");
        }
        Ok(())
    }

    fn run(&mut self, source: String, prompt_mode: bool) -> Result<(), Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("[DEBUG] TOKEN : {:?}", token);
        }
        // CHECKS FOR BONG SLANH OUN
        /* if tokens[0].ttype != TokenType::BongSlanhOun && prompt_mode == false{
            println!("[FATAL] BONG_SLANH_OUN ERROR: na `bongSlanhOun`?!!!\nplease inlcude `bongSlanhOun` in the first line of the file!");
            panic!()
        } */
        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;
        self.interpreter.interpret(&statements)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = std::env::args().collect();
    let mut bt_ach = Bts::new();
    match args.as_slice() {
        [_, file] => match bt_ach.run_file(file) {
            Ok(_) => (),
            Err(Error::Runtime { .. }) => exit(70),
            Err(Error::Parse) => exit(65),
            Err(Error::Io(_)) => unimplemented!(),
        },
        [_] => bt_ach.run_prompt()?,
        _ => {
            eprintln!("Usage: bts [script]");
            exit(64)
        }
    }
    Ok(())
}
