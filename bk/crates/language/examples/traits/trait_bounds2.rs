// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Trait bounds written in several ways.

/// Something that can be summarized.
trait Summary {
    /// Returns a summary.
    fn summarize(&self) -> String;
}

/// This function accepts a reference to any type `T` that implements the
/// `Summary` trait.
fn notify<T: Summary>(item: &T) {
    // Since `item` is guaranteed to implement `Summary`, we can call its
    // `summarize` method.
    println!("Breaking news! {}", item.summarize());
}

/// It may be equivalently be written in a `where` clause:
fn notify2<T>(_item: &T)
where
    T: Summary,
{
}

// The "impl Trait" syntax is in this case equivalent.
// `impl Summary` is an opaque type that implements the `Summary` trait:
/// Accepts any type that implements [`Summary`].
fn notify3(_item: &impl Summary) {}

/// A struct that implements the `Summary` trait.
struct Article {
    /// The text of the article.
    txt: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        self.txt.clone()
    }
}

/// Passes an article to functions that require [`Summary`].
fn main() {
    let a = Article {
        txt: String::from("Some text"),
    };
    notify(&a);
    notify2(&a);
    notify3(&a);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the trait bounds2 example through its entry point.
    #[test]
    fn test_trait_bounds2_example_entry_point() {
        main();
    }

    /// The summary is the article text; all bound styles accept it.
    #[test]
    fn test_summary() {
        let a = Article {
            txt: String::from("Some text"),
        };
        assert_eq!(a.summarize(), "Some text");
        notify(&a);
        notify2(&a);
        notify3(&a);
        assert_eq!(Article { txt: String::new() }.summarize(), "");
    }
}
