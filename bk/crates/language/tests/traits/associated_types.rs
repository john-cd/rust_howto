#![allow(dead_code)]
// ANCHOR: example
/// A trait that represents an iterator.
trait Iterator {
    /// The type of the elements yielded by the iterator.
    type Item;

    /// Returns the next element in the iterator, or `None` if the iterator is
    /// exhausted. Note the use of `::` to refer to the associated type.
    fn next(&mut self) -> Option<Self::Item>;
}

struct MyIterator(u32);

// We implement the trait for a given struct
impl Iterator for MyIterator {
    // ...and define what associated type should be used here
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0)
    }
}

fn use_iterator(it: &mut impl Iterator<Item = u32>) -> Option<u32> {
    it.next()
}

/// A trait that represents the ability to add two values together.
///
/// A common pattern is a generic type (with a default) and an associated type.
trait Add<Rhs = Self> {
    /// The type of the result of the addition.
    type Output;

    /// Adds two values together.
    fn add(self, rhs: Rhs) -> Self::Output;
}

fn main() {
    let mut it = MyIterator(42);
    println!("{:?}", use_iterator(&mut it));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
