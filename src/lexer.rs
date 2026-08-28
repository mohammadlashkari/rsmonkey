use std::num::ParseIntError;

use crate::token::{Token, lookup_identifier};

// TODO: Support UTF-8
pub struct Lexer {
    input: String,
    // Current position in input
    // Points to the character in the input that corresponds to the ch byte
    position: usize,
    // Points  to the next character in the input
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut s = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        s.read_char();

        s
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            0 => Token::Eof,
            c if is_letter(c) => return lookup_identifier(self.read_identifier()),
            c if c.is_ascii_digit() => {
                return match self.read_number() {
                    Ok(num) => Token::Int(num),
                    Err(_) => Token::Illegal,
                };
            }
            _ => Token::Illegal,
        };

        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> &str {
        let start = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        &self.input[start..self.position]
    }

    fn read_number(&mut self) -> Result<i64, ParseIntError> {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        self.input[start..self.position].parse::<i64>()
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_next_token_symbols() {
        let mut lexer = Lexer::new(String::from("=+(){},;"));
        let cases = [
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        for (i, want) in cases.into_iter().enumerate() {
            let got = lexer.next_token();
            assert_eq!(got, want, "test_case[{i}]");
        }
    }

    #[test]
    fn t_next_token_basic() {
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
"#;
        let mut lexer = Lexer::new(input.to_string());

        let cases = [
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
        ];

        for (i, want) in cases.into_iter().enumerate() {
            let got = lexer.next_token();
            assert_eq!(got, want, "test_case[{i}]");
        }
    }
}
