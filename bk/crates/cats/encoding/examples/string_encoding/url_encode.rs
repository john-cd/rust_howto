// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to URL-encode and decode strings using the
//! `url` crate.

use url::form_urlencoded::Serializer;
use url::form_urlencoded::parse;

/// Encodes bytes in the URL-encoded form format, then decodes them.
fn main() {
    let urlencoded = Serializer::new(String::new())
        .append_pair("message", "What is ❤?")
        .finish();
    assert_eq!(urlencoded, "message=What+is+%E2%9D%A4%3F");
    println!("urlencoded: {urlencoded}");

    let decoded: Vec<(String, String)> = parse(urlencoded.as_bytes())
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    assert_eq!(decoded, [("message".to_owned(), "What is ❤?".to_owned())]);
    println!("decoded: {decoded:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the url encode example through its entry point.
    #[test]
    fn test_url_encode_example_entry_point() {
        main();
    }
}
