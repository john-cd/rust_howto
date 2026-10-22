#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Match guards, the `if` conditions of match arms.

/// Matches an optional number with guards.
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 0 => println!("Negative: {x}"),
        Some(x) if x % 2 == 0 => println!("Even: {x}"),
        Some(x) => println!("Odd positive: {x}"),
        None => (),
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
