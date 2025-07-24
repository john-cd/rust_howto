//! Process {{#example some_example}} directives.

use core_lib::RegexAndReplacement;
use regex::Regex;

/// The {{#example some_example}} should be replaced by an {{#include ...}} directive
/// that embeds the contents of the file `some_example.md` in the proper `crates` subfolder.
pub(super) fn example_regexes() -> Vec<RegexAndReplacement> {
    let re_string: String = r"\{\{\s*#\s*example\s*:?\s+([^}]+)\s*\}\}".into();
    let re = Regex::new(&re_string).expect("Invalid regex");
    let replacement = "$1"; // FIXME
    vec![RegexAndReplacement {
        re,
        replacement: Some(Box::new(|_| replacement.into())),
    }]
}

// The following variations should be accepted:
//
// {{#example some_example}}
// {{#example some_example }}
// {{# example some_example}}
// {{ #example some_example}}
//
// The following are incomplete directives:
//
// {{#example}}
// {{#example }}
#[cfg(test)]
mod tests {
    // use super::*;

    // // TODO finish  tests
    // #[test]
    // fn test_replace_example() {
    //     let text = "{{#example some_example}}";
    //     let expected = "some_example";
    //     let r = example_regexes();
    //     let res = r[0]
    //         .re
    //         .replace_all(text, r[0].replacement.as_ref().unwrap());
    //     assert_eq!(res, expected);
    // }
}
