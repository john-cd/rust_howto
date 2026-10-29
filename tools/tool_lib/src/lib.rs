//! Library used by all CLI tools (`tools` folder)
//!
//! TODO Reviewed ??-??-2026.

mod all_categories;
mod cargo_toml;
mod crates_io;
mod model;
mod read_write;
mod templates;
mod utils;

/// Serializes tests that mutate process-wide environment variables.
#[cfg(test)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub use all_categories::*;
pub use cargo_toml::*;
pub use crates_io::*;
pub use model::*;
pub use read_write::*;
pub use templates::*;
