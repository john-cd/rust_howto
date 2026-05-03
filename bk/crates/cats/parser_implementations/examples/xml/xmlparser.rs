#![allow(dead_code)]
// ANCHOR: example
//! Parse a small XML document token by token with `xmlparser`.

use xmlparser::ElementEnd;
use xmlparser::Token;
use xmlparser::Tokenizer;

fn main() {
    let xml = r#"<library><book id="b1">Rust</book></library>"#;

    for token in Tokenizer::from(xml) {
        match token {
            Ok(Token::ElementStart { local, .. }) => {
                println!("start element: {}", local.as_str());
            }
            Ok(Token::Attribute { local, value, .. }) => {
                println!("attribute: {}={}", local.as_str(), value.as_str());
            }
            Ok(Token::Text { text, .. }) => {
                let trimmed = text.as_str().trim();
                if !trimmed.is_empty() {
                    println!("text: {trimmed}");
                }
            }
            Ok(Token::ElementEnd { end, .. }) => match end {
                ElementEnd::Close(_, local) => {
                    println!("end element: {}", local.as_str());
                }
                ElementEnd::Empty => println!("empty element"),
                ElementEnd::Open => {}
            },
            Err(error) => {
                eprintln!("parse error: {error}");
                break;
            }
            _ => {}
        }
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/1102)
