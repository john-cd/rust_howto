#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Using a closure to filter a list.

/// Finds all strings in a list that contain the '@' character.
fn find_emails(list: Vec<String>) -> Vec<String> {
    list.into_iter()
        // `filter` takes a predicate, a closure that returns a `bool`.
        .filter(|s| s.contains('@')) // Closure with a simple expression as its body. Note the `| |`.
        .collect()
}

/// Filters a list of strings with `find_emails`.
fn main() {
    for s in find_emails(vec![
        String::from("example"),
        String::from("example@example.com"),
    ]) {
        println!("{s}");
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
