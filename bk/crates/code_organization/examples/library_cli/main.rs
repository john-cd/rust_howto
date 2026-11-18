//! A thin binary that delegates its work to the package's library.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
/// Interprets the optional CLI name before delegating to library formatting.
mod cli;

use code_organization::format_welcome;

/// Reads CLI input and delegates formatting to the library.
fn main() {
    println!("{}", render(std::env::args().nth(1)));
}

/// Formats a greeting for the name supplied by the CLI.
fn render(argument: Option<String>) -> String {
    format_welcome(&cli::name(argument))
}
// ANCHOR_END: example

/// The CLI supplies either the requested name or a library default.
#[test]
fn renders_explicit_and_default_greetings() {
    assert_eq!(render(Some("Ada".into())), "Hello, Ada!");
    assert_eq!(render(None), "Hello, reader!");
}
