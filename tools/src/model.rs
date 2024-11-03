#![allow(dead_code)]

use serde::Serialize;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Serialize)]
pub struct Category {
    pub category: String,
    pub slug: String,
    pub description: String,
}

// impl Category {
//     fn builder(category: String, slug: String, description: String) -> Self {
//         Self {
//             category,
//             slug,
//             description,
//         }
//     }
// }

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct Recipe {
    name: String,
    crates: Vec<Crate>,
}

impl Recipe {
    fn new(name: String) -> Self {
        Self {
            name,
            crates: Vec::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Subchapter {
    recipes: Vec<Recipe>,
}

impl Subchapter {
    fn new() -> Self {
        Self {
            recipes: Vec::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Chapter {
    description: String,
    subchapters: Vec<Subchapter>,
}

impl Chapter {
    fn new(description: String) -> Self {
        Self {
            description,
            subchapters: Vec::new(),
        }
    }
}
