// TODO Reviewed ??-??-2026.
//! Encodes a state transition in the type system.

// ANCHOR: example
use std::marker::PhantomData;

/// An article that has not yet been published.
struct Draft;

/// An article that is ready to be read.
struct Published;

/// An article whose available operations depend on its state.
struct Article<State> {
    /// Article title carried unchanged across the publishing transition.
    title: String,
    /// Records the compile-time state without allocating a runtime state value.
    _state: PhantomData<State>,
}

impl Article<Draft> {
    /// Starts a new article in the draft state.
    fn draft(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            _state: PhantomData,
        }
    }

    /// Consumes a draft and returns an article in the published state.
    fn publish(self) -> Article<Published> {
        Article {
            title: self.title,
            _state: PhantomData,
        }
    }
}

impl Article<Published> {
    /// Returns the title of a published article.
    fn title(&self) -> &str {
        &self.title
    }
}

/// Publishes an article and prints its title.
pub fn run() {
    let article = Article::draft("A type-safe transition").publish();
    println!("Published: {}", article.title());
}

/// Standalone entry point for the book snippet.
#[allow(dead_code)]
fn main() {
    run();
}
// ANCHOR_END: example

/// Verifies that changing the article state preserves its content.
#[cfg(test)]
mod tests {
    use super::Article;

    /// Publishing transfers the draft title into the published article.
    #[test]
    fn publishing_preserves_the_title() {
        let article = Article::draft("A type-safe transition").publish();

        assert_eq!(article.title(), "A type-safe transition");
    }
}
