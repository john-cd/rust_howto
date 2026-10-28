//! A chapter of the book.
//!
//! TODO Reviewed ??-??-2026.



use serde::Serialize;

use super::*;

/// A chapter of the book, made of subchapters.
#[derive(Serialize, Debug)]
pub struct Chapter {
    /// Description of the chapter.
    description: String,
    /// The subchapters of the chapter.
    subchapters: Vec<Subchapter>,
}

// impl Renderable for Chapter {
//     fn get_what_to_render(&self) -> Vec<RenderInfo> {
//         vec![ RenderInfo::new("chapter/index.md".into(),
// "categories/{{chapter}}".into()),         RenderInfo::new("chapter/index.
// incl.md".into(), "categories/{{chapter}}".into()),         RenderInfo::new("
// chapter/refs.incl.md".into(), "categories/{{chapter}}".into()) ]     }
//     fn get_children(&self) -> impl IntoIterator<Item = impl Renderable> {
//         self.subchapters.iter()
//     }
// }

impl Chapter {
    /// Creates a new `Chapter` instance.
    ///
    /// # Arguments
    ///
    /// * `description` - The description of the chapter.
    /// * `subchapters` - A vector of subchapters within this chapter.
    fn new(description: String, subchapters: Vec<Subchapter>) -> Self {
        Self {
            description,
            subchapters,
        }
    }
}
