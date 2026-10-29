// TODO Reviewed ??-??-2026.

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the closures example through its entry point.
    #[test]
    fn test_closures_example_entry_point() {
        main();
    }

    /// Only strings containing `@` are kept, in order.
    #[test]
    fn test_find_emails() {
        let emails = find_emails(vec![
            "example".to_owned(),
            "a@b.com".to_owned(),
            "no-at".to_owned(),
            "c@d.org".to_owned(),
        ]);
        assert_eq!(emails, vec!["a@b.com", "c@d.org"]);
        assert!(find_emails(vec![]).is_empty());
        assert!(find_emails(vec!["plain".to_owned()]).is_empty());
    }
}
