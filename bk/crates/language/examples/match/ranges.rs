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

    // Open-ended ranges have only a lower or upper bound.
    match x {
        5.. => println!("five or greater"),
        _ => println!("less than five"),
    }

    // Floating-point bounds are also allowed in range patterns.
    let temperature = 1.5_f32;
    match temperature {
        0.0..=2.0 => println!("within the range"),
        _ => println!("outside the range"),
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
