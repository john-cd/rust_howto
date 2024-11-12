use serde::Serialize;

use super::*;

#[derive(Serialize, Debug)]
pub struct Subchapter {
    recipes: Vec<Recipe>,
}

// impl Renderable for Subchapter {
//     fn get_templates() -> Vec<String> {
//         vec!["chapter/subchapter.md".into()]
//     }
// }

impl Subchapter {
    fn new(recipes: Vec<Recipe>) -> Self {
        Self { recipes }
    }
}
