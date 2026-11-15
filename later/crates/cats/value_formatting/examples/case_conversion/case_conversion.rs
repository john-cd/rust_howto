// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates identifier case conversion with heck.
//!
//! Naming styles: `snake_case`, `camelCase`, `PascalCase` and `kebab-case`.
//! Converting between them helps when mapping names between languages or file
//! formats, for example a JSON API that uses camelCase to Rust fields.

use heck::ToUpperCamelCase as _;
/// Demonstrates identifier case conversion with heck.
/// Converts a Rust-style identifier to upper camel case.
fn main() {
    assert_eq!("http_client".to_upper_camel_case(), "HttpClient");
}
// ANCHOR_END: example

/// Demonstrates identifier case conversion with heck.
pub fn run() {
    main();
}

/// case.
#[cfg(test)]
mod tests {
    use super::*;

    /// case.
    #[test]
    fn converts_snake_case_to_upper_camel_case() {
        main();
    }
}
