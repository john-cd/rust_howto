// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Reject syntactically invalid and disposable addresses with [`mailchecker`].

/// Returns the addresses worth sending to.
pub fn deliverable<'a>(addresses: &[&'a str]) -> Vec<&'a str> {
    addresses
        .iter()
        .copied()
        .filter(|a| mailchecker::is_valid(a))
        .collect()
}

/// Prints which sample addresses are worth sending to.
fn main() {
    let candidates = ["a@gmail.com", "b@mailinator.com", "not an address"];
    println!("Deliverable: {:?}", deliverable(&candidates));
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailchecker_example_entry_point() {
        main();
    }

    #[test]
    fn filters_disposable_addresses() {
        assert!(!mailchecker::is_valid("user@mailinator.com"));
        assert!(mailchecker::is_valid("user@gmail.com"));
        assert!(!mailchecker::is_valid("not an address"));
        assert_eq!(
            deliverable(&["a@gmail.com", "b@mailinator.com"]),
            vec!["a@gmail.com"]
        );
    }
}
