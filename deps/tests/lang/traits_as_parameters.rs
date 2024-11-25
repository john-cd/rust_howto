// ANCHOR: example
// Accepts any type that implements the specified trait:
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax (mostly equivalent):
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

trait Summary {
    fn summarize(&self) -> String;
}

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
