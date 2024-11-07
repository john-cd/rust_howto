//! Library used by all CLI tools located in the `bin` folder
mod crates_io;
mod model;
mod templates;
pub use crates_io::*;
pub use model::*;
pub use templates::*;
