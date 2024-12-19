use serde::Serialize;

use super::*;

#[derive(Serialize, Debug)]
pub struct Subchapter {
    recipes: Vec<Recipe>,
}

impl Renderable for Subchapter {
    fn get_what_to_render(&self) -> Vec<RenderInfo> {
        vec![RenderInfo::new(
            "chapter/subchapter.md".into(),
            "{{}}".into(),
        )]
    }

    fn get_children(&self) -> impl IntoIterator<Item = impl Renderable> {
        std::iter::empty::<Render>()
    }
}

impl Subchapter {
    fn new(recipes: Vec<Recipe>) -> Self {
        Self { recipes }
    }
}