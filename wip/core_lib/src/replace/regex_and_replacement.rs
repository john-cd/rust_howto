use regex::Captures;
use regex::Regex;

// Replacement closure.
type Replacement = Box<dyn Fn(&Captures) -> String>;

/// A compiled Regex and replacement string or function.
pub struct RegexAndReplacement {
    pub re: Regex,
    // Most often set to `None` for no replacement, meaning deletion.
    pub replacement: Option<Replacement>,
}

pub trait Match {
    fn has_match(&self, content: &str) -> bool;
}

impl Match for &[RegexAndReplacement] {
    /// Returns true if at least one regex match is found in the string slice.
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
}
