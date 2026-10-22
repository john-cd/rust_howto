//! Implementing `Borrow` for a case-insensitive string type.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
/// A case-insensitive string type.
mod cis {
    use std::borrow::Borrow;
    use std::convert::From;
    use std::hash::Hash;

    // Case-insensitive string.
    // Implement `Eq`, `PartialEq`, `Hash` (used by `HashMap`).
    /// A string that is compared and hashed ignoring case.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CaseInsensitiveString(String);

    impl From<String> for CaseInsensitiveString {
        fn from(s: String) -> Self {
            // We store the lowercase version of the string.
            Self(s.to_ascii_lowercase())
        }
    }

    // Equality with `str`, both ways:
    impl PartialEq<str> for CaseInsensitiveString {
        fn eq(&self, other: &str) -> bool {
            self.0.eq_ignore_ascii_case(other)
        }
    }

    impl PartialEq<CaseInsensitiveString> for str {
        fn eq(&self, other: &CaseInsensitiveString) -> bool {
            self.eq_ignore_ascii_case(&other.0)
        }
    }

    // Implement `Borrow<str>` to allow lookup.
    impl Borrow<str> for CaseInsensitiveString {
        fn borrow(&self) -> &str {
            &self.0
        }
    }
}

/// Returns the hash of `t`.
fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::Hasher as _;
    let mut s = std::hash::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Looks up a map with case-insensitive keys.
fn main() {
    use std::borrow::Borrow as _;
    use std::collections::HashMap;

    use cis::CaseInsensitiveString;

    let x = CaseInsensitiveString::from("Hello".to_owned());
    let y = CaseInsensitiveString::from("HELLO".to_owned());

    assert!(x == *"heLLo");
    assert!(*"HellO" == y);

    // The `Borrow`-required invariants must hold:
    let xb: &str = x.borrow();
    let yb: &str = y.borrow();

    // - Equality holds:
    assert!(x == y);
    assert_eq!(xb, yb);

    // - Hashes are the same:
    assert_eq!(calculate_hash(&x), calculate_hash(&y));
    assert_eq!(calculate_hash(&xb), calculate_hash(&yb));

    // Conversely:
    let z = CaseInsensitiveString::from("World".to_owned());
    let zb: &str = z.borrow();
    assert!(x != z);
    assert_ne!(calculate_hash(&x), calculate_hash(&z));
    assert!(xb != zb);
    assert_ne!(calculate_hash(&xb), calculate_hash(&zb));

    // Use the new `CaseInsensitiveString` type:
    let mut map: HashMap<CaseInsensitiveString, usize> = HashMap::new();
    map.insert(CaseInsensitiveString::from("HelLO".to_owned()), 1);

    // Lookup with a (lowercase) `&str` thanks to `Borrow<str>`.
    if let Some(value) = map.get("hello") {
        println!("Found: {value}");
    } else {
        panic!("Not found.");
    }
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
