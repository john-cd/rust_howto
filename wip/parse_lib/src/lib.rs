//! Parsers for Markdown and related elements.
//!
//! The parsers do not cover all corner cases of the CommonMark spec, just what we need.


// A nom parser has the following signature:
//
// `fn parser(input: I) -> Result<I, O, E>;`
//
// with `Result` defined as:
//
// ```rust
// pub type Result<I, O, E=(I,ErrorKind)> = Result<(I, O), Err<E>>;

// ```


// Combinators like `take_while` return a function. That function is a
// parser, to which we can pass the input.

pub mod ast;
pub mod directives;
pub mod hidden;
pub mod markdown;
pub mod parts;
pub mod urls;
pub mod utils;
// pub mod wikilinks;
