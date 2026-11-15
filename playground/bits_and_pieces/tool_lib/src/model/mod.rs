//! The data model of the book and its crates.
//!
//! TODO Reviewed ??-??-2026.

/// The objects that make up the book.
pub mod book;

/// Crates and their metadata.
mod crates;

pub use crates::*;
