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
    // (default = true)
    pub remove_hidden_sections: bool,
    // Do not {{#include }} hidden files i.e. files that start with
    // `hidden_chapter_prefix` (default = true)
    pub do_not_include_hidden_chapters: bool,
    // Define the prefix for hidden chapters (default = '_')
    pub hidden_chapter_prefix: String,
    // Remove any left-over {{#example }} directives and log a warning
    // {{#example }} is a custom directive for this book
    pub scrub_example_directives: bool,
    // Remove any left-over {{#crate }} directives and log a warning
    // {{#crate }} is a custom directive for this book
    pub scrub_crate_directives: bool,
    // Remove any left-over [[file | title]] wikilinks and log a warning
    pub scrub_wikilinks: bool,
    // FIXME
    // Convert {{c: <crate name> }} into links
    pub process_crate_directives: bool,
    // Convert category directive {{cat: <category> <optional categories>.. }}
    // into links
    pub process_category_directives: bool,
    // Convert page directive {{p: <page/chapter_name>}} into a link
    pub process_page_directives: bool,
    // // Convert {{crate: <crate name> <extra categories> }} into Markdown
    // // links to the crate and its categories
    // pub process_crate_block_directives: bool,

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
            // FIXME
            process_crate_directives: true,
            process_category_directives: true,
            process_page_directives: true,
        }
    }
}
