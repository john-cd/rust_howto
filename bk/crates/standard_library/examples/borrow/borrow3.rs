// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Implementing `Borrow` for a string type with canonical lowercase storage.

/// A string wrapper that normalizes input for case-insensitive comparisons.
mod cis {
    use std::borrow::Borrow;
    use std::convert::From;
    use std::hash::Hash;

    // Case-insensitive string.
    // Implement `Eq`, `PartialEq`, `Hash` (used by `HashMap`).
    /// A string normalized to lowercase when it is constructed.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CaseInsensitiveString(String);

    impl From<String> for CaseInsensitiveString {
        fn from(s: String) -> Self {
            // We store the lowercase version of the string.
            Self(s.to_ascii_lowercase())
        }
    }

    // Implement `Borrow<str>` to allow lookup by the canonical lowercase form.
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

/// Looks up a map with normalized lowercase keys.
fn main() {
    use std::borrow::Borrow as _;
    use std::collections::HashMap;

    use cis::CaseInsensitiveString;

    let x = CaseInsensitiveString::from("Hello".to_owned());
    let y = CaseInsensitiveString::from("HELLO".to_owned());

    assert_eq!(x, y);

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
    assert!(xb != zb);

    // Use the new `CaseInsensitiveString` type:
    let mut map: HashMap<CaseInsensitiveString, usize> = HashMap::new();
    map.insert(CaseInsensitiveString::from("HelLO".to_owned()), 1);

    // Lookup uses the same canonical representation as the owned key.
    assert_eq!(map.get("hello"), Some(&1));
    assert_eq!(map.get("HELLO"), None);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow3_example_entry_point() {
        main();
    }

    #[test]
    fn test_case_insensitive_equality() {
        let a = cis::CaseInsensitiveString::from("Rust".to_owned());
        let b = cis::CaseInsensitiveString::from("rUST".to_owned());
        assert_eq!(a, b);
        assert_ne!(a, cis::CaseInsensitiveString::from("Go".to_owned()));
    }

    #[test]
    fn test_hash_matches_between_owned_and_borrowed() {
        use std::borrow::Borrow as _;
        let a = cis::CaseInsensitiveString::from("Rust".to_owned());
        let s: &str = a.borrow();
        assert_eq!(s, "rust");
        assert_eq!(calculate_hash(&a), calculate_hash(&s));
    }

    #[test]
    fn test_hash_differs_for_different_values() {
        assert_ne!(calculate_hash(&"a"), calculate_hash(&"b"));
        assert_eq!(calculate_hash(&"a"), calculate_hash(&"a"));
    }
}
