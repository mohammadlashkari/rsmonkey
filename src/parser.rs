use crate::{lexer, token};

pub struct Parser {
    lexer: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        let mut p = Self {
            lexer,
            cur_token: token::Token::Eof,
            peek_token: token::Token::Eof,
        };

        p.next_toke();
        p.next_toke();

        p
    }

    pub fn next_toke(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program() {
        todo!()
    }
}
