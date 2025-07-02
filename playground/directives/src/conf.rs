use std::default::Default;

use serde::Deserialize;
use serde::Serialize;

// Configuration struct
// When deserializing, any missing fields are filled in from the struct's
// implementation of Default. Only allowed on structs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
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

impl Default for Config {
    fn default() -> Self {
        Self {
            process_crate_directives: true,
            process_category_directives: true,
            process_page_directives: true,
        }
    }
}
