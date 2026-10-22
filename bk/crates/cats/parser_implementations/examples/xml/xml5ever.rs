#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Tokenize XML with `xml5ever` and print the parsed tokens.

use markup5ever::buffer_queue::BufferQueue;
use xml5ever::tendril::ByteTendril;
use xml5ever::tokenizer::Doctype;
use xml5ever::tokenizer::Pi;
use xml5ever::tokenizer::ProcessResult;
use xml5ever::tokenizer::Token;
use xml5ever::tokenizer::TokenSink;
use xml5ever::tokenizer::XmlTokenizer;

/// A token sink that prints the tokens of an XML document.
struct TokenPrinter;

impl TokenSink for TokenPrinter {
    type Handle = ();

    fn process_token(&self, token: Token) -> ProcessResult<()> {
        match token {
            Token::Characters(text) => println!("text: {text}"),
            Token::Tag(tag) => {
                println!("tag: {:?} {}", tag.kind, tag.name.local);
            }
            Token::ProcessingInstruction(Pi { target, data }) => {
                println!("pi: {target} {data}");
            }
            Token::Doctype(Doctype { name, .. }) => {
                println!("doctype: {name:?}");
            }
            Token::ParseError(err) => println!("error: {err}"),
            Token::Comment(comment) => println!("comment: {comment}"),
            Token::NullCharacter => println!("null"),
            Token::EndOfFile => println!("eof"),
        }
        ProcessResult::Continue
    }
}

/// Tokenizes an XML string and prints the tokens.
fn main() {
    let xml = r#"<library><book id="b1">Rust</book></library>"#;
    let mut input = ByteTendril::new();
    input.extend(xml.as_bytes());

    let input_buffer = BufferQueue::default();
    input_buffer.push_back(input.try_reinterpret().expect("valid UTF-8 XML"));

    let tokenizer = XmlTokenizer::new(TokenPrinter, Default::default());
    let _ = tokenizer.feed(&input_buffer);
    tokenizer.end();
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
    // [finish](https://github.com/john-cd/rust_howto/issues/1103)
}
