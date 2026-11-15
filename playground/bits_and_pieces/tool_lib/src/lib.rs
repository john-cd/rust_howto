//! A library that models the book and renders it with templates.
//!
//! TODO Reviewed ??-??-2026.

/// The data model of the book and its crates.
mod model;

/// Rendering objects with the Tera template engine.
mod tera;

pub use model::*;
pub use tera::*;
