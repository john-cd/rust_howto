//! Runs the Flutter interoperability example.
//!
//! TODO Reviewed ??-??-2026.

mod flutter_rust_bridge;

/// Runs the flutter example.
fn main() {
    flutter_rust_bridge::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
