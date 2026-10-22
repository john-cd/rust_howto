#![allow(dead_code)]
// ANCHOR: example
//! Tokenize a small CSS declaration list with `cssparser`.
//!
//! TODO Reviewed ??-??-2026

use cssparser::Parser;
use cssparser::ParserInput;
use cssparser::ToCss;
use cssparser::Token;

fn main() {
    let css = "color: red; font-size: 16px;";
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);

    while let Ok(token) = parser.next() {
        match token {
            Token::Ident(ident) => println!("identifier: {}", ident.as_ref()),
            Token::Colon => println!("colon"),
            Token::Semicolon => println!("semicolon"),
            Token::Dimension { value, unit, .. } => {
                println!("dimension: {value}{unit}")
            }
            other => println!("token: {}", other.to_css_string()),
        }
    }
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/1092)
}
