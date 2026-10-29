//! Runs the case-conversion example.

mod case_conversion;

/// Runs the case-conversion example.
fn main() {
    case_conversion::run();
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
