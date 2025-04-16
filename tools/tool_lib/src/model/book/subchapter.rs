use serde::Serialize;

use super::*;

#[derive(Serialize, Debug)]
pub struct Subchapter {
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
// [finish; unit tests](https://github.com/john-cd/rust_howto/issues/1365)
