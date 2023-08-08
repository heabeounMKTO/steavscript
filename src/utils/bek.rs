#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    level: String
}

impl SyntaxError {
    pub fn lex_error(message: String) -> Self {
        SyntaxError {
            message, 
            level: String::from("bek hz bro (lexing)")
        }
    }
}
