#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Tokenize a small CSS declaration list with `cssparser`.

use cssparser::Parser;
use cssparser::ParserInput;
use cssparser::ToCss as _;
use cssparser::Token;

/// Tokenizes a CSS declaration list and prints each token.
///
/// # Errors
///
/// Returns an error if the declaration list contains an invalid token.
fn main() -> anyhow::Result<()> {
    let css = "color: red; font-size: 16px;";
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);

    while !parser.is_exhausted() {
        let token = parser.next().map_err(|error| {
            anyhow::anyhow!(
                "invalid CSS token at {:?}: {}",
                error.location,
                error.kind
            )
        })?;
        match token {
            Token::Ident(ident) => println!("identifier: {}", ident.as_ref()),
            Token::Colon => println!("colon"),
            Token::Semicolon => println!("semicolon"),
            Token::Dimension { value, unit, .. } => {
                println!("dimension: {value}{unit}");
            }
            other => println!("token: {}", other.to_css_string()),
        }
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the CSS tokenization example.
///
/// # Errors
///
/// Returns an error if the declaration list contains an invalid token.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the sample CSS declaration list can be tokenized.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
