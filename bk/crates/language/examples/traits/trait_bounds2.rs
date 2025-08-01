#![allow(dead_code)]
// ANCHOR: example
trait Summary {
    fn summarize(&self) -> String;
}

/// This function accepts a reference to any type `T` that implements the
/// `Summary` trait.
fn notify<T: Summary>(item: &T) {
    // Since `item` is guaranteed to implement `Summary`, we can call its
    // `summarize` method.
    println!("Breaking news! {}", item.summarize());
}

/// It  may be equivalently be written in a `where` clause:
fn notify2<T>(_item: &T)
where
    T: Summary,
{
}

// The "impl Trait" syntax is in this case equivalent.
// `impl Summary` is an opaque type that implements the `Summary` trait:
fn notify3(_item: &impl Summary) {}

/// A struct that implements the `Summary` trait.
struct Article {
    txt: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        self.txt.clone()
    }
}

fn main() {
    let a = Article {
        txt: String::from("Some text"),
    };
    notify(&a);
    notify2(&a);
    notify3(&a);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
