//! Parsers for Markdown and related elements.
//!
//! The parsers do not cover all corner cases of the CommonMark spec, just what we need.

// TODO
#![allow(unused)]
#![allow(dead_code)]

pub mod ast;
pub mod directives;
pub mod errors;
pub mod hidden;
pub mod markdown;
pub mod parts;
pub mod urls;
// pub mod wikilinks;
