//! Library used by all CLI tools located in the `bin` folder
mod all_categories;
mod crates_io;
mod model;
mod templates;
pub use all_categories::*;
pub use crates_io::*;
pub use model::*;
pub use templates::*;
