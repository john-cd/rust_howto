// TODO Reviewed ??-??-2026.

// ANCHOR: example
/// The notice printed while this category has no implemented example.
const PLACEHOLDER_NOTICE: &str = "No API bindings example is implemented yet.";

/// Prints the placeholder notice for the API bindings category.
fn example() {
    println!("{PLACEHOLDER_NOTICE}");
}

/// Prints a notice that this category still needs a bindings example.
fn main() {
    example();
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod entry_point_tests {
    use super::*;

    #[test]
    fn private_main_runs() {
        main();
    }
}
