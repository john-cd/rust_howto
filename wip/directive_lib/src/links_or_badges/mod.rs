//! Process links and badge directives.
//!
//! * Crate links:
//!
//! - Internal crate page: {{crate xyz}}
//! - Variations:
//!   - {{ crate xyz}}
//!   - {{crate xyz }}
//!   - {{crate: xyz}}
//!   - {{crate : xyz}}
//!   - {{crate x_y-z}}
//! - `docs.rs` link: {{docs xyz}}
//! - Github link: {{github xyz}}
//! - `lib.rs` link: {{lib.rs xyz}}
//! - `crates.io` link: {{crates.io xyz}}
//! - Website for the crate: {{web xyz}}
//!
//! * Category links:
//!
//! {{cat: accessibility}} -> [Accessibility][cat~accessibility]↗{{hi:accessibility}}
//!
//! * Crate badges:
//!
//!- Internal crate page: {{!crate xyz}}
//!   - {{!crate xyz}}
//!   - {{!crate xyz }}
//!   - {{ ! crate xyz }}
//!   - {{!crate: x_y-z}}
//!   - {{!crate : x_y-z}}
//! - {{!docs xyz}}
//! - {{!github xyz}}
//! - {{!lib.rs xyz}}
//! - {{!crates.io xyz}}
//! - {{!web xyz}}
//!
//! * Category badges:
//!
//! {{!cat mathematics}} -> [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}
//! {{!cat no-std}} -> [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

#[cfg(test)]
mod category_links_badges_tests;

use core_lib::RegexAndReplacement;
use regex::Captures;
use regex::Regex;

use super::Directive;

/// Build the Regex for link or badge directives (both for categories and crates).
pub(super) fn link_or_badge_regexes() -> Vec<RegexAndReplacement> {
    let re_string: String =
        r"\{\{\s*(!?)\s*(cat|docs|github|lib\.rs|crates\.io|web|crate)\s*:?\s+([^}]+)\s*\}\}"
            .into();
    let re = Regex::new(&re_string).expect("Invalid regex");
    let replacement = "$1"; // TODO call function in template.
    vec![RegexAndReplacement {
        re,
        replacement: Some(Box::new(|_| replacement.into())),
    }]
}

/// Extract directive from the Regex captures.
pub fn extract_directive<'a>(caps: &'a Captures<'a>) -> Directive<'a> {
    // let whole_matching_string = caps.get(0).map_or("", |m| m.as_str()).to_string();
    let is_badge = caps.get(1).map(|m| m.as_str()) == Some("!");
    let kind_str = caps
        .get(2)
        .map(|m| m.as_str())
        .expect("There is always a string.");
    let name = caps
        .get(3)
        .map(|m| m.as_str())
        .expect("There is always a string.");

    use super::DestinationKind::*;
    let kind = match kind_str {
        "cat" => Category,
        "crate" => Crate,
        "docs" => Docs,
        "github" => GithubRepo,
        "lib.rs" => LibRs,
        "crates.io" => CratesIo,
        "web" => Web,
        _ => unreachable!(),
    };

    if is_badge {
        Directive::Badge {
            kind,
            name: name.trim(),
        }
    } else {
        Directive::Link {
            kind,
            name: name.trim(),
        }
    }
}

// [impl From?](https://github.com/john-cd/rust_howto/issues/1436)
// impl From<&str> for Directive<'_> {
//    fn from(i: &str) -> Self {}
// }
