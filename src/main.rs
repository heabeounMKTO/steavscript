mod scanner;
mod token;
mod error;

use lazy_static::lazy_static;
use token::TokenType;
use anyhow::{Error, Result};
use scanner::Scanner;
use std::io;
use std::{env, fs};
use std::collections::HashMap;
use std::process::exit;


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
    ("talob", TokenType::Return),
    ("super", TokenType::Super),
    ("nis", TokenType::This),
    ("ok", TokenType::True ),
    ("akthe", TokenType::Var),
    ("nvpel", TokenType::While)
]);
    keywords
    };

}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, file] => run_file(file)?,
        [_] => run_prompt()?,
        _ => {
            eprintln!("usage: bts [script]");
            exit(64)
        }
    }
    Ok(())
}

fn run_file(path: &str) -> anyhow::Result<()> {
    let source = fs::read_to_string(path)?;
    run(source)
}

fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    for line in stdin.lines() {
        run(line?).expect("error reading line!");
        println!("> ");
    }
    Ok(())
}

fn run(source: String) -> Result<(), Error> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
