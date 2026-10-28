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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the literals example through its entry point.
    #[test]
    fn test_literals_example_entry_point() {
        main();
    }
}
