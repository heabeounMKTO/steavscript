use crate::utils::bek::SyntaxError;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BerkSekdey,
    BetSekdey,
    BerkVungKjork,
    BetVungKjork,
    Uhh,
    LekKutToch(i32),
    LekKutThom(i64),
    MinMan,
    Boke, 
    Dork,
    Kun,
    Jaek,
    Neng,
    Reu,
    Smer,
    OrtSmer,
    TicJeang,
    TicJeangReuSmer,
    JrenJeang, 
    JrenJeangReuSmer,
    Dak,
    Talob,
}

impl Token {
    pub fn keywords(value: &str) -> Option<Self> {
        match value {
            "LekKutToch" => Some(Self::LekKutToch(value.parse().unwrap())),
            "Talob" => Some(Self::Talob),
            _ => None
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LekKutToch(val) => f.write_fmt(format_args!("Token: Integer{{{}}}", val)),
            val => f.write_fmt(format_args!("Token: {:?}", val))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    Boke, 
    Dork,
    Kun,
    Jaek,
    Neng,
    Reu,
    Smer,
    OrtSmer,
    TicJeang,
    TicJeangReuSmer,
    JrenJeang, 
    JrenJeangReuSmer
}

impl TryFrom<Token> for BinaryOperator {
   type Error = SyntaxError;

   fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value{
            Token::Boke => Ok(Self::Boke),
            Token::MinMan => Ok(Self::Dork),
            Token::Kun => Ok(Self::Kun),
            Token::Jaek => Ok(Self::Jaek),
            Token::Neng => Ok(Self::Neng),
            Token::Reu => Ok(Self::Reu),
            Token::Smer => Ok(Self::Smer),
            Token::OrtSmer => Ok(Self::OrtSmer),
            Token::TicJeang => Ok(Self::TicJeang),
            Token::TicJeangReuSmer => Ok(Self::TicJeangReuSmer),
            Token::JrenJeang => Ok(Self::JrenJeang), 
            Token::JrenJeangReuSmer => Ok(Self::JrenJeangReuSmer),
            _ => Err(SyntaxError::lex_error("kae seeeenha ort jenh heh bro".to_string()))
        }
    }

}
