use std::fmt;

use crate::{
    ast::{LetStatement, Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        todo!()
    }

    pub fn parse_program(&mut self) -> Program {
        todo!()
    }

    fn parse_statement(&mut self) -> Statement {
        match self.cur_token {
            Token::Let => Statement::Let(self.parse_let_statement()),
            _ => todo!(),
        }
    }

    fn parse_let_statement(&mut self) -> LetStatement {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub want: Expected,
    // None means end of input
    pub got: Option<Token>,
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
    use crate::ast::Statement;

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
    #[ignore = "parser not implemented yet (Task 5)"]
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
