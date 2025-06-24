#![allow(dead_code)]
// ANCHOR: example
/// A trait that represents an iterator (similar to what is in the standard
/// library).
trait Iterator {
    /// This associated type is the type of the elements yielded by the
    /// iterator. You can't provide a default implementation at this point.
    type Item;

    /// A method to return the next element in the iterator, or `None` if the
    /// iterator is exhausted. Note the use of `Self::` to refer to the
    /// associated type.
    fn next(&mut self) -> Option<Self::Item>;
}

struct MyIterator(u32);

// We implement the trait for a given struct...
impl Iterator for MyIterator {
    // ...and define what associated type should be used here.
    // You can use `Self` in the associated type definition.
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // This example iterator returns the same value forever.
        // `&mut self` is a shortcut for `self: &mut Self`,
        // where `Self` is the implementing type e.g. `MyIterator`.
        Some(self.0)
    }
}

/// Use the trait. Note how the associated type is specified within < and >.
fn use_iterator(it: &mut impl Iterator<Item = u32>) -> Option<u32> {
    it.next()
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
