//! Demonstrates stable references in an `elsa::FrozenMap`.

// ANCHOR: example
use elsa::FrozenMap;

/// Shows that a map value reference remains valid after another insertion.
fn main() {
    let map: FrozenMap<&str, Box<u32>> = FrozenMap::new();

    map.insert("one", Box::new(1));
    map.insert("two", Box::new(2));

    // References remain stable after further insertions:
    let one = map.get("one").expect("the `one` entry was inserted above");
    map.insert("three", Box::new(3));
    assert_eq!(*one, 1);
    println!("one is still: {one}");
}

/// Runs this example for its parent module.
pub(super) fn run() {
    main();
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Verifies that the example can read a value after another insertion.
    #[test]
    fn test_value_reference_survives_insertion() {
        super::main();
    }
}
