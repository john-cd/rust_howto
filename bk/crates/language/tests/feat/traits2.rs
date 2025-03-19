#![allow(dead_code)]
// ANCHOR: example
trait Summary {
    // Method signature that describe the behavior of the types that implement
    // this trait. Function signatures do not include a function body
    // (note the ;).
    fn summarize_author(&self) -> String;

    // Traits can provide default implementations
    // for some or all of their methods.
    fn summarize(&self) -> String {
        // The default implementation has a body.
        // Default implementations can call other methods in the same trait,
        // even if those other methods don't have a default implementation.
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Let's define a `Blog` struct...
struct Blog {
    author: String,
}

// ...that implements the `Summary` trait.
impl Summary for Blog {
    // All non-default-implemented methods must be implemented for the struct.
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

fn main() {
    // Instantiate a Blog struct...
    let blog = Blog {
        author: "ferris".into(),
    };
    // ...and call the function implemented by default in the trait.
    println!("{}", blog.summarize());
    // Because we've implemented `summarize_author` for `Blog`, the `Summary`
    // trait has given us the behavior of the `summarize` method without
    // requiring us to write any more code.
}
// Adapted from https://doc.rust-lang.org/book/ch10-02-traits.html
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
