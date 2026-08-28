#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    // Identifiers + literal
    Ident(String),
    Int(i64),
    // Operators
    Assign,
    Plus,
    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
}

pub fn lookup_identifier(identifier: &str) -> Token {
    match identifier {
        "fn" => Token::Function,
        "let" => Token::Let,
        _ => Token::Ident(identifier.to_string()),
    }
}
