use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::default::Default;

// Configuration struct for the preprocessor
// When deserializing, any missing fields are filled in from the struct's implementation of Default. Only allowed on structs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "kebab-case")]
pub struct PreprocConfig {
    // Remove all markdown within HTML tags with class `hidden` from the book (default = true)
    pub remove_hidden_sections: bool,
    // Do not include hidden files i.e. files that start with  `hidden_chapter_prefix` (default = true)
    pub do_not_include_hidden_chapters: bool,
    // Define the prefix for hidden chapters (default = '_')
    pub hidden_chapter_prefix: String,
    // // FUTURE
    // // Check internal URLs (and external URLs if check_external_urls = true) (default = true)
    // pub check_urls: bool,
    // // Check whether external URLs (e.g. to http://... or https://...) are valid; default: false.
    // // Has no effect if check_urls = false
    // // Beware of the performance penalty when true.
    // pub check_external_urls: bool,
    // // If true (default), detect e.g. [ref]: http://example.com without a corresponding [some_text][ref]
    // pub detect_unused_reference_definitions: bool,
    // // Remove unused ref defs from the markdown, if true (default)
    // pub delete_unused_reference_definitions: bool,
}

impl Default for PreprocConfig {
    fn default() -> Self {
        Self {
            remove_hidden_sections: true,
            do_not_include_hidden_chapters: true,
            hidden_chapter_prefix: "_".into(),
            // check_urls: true,
            // check_external_urls: false,
            // detect_unused_reference_definitions: true,
            // delete_unused_reference_definitions: true,
        }
    }
}
