#![allow(dead_code)]
// ANCHOR: example
//! Lazily initializes shared global data with `std::sync::LazyLock`.

use std::collections::HashMap;
use std::sync::LazyLock;

/// A process-wide map initialized on its first access.
static CONFIG: LazyLock<HashMap<&str, &str>> =
    LazyLock::new(|| HashMap::from([("host", "localhost"), ("port", "8080")]));

/// Reads a value from the lazily initialized global map.
fn main() {
    assert_eq!(CONFIG.get("host"), Some(&"localhost"));
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `LazyLock` initializes shared global data on first access.
    #[test]
    fn initializes_shared_global_data() {
        main();
    }
}
