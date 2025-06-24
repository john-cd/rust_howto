#![allow(dead_code)]
// ANCHOR: example
//! Trait example.

/// A trait with one method.
pub trait Summary {
    /// Returns a string that summarizes the item.
    /// Note that it can be just a method signature (no function body).
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement the `Summary` trait on the `NewsArticle` type.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let na = NewsArticle {
        headline: "headline".to_string(),
        location: "location".to_string(),
        author: "author".to_string(),
        content: "...".to_string(),
    };
    // Since the type implements the trait, we can call its method.
    println!("Summary: {}", na.summarize());

    // The method call above is equivalent to:
    NewsArticle::summarize(&na);
    // Or, if we needed to fully disambiguate:
    <NewsArticle as Summary>::summarize(&na);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
