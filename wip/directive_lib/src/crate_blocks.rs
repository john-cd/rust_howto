//! Process crate block directives e.g. `{{#crate crt}}`
//!
//! Crate blocks combine multiple crate and categories badges.

use core_lib::RegexAndReplacement;
use regex::Regex;

// [finish](https://github.com/john-cd/rust_howto/issues/1438)

/// {{cat parsing }} -> [parsing][cat~parsing]⮳{{hi:parsing}}
pub(super) fn crate_block_regexes() -> Vec<RegexAndReplacement> {
    let re_string: String = r"\{\{\s*#crate\s*:?\s+([^}]+)\s*\}\}".into();
    let re = Regex::new(&re_string).expect("Invalid regex");
    let replacement = "$1"; // FIXME call create_crate_badges_or_refdefs
    vec![RegexAndReplacement {
        re,
        replacement: Some(Box::new(|_| replacement.into())), // FIXME
    }]
}

// // The following variations should be accepted:
// // {{#crate crt}}
// // {{#crate crt }}
// // {{#crate: crt}}
// // {{#crate : crt}}
// // {{#crate x_y-z}}
// //
// // * Crate Blocks with Additional Categories
// //
// // {{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // FIXME
//     #[test]
//     fn test_crate_block_regexes()() {
//         let text = "{{#crate crt}}";
//         let expected = "crt";
//         let r = crate_block_regexes();
//         let res = r[0].re.replace_all(text, r[0].replacement.as_ref().unwrap());
//         assert_eq!(res, expected);
//     }
// }
