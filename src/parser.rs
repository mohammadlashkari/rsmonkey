use std::{fmt, iter::Peekable};

use crate::ast::{Identifier, LetStatement, Program, Statement};

use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    tokens: Peekable<Lexer>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            tokens: lexer.peekable(),
        }
    }

    fn eat(&mut self, token: &Token) -> bool {
        self.tokens.next_if_eq(token).is_some()
    }

    fn expect(&mut self, want: Token) -> Result<(), ParseError> {
        match self.tokens.next() {
            Some(got) if got == want => Ok(()),
            got => Err(ParseError {
                got,
                want: Expected::Token(want),
            }),
        }
    }

    fn expect_ident(&mut self) -> Result<Identifier, ParseError> {
        match self.tokens.next() {
            Some(Token::Ident(name)) => Ok(Identifier { name }),
            got => Err(ParseError {
                got,
                want: Expected::Ident,
            }),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        todo!()
    }

    fn parse_statement(&mut self) -> Statement {
        todo!()
    }

    fn parse_let_statement(&mut self) -> LetStatement {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    // None means end of input
    pub got: Option<Token>,
    pub want: Expected,
}

#[derive(Debug, PartialEq)]
pub enum Expected {
    Token(Token),
    Ident,
    Expression,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expected {}, got ", self.want)?;
        match &self.got {
            Some(token) => write!(f, "'{token}'"),
            None => write!(f, "end of input"),
        }
    }
}

impl std::error::Error for ParseError {}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expected::Token(token) => write!(f, "'{token}'"),
            Expected::Ident => write!(f, "identifier"),
            Expected::Expression => write!(f, "expression"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser(input: &str) -> Parser {
        let lexer = Lexer::new(input.to_string());
        Parser::new(lexer)
    }

    #[test]
    fn parse_error_display() {
        let err = ParseError {
            want: Expected::Token(Token::Assign),
            got: Some(Token::Int(5)),
        };
        assert_eq!(err.to_string(), "expected '=', got '5'");

        let err = ParseError {
            want: Expected::Ident,
            got: None,
        };
        assert_eq!(err.to_string(), "expected identifier, got end of input");
    }

    #[test]
    fn eat_consumes_only_matching_token() {
        let input = "= 5";
        let mut parser = parser(input);

        assert!(parser.eat(&Token::Assign));
        assert!(!parser.eat(&Token::Assign));
        assert_eq!(parser.tokens.next(), Some(Token::Int(5)));
    }

    #[test]
    fn expect_consumes_matching_token() {
        let mut parser = parser("= 5");

        let got = parser.expect(Token::Assign);
        let want = Ok(());

        assert_eq!(got, want);
        assert_eq!(parser.tokens.next(), Some(Token::Int(5)));
    }

    #[test]
    fn expect_fails_on_wrong_token() {
        let mut parser = parser("5");

        let got = parser.expect(Token::Assign);
        let want = Err(ParseError {
            got: Some(Token::Int(5)),
            want: Expected::Token(Token::Assign),
        });

        assert_eq!(got, want);
    }

    #[test]
    fn expect_fails_at_end_of_input() {
        let mut parser = parser("");

        let got = parser.expect(Token::Assign);
        let want = Err(ParseError {
            got: None,
            want: Expected::Token(Token::Assign),
        });

        assert_eq!(got, want);
    }

    #[test]
    fn expect_ident_fails_at_end_of_input() {
        let mut parser = parser("");

        let got = parser.expect_ident();
        let want = Err(ParseError {
            got: None,
            want: Expected::Ident,
        });

        assert_eq!(got, want);
    }

    #[test]
    fn expect_ident_fails_on_wrong_token() {
        let mut parser = parser("5");

        let got = parser.expect_ident();
        let want = Err(ParseError {
            got: Some(Token::Int(5)),
            want: Expected::Ident,
        });

        assert_eq!(got, want);
    }

    #[test]
    fn expect_ident_moves_name_out() {
        let mut parser = parser("foo");

        let got = parser.expect_ident();
        let want = Ok(Identifier {
            name: "foo".to_string(),
        });

        assert_eq!(got, want);
    }

    #[test]
    #[ignore = "parser not implemented yet"]
    fn let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foo = 424242;
"#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements"
        );

        let want_identifiers = ["x", "y", "foo"];
        for (stmt, want) in program.statements.iter().zip(want_identifiers) {
            assert_let_statement(stmt, want);
        }
    }

    fn assert_let_statement(stmt: &Statement, want: &str) {
        let Statement::Let(let_stmt) = stmt else {
            panic!("stmt is not Statement::Let got={stmt:?}");
        };

        let got = &let_stmt.ident.name;
        assert_eq!(
            got, want,
            "let_stmt.ident.name wrong. got={got}, want={want}"
        );
    }
}
