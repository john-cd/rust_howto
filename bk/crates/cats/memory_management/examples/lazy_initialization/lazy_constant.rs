// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates a lazily initialized global map with `std::sync::LazyLock`.
//!
//! A `static` must be built at compile time, which rules out maps and other
//! heap data. `LazyLock` builds the value on first access instead, and it is
//! safe to use from several threads.

use std::collections::HashMap;
use std::sync::LazyLock;

/// Permissions keyed by user name, initialized on first access.
static PRIVILEGES: LazyLock<HashMap<&'static str, Vec<&'static str>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    });

// Example function: display the access privileges for a given name.
/// Prints the access level of the user `name`.
fn show_access(name: &str) {
    let access = PRIVILEGES.get(name);
    println!("{name}: {access:?}");
}

/// Looks up users in a lazily initialized global map.
fn main() {
    show_access("James");
    show_access("Jim");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the lazy global map contains the configured users.
    #[test]
    fn lazy_map_contains_configured_users() {
        assert_eq!(PRIVILEGES.get("James"), Some(&vec!["user", "admin"]));
        assert_eq!(PRIVILEGES.get("Jim"), Some(&vec!["user"]));
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }

    /// Unknown users have no privileges.
    #[test]
    fn unknown_user_has_no_access() {
        assert_eq!(PRIVILEGES.get("Nobody"), None);
        assert_eq!(PRIVILEGES.len(), 2);
    }
}
