#[derive(Debug)]
pub struct Token{
    pub kind: TokenKind,
    pub literal: String
}
#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Illegal,
    Eof,

    Int,
    Ident,

    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,

    Eq,
    NotEq,

    Bang,
    Gt,
    Lt,
    GtEq,
    LtEq,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Fn,
    Let
}

pub fn match_literal_to_kind(literal: &str) -> TokenKind{
    let res = match literal{
        "fn" => TokenKind::Fn,
        "let" => TokenKind::Let,
        _ => TokenKind::Ident
    };
    res
}