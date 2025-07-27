#![allow(dead_code)]
// ANCHOR: example
mod cis {
    use std::borrow::Borrow;
    use std::convert::From;
    use std::hash::Hash;

    // Case-insensitive string.
    // Implement `Eq`, `PartialEq`, `Hash` (used by `HashMap`).
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

fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::Hasher;
    let mut s = std::hash::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    use std::borrow::Borrow;
    use std::collections::HashMap;

    use cis::CaseInsensitiveString;

    let x = CaseInsensitiveString::from("Hello".to_string());
    let y = CaseInsensitiveString::from("HELLO".to_string());

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
    let z = CaseInsensitiveString::from("World".to_string());
    let zb: &str = z.borrow();
    assert!(x != z);
    assert_ne!(calculate_hash(&x), calculate_hash(&z));
    assert!(xb != zb);
    assert_ne!(calculate_hash(&xb), calculate_hash(&zb));

    // Use the new `CaseInsensitiveString` type:
    let mut map: HashMap<CaseInsensitiveString, usize> = HashMap::new();
    map.insert(CaseInsensitiveString::from("HelLO".to_string()), 1);

    // Lookup with a (lowercase) `&str` thanks to `Borrow<str>`.
    if let Some(value) = map.get("hello") {
        println!("Found: {}", value);
    } else {
        panic!("Not found.");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
