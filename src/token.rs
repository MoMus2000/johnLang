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

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Fn,
    Let
}