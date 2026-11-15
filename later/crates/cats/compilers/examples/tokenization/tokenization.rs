// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Tokenizes a small arithmetic expression with logos.
//!
//! Tokenization (lexing) is the first stage of a compiler or interpreter: it
//! splits source text into tokens such as numbers, operators and identifiers. A
//! parser then builds structure from the tokens. `logos` generates a fast lexer
//! from annotated enums.

use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
/// Defines the outcomes available to the tokenization workflow, including identifier, plus.
#[logos(skip r"[ \t]+")]
enum Token {
    /// An identifier made from ASCII letters.
    #[regex(r"[a-zA-Z]+")]
    Identifier,
    /// The addition operator.
    #[token("+")]
    Plus,
}

/// Lexes two identifiers and an addition operator.
fn main() {
    let tokens = Token::lexer("left + right")
        .map(|token| token.expect("the sample contains only declared tokens"))
        .collect::<Vec<_>>();

    assert_eq!(tokens, [Token::Identifier, Token::Plus, Token::Identifier]);
}
// ANCHOR_END: example

/// Tokenizes a small arithmetic expression with logos.
pub fn run() {
    main();
}

/// order.
#[cfg(test)]
mod tests {
    use super::*;

    /// order.
    #[test]
    fn tokenizes_identifiers_around_addition() {
        main();
    }
}
