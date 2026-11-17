// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Matching literals.

/// Matches a number against literals.
fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    }
}
// ANCHOR_END: example

/// Matches a number against literals.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the literals example through its entry point.
    #[test]
    fn test_literals_example_entry_point() {
        main();
    }

    // Output-only example: `main` only prints values; there is nothing else to
    // check.
}
