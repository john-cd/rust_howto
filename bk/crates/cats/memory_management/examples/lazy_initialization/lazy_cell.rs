// ANCHOR: example
//! Lazily initializes a single-threaded value with `std::cell::LazyCell`.
//!
//! Lazy initialization delays an expensive computation until the value is first
//! used, and then caches it. `LazyCell` is for a single thread; the thread-safe
//! version is `LazyLock`.

use std::cell::LazyCell;

/// Creates and reads a value initialized on first access.
fn main() {
    let message = LazyCell::new(|| String::from("initialized on first access"));
    assert_eq!(&*message, "initialized on first access");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `LazyCell` initializes its value on first access.
    #[test]
    fn initializes_on_first_access() {
        main();
    }

    /// The initializer runs only on first access, and only once.
    #[test]
    fn initializer_runs_once_and_lazily() {
        let calls = std::cell::Cell::new(0);
        let cell = LazyCell::new(|| {
            calls.set(calls.get() + 1);
            7
        });
        assert_eq!(calls.get(), 0);
        assert_eq!(*cell, 7);
        assert_eq!(*cell, 7);
        assert_eq!(calls.get(), 1);
    }
}
