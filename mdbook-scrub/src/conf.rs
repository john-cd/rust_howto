use std::default::Default;

use serde::Deserialize;
use serde::Serialize;

// Configuration struct for the preprocessor
// When deserializing, any missing fields are filled in from the struct's
// implementation of Default. Only allowed on structs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "kebab-case")]
pub struct PreprocConfig {
    // Remove all markdown within HTML tags with class `hidden` from the book
    // (default = true).
    pub remove_hidden_sections: bool,
    // Do not {{#include }} hidden files i.e. files that start with
    // `hidden_chapter_prefix` (default = true).
    pub do_not_include_hidden_chapters: bool,
    // Define the prefix for hidden chapters (default = '_').
    pub hidden_chapter_prefix: String,
    // Remove any left-over {{#example }} directives and log a warning (default = true).
    // {{#example }} is a custom directive for this book.
    pub scrub_example_directives: bool,
    // Remove any left-over {{#crate }} directives and log a warning (default = true).
    // {{#crate }} is a custom directive for this book.
    pub scrub_crate_directives: bool,
    // Remove any left-over [[file | title]] wikilinks and log a warning (default = true).
    pub scrub_wikilinks: bool,
}

impl Default for PreprocConfig {
    fn default() -> Self {
        Self {
            remove_hidden_sections: true,
            do_not_include_hidden_chapters: true,
            hidden_chapter_prefix: "_".into(),
            scrub_example_directives: true,
            scrub_crate_directives: true,
            scrub_wikilinks: true,
        }
    }
}
