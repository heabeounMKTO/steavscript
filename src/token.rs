use std::fmt;

#[derive(Debug, Clone)]
pub enum TokenType {
    // single char tokens (ti eh)
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma, 
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two char tokens
    Bang,  // !
    BangEqual, // != 
    Equal, // =
    EqualEqual, // ==
    Greater, // >
    GreaterEqual, // >=
    Less, // <
    LessEqual, // <=
    
    // literals
    Identifier,
    String {literal: String},
    Number {literal: f64},

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,


    // end of fiels 
    EOF,
}


#[derive(Debug, Clone)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line: i32
}

impl Token{
    pub fn new(ttype: TokenType, lexeme: &str, line: i32) -> Self {
       Self {
            ttype,
            lexeme: lexeme.to_string(),
            line
        } 
    }
}
