// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Placeholder for API bindings examples.

// Deferred (#1748): no API bindings example yet.

/// The notice printed while this category has no implemented example.
const PLACEHOLDER_NOTICE: &str = "No API bindings example is implemented yet.";

/// Prints a notice that this category still needs a bindings example.
fn main() {
    example();
}

/// Prints the placeholder notice for the API bindings category.
fn example() {
    println!("{PLACEHOLDER_NOTICE}");
}
// ANCHOR_END: example

pub fn run() {
    let entry_point = main;
    entry_point();
}

#[cfg(test)]
mod entry_point_tests {
    use super::*;

    #[test]
    fn private_main_runs() {
        let entry_point = main;
        entry_point();
    }
}
