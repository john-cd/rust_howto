use serde::Serialize;

/// Represents a category.
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Debug)]
pub struct Category {
    /// The name of the category.
    pub category: String,
    /// A URL-friendly slug for the category.
    pub slug: String,
    /// The description of the category.
    pub description: String,
}
