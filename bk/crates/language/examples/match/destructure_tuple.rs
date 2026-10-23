#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Destructuring a tuple in a `match`.

/// Matches a pair against several patterns.
fn main() {
    let pair = (0, -5);
    match pair {
        (0, y) => println!("First element is 0, y = {y}."),
        (x, 0) => println!("Second element is 0, x = {x}."),
        _ => println!("It doesn't matter what they are."),
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
