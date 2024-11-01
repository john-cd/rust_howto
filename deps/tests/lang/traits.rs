#![allow(dead_code)]
// ANCHOR: example

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement Trait on a Type
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
    println!("Summary: {}", na.summarize());
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
