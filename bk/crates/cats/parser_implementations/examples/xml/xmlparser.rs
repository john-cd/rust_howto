// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Use `xmlparser` when an application needs low-level tokens and control over
//! XML processing without building a tree. Prefer `roxmltree` for convenient
//! read-only tree access or `quick-xml` for higher-level streaming APIs.

use xmlparser::ElementEnd;
use xmlparser::Token;
use xmlparser::Tokenizer;

/// Tokenizes a small XML string and returns its element names and attributes.
fn tokenize(xml: &str) -> anyhow::Result<Vec<String>> {
    let mut tokens = Vec::new();

    for token in Tokenizer::from(xml) {
        match token? {
            Token::ElementStart { local, .. } => {
                tokens.push(format!("start:{}", local.as_str()));
            }
            Token::Attribute { local, value, .. } => {
                tokens.push(format!(
                    "attribute:{}={}",
                    local.as_str(),
                    value.as_str()
                ));
            }
            Token::ElementEnd { end, .. } => match end {
                ElementEnd::Close(_, local) => {
                    tokens.push(format!("end:{}", local.as_str()));
                }
                ElementEnd::Empty => tokens.push("empty".to_owned()),
                ElementEnd::Open => {}
            },
            _ => {}
        }
    }

    Ok(tokens)
}

/// Prints the tokens collected from a small XML document.
fn main() -> anyhow::Result<()> {
    let xml = r#"<library><book id="b1">Rust</book></library>"#;
    for token in tokenize(xml)? {
        println!("{token}");
    }

    Ok(())
}
// ANCHOR_END: example

/// Runs the example and propagates tokenization errors.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the tokenizer collects element names and attributes.
    #[test]
    fn xmlparser_collects_element_names_and_attributes() -> anyhow::Result<()> {
        let tokens = tokenize(r#"<library><book id="b1"/></library>"#)?;

        assert_eq!(
            tokens,
            [
                "start:library",
                "start:book",
                "attribute:id=b1",
                "empty",
                "end:library"
            ]
        );
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
