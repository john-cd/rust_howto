#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Reformatting dates with a regular expression replacement.

use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

/// Rewrites the ISO 8601 dates in `before` as `MM/DD/YYYY`.
fn reformat_dates(before: &str) -> Cow<'_, str> {
    lazy_static! {
    /// Captures the year, month, and day components of an ISO 8601 date.
            static ref ISO8601_DATE_REGEX: Regex =
                Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        }
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}

/// Reformats the dates in a sample text.
fn main() {
    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = reformat_dates(before);
    println!("{after}");
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
