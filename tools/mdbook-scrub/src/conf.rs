// TODO Reviewed ??-??-2026
use std::default::Default;

use serde::Deserialize;
use serde::Serialize;

/// Settings for removing hidden sections and suppressing hidden chapter includes.
///
/// Missing fields use the values from the `Default` implementation during deserialization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct PreprocConfig {
    /// Whether HTML elements with the `hidden` class are removed from the book.
    pub remove_hidden_sections: bool,
    /// Whether includes whose paths start with `hidden_chapter_prefix` are skipped.
    pub do_not_include_hidden_chapters: bool,
    /// Prefix that marks hidden chapter files; defaults to `_`.
    pub hidden_chapter_prefix: String,
}

impl Default for PreprocConfig {
    fn default() -> Self {
        Self {
            remove_hidden_sections: true,
            do_not_include_hidden_chapters: true,
            hidden_chapter_prefix: "_".into(),
        }
    }
}

/// Settings for in-memory custom directive processing.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct DirectivePreprocConfig {
    /// Directive values parsed from the preprocessor configuration table.
    #[serde(flatten)]
    pub directives: book_tool::directives::Config,
}
