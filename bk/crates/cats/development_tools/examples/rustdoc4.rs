// ANCHOR: example
//! Add a `Run` button to code examples embedded in your crate documentation.
//!
//! Place the following attribute at the top of your crate.
//! Note it is an _inner_ attribute that starts with `#!`.
//! Adjust the URL as needed. The official playground is <https://play.rust-lang.org/>
#![doc(html_playground_url = "https://playground.example.com/")]

/// Example of a code example embedded into doc comments:
///
/// ```rust,editable
/// println!("{}", return_a_string());
/// ```
fn return_a_string() -> String {
    "This will be printed.".to_string()
}

fn main() {
    println!("{}", return_a_string());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
