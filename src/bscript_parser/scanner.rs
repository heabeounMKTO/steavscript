use super::tokens::BscriptToken;

pub struct Scanner {
    source: String,
    tokens: Vec<BscriptToken>
}


impl Scanner {
    pub fn new(source: String, tokens: Vec<BscriptToken>) -> Scanner {
        Scanner {
            source: source,
            tokens: tokens
        }
    }
}