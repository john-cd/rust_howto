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

///
/// {{cat parsing }} -> [parsing][cat~parsing]⮳{{hi:parsing}}
pub fn example_regexes() -> Vec<RegexAndReplacement> {
    let re_string: String = r"\{\{\s*#\s*example\s*:?\s+([^}]+)\s*\}\}".into();
    let re = Regex::new(&re_string).expect("Invalid regex");
    let replacement = "$1"; // TODO
    vec![RegexAndReplacement {
        re,
        replacement: Some(Box::new(|_| replacement.into())),
    }]
}
