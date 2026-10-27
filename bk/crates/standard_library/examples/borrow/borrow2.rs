#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A generic lookup that accepts any type that `String` borrows as.

use std::borrow::Borrow;
use std::collections::HashMap;

/// Returns the score for `key`, which may be a `&str` or a `String`.
fn find_score<K>(map: &HashMap<String, u32>, key: K) -> Option<u32>
where
    K: Borrow<str>,
{
    map.get(key.borrow()).copied()
}

/// Looks up a score with several key types.
fn main() {
    let mut scores = HashMap::new();
    scores.insert("eve".to_owned(), 30);

    // Pass a `&str`.
    assert_eq!(find_score(&scores, "eve"), Some(30));

    // Pass a `String`.
    let name = "eve".to_owned();
    assert_eq!(find_score(&scores, name), Some(30));
    println!("eve's score: {:?}", find_score(&scores, "eve"));
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the borrow2 example through its entry point.
    #[test]
    fn test_borrow2_example_entry_point() {
        main();
    }
}
