use crate::token::{Token, TokenType};
use crate::error::error;
use crate::KEYWORDS;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn add_token(&mut self, ttype: TokenType) {
        let text = self.source.get(self.start..self.current)
            .expect("da source token is empty (ort mean token te bro)");
        self.tokens.push(Token::new(ttype, text, self.line))
    }
    pub fn advance(&mut self) -> char {
        self.current += 1;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    /// advance to the next character and see if there is anything 
    pub fn nextmatch(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).expect("unexpected end of source.") != expected {
            return false;
        }
        self.current += 1;
        true
    }
    pub fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBrace),
            ']' => self.add_token(TokenType::RightBrace),
            '.' => self.add_token(TokenType::Dot),
            ',' => self.add_token(TokenType::Comma),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Comma),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.nextmatch('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.nextmatch('=') {
                    self.add_token(TokenType::EqualEqual)
                }else {
                    self.add_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.nextmatch('=') {
                    self.add_token(TokenType::LessEqual)
                }else {
                    self.add_token(TokenType::Less)
                }
            },
            '>'=> {
                if self.nextmatch('=') {
                    self.add_token(TokenType::GreaterEqual)
                }else {
                    self.add_token(TokenType::Greater)
                }
            },
            '/' => {
                if self.nextmatch('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => (), // mfw no whitespace
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if c.is_digit(10) {
                    self.number()
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier()
                } else {
                    error(self.line, "nhom ort skol tha ah neng saey ke te bro (jes c code ort neng yiiii)")
                }
            }
        }
    }

    pub fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let n: f64 = self.source.get(self.start..self.current).expect("unexpected end.").parse().expect("parse lek ng ort jenh te bro (tae scan ban jeng ort deng dea)");
        self.add_token(TokenType::Number {literal: n})
    } 
    pub fn peek_next(&self) -> char{
        self.source.chars().nth(self.current + 1).unwrap_or('\0')

    } 
    pub fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "jong yey tha bro ort bet string te ey? jam na ther oy thom thom os hz");
        }

        self.advance();

        let literal = self.source.get((self.start + 1)..(self.current - 1)).expect("unexpected end.").to_string();
        self.add_token(TokenType::String {literal})
    }

    pub fn identifier(&mut self) {
        while self.peek().is_alphabetic() || self.peek() == '_' {
            self.advance();
        }

        let text = self.source.get(self.start..self.current).expect("unexpected end!");
        let ttype: TokenType = KEYWORDS.get(text).cloned().unwrap_or(TokenType::Identifier);
        self.add_token(ttype);
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        &self.tokens
    }
}
