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
    Chapter(Chapter),
    Subchapter(Subchapter),
    ExampleIndex(ExampleIndex),
}
// TODO finish; unit tests
