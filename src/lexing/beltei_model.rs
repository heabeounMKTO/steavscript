
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    BerkSekdey,
    BetSekdey,
    BerkVungKjork,
    BetVungKjork,
    Uhh,
    LekKutToch,
    LekKutThom,
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
    ThomJeang, 
    ThomJeangReuSmer,
    Dak
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
    ThomJeang, 
    ThomJeangReuSmer
}

impl TryFrom<Token> for BinaryOperator {
    

}
