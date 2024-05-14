use std::fmt;
use crate::token::{TokenType, Token};


pub fn parser_error(token: &Token, message: &str) {
    if token.ttype == TokenType::EOF {
        error(token.line, " at end", message);
    }
}

pub fn error(line: i32, where_: &str,message: &str) -> () {
    println!("ERROR AT LINE: {}, WHERE: {}, MESSAGE: {}", line, where_,message);
}
