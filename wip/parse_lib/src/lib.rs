//! Parsers for Markdown and related elements.
//!
//! The parsers do not cover all corner cases of the CommonMark spec, just what we need.

mod ast;
mod parse_directives;
mod errors;
mod parse_hidden;
mod markdown;
mod parse_parts;
mod parse_urls;
// mod wikilinks;
mod parse_document;

pub use ast::*;
pub use parse_document::parse_document;
pub use errors::*;
