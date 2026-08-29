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
            b'=' if self.peek_char() == b'=' => {
                self.read_char();
                Token::Eq
            }
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'!' if self.peek_char() == b'=' => {
                self.read_char();
                Token::NotEq
            }
            b'!' => Token::Bang,
            b'/' => Token::Slash,
            b'>' => Token::Gt,
            b'<' => Token::Lt,
            0 => Token::Eof,
            c if is_letter(c) => return lookup_identifier(self.read_identifier()),
            c if c.is_ascii_digit() => {
                let lit = self.read_number();
                return match lit.parse::<i64>() {
                    Ok(num) => Token::Int(num),
                    Err(_) => Token::Illegal(lit.to_string()),
                };
            }
            c => Token::Illegal((c as char).to_string()),
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

    fn read_number(&mut self) -> &str {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        &self.input[start..self.position]
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_symbols() {
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
            assert_eq!(got, want, "test_case[{i}] at byte {}", lexer.position);
        }
    }

    #[test]
    fn next_token_basic() {
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
  return false;
}

10 == 10;
5 != 10;
@#
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
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(5),
            Token::NotEq,
            Token::Int(10),
            Token::Semicolon,
            Token::Illegal("@".to_string()),
            Token::Illegal("#".to_string()),
            Token::Eof,
        ];

        for (i, want) in cases.into_iter().enumerate() {
            let got = lexer.next_token();
            assert_eq!(got, want, "test_case[{i}] at byte {}", lexer.position);
        }
    }
}
