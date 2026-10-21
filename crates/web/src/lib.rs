use rsmonkey::lexer::Lexer;
use rsmonkey::parser::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(input: &str) -> String {
    let width = input.len().to_string().len();

    Lexer::new(input)
        .map(|token| {
            let span = token.span;
            format!(
                "{:>width$}..{:<width$} {:?}\n",
                span.start, span.end, token.node
            )
        })
        .collect()
}

// In JS, Err is thrown as a string
#[wasm_bindgen]
pub fn parse(input: &str) -> Result<String, String> {
    Parser::parse(input)
        .map(|program| format!("{program:#?}"))
        .map_err(|errors| errors.iter().map(|err| format!("{err}\n")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_prints_one_token_per_line() {
        let input = "let x";
        let want = "0..3 Let\n4..5 Ident(\"x\")\n";
        let got = tokenize(input);
        assert_eq!(got, want);
    }

    #[test]
    fn tokenize_aligns_spans() {
        let input = "let x = 20;";
        let want = r#" 0..3  Let
 4..5  Ident("x")
 6..7  Assign
 8..10 Int(20)
10..11 Semicolon
"#;
        let got = tokenize(input);
        assert_eq!(got, want);
    }

    #[test]
    fn tokenize_empty_input() {
        assert_eq!(tokenize(""), "");
    }

    #[test]
    fn parse_returns_ast_on_success() {
        let got = parse("let x = 3;").unwrap_or_else(|errs| panic!("parser errors: {errs}"));
        assert!(got.starts_with("Program {"), "got: {got}");
    }

    #[test]
    fn parse_returns_one_error_per_line() {
        let input = "let x 5;";
        let want = "expected '=', got '5' at 6..7\nexpected expression, got ';' at 7..8\n";
        let got = parse(input);
        assert_eq!(got, Err(want.to_string()));
    }
}
