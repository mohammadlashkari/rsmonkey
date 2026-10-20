use std::{fmt, iter::Peekable};

use crate::ast::{self, Expression, Ident, LetStatement, Program, Statement};

use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    tokens: Peekable<Lexer>,
}

enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            tokens: lexer.peekable(),
        }
    }

    pub fn parse(input: impl Into<String>) -> Result<Program, Vec<ParseError>> {
        Self {
            tokens: Lexer::new(input).peekable(),
        }
        .parse_program()
    }

    fn parse_program(mut self) -> Result<Program, Vec<ParseError>> {
        let mut program = Program::default();
        let mut errors = Vec::new();

        while self.tokens.peek().is_some() {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(err) => errors.push(err),
            }
        }
        if errors.is_empty() {
            Ok(program)
        } else {
            Err(errors)
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let stmt = if self.eat(&Token::Let) {
            Statement::Let(self.parse_let_statement()?)
        } else if self.eat(&Token::Return) {
            Statement::Return(self.parse_expression(Precedence::Lowest)?)
        } else {
            Statement::Expression(self.parse_expression(Precedence::Lowest)?)
        };

        // TODO: Semicolons are optional
        self.eat(&Token::Semicolon);

        Ok(stmt)
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, ParseError> {
        let ident = self.expect_ident()?;
        self.expect(Token::Assign)?;

        let value = self.parse_expression(Precedence::Lowest)?;

        Ok(LetStatement { ident, value })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        self.parse_prefix_expression()
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        match self.tokens.next() {
            Some(Token::Int(int)) => Ok(Expression::Integer(int)),
            Some(Token::Ident(name)) => Ok(Expression::Ident(Ident { name })),
            got => Err(ParseError {
                got,
                want: Expected::Expression,
            }),
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

    fn expect_ident(&mut self) -> Result<Ident, ParseError> {
        match self.tokens.next() {
            Some(Token::Ident(name)) => Ok(Ident { name }),
            got => Err(ParseError {
                got,
                want: Expected::Ident,
            }),
        }
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
    use core::panic;

    use super::*;

    fn parser(input: &str) -> Parser {
        let lexer = Lexer::new(input);
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
        let want = Ok(Ident {
            name: "foo".to_string(),
        });

        assert_eq!(got, want);
    }

    #[test]
    fn let_statements() {
        let input = r"
let x = 5;
let y = 10;
let foo = 424242;
";

        let program = parser(input)
            .parse_program()
            .unwrap_or_else(|errs| panic!("parser errors: {errs:?}"));

        let want = [let_int("x", 5), let_int("y", 10), let_int("foo", 424242)];

        assert_eq!(program.statements, want);
    }

    #[test]
    fn let_statements_without_semicolons() {
        let program = parser("let x = 5 let y = x")
            .parse_program()
            .unwrap_or_else(|errs| panic!("parser errors: {:?}", errs));

        let want = [
            let_int("x", 5),
            Statement::Let(LetStatement {
                ident: ident("y"),
                value: Expression::Ident(ident("x")),
            }),
        ];

        assert_eq!(program.statements, want);
    }

    #[test]
    fn let_statement_fails_at_end_of_input() {
        let got = parser("let x").parse_program();
        let want = Err(vec![ParseError {
            got: None,
            want: Expected::Token(Token::Assign),
        }]);

        assert_eq!(got, want);
    }

    #[test]
    #[ignore = "needs error recovery (Task 6)"]
    fn let_statement_reports_one_error_per_mistake() {
        let got = parser("let x 5;").parse_program();
        let want = Err(vec![ParseError {
            got: Some(Token::Int(5)),
            want: Expected::Token(Token::Assign),
        }]);

        assert_eq!(got, want);
    }

    #[test]
    fn return_statement() {
        let input = r"
return 5;
return 10;
return 4242;";
        let got = Parser::parse(input).unwrap_or_else(|errs| panic!("parser errors: {:?}", errs));
        let want = [
            Statement::Return(Expression::Integer(5)),
            Statement::Return(Expression::Integer(10)),
            Statement::Return(Expression::Integer(4242)),
        ];

        assert_eq!(got.statements, want);
    }

    #[test]
    fn return_statement_report_error() {
        let got = Parser::parse("return;");

        let want = Err(vec![ParseError {
            got: Some(Token::Semicolon),
            want: Expected::Expression,
        }]);

        assert_eq!(got, want);
    }

    #[test]
    fn parse_ident_expression() {
        let got = Parser::parse("foo;").unwrap_or_else(|errs| panic!("parser errors: {:?}", errs));
        let want = [Statement::Expression(Expression::Ident(ident("foo")))];

        assert_eq!(got.statements, want);
    }

    #[test]
    fn parse_int_literal_expression() {
        let got = Parser::parse("42;").unwrap_or_else(|errs| panic!("parser errors: {:?}", errs));
        let want = [Statement::Expression(Expression::Integer(42))];

        assert_eq!(got.statements, want);
    }

    fn ident(name: &str) -> Ident {
        Ident {
            name: name.to_string(),
        }
    }

    fn let_int(name: &str, value: i64) -> Statement {
        Statement::Let(LetStatement {
            ident: ident(name),
            value: Expression::Integer(value),
        })
    }
}
