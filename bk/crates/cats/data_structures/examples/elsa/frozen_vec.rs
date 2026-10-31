//! Demonstrates stable references in an `elsa::FrozenVec`.

// ANCHOR: example
use elsa::FrozenVec;

/// Shows that earlier element references remain valid after more pushes.
fn main() {
    let frozen: FrozenVec<Box<String>> = FrozenVec::new();

    // `push_get` inserts an element and returns a reference to it.
    // This reference remains valid even after further insertions.
    let first: &String = frozen.push_get(Box::new("first".to_owned()));
    let second: &String = frozen.push_get(Box::new("second".to_owned()));
    let third: &String = frozen.push_get(Box::new("third".to_owned()));

    // Importantly, `first` and `second` are still valid even though we added
    // `third`:
    assert_eq!(first, "first");
    assert_eq!(second, "second");
    assert_eq!(third, "third");
    assert_eq!(frozen.len(), 3);
    println!("first:  {first}");
    println!("second: {second}");
    println!("third:  {third}");
    println!("total:  {}", frozen.len());

    // `FrozenVec` also supports indexing:
    assert_eq!(frozen[0].as_str(), "first");
    println!("index 0: {}", frozen[0]);
}

/// Runs this example for its parent module.
pub(super) fn run() {
    main();
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Verifies that references and indexing retain the inserted values.
    #[test]
    fn test_references_survive_pushes() {
        super::main();
    }
}
