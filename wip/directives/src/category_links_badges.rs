//! Process directives related to categories:
//!
//! Category links:
//!
//! {{cat xyz}}
//!
//! Variants:
//! {{cat xyz }}
//! {{ cat xyz}}
//! {{cat: xyz}}
//! {{cat : xyz}}
//! {{cat x-y_z::a-b_c }}
//!
//! Example: {{cat: accessibility}} -> [Accessibility][cat~accessibility]⮳{{hi:accessibility}}
//!
//! Category Links with Title:
//!
//! {{cat xyz | XYZ }}
//! {{cat development-tools::testing | Testing }} -> [Testing][cat~development-tools::testing]⮳{{hi:development-tools::testing}}
//!
//! Category links:
//!
//! {{!cat mathematics}} -> [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}
//! {{!cat no-std}} -> [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
//! {{!cat no_std}}
//! {{!cat mathematics }}
//! {{ !cat mathematics}}
//! {{!cat: mathematics}}
//! {{!cat : mathematics}}
//!
//! Category Badges with Multiple categories:
//!
//! {{!cat mathematics science }}

use regex::Regex;

use super::common::RegexAndReplacement;

///
/// {{cat parsing }} -> [parsing][cat~parsing]⮳{{hi:parsing}}
fn category_link_regexes() -> Vec<RegexAndReplacement> {
    let re_string: String = r"\{\{\s*cat\s*:?\s+([^}]+)\s*\}\}".into();
    let re = Regex::new(&re_string).expect("Invalid regex");
    let replacement = "[$1][cat~$1]{{hi: $1}}";
    vec![RegexAndReplacement {
        re,
        replacement: Some(Box::new(|_| replacement.into())),
    }]
}

///
/// {{!cat parsing }} -> [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
fn category_badge_regexes() -> Vec<RegexAndReplacement> {
    let re_string: String = r"\{\{\s*!\s*cat\s*:?\s+([^}]+)\s*\}\}".into();
    let re = Regex::new(&re_string).expect("Invalid regex");
    let replacement = "[![cat~$1][cat~$1~badge]][cat~$1]{{hi:$1}}";
    vec![RegexAndReplacement {
        re,
        replacement: Some(Box::new(|_| replacement.into())),
    }]
}
