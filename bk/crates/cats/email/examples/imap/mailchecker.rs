// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
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
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

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
