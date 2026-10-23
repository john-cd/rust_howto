#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Tokenize HTML with `html5ever` and print the parsed tokens.

use std::cell::Cell;

use html5ever::tendril::ByteTendril;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::CharacterTokens;
use html5ever::tokenizer::NullCharacterToken;
use html5ever::tokenizer::ParseError;
use html5ever::tokenizer::TagToken;
use html5ever::tokenizer::Token;
use html5ever::tokenizer::TokenSink;
use html5ever::tokenizer::TokenSinkResult;
use html5ever::tokenizer::Tokenizer;

/// A token sink that prints the tokens of an HTML document.
#[derive(Clone)]
struct TokenPrinter {
    /// Whether the previous token was a character, so that consecutive
    /// characters are printed as one text.
    in_char_run: Cell<bool>,
}

impl TokenPrinter {
    /// Starts or ends the printing of a run of characters, depending on whether
    /// the current token is a character.
    fn is_char(&self, is_char: bool) {
        match (self.in_char_run.get(), is_char) {
            (false, true) => print!("text: \""),
            (true, false) => println!("\""),
            _ => {}
        }
        self.in_char_run.set(is_char);
    }
}

impl TokenSink for TokenPrinter {
    type Handle = ();

    fn process_token(
        &self,
        token: Token,
        _line_number: u64,
    ) -> TokenSinkResult<()> {
        match token {
            CharacterTokens(text) => {
                self.is_char(true);
                print!("{text}");
            }
            NullCharacterToken => {
                self.is_char(true);
                print!("\\0");
            }
            TagToken(tag) => {
                self.is_char(false);
                println!("tag: {:?} {}", tag.kind, tag.name);
            }
            ParseError(err) => {
                self.is_char(false);
                println!("error: {err}");
            }
            _ => {
                self.is_char(false);
            }
        }
        TokenSinkResult::Continue
    }
}

/// Tokenizes an HTML string and prints the tokens.
fn main() {
    let html = "<html><body><p>Hello, world!</p></body></html>";
    let sink = TokenPrinter {
        in_char_run: Cell::new(false),
    };

    let mut chunk = ByteTendril::new();
    chunk.extend(html.as_bytes());
    let input = BufferQueue::default();
    input.push_back(chunk.try_reinterpret().expect("valid UTF-8 HTML"));

    let tokenizer = Tokenizer::new(sink.clone(), Default::default());
    let _ = tokenizer.feed(&input);
    tokenizer.end();
    sink.is_char(false);
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
    // [finish](https://github.com/john-cd/rust_howto/issues/1090)
}
