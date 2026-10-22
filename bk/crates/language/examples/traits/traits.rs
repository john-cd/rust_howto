#![allow(dead_code)]
// ANCHOR: example
//! Trait example.
//!
//! TODO Reviewed ??-??-2026

/// A trait with one method.
pub trait Summary {
    /// Returns a string that summarizes the item.
    /// Note that it can be just a method signature (no function body).
    fn summarize(&self) -> String;
}

/// A news article, which can be summarized.
pub struct NewsArticle {
    /// The headline.
    pub headline: String,
    /// The location where the news happened.
    pub location: String,
    /// The author.
    pub author: String,
    /// The text of the article.
    pub content: String,
}

// Implement the `Summary` trait on the `NewsArticle` type.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

/// Creates an article and calls the method of the trait.
fn main() {
    let na = NewsArticle {
        headline: "headline".to_owned(),
        location: "location".to_owned(),
        author: "author".to_owned(),
        content: "...".to_owned(),
    };
    // Since the type implements the trait, we can call its method.
    println!("Summary: {}", na.summarize());

    // The method call above is equivalent to:
    NewsArticle::summarize(&na);
    // Or, if we needed to fully disambiguate:
    <NewsArticle as Summary>::summarize(&na);
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
