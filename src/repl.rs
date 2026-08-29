use std::io::{BufRead, Write};

use crate::lexer::Lexer;
use crate::token::Token;

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) -> std::io::Result<()> {
    loop {
        write!(output, "{PROMPT}")?;
        output.flush()?;

        let mut line = String::new();
        if input.read_line(&mut line)? == 0 {
            return Ok(());
        }

        let mut lexer = Lexer::new(line);
        loop {
            let token = lexer.next_token();
            if token == Token::Eof {
                break;
            }
            writeln!(output, "{token:?}")?;
        }
    }
}
