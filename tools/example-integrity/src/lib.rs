//! Read-only chapter, include, and example integrity inventory.
//!
//! TODO Reviewed ??-??-2026.

/// Repository-wide chapter, include, and Rust target integrity checks.
mod audit;

/// Stable, path-aware report records for the integrity inventory.
mod model;

pub use audit::build_report;
pub use model::Report;
