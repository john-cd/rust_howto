//! Checks generated book pages and visible authored Markdown.
//!
//! TODO Reviewed ??-??-2026.

/// Validates rendered chapter links, previews, word-index fragments, and Playground imports.
mod rendered;

/// Checks the generated mdBook search index.
mod search_index;

/// Finds visible TODO placeholders in published source.
mod visible_todos;

pub use rendered::check_playground;
pub use rendered::check_preview_links;
pub use rendered::check_rendered_links;
pub use rendered::check_word_index;
pub use search_index::SearchIndexReport;
pub use search_index::check_search_index;
pub use visible_todos::TodoFinding;
pub use visible_todos::check_visible_todos;
