//! A `crates.io` category.
//!
//! TODO Reviewed ??-??-2026

use serde::Serialize;

/// Represents a category.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Debug)]
pub struct Category {
    /// The name of the category.
    pub category: String,
    /// A URL-friendly slug for the category.
    pub slug: String,
    /// The description of the category.
    pub description: String,
}
