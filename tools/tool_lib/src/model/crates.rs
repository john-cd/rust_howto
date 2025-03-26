use serde::Serialize;

use super::Category;

#[derive(Serialize, Debug)]
/// Represents a Rust crate with its name, (GitHub) repository URL, and associated categories.
pub struct Crate {
    name: String,
    repo_url: String, // Github url
    categories: Vec<Category>,
}

/// Implementation block for the `Crate` struct.
impl Crate {
    fn new(name: String, repo_url: String) -> Self {
        Self {
            name,
            repo_url,
            categories: Vec::new(),
        }
    }
}
