// ANCHOR: example

trait Summary {
    fn summarize(&self) -> String;
}

// Accepts any type that implements the specified trait.
// This is called "impl Trait" syntax.
fn notify(item: &impl Summary) {
    // Since `item` is guaranteed to implement `Summary`, we can call its
    // `summarize` method.
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax (mostly equivalent).
// This is called "trait bound" syntax.
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

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
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
