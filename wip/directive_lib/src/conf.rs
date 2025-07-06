use std::default::Default;

use serde::Deserialize;
use serde::Serialize;

// Configuration struct.
// When deserializing, any missing fields are filled in from the struct's
// implementation of `Default`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    /// If true, process crate block directives e.g.
    /// `{{#crate my_crate}}`
    /// into several badges for the crate, its categories, etc...
    pub process_crate_block_directives: bool,

    /// If true, process example directives, e.g.
    /// `{{#example some_example}}`
    /// into a fenced code block that includes an example file.
    pub process_example_directives: bool,

    /// If true, convert crate-level links and badges directives, e.g.,
    /// `{{crate xyz}}`, `{{docs xyz}}`, `{{github xyz}}`, `{{lib.rs xyz}}`,
    /// `{{crates.io xyz}}`, `{{web xyz}}`
    /// and
    /// `{{!crate xyz}}`, etc.
    ///  into Markdown links to or badges for the crate's corresponding web pages.
    ///
    /// Also convert category links and badges directives, e.g.,
    ///   `{{cat: accessibility}}`
    ///   `{{!cat mathematics}}`
    /// into links to or badges for the category.
    pub process_link_and_badge_directives: bool,

    /// If true, process wikilinks, e.g.
    /// `[[file | title1]]`
    /// and convert them to markdown links to internal pages.
    pub process_wikilinks: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            process_crate_block_directives: true,
            process_example_directives: true,
            process_link_and_badge_directives: true,
            process_wikilinks: true,
        }
    }
}
