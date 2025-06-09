#![allow(dead_code)]
// ANCHOR: example
/// A trait.
trait Summary {
    /// Method signature.
    /// Types that implement the `Summary` trait must implement a method with
    /// the exact signature.
    fn summarize_author(&self) -> String;

    /// Traits can also provide _default implementations_ for some or all of
    /// their methods or functions.
    /// - Default implementations have a body.
    /// - Types that implement the trait can override the default
    ///   implementations, if desired.
    fn summarize(&self) -> String {
        // Default implementations can call other methods in the same trait,
        // even if these other methods don't have a default implementation.
        // Here, the default implementation uses the `summarize_author` method.
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/// Let's define a `Blog` struct...
struct Blog {
    author: String,
}

/// ...that implements the `Summary` trait.
impl Summary for Blog {
    /// All non-default-implemented methods must be implemented for the struct.
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

fn main() {
    // Instantiate a Blog struct...
    let blog = Blog {
        author: "ferris".into(),
    };
    // ...and call the method implemented by default in the trait.
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
