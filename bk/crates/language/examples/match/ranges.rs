#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Matching ranges of values.

/// Matches a number against ranges.
fn main() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        6..10 => println!("six through nine"),
        _ => println!("something else"),
    }
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
