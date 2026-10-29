// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Tokenize a small CSS declaration list with `cssparser`.

use cssparser::Parser;
use cssparser::ToCss as _;
use cssparser::Token;

/// Tokenizes a CSS declaration list and prints each token.
///
/// # Errors
///
/// Returns an error if the declaration list contains an invalid token.
fn main() -> anyhow::Result<()> {
    let css = "color: red; font-size: 16px;";
    let mut parser = Parser::new(css);

    while !parser.is_exhausted() {
        let location = parser.current_source_location();
        let token = parser.next().map_err(|error| {
            anyhow::anyhow!("invalid CSS token at {location:?}: {}", error.kind)
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
    /// Runs the cssparser example through its entry point.
    #[test]
    fn test_cssparser_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Collects the tokens of a CSS snippet as strings.
    fn tokens(css: &str) -> Vec<String> {
        let mut parser = Parser::new(css);
        let mut out = Vec::new();
        while let Ok(token) = parser.next() {
            out.push(token.to_css_string());
        }
        out
    }

    /// The declaration list is split into identifiers, colons and dimensions.
    #[test]
    fn tokenizes_a_declaration_list() {
        let tokens = tokens("color: red; font-size: 16px;");
        assert_eq!(
            tokens,
            ["color", ":", "red", ";", "font-size", ":", "16px", ";"]
        );
    }

    /// An empty input yields no tokens.
    #[test]
    fn empty_input_has_no_tokens() {
        assert!(tokens("").is_empty());
    }
}
