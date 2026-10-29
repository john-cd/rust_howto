// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Importing items from the standard library with `use`.

// This `use` statement allows us to use `HashMap` directly without having to
// specify its full path.
// IDIOM: import the `struct` itself, not its parent module.
use std::collections::HashMap;

// `HashMap` is declared in the standard library (`std`), which is an external
// crate. We therefore prefix the path with the crate's name.

/// Builds a map with the imported `HashMap`.
fn build_map() -> HashMap<u32, String> {
    // We now refer to `HashMap` by its name.
    let mut mymap: HashMap<u32, String> = HashMap::new();

    // Let's add something to it...
    mymap.entry(42).or_insert("my favorite number".into());
    mymap
}

/// Uses an imported `HashMap`.
fn main() {
    let mymap = build_map();
    println!("{mymap:?}");
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

    #[test]
    fn test_build_map() {
        let map = build_map();
        assert_eq!(map.len(), 1);
        assert_eq!(
            map.get(&42).map(String::as_str),
            Some("my favorite number")
        );
        assert_eq!(map.get(&0), None);
    }

    #[test]
    fn test_entry_or_insert_keeps_existing_value() {
        let mut map = build_map();
        map.entry(42).or_insert("other".into());
        assert_eq!(map[&42], "my favorite number");
    }
}
