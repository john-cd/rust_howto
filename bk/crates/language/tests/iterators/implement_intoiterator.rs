// ANCHOR: example
#[derive(Debug)]
struct MyVec<T> {
    data: Vec<T>,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }
}

impl<T> IntoIterator for MyVec<T> {
    // Type of the elements that the iterator will yield.
    type Item = T;
    // Associated type for the actual iterator that will be created.
    // It must implement the `Iterator` trait and produce items of type `Item`.
    type IntoIter = MyVecIterator<T>;

    /// The `into_iter` method takes self by value, meaning it consumes the collection.
    fn into_iter(self) -> Self::IntoIter {
        MyVecIterator {
            data: self.data,
            index: 0,
        }
    }
}

#[derive(Debug)]
struct MyVecIterator<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> Iterator for MyVecIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let result = Some(self.data[self.index].clone()); // Or move if T is not Copy
            self.index += 1;
            result
        } else {
            None
        }
    }
}

fn main() {
    let mut my_vec = MyVec::new();
    my_vec.push(1);
    my_vec.push(2);

    for element in my_vec {
        println!("Element: {}", element);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
