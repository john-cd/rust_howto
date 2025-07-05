//! {{#example some_example}}
//! {{#example some_example }}
//! {{# example some_example}}
//! {{ #example some_example}}
//!
//! Incomplete directives:
//!
//! {{#example}}
//! {{#example }}

// TODO

use regex::Regex;

use super::common::RegexAndReplacement;

/// Example: {{#example some_example}}
///
/// This directive is replaced by the contents of the file `some_example.md`.
///
pub fn example_regexes() -> Vec<RegexAndReplacement> {
    let re_string: String = r"\{\{\s*#\s*example\s*:?\s+([^}]+)\s*\}\}".into();
    let re = Regex::new(&re_string).expect("Invalid regex");
    let replacement = "$1"; // TODO
    vec![RegexAndReplacement {
        re,
        replacement: Some(Box::new(|_| replacement.into())),
    }]
}
