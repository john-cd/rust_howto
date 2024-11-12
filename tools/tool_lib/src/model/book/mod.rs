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

pub enum RenderableObjects {
    Chapter(Chapter),
    Subchapter(Subchapter),
    ExampleIndex(ExampleIndex),
}

// TODO
// impl Renderable for RenderableObjects {
// }
