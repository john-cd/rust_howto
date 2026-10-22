//! A subchapter of the book.
//!
//! TODO Reviewed ??-??-2026

use serde::Serialize;

use super::*;

/// A subchapter, made of recipes.
#[derive(Serialize, Debug)]
pub struct Subchapter {
    /// The recipes of the subchapter.
    recipes: Vec<Recipe>,
}

impl Renderable for Subchapter {
    /// Returns the information needed to render this subchapter.
    fn get_what_to_render(&self) -> Vec<RenderInfo> {
        vec![RenderInfo::new(
            "chapter/subchapter.md".into(),
            "{{}}".into(),
        )]
    }

    /// Returns the children of this subchapter.
    fn get_children(&self) -> impl IntoIterator<Item = impl Renderable> {
        std::iter::empty::<Render>()
    }
}

impl Subchapter {
    /// Creates a new subchapter.
    fn new(recipes: Vec<Recipe>) -> Self {
        Self { recipes }
    }
}
