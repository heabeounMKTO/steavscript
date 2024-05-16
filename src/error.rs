use crate::token::{Token, TokenType};
use std::convert;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse,
    Runtime { token: Token, message: String },
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
            Error::Io(underlying) => write!(f, "IO ERROR: {}", underlying),
            Error::Parse => write!(f, "ParseError"),
            Error::Runtime { message, .. } => write!(f, "RUNTIME ERROR {}", message),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "BTS ERROR LMAOOO"
    }
}

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
