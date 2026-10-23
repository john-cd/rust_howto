//! The objects that make up the book.
//!
//! TODO Reviewed ??-??-2026.

mod chapter;
mod exampleindex;
mod recipe;
mod renderable;
mod subchapter;

pub use chapter::*;
pub use exampleindex::*;
pub use recipe::*;
pub use renderable::*;
pub use subchapter::*;

/// Represents the different types of objects that can be rendered in the book.
pub enum RenderableObjects {
    /// A chapter.
    Chapter(Chapter),
    /// A subchapter.
    Subchapter(Subchapter),
    /// The index of the examples.
    ExampleIndex(ExampleIndex),
}
