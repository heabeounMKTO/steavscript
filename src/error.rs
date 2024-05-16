use crate::token::{Token, TokenType};
use std::fmt;

pub enum Error {
    Parse,
}

pub fn parser_error(token: &Token, message: &str) {
    if token.ttype == TokenType::EOF {
        error(token.line, " at end", message);
    }
}

pub fn error(line: i32, where_: &str, message: &str) -> () {
    println!(
        "ERROR AT LINE: {}, WHERE: {}, MESSAGE: {}",
        line, where_, message
    );
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse => write!(f, "ParseError"),
        }
    }
}
