// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Snippet with angle brackets, attributes and comments.

use std::collections::HashMap;

/// A <b>comment</b> with HTML-like text.
#[derive(Debug, Default)]
struct Wrapper<T: Clone> {
    // Generic types must stay visible as text, not become HTML tags.

    /// Collection consumed by wrapper while preserving its ordering and relationships.
    items: HashMap<String, Vec<Option<T>>>,
}

/// Prints the wrapper values used by the generic snippet parser fixture.
fn main() {
    let w: Wrapper<u8> = Wrapper::default();
    println!("{w:?}");
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
pub fn run() {
    main();
}
