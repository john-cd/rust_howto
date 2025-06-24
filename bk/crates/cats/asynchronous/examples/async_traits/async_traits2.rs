#![allow(dead_code)]
// ANCHOR: example
/// This example demonstrates how to use `impl Trait` in a trait definition.
///
/// The `Container` trait defines a method `items` that returns
/// an iterator over `u8` values. The specific type of the iterator is not
/// specified in the trait definition, but is instead left to the implementor
/// to decide.
#[allow(dead_code)]
trait Container {
    /// Returns an iterator over the items in the container.
    fn items(&self) -> impl Iterator<Item = u8>;
}

/// A concrete implementation of the `Container` trait.
struct MyContainer {
    items: Vec<u8>,
}

/// Implementation of the `Container` trait for `MyContainer`.
impl Container for MyContainer {
    fn items(&self) -> impl Iterator<Item = u8> {
        self.items.iter().cloned()
    }
}

fn main() {
    let c = MyContainer {
        items: vec![1, 2, 3],
    };
    for i in c.items {
        println!("{}", i);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
