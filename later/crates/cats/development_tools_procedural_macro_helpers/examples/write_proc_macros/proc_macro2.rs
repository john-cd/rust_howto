// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `proc_macro2` mirrors the compiler's token API, but works in ordinary Rust
//! crates and tests. Parse source, inspect token trees, and build a stream
//! without invoking a procedural-macro compiler entry point.

use anyhow::Result;
use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::Punct;
use proc_macro2::Spacing;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

/// Parses Rust source into a token stream.
///
/// # Errors
///
/// Returns an error if the source is malformed.
fn parse_stream(source: &str) -> Result<TokenStream> {
    source.parse().map_err(|error: proc_macro2::LexError| {
        anyhow::anyhow!("failed to parse source as a token stream: {error}")
    })
}

/// Appends readable descriptions of tokens, recursively visiting groups.
fn describe_tokens(stream: TokenStream, descriptions: &mut Vec<String>) {
    for token in stream {
        match token {
            TokenTree::Group(group) => {
                descriptions
                    .push(format!("group {:?}: {group}", group.delimiter()));
                describe_tokens(group.stream(), descriptions);
            }
            TokenTree::Ident(ident) => {
                descriptions.push(format!("identifier: {ident}"));
            }
            TokenTree::Punct(punct) => {
                descriptions.push(format!("punctuation: {}", punct.as_char()));
            }
            TokenTree::Literal(literal) => {
                descriptions.push(format!("literal: {literal}"));
            }
        }
    }
}

/// Counts identifiers in a token stream, including identifiers in groups.
#[cfg(test)]
fn identifier_count(stream: TokenStream) -> usize {
    let mut count = 0;
    for token in stream {
        match token {
            TokenTree::Group(group) => {
                count += identifier_count(group.stream());
            }
            TokenTree::Ident(_) => count += 1,
            TokenTree::Punct(_) | TokenTree::Literal(_) => {}
        }
    }
    count
}

/// Builds a small token stream from individual token-tree values.
fn build_stream() -> TokenStream {
    let identifier = Ident::new("answer", Span::call_site());
    let equals = Punct::new('=', Spacing::Alone);
    let value = Literal::i32_unsuffixed(42);
    [
        TokenTree::Ident(identifier),
        TokenTree::Punct(equals),
        TokenTree::Literal(value),
    ]
    .into_iter()
    .collect()
}

/// Prints parsed and manually constructed token streams.
///
/// # Errors
///
/// Returns an error if the example source cannot be parsed.
fn main() -> Result<()> {
    let stream = parse_stream("fn hello() { let answer = 42; }")?;
    let mut descriptions = Vec::new();
    describe_tokens(stream, &mut descriptions);
    println!("{}", descriptions.join("\n"));
    println!("Manually built: {}", build_stream());
    Ok(())
}
// ANCHOR_END: example

/// Runs the `proc_macro2` token-stream example.
///
/// # Errors
///
/// Returns an error if the example source cannot be parsed.
pub fn run() -> Result<()> {
    main()
}

/// Verifies identifier counting traverses nested token groups.
///
/// # Errors
///
/// Returns an error if the test source cannot be parsed.
#[test]
fn counts_identifiers_inside_groups() -> Result<()> {
    let stream = parse_stream("fn hello() { let answer = 42; }")?;
    assert_eq!(identifier_count(stream), 4);
    Ok(())
}

/// Rejects an unbalanced token group with a parse error.
#[test]
fn rejects_malformed_input() {
    assert!(parse_stream("fn hello( {").is_err());
}

/// Verifies that manually constructed token trees form valid Rust tokens.
#[test]
fn builds_a_token_stream() {
    assert_eq!(build_stream().to_string(), "answer = 42");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
