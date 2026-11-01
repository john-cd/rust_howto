//! Demonstrates identifier case conversion with heck.
//!
//! Naming styles: `snake_case`, `camelCase`, `PascalCase` and `kebab-case`.
//! Converting between them helps when mapping names between languages or file
//! formats, for example a JSON API that uses camelCase to Rust fields.

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
