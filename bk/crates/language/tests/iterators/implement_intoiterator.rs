// ANCHOR: example
//! Example of `IntoIterator` implementation.

/// A collection type.
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

/// By implementing `IntoIterator`, you define how the type will be converted to
/// an iterator.
impl<T> IntoIterator for MyVec<T> {
    // Associated type for the actual iterator that will be created.
    // It must implement the `Iterator` trait and produce items of type `Item`.
    type IntoIter = MyVecIntoIterator<T>;
    // Type of the elements that the iterator will yield.
    type Item = T;

    /// The `into_iter` method takes `self` by value,
    /// meaning it consumes the collection.
    fn into_iter(self) -> Self::IntoIter {
        MyVecIntoIterator { data: self.data }
    }
}

/// The iterator type.
/// It is a consuming iterator - it takes ownership of the `MyVec` data.
#[derive(Debug)]
struct MyVecIntoIterator<T> {
    data: Vec<T>,
    // Iterator types often also store indices to track the progress of the
    // iteration. In this simple case, this is not necessary.
}

impl<T> Iterator for MyVecIntoIterator<T> {
    type Item = T;

    /// The core iterator method.
    fn next(&mut self) -> Option<Self::Item> {
        if !self.data.is_empty() {
            // In this example, we use `remove` for simplicity, noting that its
            // complexity is O(n). `remove` takes the item at the
            // position n and returns it to us, giving the ownership.
            // We can always use "0" to return the first element, instead of
            // storing an index and increasing it sequentially.
            // We could also use `pop` to remove the last element.
            Some(self.data.remove(0))
        } else {
            None
        }
    }
}

fn main() {
    let mut my_vec = MyVec::new();
    my_vec.push(1);
    my_vec.push(2);

    // The `for` loop calls `into_iter` under the covers.
    for element in my_vec {
        println!("Element: {}", element);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
