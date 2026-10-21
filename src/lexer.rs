use crate::span::{Span, Spanned};
use crate::token::{Token, lookup_identifier};

// TODO: Support UTF-8

pub struct Lexer {
    input: String,
    // Points to the character in the input
    position: usize,
}

impl Lexer {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            position: 0,
        }
    }

    fn current(&self) -> Option<u8> {
        self.input.as_bytes().get(self.position).copied()
    }

    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.position + 1).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn next_token(&mut self) -> Option<Spanned<Token>> {
        self.skip_whitespace();

        let start = self.position;
        let current = self.current()?;

        let token = match (current, self.peek()) {
            (b'=', Some(b'=')) => self.take(2, Token::Eq),
            (b'!', Some(b'=')) => self.take(2, Token::NotEq),
            (b'=', _) => self.take(1, Token::Assign),
            (b'+', _) => self.take(1, Token::Plus),
            (b'(', _) => self.take(1, Token::LParen),
            (b')', _) => self.take(1, Token::RParen),
            (b'{', _) => self.take(1, Token::LBrace),
            (b'}', _) => self.take(1, Token::RBrace),
            (b',', _) => self.take(1, Token::Comma),
            (b';', _) => self.take(1, Token::Semicolon),
            (b'-', _) => self.take(1, Token::Minus),
            (b'*', _) => self.take(1, Token::Asterisk),
            (b'!', _) => self.take(1, Token::Bang),
            (b'/', _) => self.take(1, Token::Slash),
            (b'>', _) => self.take(1, Token::Gt),
            (b'<', _) => self.take(1, Token::Lt),
            (c, _) if is_letter(c) => self.lex_identifier(),
            (c, _) if c.is_ascii_digit() => self.lex_number(),
            (c, _) => self.take(1, Token::Illegal(char::from(c).to_string())),
        };

        Some(Spanned {
            node: token,
            span: Span::new(start, self.position),
        })
    }

    /// Consume `len` bytes and return `token`.
    fn take(&mut self, len: usize, token: Token) -> Token {
        self.position += len;
        token
    }

    fn lex_identifier(&mut self) -> Token {
        lookup_identifier(self.read_identifier())
    }

    fn lex_number(&mut self) -> Token {
        let literal = self.read_number();
        match literal.parse() {
            Ok(num) => Token::Int(num),
            Err(_) => Token::Illegal(literal.to_string()),
        }
    }

    fn read_identifier(&mut self) -> &str {
        let start = self.position;
        while self.current().is_some_and(is_ident_char) {
            self.advance();
        }

        &self.input[start..self.position]
    }

    fn read_number(&mut self) -> &str {
        let start = self.position;
        while self.current().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        &self.input[start..self.position]
    }

    fn skip_whitespace(&mut self) {
        while self.current().is_some_and(|c| c.is_ascii_whitespace()) {
            self.advance();
        }
    }
}

impl Iterator for Lexer {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

// Digits are allowed after the first letter: `foo1` is one identifier
fn is_ident_char(ch: u8) -> bool {
    is_letter(ch) || ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[track_caller]
    fn assert_tokens(input: &str, want: &[Token]) {
        let got: Vec<Spanned<Token>> = Lexer::new(input).collect();

        for (i, (got, want)) in got.iter().zip(want).enumerate() {
            assert_eq!(got.node, *want, "token[{i}] for input {input:?}");
        }
        assert_eq!(got.len(), want.len(), "token count for input {input:?}");
    }

    #[test]
    fn lex_ends_with_none() {
        let mut lexer = Lexer::new(String::new());
        assert_eq!(lexer.next(), None);
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn lex_nul_byte_is_illegal() {
        let input = "let x\0 = 5;";
        let want = [
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Illegal("\0".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
        ];

        assert_tokens(input, &want);
    }

    #[test]
    fn lex_identifiers_with_digits() {
        let input = "foo1 x2y _a9 1x";
        let want = [
            Token::Ident("foo1".to_string()),
            Token::Ident("x2y".to_string()),
            Token::Ident("_a9".to_string()),
            // Starts with a digit, so it's a number followed by an identifier
            Token::Int(1),
            Token::Ident("x".to_string()),
        ];

        assert_tokens(input, &want);
    }

    #[test]
    fn lex_simple() {
        let input = r"let foo = 5;";
        let want = [
            Token::Let,
            Token::Ident("foo".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
        ];

        assert_tokens(input, &want);
    }

    #[test]
    fn lex_symbols() {
        let input = r#"=+(){},;"#;
        let want = [
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        assert_tokens(input, &want);
    }

    #[test]
    fn lex_complex() {
        let input = r"
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
";

        let want = [
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
        ];

        assert_tokens(input, &want);
    }
}
