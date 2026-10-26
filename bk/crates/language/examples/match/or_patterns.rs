#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Matching one of several patterns with `|`.

/// Matches a number against alternatives.
fn main() {
    let x = 3;
    match x {
        1 | 2 => println!("one or two"),
        3 | 4 => println!("three or four"),
        _ => println!("something else"),
    }
}

// ANCHOR_END: example

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
