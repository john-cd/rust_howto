#![allow(dead_code)]
// ANCHOR: example
use std::collections::BTreeSet;

fn main() {
    // Create a new `BTreeSet`.
    let mut names = BTreeSet::new();

    // Insert some names.
    names.insert("Alice");
    names.insert("Bob");
    names.insert("Charlie");
    names.insert("Alice"); // Duplicate insertion, will be ignored.
    names.insert("David");
    names.insert("Eve");

    // Print the set (elements are printed in sorted order).
    println!("Names in the set:");
    for name in &names {
        println!("{}", name);
    }

    // Check if a name is in the set.
    if names.contains("Bob") {
        println!("Bob is in the set.");
    }

    // Remove a name.
    names.remove("Charlie");

    // Iterate over a range of names.
    println!("\nNames starting with 'B' and beyond:");
    for name in names.range("B"..) {
        println!("{}", name);
    }

    // Find the first and last elements.
    println!("\nFirst name: {:?}", names.first());
    println!("Last name: {:?}", names.last());

    // Get the length of the set.
    println!("\nNumber of names in the set: {}", names.len());

    // Check if the set is empty.
    println!("Is the set empty? {}", names.is_empty());

    // Example of using the difference method to find elements in one set but
    // not another.
    let other_names = BTreeSet::from(["Bob", "Eve", "Frank"]);
    let difference = names.difference(&other_names);
    println!("\nNames in the set but not in other_names:");
    for name in difference {
        println!("{}", name);
    }

    // Example of using the union method to combine two sets.
    let union = names.union(&other_names);
    println!("\nUnion of both sets:");
    for name in union {
        println!("{}", name);
    }

    // Example of using the intersection method to find common elements.
    let intersection = names.intersection(&other_names);
    println!("\nIntersection of both sets:");
    for name in intersection {
        println!("{}", name);
    }

    // Example of using the symmetric_difference method to find elements in
    // either set but not both.
    let symmetric_difference = names.symmetric_difference(&other_names);
    println!("\nSymmetric difference of both sets:");
    for name in symmetric_difference {
        println!("{}", name);
    }

    // Clear the set.
    names.clear();
    println!("Set is empty: {}", names.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
