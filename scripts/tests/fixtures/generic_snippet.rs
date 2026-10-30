// ANCHOR: example
//! Snippet with angle brackets, attributes and comments.

use std::collections::HashMap;

/// A <b>comment</b> with HTML-like text.
#[derive(Debug, Default)]
struct Wrapper<T: Clone> {
    // Generic types must stay visible as text, not become HTML tags.
    items: HashMap<String, Vec<Option<T>>>,
}

fn main() {
    let w: Wrapper<u8> = Wrapper::default();
    println!("{w:?}");
}
// ANCHOR_END: example
