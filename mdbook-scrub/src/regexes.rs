use regex::Captures;
use regex::Regex;

use super::conf::PreprocConfig;

// TODO share RegexAndReplacement code with CLIs (core_lib)

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
    // Remove any left-over {{#example }} directives and log a warning.
    // {{#example }} is a custom directive for this book.
    if conf.scrub_example_directives {
        let re = Regex::new(r#"[{]{2} *#example *[^}]*?[}]{2}"#)
            .expect("Invalid regex");
        rr.push(RegexAndReplacement {
            re,
            replacement: None,
        });
    }
    // Remove any left-over {{#crate }} directives and log a warning.
    // {{#crate }} is a custom directive for this book.
    if conf.scrub_crate_block_directives {
        let re = Regex::new(r#"[{]{2} *#crate *[^}]*?[}]{2}"#)
            .expect("Invalid regex");
        rr.push(RegexAndReplacement {
            re,
            replacement: None,
        });
    }
    // Remove any left-over [[file | title]] wikilinks and log a warning.
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
    rr
}
