//! Library used by all CLI tools (`tools` folder)
//!
//! TODO Reviewed ??-??-2026.

/// Getting the list of categories of `crates.io`.
mod all_categories;
/// Read Cargo.toml files and extract dependencies
mod cargo_toml;
/// Returns information / list of categories for a given crate, using the crates.io API
mod crates_io;
/// The data model.
mod model;
/// Reading, writing and backing up files.
mod read_write;
/// Build badges, index of the book, etc...
mod templates;
/// Utilities for lazily compiled regular expressions.
mod utils;

pub use all_categories::*;
pub use cargo_toml::*;
pub use crates_io::*;
pub use model::*;
pub use read_write::*;
pub use templates::*;
