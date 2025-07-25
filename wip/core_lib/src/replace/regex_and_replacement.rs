use std::borrow::Cow;

use regex::Captures;
use regex::Regex;

// Replacement closure.
type Replacement = Box<dyn Fn(&Captures) -> String>;

/// A compiled Regex plus a replacement string or function.
pub struct RegexAndReplacement {
    pub re: Regex,
    // Most often set to `None` for no replacement, meaning deletion.
    pub replacement: Option<Replacement>,
}

pub trait MatchAndReplace {
    fn has_match(&self, content: &str) -> bool;
    fn replace_all<'a>(&self, content: &'a str) -> Cow<'a, str>;
}

impl MatchAndReplace for &[RegexAndReplacement] {
    /// Returns true if at least one match for at least one regex is found
    /// in the `content` string slice.
    fn has_match(&self, content: &str) -> bool {
        if self.is_empty() {
            return false;
        }
        for rr in self.iter() {
            if rr.re.is_match(content) {
                return true;
            }
        }
        false
    }

    /// For all provided regexes, replaces all matches in the content by the corresponding replacement.
    ///
    /// # Arguments
    ///
    /// * `content` - The text to process.
    ///
    /// # Returns the (updated) content as a `Cow<str>`.
    fn replace_all<'a>(&self, content: &'a str) -> Cow<'a, str> {
        let mut result = Cow::Borrowed(content);
        if !self.is_empty() {
            for rr in self.iter() {
                result = if let Some(ref repl) = rr.replacement {
                    rr.re.replace_all(content, repl)
                } else {
                    // If replacement is `None`,
                    // just delete the matching text.
                    rr.re.replace_all(content, "")
                };
                // tracing::debug!("Content: {content}");
            }
        }
        result
    }
}

// [use Box<dyn Replacer> instead of Box<dyn Fn(TODO use Box<dyn Replacer> instead of Box<dyn Fn(&Captures) -> String> ??Captures) -> String> ??](https://github.com/john-cd/rust_howto/issues/1430)

// use regex::Replacer;
//
// struct Replacement;
//
// impl Replacer for Replacement {
//     fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
//         let directive = extract_directive(caps);
//         dst.push_str("...");
//     }
// }
