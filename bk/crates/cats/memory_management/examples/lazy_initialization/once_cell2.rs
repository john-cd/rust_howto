#![allow(dead_code)]
// TODO Reviewed ??-??-2026

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
    println!("{:?}", GLOBAL_DATA.lock().unwrap());
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
