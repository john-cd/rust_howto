//! Parsers for Markdown and related elements.
//!
//! The parsers do not cover all corner cases of the CommonMark spec, just what we need.

mod ast;
mod errors;
mod markdown;
mod parse_directives;
mod parse_document;
mod parse_hidden;
mod parse_parts;
mod parse_urls;
mod parse_wikilinks;

pub use ast::*;
pub use errors::*;
pub use parse_document::parse_document;
