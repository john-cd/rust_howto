//! Process wikilinks in the markdown:
//!
//!  [[file]]
//! [[ file ]]
//! [[ file |]]
//! [[ file |  ]]
//! [[ file | title1 ]]
//! [[file|title2]]
//! [[ file|title3]]
//! [[file |title4]]
//! [[file| title5]]
//! [[file|title6 ]]
//!
//! Bad Wikilinks:
//!
//! [[]]
//! [[|title7]]
//! [[| title8]]
//! [[|title9 ]]
//! [[| title10 ]]

// TODO
use core_lib::RegexAndReplacement;
use regex::Regex;

//         let re_string: String = r"\s*(\S+)\s*".into();
//         let re = Regex::new(&re_string).expect("Invalid regex");
//         let replacement = "[$1][p~$1]{{hi:$1}}";
//         rr.push(RegexAndReplacement {
//             re,
//             replacement: Some(Box::new(|_| replacement.into())),
//         });

pub fn wikilink_regexes() -> Vec<RegexAndReplacement> {
    let re_string: String = r"\[\[\s*(\S+)\s*(\|\s*([^}]+)\s*)?\]\]".into();
    let re = Regex::new(&re_string).expect("Invalid regex");
    let replacement = "[$3][p~$1]";
    vec![RegexAndReplacement {
        re,
        replacement: Some(Box::new(|_| replacement.into())),
    }]
}
