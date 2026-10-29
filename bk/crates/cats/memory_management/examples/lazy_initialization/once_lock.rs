// ANCHOR: example
//! Initializes a shared value once with `std::sync::OnceLock`.

use std::sync::OnceLock;

/// A process-wide value initialized when first requested.
static GREETING: OnceLock<String> = OnceLock::new();

/// Initializes and reads the shared value.
fn main() {
    let greeting = GREETING.get_or_init(|| String::from("Hello, Rust!"));
    assert_eq!(greeting, "Hello, Rust!");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `OnceLock` initializes the shared value once.
    #[test]
    fn initializes_shared_value() {
        main();
    }

    /// A second initialization attempt is rejected.
    #[test]
    fn rejects_second_initialization() {
        let lock = OnceLock::new();
        assert!(lock.get().is_none());
        assert_eq!(lock.set(1), Ok(()));
        assert_eq!(lock.set(2), Err(2));
        assert_eq!(lock.get(), Some(&1));
    }
}
