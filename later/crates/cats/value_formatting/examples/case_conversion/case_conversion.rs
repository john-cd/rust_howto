//! Demonstrates identifier case conversion with heck.

use heck::ToUpperCamelCase as _;

// ANCHOR: example
/// Converts a Rust-style identifier to upper camel case.
fn main() {
    assert_eq!("http_client".to_upper_camel_case(), "HttpClient");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that an underscore-separated identifier becomes upper camel
    /// case.
    #[test]
    fn converts_snake_case_to_upper_camel_case() {
        main();
    }
}
