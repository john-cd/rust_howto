#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A newtype whose equality ignores a presentation field.
//!
//! `PartialEq` and `Hash` must agree: values that compare equal must hash
//! equally, so both ignore `style`.

use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

/// How a phone number is displayed.
#[derive(Debug, Clone, Copy)]
enum Style {
    /// `+1 555 0100`
    International,
    /// `555-0100`
    National,
}

/// A phone number: the digits matter, the display style does not.
#[derive(Debug, Clone)]
struct PhoneNumber {
    country_code: u8,
    number: u32,
    style: Style,
}

impl PartialEq for PhoneNumber {
    fn eq(&self, other: &Self) -> bool {
        self.country_code == other.country_code && self.number == other.number
    }
}

impl Eq for PhoneNumber {}

impl Hash for PhoneNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash exactly the fields used by `eq`.
        self.country_code.hash(state);
        self.number.hash(state);
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.style {
            Style::International => {
                write!(f, "+{} {:07}", self.country_code, self.number)
            }
            Style::National => write!(f, "{:07}", self.number),
        }
    }
}

/// Shows that the style does not affect equality or set membership.
fn main() {
    let a = PhoneNumber {
        country_code: 1,
        number: 5_550_100,
        style: Style::International,
    };
    let b = PhoneNumber {
        style: Style::National,
        ..a.clone()
    };
    assert_ne!(a.to_string(), b.to_string());
    assert_eq!(a, b);

    let set: HashSet<PhoneNumber> = [a, b].into_iter().collect();
    assert_eq!(set.len(), 1);
}
// ANCHOR_END: example

/// Runs the example.
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
