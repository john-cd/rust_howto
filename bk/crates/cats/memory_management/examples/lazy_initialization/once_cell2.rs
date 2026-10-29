// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Lazily initializes a shared value with the `once_cell` crate.

use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

/// A global, lazily initialized `HashMap` wrapped in a `Mutex`.
///
/// Must be `static`, not `const`.
static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_owned());
    m.insert(74, "Hoyten".to_owned());
    Mutex::new(m)
});

/// Prints the lazily initialized global data.
fn main() {
    println!(
        "{:?}",
        GLOBAL_DATA.lock().expect("mutex should not be poisoned")
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the once cell2 example through its entry point.
    #[test]
    fn test_once_cell2_example_entry_point() {
        main();
    }

    /// The global map holds the seeded entries and can be updated.
    #[test]
    fn global_map_is_seeded_and_mutable() {
        let mut data =
            GLOBAL_DATA.lock().expect("mutex should not be poisoned");
        assert_eq!(data.get(&13).map(String::as_str), Some("Spica"));
        assert_eq!(data.get(&74).map(String::as_str), Some("Hoyten"));
        data.insert(99, "Test".to_owned());
        assert_eq!(data.get(&99).map(String::as_str), Some("Test"));
    }
}
