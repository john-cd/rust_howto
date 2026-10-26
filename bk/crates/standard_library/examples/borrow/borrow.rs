#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Looking up a `HashMap` of `String` keys with a `&str`, thanks to `Borrow`.

use std::collections::HashMap;

// This is the most common use of `Borrow`.
// A `HashMap<String, V>` can be queried with a `&str`,
// because `String: Borrow<str>`.
/// Looks up scores with `&str` keys.
fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("alice".to_owned(), 10);
    scores.insert("bob".to_owned(), 20);

    // Lookup with `&str` - no need to allocate a `String`:
    assert_eq!(scores.get("alice"), Some(&10));
    assert_eq!(scores.get("bob"), Some(&20));
    println!(
        "alice: {:?}, bob: {:?}",
        scores.get("alice"),
        scores.get("bob")
    );
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
