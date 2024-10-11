use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(BigInt),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    GreaterThan,
    LessThan,
    Assign,
    Comma,
    Print,
    LBrace,
    RBrace,
    LParen,
    RParen,
    If,
    Else,
    StringLiteral(String),
    DewPoint,
    FToC,
    CToF,
    EOF,
}