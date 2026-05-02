#![allow(dead_code)]
// ANCHOR: example
//! Buildable placeholder example.
//!
//! Original placeholder comments are preserved below as comments.
fn main() {
    println!("Placeholder example.");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "later"]
    fn test() {
        main();
    }
    // // [write LATER](https://github.com/john-cd/rust_howto/issues/831)
}
