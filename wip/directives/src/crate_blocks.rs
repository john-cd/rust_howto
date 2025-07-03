// {{#crate crt}}
// {{#crate crt }}
// {{#crate: crt}}
// {{#crate : crt}}
// {{#crate x_y-z}}
// ## Crate Blocks with Additional Categories
// {{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}

use regex::Regex;
use regex::bytes::Captures;

use super::common::RegexAndReplacement;

// // TODO

// ///
// /// {{cat parsing }} -> [parsing][cat~parsing]⮳{{hi:parsing}}
// pub fn crate_block_regexes() -> Vec<RegexAndReplacement> {
//     let re_string: String = r"\{\{\s*#crate\s*:?\s+([^}]+)\s*\}\}".into();
//     let re = Regex::new(&re_string).expect("Invalid regex");
//     let replacement = "$1"; // TODO
//     vec![RegexAndReplacement {
//         re,
//         replacement: Some(Box::new(replace)),
//     }]
// }

// fn replace<'a, 'b>(caps: &'a Captures<'b>) -> String {
//     format!("{} {}", &caps[2], &caps[1])
// }
