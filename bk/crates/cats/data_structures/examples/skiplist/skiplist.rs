// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A `SkipMap`, an ordered map built on a skip list.

use skiplist::SkipMap;

/// Inserts into a `SkipMap` and iterates in key order.
fn main() {
    let mut map: SkipMap<i32, &str, 16> = SkipMap::new();

    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");

    for (k, v) in &map {
        println!("{k}: {v}");
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
