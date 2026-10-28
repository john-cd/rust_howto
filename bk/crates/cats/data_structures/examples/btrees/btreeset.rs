#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A `BTreeSet`, an ordered set of unique names.

use std::collections::BTreeSet;

/// Populates a `BTreeSet` with sample names.
fn populate_set() -> BTreeSet<&'static str> {
    let mut names = BTreeSet::new();
    names.insert("Alice");
    names.insert("Bob");
    names.insert("Charlie");
    names.insert("Alice"); // Duplicate insertion, will be ignored.
    names.insert("David");
    names.insert("Eve");
    names
}

/// Performs set operations (difference, union, intersection, symmetric difference) between two sets.
fn set_operations(names: &BTreeSet<&'static str>, other_names: &BTreeSet<&'static str>) {
    let diff: Vec<_> = names.difference(other_names).cloned().collect();
    let union: Vec<_> = names.union(other_names).cloned().collect();
    let inter: Vec<_> = names.intersection(other_names).cloned().collect();
    let sym_diff: Vec<_> = names.symmetric_difference(other_names).cloned().collect();

    assert!(diff.contains(&"Alice"));
    assert!(union.contains(&"Frank"));
    assert!(inter.contains(&"Bob"));
    assert!(sym_diff.contains(&"Frank"));
}

/// Creates, fills and queries a `BTreeSet`.
fn main() {
    let mut names = populate_set();

    println!("Names in the set:");
    for name in &names {
        println!("{name}");
    }

    if names.contains("Bob") {
        println!("Bob is in the set.");
    }

    names.remove("Charlie");

    println!("\nNames starting with 'B' and beyond:");
    for name in names.range("B"..) {
        println!("{name}");
    }

    println!("\nFirst name: {:?}", names.first());
    println!("Last name: {:?}", names.last());

    let other_names = BTreeSet::from(["Bob", "Eve", "Frank"]);
    set_operations(&names, &other_names);

    names.clear();
    println!("Set is empty: {}", names.is_empty());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the btreeset example through its entry point.
    #[test]
    fn test_btreeset_example_entry_point() {
        main();
    }

    #[test]
    fn test_populate_set() {
        let set = populate_set();
        assert_eq!(set.len(), 5); // Alice duplicate was ignored
        assert!(set.contains("Alice"));
        assert!(set.contains("Eve"));
    }

    #[test]
    fn test_set_operations() {
        let set1 = BTreeSet::from(["Alice", "Bob"]);
        let set2 = BTreeSet::from(["Bob", "Charlie", "Frank"]);
        set_operations(&set1, &set2);
    }
}
