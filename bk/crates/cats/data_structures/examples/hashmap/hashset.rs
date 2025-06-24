#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates the basic usage of `HashSet`.
//!
//! `HashSet` is a collection that stores unique values. It's useful for
//! scenarios where you need to ensure that no duplicate values are present.
use std::collections::HashSet;

fn main() {
    // Create a new `HashSet`:
    let mut languages = HashSet::new();

    // Insert values:
    languages.insert("Rust");
    languages.insert("Python");
    languages.insert("JavaScript");

    // Insert duplicate values (which are ignored).
    let added = languages.insert("Rust");
    println!("Was 'Rust' added? {}", added); // false - it already exists.

    // Create a `HashSet` from iterators:
    let numbers = vec![1, 2, 3, 4, 5, 1, 2, 3];
    let unique_numbers: HashSet<_> = numbers.into_iter().collect();
    println!("Unique numbers: {:?}", unique_numbers); // 1, 2, 3, 4, 5 (order may vary).

    // Check if an element exists:
    if languages.contains("Rust") {
        println!("Rust is in the set.");
    }

    // Remove elements:
    if languages.remove("Python") {
        println!("Python was removed from the set.");
    }

    // Iterate over elements:
    for language in &languages {
        println!("{}", language);
    }

    // Set operations:
    let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5].iter().cloned().collect();

    // Union.
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union); // 1, 2, 3, 4, 5.

    // Intersection.
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersection: {:?}", intersection); // 3.

    // Difference.
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Difference (set1 - set2): {:?}", difference); // 1, 2.

    // Symmetric difference (elements in either set but not both).
    let symmetric_difference: HashSet<_> =
        set1.symmetric_difference(&set2).cloned().collect();
    println!("Symmetric Difference: {:?}", symmetric_difference); // 1, 2, 4, 5.

    // Check if a set is a subset of another:
    let subset: HashSet<_> = [1, 2].iter().cloned().collect();
    println!(
        "Is {:?} a subset of {:?}? {}",
        subset,
        set1,
        subset.is_subset(&set1)
    ); // true

    // Get the length:
    println!("Number of languages: {}", languages.len());

    // Clear the set:
    languages.clear();
    println!("After clearing, languages: {:?}", languages); // {}
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
