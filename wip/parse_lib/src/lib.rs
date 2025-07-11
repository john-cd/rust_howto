//! Parsers for Markdown and related elements.
//!
//! The parsers do not cover all corner cases of the CommonMark spec, just what we need.

mod ast;
mod directives;
mod errors;
mod hidden;
mod markdown;
mod parts;
mod urls;
// mod wikilinks;
mod document;

pub use ast::*;
pub use document::parse_document;
pub use errors::*;
