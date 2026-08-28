#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    Eof,
    // Identifiers + literal
    Ident(String),
    Int(i64),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    If,
    Else,
    Let,
    True,
    False,
    Return,
}

pub fn lookup_identifier(identifier: &str) -> Token {
    match identifier {
        "fn" => Token::Function,
        "let" => Token::Let,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Ident(identifier.to_string()),
    }
}
