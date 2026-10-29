//! Runs the C++ interoperability example.
//!
//! TODO Reviewed ??-??-2026.

mod cxx;

/// Runs the cpp example.
fn main() {
    cxx::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
