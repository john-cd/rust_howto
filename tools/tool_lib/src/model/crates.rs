use serde::Serialize;

use super::Category;

#[derive(Serialize, Debug)]
pub struct Crate {
    name: String,
    repo_url: String, // Github url
    categories: Vec<Category>,
}

impl Crate {
    fn new(name: String, repo_url: String) -> Self {
        Self {
            name,
            repo_url,
            categories: Vec::new(),
        }
    }
}
