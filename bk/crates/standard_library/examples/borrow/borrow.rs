#![allow(dead_code)]
// ANCHOR: example
use std::collections::HashMap;

// This is the most common use of `Borrow`.
// A `HashMap<String, V>` can be queried with a `&str` because `String:
// Borrow<str>`.
fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("alice".to_string(), 10);
    scores.insert("bob".to_string(), 20);

    // Lookup with `&str` - no need to allocate a `String`:
    assert_eq!(scores.get("alice"), Some(&10));
    assert_eq!(scores.get("bob"), Some(&20));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO
