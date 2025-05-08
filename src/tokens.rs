pub struct Token {
    kind: TokenType,
    children: Vec<Token>,
}

pub enum TokenType {
    Pub,
    Crate,
    Super,
    PathDelim,
    Struct,
    Name(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    LArrow,
    RArrow,
    ScalarTypes,
    AdvancedType,
}

// TODO: Add feature support for nightly
pub enum LiteralTypes {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Bool,
    Char,
}
