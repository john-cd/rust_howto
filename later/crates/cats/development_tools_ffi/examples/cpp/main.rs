//! Runs the C++ interoperability example.
//!
//! TODO Reviewed ??-??-2026.

/// Calling C++ from Rust with the `cxx` crate.
mod cxx;
/// Runs the example.
/// Runs the cpp example.

fn main() {
    cxx::run();
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
