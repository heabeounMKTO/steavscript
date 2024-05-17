use std::fmt;

#[derive(Debug, PartialEq, Clone)]
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
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    // literals
    Identifier,
    String { literal: String },
    Number { literal: f64 },

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
     
     /* 
    * literally does nothing 
     but BTS will refuse to run without
     you writing bongSlanhOun on the first line
     SMH (i literally made this choice) 
    putting this here because i might shoot 
    myself in the foot in a later date
    */
    
    BongSlanhOun,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: &str, line: i32) -> Self {
        Self {
            ttype,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ttype {
            TokenType::String { literal } => write!(f, "String {:?} {:?}", self.lexeme, literal),
            TokenType::Number { literal } => write!(f, "Number {:?} {:?}", self.lexeme, literal),
            _ => write!(f, "{:?} {:?}", self.ttype, self.lexeme),
        }
    }
}
