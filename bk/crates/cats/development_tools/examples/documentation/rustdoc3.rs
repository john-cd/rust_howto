#![allow(dead_code)]
// ANCHOR: example
//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue. When the abstraction is used
//! there are these advantages:
//! - Fast
//! - `[Easy]`
//!
//! [Easy]: http://thatwaseasy.example.com

fn main() {
    println!(
        "//! ... are `inner` comments that apply to the containing module (or crate)."
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
