use regex::Captures;
use regex::Regex;

use super::conf::PreprocConfig;

// Replacement closure
type Replacement = Box<dyn Fn(&Captures) -> String>;

pub struct RegexAndReplacement {
    pub re: Regex,
    // Most often set to None for no replacement, meaning deletion
    pub replacement: Option<Replacement>,
}

// Generate the replacement Regexes needed, depending on the configuration
// Can return an empty Vec.
pub fn get_regexes_and_replacements(
    conf: &PreprocConfig,
) -> Vec<RegexAndReplacement> {
    let mut rr = vec![];
    if conf.remove_hidden_sections {
        let re =
            Regex::new(r#"<div *class *= *"hidden">([^<]|<[^/])+</ *div *>"#)
                .expect("Invalid regex");
        rr.push(RegexAndReplacement {
            re,
            replacement: None,
        });
    }
    if conf.do_not_include_hidden_chapters {
        let re_string: String =
            r"[{]{2} *#(include|rustdoc_include|playground) *".to_string()
                + &conf.hidden_chapter_prefix.clone()
                + r"[^}]*?[}]{2}";
        // { and } are special for Regex, thus must be escaped with \ except if
        // within []
        // We don't use format! here, since { and } are
        // special for format! and must be escaped with the same
        // character.
        // tracing::debug!(re_string);
        let re = Regex::new(&re_string).expect("Invalid regex");
        rr.push(RegexAndReplacement {
            re,
            replacement: None,
        });
    }
    if conf.scrub_example_directives {
        let re = Regex::new(r#"[{]{2} *#example *[^}]*?[}]{2}"#)
            .expect("Invalid regex");
        rr.push(RegexAndReplacement {
            re,
            replacement: None,
        });
    }
    if conf.scrub_crate_directives {
        let re = Regex::new(r#"[{]{2} *#crate *[^}]*?[}]{2}"#)
            .expect("Invalid regex");
        rr.push(RegexAndReplacement {
            re,
            replacement: None,
        });
    }
    if conf.scrub_wikilinks {
        // Replace `[[file | title]]` by `title`
        let re = Regex::new(r#"\[\[ *[^|\]]*?(?:\| *([^\]]+)?)?\]\]"#)
            .expect("Invalid regex");
        rr.push(RegexAndReplacement {
            re,
            // Replace by $1 and trim spaces
            replacement: Some(Box::new(|caps: &Captures| {
                caps.get(1).map_or("", |m| m.as_str().trim()).to_string()
            })),
        });
    }
    // rr.append(&mut directives(conf));  // FIXME
    rr
}

// [finish NOW](https://github.com/john-cd/rust_howto/issues/1265)
// #[allow(dead_code)]
// fn directives(conf: &PreprocConfig) -> Vec<RegexAndReplacement> {
//     let mut rr = vec![];

//     if conf.process_crate_directives {
//         // {{c: xyz }}
//         let re_string: String = r"\{\{c:\s*(\S+)\s*\}\}".into();
//         let re = Regex::new(&re_string).expect("Invalid regex");
//         //let replacement = "";
//         rr.push(RegexAndReplacement {
//             re,
//             replacement: None,
//         });
//     }
//     if conf.process_category_directives {
//         // {{c: parsing }} -> [parsing][cat-parsing]⮳{{hi:parsing}}
//         // [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard
//         // library}}
//         let re_string: String = r"\{\{cat:\s*(\S+)\s*\}\}".into();
//         let re = Regex::new(&re_string).expect("Invalid regex");
//         let replacement = "[$1][cat-$1]{{hi: $1}}";
//         rr.push(RegexAndReplacement {
//             re,
//             replacement: None, Some(Box::new(|_| replacement.into())),
//         });
//     }
//     if conf.process_page_directives {
//         let re_string: String = r"\s*(\S+)\s*".into();
//         let re = Regex::new(&re_string).expect("Invalid regex");
//         let replacement = "[$1][p-$1]{{hi:$1}}";
//         rr.push(RegexAndReplacement {
//             re,
//             replacement: Some(Box::new(|_| replacement.into())),
//         });
//     }
//     rr
// }
