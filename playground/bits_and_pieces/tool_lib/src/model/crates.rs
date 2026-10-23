//! Crates and their metadata.
//!
//! TODO Reviewed ??-??-2026.

use serde::Serialize;

use super::Category;

#[derive(Serialize, Debug)]
/// Represents a Rust crate with its name, (GitHub) repository URL, and associated categories.
pub struct Crate {
    /// Name of the crate.
    name: String,
    /// URL of the GitHub repository.
    repo_url: String, // Github url
    /// Categories of the crate on `crates.io`.
    categories: Vec<Category>,
}

/// Implementation block for the `Crate` struct.
impl Crate {
    /// Creates a crate without categories.
    fn new(name: String, repo_url: String) -> Self {
        Self {
            name,
            repo_url,
            categories: Vec::new(),
        }
    }
}
