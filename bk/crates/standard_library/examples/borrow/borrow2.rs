#![allow(dead_code)]
// ANCHOR: example
use std::borrow::Borrow;
use std::collections::HashMap;

fn find_score<K>(map: &HashMap<String, u32>, key: K) -> Option<u32>
where
    K: Borrow<str>,
{
    map.get(key.borrow()).copied()
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert("eve".to_string(), 30);

    // Pass a `&str`.
    assert_eq!(find_score(&scores, "eve"), Some(30));

    // Pass a `String`.
    let name = "eve".to_string();
    assert_eq!(find_score(&scores, name), Some(30));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO
