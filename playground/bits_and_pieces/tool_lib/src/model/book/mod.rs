//! The objects that make up the book.
//!
//! TODO Reviewed ??-??-2026.

/// A chapter of the book.
mod chapter;
/// The index of the examples of a book.
mod exampleindex;
/// A recipe, a list of crates that work together.
mod recipe;
/// Converts book state into the layout and presentation consumed by the user interface. The view
/// layer calls these routines when it draws or refreshes the component.
mod renderable;
/// A subchapter of the book.
mod subchapter;

pub use chapter::*;
pub use exampleindex::*;
pub use recipe::*;
pub use renderable::*;
pub use subchapter::*;

/// Represents the different types of objects that can be rendered in the book.
/// Defines the outcomes available to the book workflow, including chapter, subchapter, example
/// index.
pub enum RenderableObjects {
    /// A chapter.
    Chapter(Chapter),
    /// A subchapter.
    Subchapter(Subchapter),
    /// The index of the examples.
    ExampleIndex(ExampleIndex),
}
