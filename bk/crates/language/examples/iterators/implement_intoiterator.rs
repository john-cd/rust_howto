#![allow(dead_code)]
// ANCHOR: example
//! Example of `IntoIterator` implementation.
//!
//! Note that the iterator is a custom `struct` separate from the collection
//! type.

/// A custom collection type.
/// In this example, we simply wrap a `Vec`.
#[derive(Debug)]
struct MyVec<T> {
    data: Vec<T>,
}

impl<T> MyVec<T> {
    /// Constructor.
    fn new() -> Self {
        MyVec { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }
}

/// Implementation of `IntoIterator`.
impl<T> IntoIterator for MyVec<T> {
    /// Associated type for the iterator.
    type IntoIter = MyVecIntoIterator<T>;
    /// Type of the elements that the iterator will yield.
    type Item = T;

    /// The `into_iter` method takes `self` by value,
    /// meaning it consumes the collection.
    fn into_iter(self) -> Self::IntoIter {
        MyVecIntoIterator { data: self.data }
    }
}

/// The iterator type for the custom collection.
/// It must implement the `Iterator` trait.
#[derive(Debug)]
struct MyVecIntoIterator<T> {
    data: Vec<T>,
    // Iterator types often store indices to track the progress of the
    // iteration. This is not necessary in this simple example.
}

impl<T> Iterator for MyVecIntoIterator<T> {
    type Item = T;

    /// The core iterator method.
    fn next(&mut self) -> Option<Self::Item> {
        if !self.data.is_empty() {
            // In this example, we use `remove` for simplicity, noting that its
            // complexity is O(n). `remove` takes the item at a given
            // position and returns it to us, giving up the ownership.
            // We use `0` to return the first element.
            Some(self.data.remove(0))
            // We could use `pop` to remove the last element instead.
        } else {
            None
        }
    }
    // `next()` is the only method we are required to implement.
    // The `Iterator` trait implements a large number of default methods, which
    // we get for free.
}

fn main() {
    let mut my_vec = MyVec::new();
    my_vec.push(1);
    my_vec.push(2);

    // Our custom collection can now be used in a `for` loop.
    // The `for` loop calls `into_iter()` under the covers.
    for element in my_vec {
        println!("Element: {element}");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
